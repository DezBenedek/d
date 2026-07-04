# d
The ultimate CLI for MacOS

commands:
gh release create v0.2.0 target/release/d d-installer.pkg --notes "v0.2.0 added push"
cargo build --release
./build-pkg.sh
