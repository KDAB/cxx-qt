# cxx-qt

cxx-qt is a library that automatically generates code to transfer data between Rust and C++ through common interfaces
such as QObjects that can be exposed directly into QML. It relies on the cxx crate internally to achieve this and thus
it is recommended that any interactions with Qt that are not covered by the built-in code generators should be done
directly in C++ and connected to relevant Rust logic by writing additional cxx code. The cxx-qt build system is based
on CMake, but is compatible with cxx on its own as well.

The root folder contains an example application using the cxx-qt crate and will be used for development and testing
purposes. The cxx-qt folder contains the source for the actual crate which contains a proc-macro. At some point we
will need to create another crate for the pre-parser which likely needs to reuse a lot of code from the proc-macro
so it might then make sense to split that common code out into yet another crate.

Initially the project in the root folder will also serve as a template for new projects should use cxx-qt.
In future we might improve upon this with a custom CMake module for instance.
