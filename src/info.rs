use crate::error::InfoError;
use goblin::{
    elf::sym,
    elf::Elf,
    elf64::header::{EM_386, EM_AARCH64, EM_ARM, EM_X86_64},
    mach::{
        cputype::{CPU_TYPE_ARM64, CPU_TYPE_X86_64},
        Mach,
    },
    pe::header::{COFF_MACHINE_ARM64, COFF_MACHINE_X86, COFF_MACHINE_X86_64},
    Object as Obj,
};
use wasm_bindgen::prelude::*;

const RUST_PERSONALITY: &str = "rust_eh_personality";
const GO_SECTION: &str = ".note.go.buildid";

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Runtime {
    Go,
    Rust,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Platform {
    Win32,
    Darwin,
    Linux,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Arch {
    X86,
    Amd64,
    Arm,
    Arm64,
}

#[wasm_bindgen]
pub struct BinaryInfo {
    pub platform: Platform,
    pub arch: Arch,
    pub runtime: Option<Runtime>,
}

pub fn get_runtime_from_elf(elf: Elf) -> Result<Option<Runtime>, goblin::error::Error> {
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
            if let Some(sym_name) = elf.strtab.get_at(s.st_name) {
                if sym_name == RUST_PERSONALITY {
                    return Ok(Some(Runtime::Rust));
                }
            }
        }
    }

    Ok(None)
}

// Implementation initially based on timfish/binary-info
// https://github.com/timfish/binary-info/blob/v0.0.3/LICENSE
pub fn get_info(buffer: &[u8]) -> Result<BinaryInfo, InfoError> {
    match Obj::parse(&buffer)? {
        Obj::Elf(elf) => {
            let arch = match elf.header.e_machine {
                EM_AARCH64 => Arch::Arm64,
                EM_X86_64 => Arch::Amd64,
                EM_ARM => Arch::Arm,
                EM_386 => Arch::X86,
                _ => return Err(InfoError::new("Unknown architecture")),
            };

            let runtime = get_runtime_from_elf(elf)?;

            Ok(BinaryInfo {
                platform: Platform::Linux,
                arch,
                runtime,
            })
        }
        Obj::PE(pe) => {
            let arch = match pe.header.coff_header.machine {
                COFF_MACHINE_ARM64 => Arch::Arm64,
                COFF_MACHINE_X86 => Arch::X86,
                COFF_MACHINE_X86_64 => Arch::Amd64,
                _ => return Err(InfoError::new("Unknown architecture")),
            };

            Ok(BinaryInfo {
                platform: Platform::Win32,
                arch: arch,
                runtime: None,
            })
        }
        Obj::Mach(mach) => match mach {
            Mach::Fat(_) => return Err(InfoError::new("Unsupported binary")),
            Mach::Binary(mach_o) => {
                let arch = match mach_o.header.cputype() {
                    CPU_TYPE_X86_64 => Arch::Amd64,
                    CPU_TYPE_ARM64 => Arch::Arm64,
                    _ => return Err(InfoError::new("Unknown architecture")),
                };

                Ok(BinaryInfo {
                    platform: Platform::Darwin,
                    arch: arch,
                    runtime: None,
                })
            }
        },
        _ => Err(InfoError::new("Not a binary")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_go_runtime_darwin_amd64() {
        let buffer =
            std::fs::read("tests/data/darwin/go-amd64").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Amd64, info.arch);
        assert_eq!(Platform::Darwin, info.platform);
        assert!(info.runtime.is_none())
    }

    #[test]
    fn test_detect_go_runtime_darwin_arm64() {
        let buffer =
            std::fs::read("tests/data/darwin/go-arm64").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Arm64, info.arch);
        assert_eq!(Platform::Darwin, info.platform);
        assert!(info.runtime.is_none())
    }

    #[test]
    fn test_detect_go_runtime_linux_x86() {
        let buffer = std::fs::read("tests/data/linux/go-x86").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::X86, info.arch);
        assert_eq!(Platform::Linux, info.platform);
        assert_eq!(Runtime::Go, info.runtime.unwrap());
    }

    #[test]
    fn test_detect_go_runtime_linux_amd64() {
        let buffer =
            std::fs::read("tests/data/linux/go-amd64").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Amd64, info.arch);
        assert_eq!(Platform::Linux, info.platform);
        assert_eq!(Runtime::Go, info.runtime.unwrap());
    }

    #[test]
    fn test_detect_go_runtime_linux_arm() {
        let buffer = std::fs::read("tests/data/linux/go-arm").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Arm, info.arch);
        assert_eq!(Platform::Linux, info.platform);
        assert_eq!(Runtime::Go, info.runtime.unwrap());
    }

    #[test]
    fn test_detect_go_runtime_linux_arm64() {
        let buffer =
            std::fs::read("tests/data/linux/go-arm64").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Arm64, info.arch);
        assert_eq!(Platform::Linux, info.platform);
        assert_eq!(Runtime::Go, info.runtime.unwrap());
    }

    #[test]
    fn test_detect_rust_runtime_darwin_amd64() {
        let buffer =
            std::fs::read("tests/data/darwin/rust-amd64").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Amd64, info.arch);
        assert_eq!(Platform::Darwin, info.platform);
        assert!(info.runtime.is_none())
    }

    #[test]
    fn test_detect_rust_runtime_linux_amd64() {
        let buffer =
            std::fs::read("tests/data/linux/rust-amd64").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Amd64, info.arch);
        assert_eq!(Platform::Linux, info.platform);
        assert_eq!(Runtime::Rust, info.runtime.unwrap());
    }

    #[test]
    fn test_detect_go_runtime_windows_amd64() {
        let buffer =
            std::fs::read("tests/data/windows/go-amd64.exe").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::Amd64, info.arch);
        assert_eq!(Platform::Win32, info.platform);
        assert!(info.runtime.is_none())
    }

    #[test]
    fn test_detect_go_runtime_windows_x86() {
        let buffer =
            std::fs::read("tests/data/windows/go-x86.exe").expect("failed to load binary file");

        let info = get_info(&buffer).expect("failed to detect runtime");
        assert_eq!(Arch::X86, info.arch);
        assert_eq!(Platform::Win32, info.platform);
        assert!(info.runtime.is_none())
    }

    #[test]
    #[should_panic]
    fn test_detect_ignores_invalid_file() {
        let buffer = std::fs::read("tests/data/text").expect("failed to load binary file");

        get_info(&buffer).expect("failed to detect runtime");
    }
}
