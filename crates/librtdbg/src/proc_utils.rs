use std::{fs::File, io::Read, ops::Deref, path::PathBuf};

use rhai::{CustomType, Dynamic, TypeBuilder};

use crate::error::Error;

#[derive(Debug, Clone, Default, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Process {
    #[rhai_type(skip)]
    pub pid: u32,
    #[rhai_type(readonly)]
    pub vmas: Vmas,
    #[rhai_type(readonly)]
    pub path: String,
}

#[derive(Debug, Clone, Default, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Vma {
    #[rhai_type(skip)]
    pub saddy: usize,
    #[rhai_type(skip)]
    pub eaddy: usize,
    #[rhai_type(readonly)]
    pub permissions: Permissions,
    #[rhai_type(skip)]
    pub offset: usize,
    #[rhai_type(readonly)]
    pub device: String,
    #[rhai_type(skip)]
    pub inode: usize,
    #[rhai_type(readonly)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Vmas {
    #[rhai_type(skip)]
    vmas: Vec<Vma>,
}

#[derive(Debug, Clone, Copy, Default, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Permissions {
    #[rhai_type(readonly)]
    pub read: bool,
    #[rhai_type(readonly)]
    pub write: bool,
    #[rhai_type(readonly)]
    pub executable: bool,
    #[rhai_type(readonly)]
    pub private: bool,
}

impl Process {
    pub fn this() -> Result<Process, Error> {
        let pid = std::process::id();

        let vmas = Vmas::this()?;

        let path = std::fs::canonicalize("/proc/self/exe")?
            .to_string_lossy()
            .to_string();

        Ok(Process { pid, vmas, path })
    }

    pub fn from_pid(pid: u32) -> Result<Process, Error> {
        let maps_file = PathBuf::from(format!("/proc/{pid}/maps"));

        let vmas = Vmas::try_from(maps_file)?;

        let path = std::fs::canonicalize(format!("/proc/{pid}/exe"))?
            .to_string_lossy()
            .to_string();

        Ok(Process { pid, vmas, path })
    }

    fn get_pid(&mut self) -> i64 {
        i64::from(self.pid)
    }

    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_get("pid", Self::get_pid)
            .on_print(|process| format!("{process:?}"));
    }
}

impl Vma {
    fn get_saddy(&mut self) -> i64 {
        self.saddy as i64
    }

    fn get_eaddy(&mut self) -> i64 {
        self.eaddy as i64
    }

    fn get_offset(&mut self) -> i64 {
        self.offset as i64
    }

    fn get_inode(&mut self) -> i64 {
        self.inode as i64
    }

    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_get("saddy", Self::get_saddy)
            .with_get("eaddy", Self::get_eaddy)
            .with_get("offset", Self::get_offset)
            .with_get("inode", Self::get_inode)
            .on_print(|vma| format!("{vma:?}"));
    }
}

impl TryFrom<PathBuf> for Vmas {
    type Error = crate::error::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut maps_file = File::open(value)?;

        let mut maps_file_contents = String::new();

        maps_file.read_to_string(&mut maps_file_contents)?;

        Vmas::try_from(maps_file_contents)
    }
}

impl TryFrom<&str> for Vmas {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut vmas = Vec::new();

        for line in value.lines() {
            let mut parts = line.split(' ');

            // Extract the fields
            let range = parts.next().ok_or(Error::VmaError(value.to_string()))?;
            let permissions = parts.next().ok_or(Error::VmaError(value.to_string()))?;
            let offset = parts.next().ok_or(Error::VmaError(value.to_string()))?;
            let device = parts.next().ok_or(Error::VmaError(value.to_string()))?;
            let inode = parts.next().ok_or(Error::VmaError(value.to_string()))?;
            let path = parts.next_back().filter(|path| !path.is_empty());

            // Split the range into saddy and eaddy
            let mut range_parts = range.split('-');

            let saddy_str = range_parts
                .next()
                .ok_or(Error::VmaError(value.to_string()))?;
            let eaddy_str = range_parts
                .next()
                .ok_or(Error::VmaError(value.to_string()))?;

            let saddy = usize::from_str_radix(saddy_str, 16)?;
            let eaddy = usize::from_str_radix(eaddy_str, 16)?;

            // Parse permissions
            let permissions = Permissions::try_from(permissions)?;

            // Parse offset
            let offset = usize::from_str_radix(offset, 16)?;

            // Parse inode
            let inode = inode.parse::<usize>()?;

            vmas.push(Vma {
                saddy,
                eaddy,
                permissions,
                offset,
                device: device.to_string(),
                inode,
                path: path.map(std::string::ToString::to_string),
            });
        }

        Ok(Vmas { vmas })
    }
}

impl TryFrom<String> for Vmas {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Deref for Vmas {
    type Target = [Vma];

    fn deref(&self) -> &Self::Target {
        &self.vmas
    }
}

impl IntoIterator for Vmas {
    type Item = Vma;

    type IntoIter = std::vec::IntoIter<Vma>;

    fn into_iter(self) -> Self::IntoIter {
        self.vmas.into_iter()
    }
}

impl Vmas {
    pub fn this() -> Result<Vmas, Error> {
        let path = PathBuf::from("/proc/self/maps");

        Vmas::try_from(path)
    }

    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.is_iterable().on_print(|vmas| format!("{vmas:?}"));
    }
}

impl TryFrom<&str> for Permissions {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();

        let read = chars
            .next()
            .ok_or(Error::PermissionsError(value.to_string()))?
            == 'r';

        let write = chars
            .next()
            .ok_or(Error::PermissionsError(value.to_string()))?
            == 'w';

        let executable = chars
            .next()
            .ok_or(Error::PermissionsError(value.to_string()))?
            == 'x';

        let private = chars
            .next()
            .ok_or(Error::PermissionsError(value.to_string()))?
            == 'p';

        Ok(Permissions {
            read,
            write,
            executable,
            private,
        })
    }
}

impl TryFrom<String> for Permissions {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Permissions {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.on_print(|permissions| format!("{permissions:?}"));
    }
}
