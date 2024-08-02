export RUSTFLAGS="-Cinstrument-coverage"
cargo build --package cxx-qt-gen
export LLVM_PROFILE_FILE="coverage/coverage_data-%p-%m.profraw"
cargo test --package cxx-qt-gen
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/