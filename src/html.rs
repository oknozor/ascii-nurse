
use crate::tree::Element;
use crate::tree::Tag::*;
use crate::tree::Tree;
pub trait ToHtml {
    fn to_html(&self) -> String;
}

impl ToHtml for Element {
    fn to_html(&self) -> String {
        match self.tag {
            Paragraph => paragraph(self),
            Heading(1) => format!("<h1>{}</h1>", self.content),
            Heading(level) => head(level, self),
            UnordereList(level) => list(level, self), 
            EOF => "".to_owned(),
        }
    }
}
impl ToHtml for Tree {
    fn to_html(&self) -> String {
        self.iter().map(ToHtml::to_html).collect::<String>()
    }
}

fn head(level: usize, element: &Element) -> String {
    section(level, element)
}

fn element_child(element: &Element) -> String {
    element
        .children
        .iter()
        .map(ToHtml::to_html)
        .collect::<String>()
}

fn section(level: usize, element: &Element) -> String {
    format!(
        "<div class=\"sect{}\">{}{}</div>",
        level - 1,
        h(level, &element),
        element_child(&element)
    )
}

fn section_body(level: usize, element: &Element) -> String {
    format!(
        "<div class=\"sectionbody\">{}</div>",
        element_child(element)
    )
}

fn h(level: usize, element: &Element) -> String {
    let content = &element.content;
    format!(
        "<h{} id=\"_{}\">{}</h{}>",
        level,
        to_snake_case(content),
        content,
        level
    )
}

fn list(level: usize, element: &Element) -> String {
    // TODO 
    "todo".to_owned()
}

fn paragraph(element: &Element) -> String {
    format!("<div class=\"paragraph\"><p>{}</p></div>", &element.content)
}

fn to_snake_case(input: &str) -> String {
    input.to_lowercase().replace(" ", "_")
}
#[cfg(test)]
mod tests {
    use crate::html::ToHtml;

    use crate::tree::Element;

    use crate::tree::Tag::*;
    use crate::tree::Tree;

    #[test]
    fn title_to_html() {
        let input = Element {
            tag: Heading(1),
            content: "Hagakure Kikigaki".to_owned(),
            children: vec![],
        };

        assert_eq!(input.to_html(), "<h1>Hagakure Kikigaki</h1>".to_owned())
    }

    #[test]
    fn heading_to_html() {
        let input = Element {
            tag: Heading(2),
            content: "Hagakure Kikigaki".to_owned(),
            children: vec![],
        };

        assert_eq!(
            input.to_html(),
            "<div class=\"sect1\"><h2 id=\"_hagakure_kikigaki\">Hagakure Kikigaki</h2></div>".to_owned()
        )
    }

    #[test]
    fn paragraph_to_html() {
        let input = Element {
            tag: Paragraph,
            content: "Although it stands to reason that a samurai should be mindful \
                      of the Way of the Samurai, it would seem that we are all negligent"
                .to_owned(),
            children: vec![],
        };

        assert_eq!(
            input.to_html(),
            "<div class=\"paragraph\"><p>Although it stands to reason that a samurai should be mindful \
            of the Way of the Samurai, it would seem that we are all negligent</p></div>".to_owned()
        )
    }

    #[test]
    fn document() {
        let input = Tree {
            0: vec![
                Element {
                    tag: Heading(1),
                    content: "The message".to_owned(),
                    children: vec![],
                },
                // Note that any paragraph following H1 considered a "preamble" and not nested into H1 element
                Element {
                    tag: Paragraph,
                    content: "this is a story that must be told".to_owned(),
                    children: vec![],
                },
                Element {
                    tag: Heading(2),
                    content: "Another title".to_owned(),
                    children: vec![
                        Element {
                            tag: Paragraph,

                            content: "with nested content".to_owned(),
                            children: vec![],
                        },
                        Element {
                            tag: Heading(3),
                            content: "And deeper nesting".to_owned(),
                            children: vec![Element {
                                tag: Paragraph,
                                content: "with some content".to_owned(),
                                children: vec![],
                            }],
                        },
                    ],
                },
                Element {
                    tag: Heading(2),
                    content: "Up a level".to_owned(),
                    children: vec![Element {
                        tag: Paragraph,
                        content: "finally!".to_owned(),
                        children: vec![],
                    }],
                },
            ],
        };

        assert_eq!(
            input.to_html(),
            "<h1>The message</h1>\
             <div class=\"paragraph\">\
             <p>this is a story that must be told</p>\
             </div>\
             <div class=\"sect1\">\
             <h2 id=\"_another_title\">Another title</h2>\
             <div class=\"paragraph\">\
             <p>with nested content</p>\
             </div><div class=\"sect2\">\
             <h3 id=\"_and_deeper_nesting\">And deeper nesting</h3>\
             <div class=\"paragraph\">\
             <p>with some content</p>\
             </div>\
             </div>\
             </div>\
             <div class=\"sect1\">\
             <h2 id=\"_up_a_level\">Up a level</h2>\
             <div class=\"paragraph\">\
             <p>finally!</p>\
             </div>\
             </div>"
        )

    }
}