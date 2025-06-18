use std::{fs::File, io::Read, path::PathBuf};

use crate::error::unwrap_or_shutdown;

#[derive(Debug, Clone, Default)]
pub struct Process {
    pub pid: i32,
    pub maps: Vec<Vma>,
}

#[derive(Debug, Clone, Default)]
pub struct Vma {
    pub saddy: usize,
    pub eaddy: usize,
    pub permissions: Permissions,
    pub offset: usize,
    pub device: String,
    pub inode: usize,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub executable: bool,
    pub private: bool,
}

impl Process {
    pub fn this() -> Process {
        let vmas = Vma::this();
        todo!()
    }

    pub fn from_pid(pid: usize) -> Process {
        let vmas = Vma::from_maps(PathBuf::from(format!("/proc/{pid}/maps")));
        todo!()
    }
}

impl Vma {
    pub fn this() -> Vec<Vma> {
        let path = PathBuf::from("/proc/self/maps");

        let mut maps_buffer = String::new();

        let mut maps_file = unwrap_or_shutdown(File::open(path));

        unwrap_or_shutdown(maps_file.read_to_string(&mut maps_buffer));

        todo!()
    }

    pub fn from_maps(maps: PathBuf) -> Vec<Vma> {
        todo!()
    }
}
