mod dicom;

use self::dicom::Dicom;

#[test]
fn it_works() {
//    let x = Dicom::new(String::from("data/test.dcm"));
//    println!("{:?}", x);
}

#[test]
fn read_syntax() {
    let dicom = Dicom::new(String::from("data/test.dcm")).unwrap();
    assert_eq!(dicom.syntax, "1.2.840.10008.1.2.1\u{0}");
}
