# Minimal template of gtk-rs app with Windows support

## How to build

### Dependencies

1. [vcpkg](https://github.com/Microsoft/vcpkg) with installed `gtk` package
2. [cargo-make](https://github.com/sagiegurari/cargo-make)
3. Rust Stable >= 1.30.1 (might work on older releases, but it's untested ATM)

### Build process

1. Navigate to project folder
2. Run build for desired configuration like: `cargo make build-win --env-file=.\environments\win64-release --env VCPKG_PATH=c:\path\to\vcpkg`  