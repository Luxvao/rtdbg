use std::{fs::File, io::Read, path::PathBuf};

use crate::error::unwrap_or_shutdown;

#[derive(Debug, Clone, Default)]
pub struct Process {
    pub pid: u32,
    pub vmas: Vec<Vma>,
}

#[derive(Debug, Clone, Default)]
pub struct Vma {
    pub saddy: usize,
    pub eaddy: usize,
    pub permissions: Permissions,
    pub offset: usize,
    pub device: String,
    pub inode: usize,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub executable: bool,
    pub private: bool,
}

impl From<String> for Permissions {
    fn from(value: String) -> Self {
        let mut chars = value.chars();

        let mut r = false;
        let mut w = false;
        let mut x = false;
        let mut p = false;

        if unwrap_or_shutdown(chars.next().ok_or("Read flag not present")) == 'r' {
            r = true;
        };

        if unwrap_or_shutdown(chars.next().ok_or("Write flag not present")) == 'w' {
            w = true;
        }

        if unwrap_or_shutdown(chars.next().ok_or("Executable flag not present")) == 'x' {
            x = true;
        }

        if unwrap_or_shutdown(chars.next().ok_or("Private flag not present")) == 'p' {
            p = true;
        }

        Permissions {
            read: r,
            write: w,
            executable: x,
            private: p,
        }
    }
}

impl From<&str> for Permissions {
    fn from(value: &str) -> Self {
        let value = value.to_string();

        Permissions::from(value)
    }
}

impl Process {
    pub fn this() -> Process {
        let pid = std::process::id();

        let vmas = Vma::this();

        Process { pid, vmas }
    }

    pub fn from_pid(pid: u32) -> Process {
        let vmas = Vma::from_maps(PathBuf::from(format!("/proc/{pid}/maps")));

        Process { pid, vmas }
    }
}

impl Vma {
    pub fn this() -> Vec<Vma> {
        let path = PathBuf::from("/proc/self/maps");

        Self::from_maps(path)
    }

    pub fn from_maps(maps: PathBuf) -> Vec<Vma> {
        let mut maps_buffer = String::new();

        let mut maps_file = unwrap_or_shutdown(File::open(maps));

        unwrap_or_shutdown(maps_file.read_to_string(&mut maps_buffer));

        let mut vmas = Vec::new();

        for line in maps_buffer.lines() {
            let mut parts = line.split(' ');

            // Extract all the fields
            let range = unwrap_or_shutdown(parts.next().ok_or("No range field"));
            let permissions = unwrap_or_shutdown(parts.next().ok_or("No permissions field"));
            let offset = unwrap_or_shutdown(parts.next().ok_or("No offset field"));
            let device = unwrap_or_shutdown(parts.next().ok_or("No device offset field"));
            let inode = unwrap_or_shutdown(parts.next().ok_or("No inode field"));
            let path = parts.next();

            // Split the range into saddy and eaddy
            let mut range_parts = range.split('-');

            let saddy_str = unwrap_or_shutdown(range_parts.next().ok_or("No saddy field"));
            let eaddy_str = unwrap_or_shutdown(range_parts.next().ok_or("No eaddy field"));

            let saddy = unwrap_or_shutdown(saddy_str.parse::<usize>());
            let eaddy = unwrap_or_shutdown(eaddy_str.parse::<usize>());

            // Parse permissions
            let permissions = Permissions::from(permissions);

            // Parse offset
            let offset = unwrap_or_shutdown(offset.parse::<usize>());

            // Parse inode
            let inode = unwrap_or_shutdown(inode.parse::<usize>());

            let vma = Vma {
                saddy,
                eaddy,
                permissions,
                offset,
                device: device.to_string(),
                inode,
                path: path.map(|s| s.to_string()),
            };

            vmas.push(vma);
        }

        vmas
    }
}
