// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::{fragment::CppFragmentPair, GeneratedCppBlocks};
use indoc::formatdoc;

/// Extract the source from a given CppFragmentPair
fn pair_as_source(pair: &CppFragmentPair) -> String {
    pair.source.clone()
}

/// For a given GeneratedCppBlocks write this into a C++ source
pub fn write_cpp_source(generated: &GeneratedCppBlocks) -> String {
    formatdoc! {r#"
        #include "cxx-qt-gen/include/{cxx_stem}.cxxqt.h"

        namespace {namespace} {{

        {ident}::{ident}(QObject* parent)
          : QObject(parent)
          , m_rustObj(createRs())
        {{
          initialiseCpp(*this);
          m_initialised = true;
        }}

        {ident}::~{ident}() = default;

        const {rust_ident}&
        {ident}::unsafe_rust() const
        {{
          return *m_rustObj;
        }}

        {rust_ident}&
        {ident}::unsafe_rust_mut()
        {{
          return *m_rustObj;
        }}

        {methods}
        {slots}
        std::unique_ptr<CppObj>
        newCppObject()
        {{
          return std::make_unique<CppObj>();
        }}

        }} // namespace {namespace}
    "#,
    cxx_stem = generated.cxx_stem,
    ident = generated.ident,
    namespace = generated.namespace,
    rust_ident = generated.rust_ident,
    methods = generated.methods.iter().map(pair_as_source).collect::<Vec<String>>().join("\n"),
    slots = generated.slots.iter().map(pair_as_source).collect::<Vec<String>>().join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::writer::cpp::tests::{create_generated_cpp, expected_source};
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_write_cpp_source() {
        let generated = create_generated_cpp();
        let output = write_cpp_source(&generated);
        assert_str_eq!(output, expected_source());
    }
}
