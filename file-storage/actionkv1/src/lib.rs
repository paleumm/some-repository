use std::{collections::HashMap, io};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
  pub key: ByteString,
  pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
  f: File,
  pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new().read(true).write(true).create(true).append(true).open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f: f, index: index })
    }

    pub fn load(&mut self)
}