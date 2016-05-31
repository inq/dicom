use super::tag::Tag;

#[derive(Debug)]
pub struct Data {
    pub tag: Tag,
    pub value_repr: String,
    pub value_length: usize,
    pub data: Vec<u8>
}

impl Data {
    pub fn new(tag: Tag, value_repr: String, value_length: usize, data: Vec<u8>)
               -> Data {
        Data { tag: tag, value_repr: value_repr, value_length: value_length, data: data }
    }

    pub fn to_u32s(&self) -> Result<Vec<u32>, String> {
        if self.value_length % 4 != 0 {
            return Err(String::from("Size mismatch"));
        }
        let mut res = vec![0u32; (self.value_length / 4) as usize];
        for i in 0..((self.value_length / 4) as usize) {
            res[i] = (self.data[i * 4 + 3] as u32) << 24 
                | (self.data[i * 4 + 2] as u32) << 16
                | (self.data[i * 4 + 1] as u32) << 8
                | (self.data[i * 4] as u32)
        }
        Ok(res)
    }

    pub fn to_string(&self) -> Result<String, String> {
        String::from_utf8(self.data.clone()).map_err(|e| e.to_string())
    }
}
