use std::num::NonZeroUsize;

use yew::{html, Html};

use crate::{Markdown, MarkdownInline, MarkdownText};

pub fn translate(md: Vec<Markdown>) -> Html {
  html! {
    {for md.into_iter().map(|md| match md {
      Markdown::Heading(size, line) => translate_heading(size, line),
      Markdown::UnorderedList(lines) => translate_unordered_list(lines),
      Markdown::OrderedList(lines) => translate_ordered_list(lines),
      Markdown::Codeblock(lang, code) => translate_codeblock(lang, code),
      Markdown::Line(line) => translate_line(line),
    })}
  }
}

fn translate_boldtext(boldtext: String) -> Html {
  html! {
    <b>{boldtext}</b>
  }
}

fn translate_italic(italic: String) -> Html {
  html! {
    <i>{italic}</i>
  }
}

fn translate_inline_code(code: String) -> Html {
  html! {
    <code class="inline">{code}</code>
  }
}

fn translate_link(text: String, url: String) -> Html {
  html! {
    <a href={url}>{text}</a>
  }
}

fn translate_image(text: String, url: String) -> Html {
  html! {
    <img src={url} alt={text} />
  }
}

fn translate_list_elements(lines: Vec<MarkdownText>) -> Html {
  html! {
    {for lines.into_iter().map(|line| html! {
      <li>{translate_text(line)}</li>
    })}
  }
}

fn translate_heading(size: NonZeroUsize, text: MarkdownText) -> Html {
  let text = translate_text(text);

  match size.get() {
    1 => html! { <h1>{text}</h1> },
    2 => html! { <h2>{text}</h2> },
    3 => html! { <h3>{text}</h3> },
    4 => html! { <h4>{text}</h4> },
    5 => html! { <h5>{text}</h5> },
    6 => html! { <h6>{text}</h6> },
    _ => unreachable!(),
  }
}

fn translate_unordered_list(lines: Vec<MarkdownText>) -> Html {
  html! {
    <ul>
      {translate_list_elements(lines)}
    </ul>
  }
}

fn translate_ordered_list(lines: Vec<MarkdownText>) -> Html {
  html! {
    <ol>
      {translate_list_elements(lines)}
    </ol>
  }
}

// fn translate_code(code: MarkdownText) -> String {
//     format!("<code>{}</code>", translate_text(code))
// }

fn translate_codeblock(lang: String, code: String) -> Html {
  let mut class = String::with_capacity(5 + lang.len());
  class.push_str("lang-");
  class.push_str(&lang);

  html! {
    <pre><code class={class}>{code}</code></pre>
  }
}

fn translate_line(text: MarkdownText) -> Html {
  if text.is_empty() {
    html! {}
  } else {
    html! {
      <p>{translate_text(text)}</p>
    }
  }
}

fn translate_text(text: MarkdownText) -> Html {
  html! {
    {for text.into_iter().map(|inline| match inline {
      MarkdownInline::Bold(text) => translate_boldtext(text),
      MarkdownInline::Italic(text) => translate_italic(text),
      MarkdownInline::InlineCode(code) => translate_inline_code(code),
      MarkdownInline::Link(text, url) => translate_link(text, url),
      MarkdownInline::Image(text, url) => translate_image(text, url),
      MarkdownInline::Plaintext(text) => html! { {text} },
    })}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_translate_boldtext() {
    assert_eq!(
      translate_boldtext(String::from("bold af")),
      String::from("<b>bold af</b>")
    );
  }

  #[test]
  fn test_translate_italic() {
    assert_eq!(
      translate_italic(String::from("italic af")),
      String::from("<i>italic af</i>")
    );
  }

  #[test]
  fn test_translate_inline_code() {
    assert_eq!(
      translate_inline_code(String::from("code af")),
      String::from("<code>code af</code>")
    );
  }

  #[test]
  fn test_translate_link() {
    assert_eq!(
      translate_link(
        String::from("click me!"),
        String::from("https://github.com")
      ),
      String::from("<a href=\"https://github.com\">click me!</a>")
    );
  }

  #[test]
  fn test_translate_image() {
    assert_eq!(
      translate_image(String::from("alt text"), String::from("https://github.com")),
      String::from("<img src=\"https://github.com\" alt=\"alt text\" />")
    );
  }

  #[test]
  fn test_translate_text() {
    let x = translate_text(vec![
      MarkdownInline::Plaintext(String::from(
        "Foobar is a Python library for dealing with word pluralization.",
      )),
      MarkdownInline::Bold(String::from("bold")),
      MarkdownInline::Italic(String::from("italic")),
      MarkdownInline::InlineCode(String::from("code")),
      MarkdownInline::Link(String::from("tag"), String::from("https://link.com")),
      MarkdownInline::Image(String::from("tag"), String::from("https://link.com")),
      MarkdownInline::Plaintext(String::from(". the end!")),
    ]);
    assert_eq!(x, String::from("Foobar is a Python library for dealing with word pluralization.<b>bold</b><i>italic</i><code>code</code><a href=\"https://link.com\">tag</a><img src=\"https://link.com\" alt=\"tag\" />. the end!"));
    let x = translate_text(vec![]);
    assert_eq!(x, String::from(""));
  }

  #[test]
  fn test_translate_heading() {
    assert_eq!(
      translate_heading(1, vec![MarkdownInline::Plaintext(String::from("Foobar"))]),
      String::from("<h1>Foobar</h1>")
    );
  }

  #[test]
  fn test_translate_list_elements() {
    assert_eq!(
      translate_list_elements(vec![
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
      ]),
      String::from("<li>Foobar</li><li>Foobar</li><li>Foobar</li><li>Foobar</li>")
    );
  }

  #[test]
  fn test_translate_unordered_list() {
    assert_eq!(
      translate_unordered_list(vec![
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
      ]),
      String::from("<ul><li>Foobar</li><li>Foobar</li><li>Foobar</li><li>Foobar</li></ul>")
    );
  }

  #[test]
  fn test_translate_ordered_list() {
    assert_eq!(
      translate_ordered_list(vec![
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
        vec![MarkdownInline::Plaintext(String::from("Foobar"))],
      ]),
      String::from("<ol><li>Foobar</li><li>Foobar</li><li>Foobar</li><li>Foobar</li></ol>")
    );
  }

  #[test]
  fn test_translate_codeblock() {
    assert_eq!(
      translate_codeblock(
        String::from("python"),
        String::from(
          r#"
import foobar

foobar.pluralize(\'word\') # returns \'words\'
foobar.pluralize(\'goose\') # returns \'geese\'
foobar.singularize(\'phenomena\') # returns \'phenomenon\'
"#
        )
      ),
      String::from(
        r#"<pre><code class="lang-python">
import foobar

foobar.pluralize(\'word\') # returns \'words\'
foobar.pluralize(\'goose\') # returns \'geese\'
foobar.singularize(\'phenomena\') # returns \'phenomenon\'
</code></pre>"#
      )
    );
  }

  #[test]
  fn test_translate_line() {
    assert_eq!(
      translate_line(vec![
        MarkdownInline::Plaintext(String::from("Foobar")),
        MarkdownInline::Bold(String::from("Foobar")),
        MarkdownInline::Italic(String::from("Foobar")),
        MarkdownInline::InlineCode(String::from("Foobar")),
      ]),
      String::from("<p>Foobar<b>Foobar</b><i>Foobar</i><code>Foobar</code></p>")
    );
  }
}
