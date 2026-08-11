// Pre-generation hook.
//
// `interfaces` is a multi-select array placeholder, which means the user can
// technically select nothing. A driver with no interface is not something this
// template can scaffold, so refuse early with a clear message instead of
// emitting a crate that does not compile.
let interfaces = variable::get("interfaces");

if interfaces.is_empty() {
    abort("Select at least one interface (gpio, i2c, spi, and/or uart).");
}
