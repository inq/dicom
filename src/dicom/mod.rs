mod reader;
mod data;
mod tag;

use self::reader::Reader;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Dicom {
    pub syntax: String        
}

impl Dicom {
    pub fn new(file_name: String) -> Result<Dicom, String> {
        let mut reader = try!(Reader::new(file_name));
        let meta = try!(reader.read_metadata());
        try!(reader.read_content());
        
        Ok(Dicom { syntax: meta["00020010"].to_string()})
    }
}
