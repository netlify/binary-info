use goblin::{elf::sym, elf::Elf, error::Error};

const RUST_PERSONALITY: &str = "rust_eh_personality";
const GO_SECTION: &str = ".note.go.buildid";

#[derive(Debug, PartialEq)]
pub enum Runtime {
    Go,
    Rust,
}

pub fn detect(data: &[u8]) -> Result<Option<Runtime>, Error> {
    let elf = match Elf::parse(data) {
        Ok(elf) => elf,
        _ => return Ok(None),
    };

    for s in elf.shdr_strtab.to_vec()? {
        if s == GO_SECTION {
            return Ok(Some(Runtime::Go));
        }
    }

    for s in elf.strtab.to_vec()? {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_go_runtime() {
        let buffer =
            std::fs::read("tests/data/hello-world-go").expect("failed to load binary file");

        let runtime = detect(&buffer)
            .expect("failed to detect runtime")
            .expect("failed to return some runtime");
        assert_eq!(Runtime::Go, runtime);
    }

    #[test]
    fn test_detect_rust_runtime() {
        let buffer =
            std::fs::read("tests/data/hello-world-rs").expect("failed to load binary file");

        let runtime = detect(&buffer)
            .expect("failed to detect runtime")
            .expect("failed to return some runtime");
        assert_eq!(Runtime::Rust, runtime);
    }

    #[test]
    fn test_detect_ignores_invalid_file() {
        let buffer =
            std::fs::read("tests/data/hello-world-text").expect("failed to load binary file");

        let runtime = detect(&buffer).expect("failed to detect runtime");
        assert_eq!(None, runtime);
    }
}
