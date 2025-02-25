cargo clean
cargo generate-lockfile
cp Cargo.lock $(rustc --print sysroot)/lib/rustlib/src/rust/Cargo.lock
cargo xbuild --release --target=./aarch64-unknown-none.json --verbose

