
use crate::tree::Tag::*;
use core::slice::Iter;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    pub tag: Tag,
    pub content: String,
    pub children: Vec<Element>,
}

impl Element {
    pub fn set_child(&mut self, child: Tree) {
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
    pub fn extend(&mut self, rhs: Tree) {
        self.0.extend(rhs.0)
    }

    pub fn new() -> Self {
        Tree(vec![])
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Paragraph,
    UnordereList(usize),
    Heading(usize),
    EOF,
}

impl Tag {
    pub fn get_depth(&self) -> usize {
        if let Heading(level) | UnordereList(level) = self {
            *level
        } else {
            panic!("called get_depth on a non Heading Tag")
        }
    }

    pub fn next(input: &str) -> Self {
        if let Some(next) = input.chars().nth(0) {
            let head_level = Tag::is_heading(input, 0);
            match next {
                '=' if head_level > 0 => Heading(head_level),
                _ => Paragraph,
            }
        } else {
            return EOF;
        }
    }
    fn is_heading(input: &str, level: usize) -> usize {
        if let Some(next) = input.chars().next() {
            match next {
                '=' => {
                    let level = level + 1;
                    Tag::is_heading(&input[1..input.len()], level)
                }
                ' ' => level,
                _ => 0,
            }
        } else {
            0
        }
    }

    fn is_unordered_list(input: &str, level: usize) -> usize {
        if let Some(next) = input.chars().next() {
            match next {
                '-' | '*' => {
                    let level = level + 1;
                    Tag::is_unordered_list(&input[1..input.len()], level)
                }
                ' ' => level,
                _ => 0,
            }
        } else {
            0
        }
    }
}
