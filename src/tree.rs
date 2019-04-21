#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    pub tag: Tag,
    pub content: String,
    pub children: Vec<Element>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Paragraph,
    Heading(usize),
}