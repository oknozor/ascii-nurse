#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    tag: Tag,
    content: String,
    children: Vec<Element>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    Paragraph,
    Heading(usize),
}


type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
}


impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}

/// Match a given literal and return the input minus this literal
fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

/// combine two parser into a single one returning a tuple result
fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

/// takes a parser and a map function, return a parser which use the map function
/// on its output
fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

/// filter left output of a parser pair
fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

/// filter right output of a parser pair
fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}

fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    right(space0(), left(parser, space0()))
}

fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

fn quoted_string<'a>() -> impl Parser<'a, String> {
    right(
        match_literal("\""),
        left(
            zero_or_more(any_char.pred(|c| *c != '"')),
            match_literal("\""),
        ),
    )
    .map(|chars| chars.into_iter().collect())
}

fn heading_level<'a>() -> impl Parser<'a, Tag> {
    left(
        one_or_more(any_char.pred(|c| *c == '=')).map(|head| Tag::Heading(head.len())),
        one_or_more(whitespace_char()),
    )
}

fn new_line<'a>() -> impl Parser<'a, ()> {
    either(match_literal("\n"), match_literal("\r\n"))
}

fn head<'a>() -> impl Parser<'a, Element> {
    left(
        pair(
            heading_level(),
            zero_or_more(any_char.pred(|c| *c != '\n')).map(|chars| chars.into_iter().collect()),
        ),
        new_line(),
    )
    .map(|(tag, content)| Element {
        tag,
        content,
        children: vec![],
    })
}


// fn single_element<'a>() -> impl Parser<'a, Element> {
//     left(element_start(), match_literal("/>")).map(|(name, attributes)| Element {
//         name,
//         attributes,
//         children: vec![],
//     })
// }

fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Tag::Heading;
    use crate::parser::*;

    #[test]
    fn parse_literal() {
        assert_eq!(
            Ok((" Once upon a time", ())),
            match_literal("=").parse("= Once upon a time")
        );
        assert_eq!(
            Ok((" Hello Robert!", ())),
            match_literal("=").parse("= Hello Robert!")
        );
        assert_eq!(
            Err("Hello Lucas!"),
            match_literal("=").parse("Hello Lucas!")
        );
    }
    #[test]
    fn parse_new_line() {
        assert_eq!(
            new_line().parse("\nJe suis le ténébreux, le voeuf, l'inconsolé"),
            Ok(("Je suis le ténébreux, le voeuf, l'inconsolé", ()))
        );
    }

    #[test]
    fn parse_heading_level() {
        assert_eq!(
            heading_level().parse("=== The return of the king"),
            Ok(("The return of the king", Heading(3)))
        );
        assert_eq!(
            heading_level().parse("= The fellowship of the ring"),
            Ok(("The fellowship of the ring", Heading(1)))
        );
    }

    #[test]
    fn parse_heading() {
        assert_eq!(
            head().parse("= Hello Dolly\nJolene Jolen Jolene"),
            Ok((
                "Jolene Jolen Jolene",
                Element {
                    tag: Heading(1),
                    content: "Hello Dolly".to_owned(),
                    children: vec![],
                }
            ))
        );
        assert_eq!(
            head().parse("=== Hello Michel\n"),
            Ok((
                "",
                Element {
                    tag: Heading(3),
                    content: "Hello Michel".to_owned(),
                    children: vec![],
                }
            ))
        );
    }
}
