use core::slice::Iter;
use crate::tree::Tag::Heading;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    pub tag: Tag,
    pub content: String,
    pub children: Vec<Element>,
}

impl Element {
    pub fn push(&mut self, child: Tree) {
        self.children = child.0;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tree(pub Vec<Element>); 

impl Tree {
    // We always push to the last elment childrens
    pub fn push(&mut self, el: Element) {
            self.0.push(el);
    }

    pub fn iter(&self) -> Iter<Element> {
        self.0.iter()
    }

    pub fn new() -> Self {
        Tree(vec![])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Paragraph,
    Heading(usize),
}

impl Tag { 
    pub fn get_depth(&self) -> usize {
        if let Heading(level) = self {
            level.clone()
        } else {
            panic!("called get_depth on a non Heading Tag")
        }
    }
}