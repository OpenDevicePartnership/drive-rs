// Post-generation hook.
//
// Compile the device-driver manifest (`device.yaml`) into `src/registers.rs`
// using the device-driver CLI. This keeps the proc-macro out of the generated
// crate (faster builds for your users) while the manifest stays the single
// source of truth.
//
// Requires `device-driver-cli` on PATH (`cargo install device-driver-cli`).
let interfaces = variable::get("interfaces");

if interfaces.contains("i2c") || interfaces.contains("spi") {
    let device = to_pascal_case(variable::get("project-name")) + "Registers";
    system::command(
        "device-driver-cli",
        ["-m", "device.yaml", "-o", "src/registers.rs", "-d", device],
    );
}
