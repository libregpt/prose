pub mod parser;
pub mod translator;

use std::num::NonZeroUsize;

use yew::{html, Html};

pub type MarkdownText = Vec<MarkdownInline>;

#[derive(Clone, Debug, PartialEq)]
pub enum Markdown {
  Heading(NonZeroUsize, MarkdownText),
  OrderedList(Vec<MarkdownText>),
  UnorderedList(Vec<MarkdownText>),
  Line(MarkdownText),
  Codeblock(String, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownInline {
  Link(String, String),
  Image(String, String),
  InlineCode(String),
  Bold(String),
  Italic(String),
  Plaintext(String),
}

pub fn process(md: &str) -> Html {
  match parser::parse(md) {
    Ok((_, md)) => translator::translate(md),
    Err(_) => html! { {"Failed to parse markdown, make sure it ends with '\\n'."} },
  }
}
