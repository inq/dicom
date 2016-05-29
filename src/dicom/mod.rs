mod reader;

use self::reader::Reader;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::str;

#[derive(Debug)]
pub struct Dicom {
    file_name: &'static str
}

impl Dicom {
    pub fn new(file_name: &'static str) -> Result<Dicom, String> {
        let mut reader = try!(Reader::new(file_name));
        reader.seek(128);        
        assert_eq!(try!(reader.read_str(4)), "DICM");

        Ok(Dicom { file_name: file_name })
    }
}
