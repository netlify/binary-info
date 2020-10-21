mod elf;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Runtime {
    Go,
    Rust,
}

impl From<elf::Runtime> for Runtime {
    fn from(rt: elf::Runtime) -> Self {
        match rt {
            elf::Runtime::Go => Runtime::Go,
            elf::Runtime::Rust => Runtime::Rust,
        }
    }
}

#[wasm_bindgen(catch)]
pub fn detect(data: &[u8]) -> Result<Option<Runtime>, JsValue> {
    set_panic_hook();
    elf::detect(data)
        .map_err(|e| JsValue::from(format!("error reading elf metadata: {}", e)))
        .map(|o| o.map(Runtime::from))
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
