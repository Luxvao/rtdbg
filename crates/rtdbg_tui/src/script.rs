use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

pub fn load_from_disk(path: &str) -> std::io::Result<(String, String)> {
    let path = PathBuf::from_str(path).unwrap();

    let mut file = File::open(&path)?;

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok((
        path.file_name()
            .ok_or(std::io::Error::other("Not a file"))?
            .to_string_lossy()
            .into_owned(),
        buffer.replace("\t", "    "),
    ))
}
