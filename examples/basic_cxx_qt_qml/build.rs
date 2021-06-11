use clang_format::ClangFormatStyle;
use cxx_qt_build::CxxQtBuilder;

fn main() {
    // TODO: Further options for building will go here similar to cpp_format
    // eg if you want a QQmlExtensionModule etc
    CxxQtBuilder::new()
        .cpp_format(ClangFormatStyle::Mozilla)
        .build();
}
