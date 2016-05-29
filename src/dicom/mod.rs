#[derive(Debug)]
pub struct Dicom {
    file_name: &'static str
}

impl Dicom {
    pub fn new(file_name: &'static str) -> Dicom {
        Dicom { file_name: file_name }
    }
}
