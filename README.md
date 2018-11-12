# Minimal template of gtk-rs app with Windows support

## How to build

### Dependencies

1. [vcpkg](https://github.com/Microsoft/vcpkg) with installed `gtk` package
2. [cargo-make](https://github.com/sagiegurari/cargo-make)
3. Rust Stable >= 1.30.1 (might work on older releases, but it's untested ATM)

### Build process

1. Navigate to project folder
2. Run build for desired configuration like: `cargo make build-win --env-file=.\environments\win64-release --env VCPKG_PATH=c:\path\to\vcpkg`

### TODO

1. Make `package` job which would copy necessary release artifacts from `target/`
2. Make sure that *nix builds are not broken 
3. Make sure GLADE support works as intended
4. Find a way to use `gtksourceview` and `webview` on Windows with minimal effort on compiling dependencies