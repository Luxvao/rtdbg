use crate::error::Error;

// Script related apis
#[derive(Debug, Clone)]
pub struct Script {
    contents: String,
}

impl TryFrom<Vec<u8>> for Script {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Script {
            contents: String::from_utf8(value)?,
        })
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::new()
    }
}

impl Script {
    // Create an empty script
    pub fn new() -> Script {
        Script {
            contents: String::new(),
        }
    }

    // Extract the script from this
    pub fn get_contents(&self) -> &String {
        &self.contents
    }
}
