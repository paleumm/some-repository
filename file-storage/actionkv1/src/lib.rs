use std::collections::HashMap;
use std::fs::File;
type ByteString = Vec<u8>;

#[derive(Debug)]
pub struct ActionKV {
  f: File,
  pub index: HashMap<ByteString, u64>,
}