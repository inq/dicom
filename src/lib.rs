mod dicom;

use self::dicom::Dicom;

#[test]
fn it_works() {
    let x = Dicom::new("data/dicom1.dcm");
    println!("{:?}", x);
}

