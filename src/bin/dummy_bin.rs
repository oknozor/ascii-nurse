extern crate ascii_nurse;

fn main() {
        let input = r#"= The message
this is a story that must be told
== Another title
with nested content
=== And deeper nesting
with some content
== Up a level
finally!
"#;

ascii_nurse::parser::parse(input);
}