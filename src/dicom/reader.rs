use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

pub struct Reader {
    file: File
}

impl Reader {
    pub fn new(file_name: &'static str) -> Result<Reader, String> {
        let file = try!(File::open(file_name).map_err(|e| e.to_string()));
        Ok(Reader{file: file})
    }

    pub fn seek(&mut self, offset: u64) {
        self.file.seek(SeekFrom::Start(offset));
    }

    pub fn read_str(&mut self, size: usize) -> Result<String, String> {
        let mut buf = vec![0; size];
        try!(self.file.read(&mut buf).map_err(|e| e.to_string()));
        String::from_utf8(buf).map_err(|e| e.to_string())
    }
}
