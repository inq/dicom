#[derive(Debug)]
pub struct Tag {
    pub group: u16,
    pub element: u16,
    pub name: String
}

impl Tag {
    pub fn new(group: u16, element: u16) -> Tag {
        Tag { group: group, element: element, name: format!("{0:02x}{1:02x}", group, element) }
    }
}
