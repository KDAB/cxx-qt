<!--
SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Matt Aber <matt.aber@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Building for WebAssembly

CXX-Qt and applications written with it can be compiled for WebAssembly, with a few limitations. Below you will find detailed instructions regarding how to build for the WASM target.

You will need to have Qt for WebAssembly installed. The next section shows versions that have been tested.

Additionally, if you haven't already, clone the `emsdk` git repo from [here](https://github.com/emscripten-core/emsdk).

## Using Correct Versions

The version of Emscripten used to build CXX-Qt and programs using it should match the one that was used to build Qt for WebAssembly. This is because Emscripten does not guarantee ABI compatibility between versions, so using different versions is not guaranteed to work fully or even at all.

Here are the associated Qt and Emscripten versions, and whether they are currently working with CXX-Qt for WebAssembly:

Qt|Emscripten
-|-
6.2|2.0.14
6.3|3.0.0
6.4|3.1.14
6.5|3.1.25
6.6|3.1.37

Info about other Qt and emscripten versions can be found in the [Qt documentation](https://doc.qt.io/qt-6/wasm.html).

## Setting Up `emsdk`

Once you know which Qt and Emscripten versions you will use, navigate to the root directory of the `emsdk` repo and run the following commands:

```bash
$ ./emsdk install <emscripten version>
$ ./emsdk activate <emscripten version>
$ source ./emsdk_env.sh
```

For example, if you are going to use Qt 6.4, the corresponding version of Emscripten is 3.1.14, so the first command will be:

```bash
$ ./emsdk install 3.1.14
```

On Windows, the third step, which sets up environment variables (`source` command above on Unix-like environments) is unnecessary because the required environment setup will already be done.

## Toolchains

When configuring with CMake, the `CMAKE_TOOLCHAIN_FILE` variable needs to be set to the correct toolchain file; for example, if using Qt 6.4.2 on WebAssembly, the toolchain file is typically located at `/path/to/Qt/6.4.2/wasm_32/lib/cmake/Qt6/qt.toolchain.cmake`. This will set CMake up to use the correct Qt path, compiler, linker, and so forth.

Generally, this does not need to be done manually. Using the `qt-cmake` binary bundled with your selected version of Qt WASM will set the toolchain file for you.

For example, if using Qt 6.4.2:

```bash
$ /path/to/Qt/6.4.2/wasm_32/bin/qt-cmake -B build .
```

However, in Qt 6.3 and below, the bundled CMake is version 3.22, while CXX-Qt requires at least version 3.24. For these versions of Qt, a more up-to-date CMake binary needs to be used to configure, so `CMAKE_TOOLCHAIN_FILE` needs to be passed into the `cmake` command.

If using a different CMake binary, instead do this:

```bash
$ cmake -DCMAKE_TOOLCHAIN_FILE=/path/to/qt.toolchain.cmake -B build .
```

For Qt 6.5 or later, use the `wasm_singlethread` toolchain. For versions earlier than 6.5 use `wasm_32`.

The `wasm_multithread` toolchain available in 6.5 and later is currently not supported. For more information, see the [Known Issues](#known-issues) section at the bottom of this page.

## Compiling Your Project for WebAssembly

To build for WebAssembly in a project that uses CXX-Qt crates, first follow the instructions in the [Using Correct Versions](#using-correct-versions) and [Setting Up `emsdk`](#setting-up-emsdk) sections.

### CMakeLists.txt

When compiling a CXX-Qt project for wasm, the Rust target must be set to `wasm32-unknown-emscripten`, and the project must be configured to use POSIX threads. Make sure you have the Emscripten target for `rustc` with `rustup target add wasm32-unknown-emscripten`.

```cmake
set(Rust_CARGO_TARGET wasm32-unknown-emscripten)
set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)
```

Using CMake, `add_executable` will not output an HTML file when targeting wasm. In order to render an HTML file, one must use `qt_add_executable` in its place. Assuming a project has a CMake flag `BUILD_WASM` to toggle wasm and native builds, one could write the following:

```cmake
if(BUILD_WASM)
    qt_add_executable(${APP_NAME} ${SOURCE_FILES})
else()
    add_executable(${APP_NAME} ${SOURCE_FILES})
endif()
```

### Configure, Build, and Run

Configure your build directory following the instructions in the [Toolchains](#toolchains) section.

Now, run `cmake --build` on the build directory to compile and link the project. This can be any CMake binary; here the OS package works just fine:

```bash
$ cmake --build build
```

You can then run your built application like so:

```bash
$ emrun ./build/path/to/<appname>.html
```

## Compiling CXX-Qt WASM from Source

If you are compiling CXX-Qt from source, the workflow is similar. First, follow the instructions in the [Using Correct Versions](#using-correct-versions) and [Setting Up `emsdk`](#setting-up-emsdk) sections.

The `CMakeLists.txt` file at the root of the CXX-Qt repository has an option `BUILD_WASM` to toggle WebAssembly builds. Simply compiling with the correct emsdk and toolchain and flipping this option `ON` should build the libraries and examples for WebAssembly.

### Building

Read the [Toolchains](#toolchains) section before proceeding. Then navigate to the root directory of the CXX-Qt repo.

If using the `qt-cmake` binary packaged with your version of Qt for WebAssembly, run the following command to configure CXX-Qt:

```bash
$ /path/to/qt-cmake -DBUILD_WASM=ON -B build .
```

If using a different CMake binary, instead do this:

```bash
$ <cmake binary> -DCMAKE_TOOLCHAIN_FILE=/path/to/qt.toolchain.cmake -DBUILD_WASM=ON -B build .
```

Finally, run `cmake --build` on the configured build directory to compile and link the project and examples. This can be any CMake binary; here the OS package works just fine:

```bash
$ cmake --build build
```

Then you can run the `qml_minimal` example like so:

```bash
$ emrun ./build/examples/qml_minimal/example_qml_minimal.html
```

### Working Examples

Not all of the examples are currently supported for WASM builds.

Example|Working
-|-
`qml-minimal-no-cmake`|❌ broken
`demo_threading`|❌ broken
`qml_features`|✅ working
`qml_minimal`|✅ working

For more information, see the [Known Issues](#known-issues) section at the bottom of this page.

## Known Issues

### `wasm_multithread` toolchain

CXX-Qt will currently not build with `wasm_multithread` versions of Qt.

```console
wasm-ld: error: --shared-memory is disallowed by qml_minimal-e6f36338b0e1fa5c.17g6vcid2nczsjj0.rcgu.o 
            because it was not compiled with 'atomics' or 'bulk-memory' features.
```

This issue is related to `pthread` in the `libc` crate. It is possible that manually compiling `cxx` and `libc` crates with `-pthread` may solve this.

### `cargo`-only builds

The example `qml-minimal-no-cmake` will not build for WebAssembly with `cargo`, and attempts to build with `cargo` without `cmake` will not work. This is due to an upstream issue with the `libc` crate, which does not support wasm and can cause breakage.

```console
cannot find function `pthread_kill` in crate `libc`
```

### `demo_threading` example

The example `demo_threading` will not build for WebAssembly due to an upstream issue with `async-std`, which does not support wasm. On Linux, the observed breakage is due to `socket2` using its `unix.rs` file to target a Unix rather than wasm environment, resulting in error messages from `unix.rs` like the following:

```console
error[E0433]: failed to resolve: use of undeclared type `IovLen`
```

`socket2` is a dependency of `async-io`, which is a dependency of `async-std`.

There is discussion around supporting wasm in the GitHub repository for `async-std`, and the progress is being tracked [here](https://github.com/async-rs/async-std/issues/220).

### Cannot use throw / try catch with exceptions disabled

CXX and CXX-Qt can generate code that attempts to use exceptions but these are disabled
for most WASM platforms.

The cmake-qt-cmake builder will attempt to set a define of `RUST_CXX_NO_EXCEPTIONS` which should
causes the code not to be compiled. But if this doesn't work try adding `-DRUST_CXX_NO_EXCEPTIONS`
to a `CXXFLAGS` enviornment variable while building.
