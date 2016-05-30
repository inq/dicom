use super::tag::Tag;
use super::data::Data;
use std::fs::File;
use std::any::Any;
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

    pub fn tell(&mut self) -> Result<u64, String> {
        self.file.seek(SeekFrom::Current(0)).map_err(|e| e.to_string())
    }

    pub fn read_str(&mut self, size: usize) -> Result<String, String> {
        let mut buf = vec![0; size];
        try!(self.file.read(&mut buf).map_err(|e| e.to_string()));
        String::from_utf8(buf).map_err(|e| e.to_string())
    }

    pub fn read_u8(&mut self) -> Result<u8, String> {
        let mut buf = vec![0; 1];
        try!(self.file.read(&mut buf).map_err(|e| e.to_string()));
        Ok(buf[0])
    }

    pub fn read_u8s(&mut self, size: usize) -> Result<Vec<u8>, String> {
        let mut buf = vec![0u8; size];
        for i in 0..buf.len() {
            buf[i] = try!(self.read_u8());
        }
        Ok(buf)
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        let mut buf = vec![0; 2];
        try!(self.file.read(&mut buf).map_err(|e| e.to_string()));
        Ok((buf[1] as u16) << 8 | (buf[0] as u16))
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        let mut buf = vec![0; 4];
        try!(self.file.read(&mut buf).map_err(|e| e.to_string()));
        Ok((buf[3] as u32) << 24 | (buf[2] as u32) << 16 |
           (buf[1] as u32) << 8  | (buf[0] as u32))
    }

    pub fn read_u16s(&mut self, size: usize) -> Result<Vec<u16>, String> {
        let mut buf = vec![0u16; size];
        for i in 0..buf.len() {
            buf[i] = try!(self.read_u16());
        }
        Ok(buf)
    }

    pub fn read_u32s(&mut self, size: usize) -> Result<Vec<u32>, String> {
        let mut buf = vec![0u32; size];
        for i in 0..buf.len() {
            buf[i] = try!(self.read_u32());
        }
        Ok(buf)
    }

    pub fn read_tag(&mut self) -> Result<Tag, String> {
        Ok(Tag::new(try!(self.read_u16()), try!(self.read_u16())))
    }

    pub fn read_data(&mut self) -> Result<Data, String> {
        let tag = try!(self.read_tag());
        let mut value_repr = String::from("");
        let mut value_length = 0usize;
        let mut is_other_vr = false; // OX OW OB OF

        if tag.group == 0xfffe { // item group
            
        } else {
            value_repr = try!(self.read_str(2));
            is_other_vr = value_repr.as_bytes()[0] == ('O' as u8);
            if is_other_vr || value_repr == "SQ" || value_repr == "UN" {
                try!(self.read_u16());
                value_length = try!(self.read_u32()) as usize;
            } else {
                value_length = try!(self.read_u16()) as usize;
            }
        }

        let size = match value_repr.as_ref() {
            "OB" => 1,
            "OX" | "OW" | "OF" => 2,
            "US" | "SS" => 2,
            "UL" | "SL" | "FL" => 4,
            "FD" => 8,
            "AT" => 2,
            _ => 1
        };

        let data = try!(self.read_u8s(value_length as usize));

        Ok(Data::new(value_repr, value_length, data))
    }
}
