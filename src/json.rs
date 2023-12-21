use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_from_file<P: AsRef<Path>, M: DeserializeOwned>(path: P) -> Result<M, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let m = serde_json::from_reader(reader)?;
    Ok(m)
}
