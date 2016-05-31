mod reader;
mod data;
mod tag;

use self::reader::Reader;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Dicom {
    file_name: &'static str

}

impl Dicom {
    pub fn new(file_name: &'static str) -> Result<Dicom, String> {
        let mut reader = try!(Reader::new(file_name));
        let meta = try!(reader.read_metadata());
        
        println!("{:?}", meta["0210"].to_string());

        Ok(Dicom { file_name: file_name })
    }
}
