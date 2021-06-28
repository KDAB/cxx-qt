// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use quote::ToTokens;
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
        BuildMode::Plain
    }
}

// TODO: we need to eventually support having multiple modules defined in a single file. This
// is currently an issue because we are using the Rust file name to derive the cpp file name
// and are blindly re-writing files.
//
// As we use struct names for the QObject files, we should actually be able to support multiple
// QObject macros and at most one "raw CXX" macro per file already. For now this remains a TODO
// as to keep things simpler. We also want to able to warn users about duplicate names eventually.

/// Retrieve the list of rust sources from the file
fn read_rs_sources() -> Vec<String> {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    let path = format!("{}/target/cxx-qt-gen/rust_sources.txt", dir_manifest);

    let contents = std::fs::read_to_string(path).expect("Could not read list of Rust source files");
    contents.split(';').map(|s| s.to_string()).collect()
}

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
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let h_path = format!(
        "{}/target/cxx-qt-gen/include/{}.h",
        dir_manifest, snake_name
    );
    let cpp_path = format!("{}/target/cxx-qt-gen/src/{}.cpp", dir_manifest, snake_name);

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
) -> Vec<String> {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    let mut generated_cpp_paths = Vec::new();

    let path = format!("{}/src/{}", dir_manifest, rs_path);
    let content = std::fs::read_to_string(path).expect("Could not read Rust file");
    let extracted = extract_modules(&content, rs_path);

    let h_path;
    let cpp_path;

    let tokens = {
        match extracted {
            ExtractedModule::Cxx(m) => {
                h_path = format!("{}/target/cxx-qt-gen/include/{}.h", dir_manifest, rs_path);
                cpp_path = format!("{}/target/cxx-qt-gen/src/{}.cpp", dir_manifest, rs_path);

                m.into_token_stream()
            }
            ExtractedModule::CxxQt(m) => {
                let qobject = extract_qobject(m).unwrap();
                let cpp_object = generate_qobject_cpp(&qobject).unwrap();
                let snake_name = qobject.ident.to_string().to_case(Case::Snake);

                // If there is a QQmlExtensionPlugin then add our QObject type to it
                if let Some(ext_plugin) = ext_plugin {
                    ext_plugin.push_type(&qobject);
                }

                h_path = format!("{}/target/cxx-qt-gen/src/{}.rs.h", dir_manifest, snake_name);
                cpp_path = format!(
                    "{}/target/cxx-qt-gen/src/{}.rs.cpp",
                    dir_manifest, snake_name
                );

                generated_cpp_paths.append(&mut write_qobject_cpp_files(cpp_object, &snake_name));
                generate_qobject_cxx(&qobject).unwrap()
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
    rs_source: &[String],
    ext_plugin: &mut Option<&mut QQmlExtensionPluginData>,
) -> Vec<String> {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let path = format!("{}/target/cxx-qt-gen/include", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt include dir");

    let path = format!("{}/target/cxx-qt-gen/src", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create cxx-qt src dir");

    let mut cpp_files = Vec::new();

    for rs_path in rs_source {
        cpp_files.append(&mut gen_cxx_for_file(rs_path, ext_plugin));
    }

    cpp_files
}

/// Write the list of C++ paths to the file
fn write_cpp_sources_list(paths: &[String]) {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let path = format!("{}/target/cxx-qt-gen", dir_manifest);
    std::fs::create_dir_all(path).expect("Could not create target dir");

    let path = format!("{}/target/cxx-qt-gen/cpp_sources.txt", dir_manifest);
    let mut file = File::create(&path).expect("Could not create cpp_sources file");

    for path in paths {
        writeln!(file, "{}", path).unwrap();
    }
}

/// Write out the qmldir and plugin.cpp for a QQmlExtensionPlugin with the given data
fn write_qqmlextensionplugin(ext_plugin: Option<QQmlExtensionPluginData>) -> Vec<String> {
    let mut paths = vec![];

    if let Some(ext_plugin) = ext_plugin {
        let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

        // Ensure that a plugin folder exists
        // We put qqmlextensionplugin data in it's own folder so we can assume filenames
        let path = format!("{}/target/cxx-qt-gen/plugin", dir_manifest);
        std::fs::create_dir_all(path).expect("Could not create cxx-qt plugin dir");

        // Generate the qqmlextensionplugin and qmldir
        let plugin_source = ext_plugin.gen_qqmlextensionplugin();
        let qmldir_source = ext_plugin.gen_qmldir();

        // We can assume plugin.cpp here because we are writing to our own directory
        let cpp_path = format!("{}/target/cxx-qt-gen/plugin/plugin.cpp", dir_manifest);
        let mut plugin = File::create(&cpp_path).expect("Could not create cpp file");
        write!(plugin, "{}", plugin_source).expect("Could not write cpp file");
        paths.push(cpp_path);

        let qmldir_path = format!("{}/target/cxx-qt-gen/plugin/qmldir", dir_manifest);
        let mut qmldir = File::create(&qmldir_path).expect("Could not create qmldir file");
        write!(qmldir, "{}", qmldir_source).expect("Could not write qmldir file");
    }

    paths
}

/// Write out the static header files for both the cxx and cxx-qt libraries
fn write_static_headers() {
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");

    let path = format!("{}/target/cxx-qt-gen/statics/rust", dir_manifest);
    std::fs::create_dir_all(&path).expect("Could not create static header dir");

    let h_path = format!("{}/cxx.h", path);
    let mut header = File::create(h_path).expect("Could not create cxx.h");
    write!(header, "{}", cxx_gen::HEADER).expect("Could not write cxx.h");

    let h_path = format!("{}/cxx_qt.h", path);
    let mut header = File::create(h_path).expect("Could not create cxx_qt.h");
    write!(header, "{}", cxx_qt_gen::HEADER).expect("Could not write cxx_qt.h");
}

/// Describes a cxx Qt builder which helps parse and generate sources for cxx-qt
#[derive(Default)]
pub struct CxxQtBuilder {
    build_mode: BuildMode,
    cpp_format: Option<ClangFormatStyle>,
}

impl CxxQtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            build_mode: BuildMode::Plain,
            cpp_format: None,
        }
    }

    /// Create a new builder as a QQmlExtensionPlugin
    pub fn qqqmlextensionplugin(
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

    /// Perform the build task, for example parsing and generating sources
    pub fn build(self) {
        // Set clang-format format
        if generate_format(self.cpp_format).is_err() {
            panic!("Failed to set clang-format.");
        }

        // Read sources
        let rs_sources = read_rs_sources();

        // Prepare a QQmlExtensionPlugin if the build mode is set
        let mut ext_plugin = match self.build_mode {
            BuildMode::QQmlExtensionPlugin {
                module_ident,
                cpp_plugin_name,
            } => Some(QQmlExtensionPluginData::new(module_ident, cpp_plugin_name)),
            _others => None,
        };

        // Generate files
        let mut cpp_paths = gen_cxx_for_files(&rs_sources, &mut ext_plugin.as_mut());

        // Write any qqmlextensionplugin if there is one and read any C++ files it creates
        cpp_paths.append(&mut write_qqmlextensionplugin(ext_plugin));

        // TODO: find a way to only do this when cargo is called during the config stage of CMake
        write_cpp_sources_list(&cpp_paths);
        write_static_headers();
    }
}
