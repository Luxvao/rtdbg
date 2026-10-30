use rhai::{CustomType, Dynamic, TypeBuilder};

use crate::error::Error;

#[derive(Debug, Clone, Default, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Vma {
    #[rhai_type(get = Self::get_saddy, set = Self::set_saddy)]
    pub saddy: u64,
    #[rhai_type(get = Self::get_eaddy, set = Self::set_eaddy)]
    pub eaddy: u64,
    pub permissions: Permissions,
    #[rhai_type(get = Self::get_offset, set = Self::set_offset)]
    pub offset: u64,
    pub device: String,
    #[rhai_type(get = Self::get_inode, set = Self::set_inode)]
    pub inode: u64,
    #[rhai_type(skip)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub executable: bool,
    pub private: bool,
}

impl Vma {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_get("path", Self::get_path)
            .with_set("path", Self::set_path)
            .on_print(|vma| format!("{vma:?}"));
    }

    fn get_saddy(&self) -> i64 {
        self.saddy as i64
    }

    fn get_eaddy(&self) -> i64 {
        self.eaddy as i64
    }

    fn get_offset(&self) -> i64 {
        self.offset as i64
    }

    fn get_inode(&self) -> i64 {
        self.inode as i64
    }

    fn get_path(&mut self) -> Dynamic {
        self.path
            .as_deref()
            .map_or_else(|| Dynamic::UNIT, |s| s.into())
    }

    fn set_saddy(&mut self, value: i64) {
        self.saddy = value as u64;
    }

    fn set_eaddy(&mut self, value: i64) {
        self.eaddy = value as u64;
    }

    fn set_offset(&mut self, value: i64) {
        self.offset = value as u64;
    }

    fn set_inode(&mut self, value: i64) {
        self.inode = value as u64;
    }

    fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }

    pub fn from_str(str: &str) -> Result<Vec<Vma>, Error> {
        let mut vmas = Vec::new();

        for line in str.lines() {
            let mut parts = line.split(' ');

            // Extract the fields
            let range = parts.next().ok_or(Error::VmaError(str.to_string()))?;
            let permissions = parts.next().ok_or(Error::VmaError(str.to_string()))?;
            let offset = parts.next().ok_or(Error::VmaError(str.to_string()))?;
            let device = parts.next().ok_or(Error::VmaError(str.to_string()))?;
            let inode = parts.next().ok_or(Error::VmaError(str.to_string()))?;
            let path = parts.next_back().filter(|path| !path.is_empty());

            // Split the range into saddy and eaddy
            let mut range_parts = range.split('-');

            let saddy_str = range_parts.next().ok_or(Error::VmaError(str.to_string()))?;
            let eaddy_str = range_parts.next().ok_or(Error::VmaError(str.to_string()))?;

            let saddy = u64::from_str_radix(saddy_str, 16)?;
            let eaddy = u64::from_str_radix(eaddy_str, 16)?;

            // Parse permissions
            let permissions = Permissions::try_from(permissions)?;

            // Parse offset
            let offset = u64::from_str_radix(offset, 16)?;

            // Parse inode
            let inode = inode.parse::<u64>()?;

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

        Ok(vmas)
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
