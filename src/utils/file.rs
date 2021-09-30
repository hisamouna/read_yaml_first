use serde_yaml::{Mapping};
use std::io::{BufReader};
use std::fs::File;
use anyhow::{Result};

pub fn read_yaml(file_name: String) -> Result<Mapping> {
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);
    let d :Mapping = serde_yaml::from_reader(reader)?;

    Ok(d)
}


