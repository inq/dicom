mod dicom;

use self::dicom::Dicom;

#[test]
fn it_works() {
    let x = Dicom::new(String::from("data/test.dcm"));
    println!("{:?}", x);
}

#[test]
fn read_syntax() {
    let dicom = Dicom::new(String::from("data/test.dcm")).unwrap();
    assert!(dicom.syntax == "dd");
}
