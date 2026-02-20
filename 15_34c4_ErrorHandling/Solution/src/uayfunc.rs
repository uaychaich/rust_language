pub fn uayfunc1(birthyear: u16) -> u16 {
    if birthyear > 2024 {
        panic!("Birth year cannot be in the future");
    } else {
        2024 - birthyear
    }
}

pub fn uayfunc2(birthyear: u16) -> Result<u16, String> {
    if birthyear > 2024 {
        Err("Birth year cannot be in the future".to_string())
    } else {
        Ok(2024 - birthyear)
    }
}