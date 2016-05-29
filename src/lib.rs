mod dicom;

use self::dicom::Dicom;

#[test]
fn it_works() {
    let x = Dicom::new("data/test.dcm");
    println!("{:?}", x);
}

