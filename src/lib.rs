use goblin::{elf::sym, elf::Elf};

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const RUST_PERSONALITY: &str = "rust_eh_personality";
const GO_SECTION: &str = ".note.go.buildid";

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub enum Runtime {
    Go,
    Rust,
}

#[wasm_bindgen(catch)]
pub fn detect(data: &[u8]) -> Result<Option<Runtime>, JsValue> {
    utils::set_panic_hook();

    let elf = match Elf::parse(data) {
        Ok(elf) => elf,
        _ => return Ok(None),
    };

    for s in elf
        .shdr_strtab
        .to_vec()
        .map_err(|e| format!("error reading symbols: {}", e))?
    {
        if s == GO_SECTION {
            return Ok(Some(Runtime::Go));
        }
    }

    for s in elf
        .strtab
        .to_vec()
        .map_err(|e| format!("error reading sections: {}", e))?
    {
        if s == RUST_PERSONALITY {
            return Ok(Some(Runtime::Rust));
        }
    }

    for s in elf.syms.iter() {
        if s.is_function() && s.st_bind() == sym::STB_GLOBAL {
            if let Some(Ok(sym_name)) = elf.strtab.get(s.st_name) {
                if sym_name == RUST_PERSONALITY {
                    return Ok(Some(Runtime::Rust));
                }
            }
        }
    }

    Ok(None)
}
