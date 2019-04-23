use crate::tree::Element;
use crate::tree::Tag::*;

trait ToHtml {
    fn to_html(&self) -> String;
}

impl ToHtml for Element {
    fn to_html(&self) -> String {
        match self.tag {
            Paragraph => format!("<div class=paragraph><p>{}</p></div>", self.content),
            Heading(level) => format!(
                "<h{} id=\"level-{}-section-title\" class=\"float\">{}</h{}>",
                level, level, self.content, level
            ),
            EOF => "".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::Element;
    use crate::tree::Tag::*;
    use crate::html::ToHtml;

    #[test]
    fn heading_to_html() {
        let input = Element {
            tag: Heading(1),
            content: "Hagakure Kikigaki".to_owned(),
            children: vec![],
        };

        assert_eq!(
            input.to_html(),
            "<h1 id=\"level-1-section-title\" class=\"float\">Hagakure Kikigaki</h1>".to_owned()
        )
    }

        #[test]
    fn paragraph_to_html() {
        let input = Element {
            tag: Paragraph,
            content: "Although it stands to reason that a samurai should be mindful \
            of the Way of the Samurai, it would seem that we are all negligent".to_owned(),
            children: vec![],
        };

        assert_eq!(
            input.to_html(),
            "<div class=paragraph><p>Although it stands to reason that a samurai should be mindful \
            of the Way of the Samurai, it would seem that we are all negligent</p></div>".to_owned()
        )
    }
}