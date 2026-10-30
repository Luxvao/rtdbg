use std::{fs::File, io::Read};

use rhai::{Array, CustomType, Dynamic, EvalAltResult, TypeBuilder};

use crate::{error::Error, os::linux::vma::Vma, rhai_utils::parse_array};

#[derive(Debug, Clone, Default, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct LinuxProcess {
    #[rhai_type(get = Self::get_pid, set = Self::set_pid)]
    pub pid: u32,
    #[rhai_type(get = Self::get_vmas, set = Self::set_vmas)]
    pub vmas: Vec<Vma>,
    pub path: String,
}

impl LinuxProcess {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.on_print(|process| format!("{process:?}"));
    }

    fn get_pid(&self) -> i64 {
        self.pid as i64
    }

    fn set_pid(&mut self, pid: i64) {
        self.pid = pid as u32;
    }

    fn get_vmas(&self) -> Array {
        self.vmas.iter().map(|v| Dynamic::from(v.clone())).collect()
    }

    fn set_vmas(&mut self, vmas: Array) -> Result<(), Box<EvalAltResult>> {
        self.vmas = parse_array(vmas).ok_or("expected Array of Vma")?;

        Ok(())
    }

    pub fn this() -> Result<LinuxProcess, Error> {
        let pid = std::process::id();

        Self::from_pid(pid)
    }

    pub fn from_pid(pid: u32) -> Result<LinuxProcess, Error> {
        let maps_file_path = format!("/proc/{pid}/maps");

        let mut maps_file = File::open(maps_file_path)?;

        let mut maps_file_contents = String::new();

        maps_file.read_to_string(&mut maps_file_contents)?;

        let vmas = Vma::from_str(&maps_file_contents)?;

        let path = std::fs::canonicalize(format!("/proc/{pid}/exe"))?
            .to_string_lossy()
            .to_string();

        Ok(LinuxProcess { pid, vmas, path })
    }
}
