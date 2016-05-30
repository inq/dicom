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
        reader.seek(128);        
        assert_eq!(try!(reader.read_str(4)), "DICM");
        let metadata = try!(reader.read_data());
        let data_size = try!(metadata.read_u32s())[0] as u64;
        let pos = try!(reader.tell());
        while try!(reader.tell()) < pos + data_size {
            let data = try!(reader.read_data());
            println!("{:?}", data);
        }

        Ok(Dicom { file_name: file_name })
    }
}
