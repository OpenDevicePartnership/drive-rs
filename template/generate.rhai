// Post-generation hook.
//
// Compile the device-driver manifest (`device.ddsl`) into `src/registers.rs`
// using the device-driver compiler. This keeps the proc-macro out of the generated
// crate (faster builds for your users) while the manifest stays the single
// source of truth.
//
// Requires `ddc` on PATH (`cargo install device-driver-cli`).
let interfaces = variable::get("interfaces");

if interfaces.contains("i2c") || interfaces.contains("spi") || interfaces.contains("uart") {
    system::command(
        "ddc",
        [
            "build",
            "rust",
            "-s",
            "device.ddsl",
            "-o",
            "src/registers.rs",
            "--rust-defmt-feature=defmt",
        ],
    );
}
