// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use syn::*;

use clang_format::ClangFormatStyle;
use cxx_qt_gen::{
    extract_qobject, generate_format, generate_qobject_cpp, generate_qobject_cxx, CppObject,
    QQmlExtensionPluginData,
};

/// The type of build to perform on the sources
#[derive(PartialEq)]
enum BuildMode {
    /// Generate a normal build
    Plain,
    /// Generate qmldir and QQmlExtensionPlugin with the parameters
    QQmlExtensionPlugin {
        module_ident: &'static str,
        cpp_plugin_name: &'static str,
    },
}

impl Default for BuildMode {
    fn default() -> Self {
        // Default to a normal build, users need to opt-in for a QQmlExtensionPlugin build
        BuildMode::Plain
    }
}

/// Representation of a generated CXX header, source, and name
#[derive(Serialize, Deserialize)]
struct GeneratedType {
    header: String,
    name: String,
    source: String,
}

// TODO: we need to eventually support having multiple modules defined in a single file. This
// is currently an issue because we are using the Rust file name to derive the cpp file name
// and are blindly re-writing files.
//
// As we use struct names for the QObject files, we should actually be able to support multiple
// QObject macros and at most one "raw CXX" macro per file already. For now this remains a TODO
// as to keep things simpler. We also want to able to warn users about duplicate names eventually.

/// Tests if an attributes matched what is expected for #[cxx::bridge]
fn is_cxx_attr(attr: &Attribute) -> bool {
    let segments = &attr.path.segments;

    if segments.len() != 2 {
        return false;
    }

    if segments[0].ident != "cxx" {
        return false;
    }

    if segments[1].ident != "bridge" {
        return false;
    }

    true
}

/// Tests if an attributes matched what is expected for #[make_qobject]
fn is_cxx_qt_attr(attr: &Attribute) -> bool {
    let segments = &attr.path.segments;

    if segments.len() != 1 {
        return false;
    }

    if segments[0].ident != "make_qobject" {
        return false;
    }

    true
}

/// Represents the cxx or cxx_qt module that could be extracted from a file
#[derive(PartialEq)]
enum ExtractedModule {
    Cxx(ItemMod),
    CxxQt(ItemMod),
    None,
}

fn manifest_dir() -> String {
    let mut manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    // CARGO_MANIFEST_DIR uses \ path separators on Windows, but the format! macros that
    // use the return value of this function use / separators. CMake's `file(STRINGS)` command
    // which is used to load list of generated file paths fails if \ and / path separators
    // are mixed. CMake also fails when using all \ separators because it treats them as escaping
    // the character after the \
    if cfg!(windows) {
        manifest_dir = manifest_dir.replace('\\', "/");
    }
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
    manifest_dir
}

fn out_dir() -> String {
    let mut out_dir = env::var("OUT_DIR").expect("Could not get out dir");
    if cfg!(windows) {
        out_dir = out_dir.replace('\\', "/");
    }
    out_dir += "/../../../..";
    println!("cargo:rerun-if-env-changed=OUT_DIR");
    out_dir
}

/// Extract the cxx or cxx_qt module from a Rust file
fn extract_modules(file_content: &str, rs_path: &str) -> ExtractedModule {
    let file = syn::parse_file(file_content).unwrap();

    // Define a helper function that will ensure that we can extract at most one
    // module for code gen from the items in the file. This function also
    // ensures that the extracted item is wrapped in the correct enum.
    let mut extracted = ExtractedModule::None;
    let mut push_module = |i: ItemMod, qt: bool| {
        if extracted != ExtractedModule::None {
            panic!(
                "Unfortunately only files with either a single cxx or a single cxx_qt module are currently supported.
                The file {} has more than one of these.",
                rs_path);
        }

        if qt {
            extracted = ExtractedModule::CxxQt(i);
        } else {
            extracted = ExtractedModule::Cxx(i);
        }
    };

    // We loop through all the items in the module searching for any that we can
    // generate code from. We do not break out of the loop so that we can detect
    // if the users placed multiple such modules in a single file and give them a
    // warning.
    for i in file.items {
        if let Item::Mod(m) = i {
            let attrs = &m.attrs;
            match attrs.len() {
                0 => continue,
                1 => {}
                _others => panic!("Multiple module attributes are currently not supported."),
            }

            let attr = &attrs[0];
            if is_cxx_attr(attr) {
                push_module(m, false);
            } else if is_cxx_qt_attr(attr) {
                push_module(m, true);
            }
        }
    }

    extracted
}

/// Write the generated cpp and h files for a qobject out to files
fn write_qobject_cpp_files(obj: CppObject, snake_name: &str) -> Vec<String> {
    let out_dir = out_dir();

    let h_path = format!("{}/cxx-qt-gen/include/{}.h", out_dir, snake_name);
    let cpp_path = format!("{}/cxx-qt-gen/src/{}.cpp", out_dir, snake_name);

    let mut file = File::create(&h_path).expect("Could not create .h file");
    write!(file, "{}", obj.header).expect("Failed to write .h file");

    let mut file = File::create(&cpp_path).expect("Could not create .cpp file");
    write!(file, "{}", obj.source).expect("Failed to write .cpp file");

    vec![h_path, cpp_path]
}

/// Generate C++ files from a given Rust file, returning the generated paths
fn gen_cxx_for_file(
    rs_path: &str,
    ext_plugin: &mut Option<&mut QQmlExtensionPluginData>,
    cpp_namespace_prefix: &[&'static str],
) -> Vec<String> {
    let manifest_dir = manifest_dir();
    let out_dir = out_dir();
    let mut generated_cpp_paths = Vec::new();

    // TODO: in the future use the module path as the file path
    // so that src/moda/lib.rs with mod modb { make_qobject(MyObject) } becomes src/moda/modb/my_object
    // this then avoids collisions later.
    //
    // This will require detecting nested modules in a file

    let path = format!("{}/{}", manifest_dir, rs_path);
    println!("cargo:rerun-if-changed={}", path);
    let content = std::fs::read_to_string(path).expect("Could not read Rust file");
    let extracted = extract_modules(&content, rs_path);

    let h_path;
    let cpp_path;

    let tokens = {
        match extracted {
            ExtractedModule::Cxx(m) => {
                // Extract just the file name of the rs_path as we don't want to include sub folders
                //
                // TODO: later this won't be required when we are tracking the module path
                let rs_file_name = {
                    if let Some(os_file_name) = std::path::Path::new(rs_path).file_name() {
                        if let Some(file_name) = os_file_name.to_str() {
                            file_name
                        } else {
                            panic!(
                                "Could not convert OsStr to str for rust source path: {}",
                                rs_path
                            );
                        }
                    } else {
                        panic!("No file name found in rust source path: {}", rs_path)
                    }
                };
                h_path = format!("{}/cxx-qt-gen/include/{}.h", out_dir, rs_file_name);
                cpp_path = format!("{}/cxx-qt-gen/src/{}.cpp", out_dir, rs_file_name);

                m.into_token_stream()
            }
            ExtractedModule::CxxQt(m) => {
                let qobject = extract_qobject(m, cpp_namespace_prefix).unwrap();
                let cpp_object = generate_qobject_cpp(&qobject).unwrap();
                let snake_name = qobject.ident.to_string().to_case(Case::Snake);

                // If there is a QQmlExtensionPlugin then add our QObject type to it
                if let Some(ext_plugin) = ext_plugin {
                    ext_plugin.push_type(&qobject);
                }

                h_path = format!("{}/cxx-qt-gen/src/{}.rs.h", out_dir, snake_name);
                cpp_path = format!("{}/cxx-qt-gen/src/{}.rs.cpp", out_dir, snake_name);

                generated_cpp_paths.append(&mut write_qobject_cpp_files(cpp_object, &snake_name));
                generate_qobject_cxx(&qobject, cpp_namespace_prefix).unwrap()
            }
            _others => panic!(
                "No module to generate cxx code from could be found in {}",
                rs_path
            ),
        }
    };

    let opt = cxx_gen::Opt::default();
    let gen_result = cxx_gen::generate_header_and_cc(tokens, &opt)
        .expect("Could not generate C++ from Rust file");

    let mut header = File::create(&h_path).expect("Could not create header file");
    header
        .write_all(&gen_result.header)
        .expect("Could not write header file");

    let mut cpp = File::create(&cpp_path).expect("Could not create cpp file");
    cpp.write_all(&gen_result.implementation)
        .expect("Could not write cpp file");

    // TODO: find a "nice" way to write this
    generated_cpp_paths.push(h_path);
    generated_cpp_paths.push(cpp_path);
    generated_cpp_paths
}

/// Generate C++ files from a given list of Rust files, returning the generated paths
fn gen_cxx_for_files(
    rs_source: &[&'static str],
    ext_plugin: &mut Option<&mut QQmlExtensionPluginData>,
    cpp_namespace_prefix: &[&'static str],
) -> Vec<String> {
    let out_dir = out_dir();
    let path = format!("{}/cxx-qt-gen/include", out_dir);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt include dir");

    let path = format!("{}/cxx-qt-gen/src", out_dir);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt src dir");

    let mut cpp_files = Vec::new();

    for rs_path in rs_source {
        cpp_files.append(&mut gen_cxx_for_file(
            rs_path,
            ext_plugin,
            cpp_namespace_prefix,
        ));
    }

    cpp_files
}

fn write_cpp_namespace_prefix(cpp_namespace_prefix: &[&'static str]) {
    let out_dir = out_dir();

    std::fs::create_dir_all(format!("{}/cxx-qt-gen", out_dir)).expect("Could not create target dir");

    let path = format!("{}/cxx-qt-gen/cpp_namespace_prefix.txt", out_dir);
    let mut file = File::create(&path).expect("Could not create cpp_namespace_prefix file");
    write!(file, "{}", cpp_namespace_prefix.join("::"))
        .expect("Could not write cpp_namespace_prefix file");
}

/// Write the list of C++ paths to the file
fn write_cpp_sources_list(paths: &[String]) {
    let out_dir = out_dir();

    let path = format!("{}/cxx-qt-gen", out_dir);
    std::fs::create_dir_all(path).expect("Could not create target dir");

    let path = format!("{}/cxx-qt-gen/cpp_sources.txt", out_dir);
    let mut file = File::create(&path).expect("Could not create cpp_sources file");

    for path in paths {
        writeln!(file, "{}", path).unwrap();
    }
}

/// Write out the qmldir and plugin.cpp for a QQmlExtensionPlugin with the given data
fn write_qqmlextensionplugin(ext_plugin: Option<QQmlExtensionPluginData>) -> Vec<String> {
    let mut paths = vec![];

    if let Some(ext_plugin) = ext_plugin {
        let out_dir = out_dir();

        // Ensure that a plugin folder exists
        // We put qqmlextensionplugin data in it's own folder so we can assume filenames
        let path = format!("{}/cxx-qt-gen/plugin", out_dir);
        std::fs::create_dir_all(path).expect("Could not create cxx-qt plugin dir");

        // Generate the qqmlextensionplugin and qmldir
        let plugin_source = ext_plugin.gen_qqmlextensionplugin();
        let qmldir_source = ext_plugin.gen_qmldir();

        // We can assume plugin.cpp here because we are writing to our own directory
        let cpp_path = format!("{}/cxx-qt-gen/plugin/plugin.cpp", out_dir);
        let mut plugin = File::create(&cpp_path).expect("Could not create cpp file");
        write!(plugin, "{}", plugin_source).expect("Could not write cpp file");
        paths.push(cpp_path);

        let qmldir_path = format!("{}/cxx-qt-gen/plugin/qmldir", out_dir);
        let mut qmldir = File::create(&qmldir_path).expect("Could not create qmldir file");
        write!(qmldir, "{}", qmldir_source).expect("Could not write qmldir file");
    }

    paths
}

/// Write our a given cxx-qt-lib header and source set to the given folder
fn write_cxx_qt_lib_set(
    file_name: &str,
    target_dir: &str,
    header: &str,
    source: &str,
) -> Vec<String> {
    let mut paths = vec![];
    let path_h = format!("{}/include/{}.h", target_dir, file_name);
    let path_cpp = format!("{}/src/{}.cpp", target_dir, file_name);

    let mut file = std::fs::File::create(&path_h).expect("Could not create header file");
    file.write_all(header.as_bytes())
        .expect("Could not write header file");
    paths.push(path_h);

    let mut file = std::fs::File::create(&path_cpp).expect("Could not create source file");
    file.write_all(source.as_bytes())
        .expect("Could not write source file");
    paths.push(path_cpp);

    paths
}

/// Find all the cxx-qt-lib sources and write them to the target directory
fn write_cxx_qt_lib_sources() -> Vec<String> {
    let cxx_qt_lib_target_dir = format!("{}/cxx-qt-lib", out_dir());
    let cxx_qt_lib_include_dir = format!("{}/include", cxx_qt_lib_target_dir);
    let cxx_qt_lib_src_dir = format!("{}/src", cxx_qt_lib_target_dir);
    std::fs::create_dir_all(&cxx_qt_lib_include_dir).unwrap();
    std::fs::create_dir_all(&cxx_qt_lib_src_dir).unwrap();

    let mut paths = vec![];
    // Add the hand written qt_types file
    paths.append(&mut write_cxx_qt_lib_set(
        "qt_types",
        &cxx_qt_lib_target_dir,
        cxx_qt_lib::QT_TYPES_HEADER,
        cxx_qt_lib::QT_TYPES_SOURCE,
    ));
    // Add the generated CXX files
    let generated: Vec<GeneratedType> =
        serde_json::from_str(cxx_qt_lib::QT_TYPES_CXX_JSON).unwrap();
    for gen in generated {
        paths.append(&mut write_cxx_qt_lib_set(
            &gen.name,
            &cxx_qt_lib_target_dir,
            &gen.header,
            &gen.source,
        ));
    }

    paths
}

/// Write out the static header file for both the cxx
fn write_cxx_static_header() {
    // let manifest_dir = manifest_dir();
    let out_dir = out_dir();

    let path = format!("{}/cxx-qt-gen/statics/rust", out_dir);
    std::fs::create_dir_all(&path).expect("Could not create static header dir");

    let h_path = format!("{}/cxx.h", path);
    let mut header = File::create(&h_path).expect("Could not create cxx.h");
    write!(header, "{}", cxx_gen::HEADER).expect("Could not write cxx.h");
}

/// Describes a cxx Qt builder which helps parse and generate sources for cxx-qt
#[derive(Default)]
pub struct CxxQtBuilder {
    build_mode: BuildMode,
    cpp_format: Option<ClangFormatStyle>,
    cpp_namespace_prefix: Vec<&'static str>,
    rust_sources: Vec<&'static str>,
    qt_enabled: bool,
}

impl CxxQtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            build_mode: BuildMode::Plain,
            cpp_format: None,
            cpp_namespace_prefix: vec!["cxx_qt"],
            rust_sources: vec![],
            qt_enabled: true,
        }
    }

    /// Create a new builder as a QQmlExtensionPlugin
    pub fn qqmlextensionplugin(
        mut self,
        module_ident: &'static str,
        cpp_plugin_name: &'static str,
    ) -> Self {
        self.build_mode = BuildMode::QQmlExtensionPlugin {
            module_ident,
            cpp_plugin_name,
        };
        self
    }

    /// Choose the ClangFormatStyle to use for generated C++ files
    pub fn cpp_format(mut self, format: ClangFormatStyle) -> Self {
        self.cpp_format = Some(format);
        self
    }

    /// Choose the C++ namespace prefix that generated objects should be created inside
    ///
    /// Defaults to `cxx_qt`.
    pub fn cpp_namespace_prefix(mut self, namespace: Vec<&'static str>) -> Self {
        self.cpp_namespace_prefix = namespace;
        self
    }

    /// Choose to disable Qt support
    ///
    /// This will disable including cxx-qt-lib headers and prevent qqmlextensionplugin from being built.
    pub fn disable_qt(mut self) -> Self {
        self.qt_enabled = false;
        self
    }

    /// Specify rust file paths to parse through the cxx-qt marco
    ///
    /// Currently the path should be relative to OUT_DIR
    pub fn file(mut self, rust_source: &'static str) -> Self {
        self.rust_sources.push(rust_source);
        self
    }

    // TODO: support globs with files("src/**/*.rs")

    /// Perform the build task, for example parsing and generating sources
    pub fn build(self) {
        // Set clang-format format
        if generate_format(self.cpp_format).is_err() {
            panic!("Failed to set clang-format.");
        }

        // Set the cpp namespace prefix to a file
        //
        // This is so that the make_qobject macro can read this back later
        write_cpp_namespace_prefix(&self.cpp_namespace_prefix);

        // TODO: somewhere check that we don't have duplicate class names
        // TODO: later use the module::object to turn into module/object.h and namespace

        // Prepare a QQmlExtensionPlugin if the build mode is set
        let mut ext_plugin = match self.build_mode {
            BuildMode::QQmlExtensionPlugin {
                module_ident,
                cpp_plugin_name,
            } => Some(QQmlExtensionPluginData::new(module_ident, cpp_plugin_name)),
            _others => None,
        };

        // Generate files
        let mut cpp_paths = gen_cxx_for_files(
            &self.rust_sources,
            &mut ext_plugin.as_mut(),
            &self.cpp_namespace_prefix,
        );

        // TODO: in large projects where where CXX-Qt is used in multiple individual
        // components that end up being linked together, having these same static
        // files in each one could cause issues.
        write_cxx_static_header();

        // Check if we have Qt support enabled
        if self.qt_enabled {
            // Write any qqmlextensionplugin if there is one and read any C++ files it creates
            cpp_paths.append(&mut write_qqmlextensionplugin(ext_plugin));

            // Write the cxx-qt-lib sources into the folder
            cpp_paths.append(&mut write_cxx_qt_lib_sources());
        }

        // TODO: find a way to only do this when cargo is called during the config stage of CMake
        write_cpp_sources_list(&cpp_paths);
    }
}
