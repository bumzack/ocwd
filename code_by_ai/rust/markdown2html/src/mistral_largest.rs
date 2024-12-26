#[derive(Debug)]
enum HtmlElement {
    Header(u8, String),
    Paragraph(String),
    Bold(String),
    Italic(String),
    Link(String, String),
}

pub fn test_mistral_largest() {
    let markdown = r#"
# Header 1
## Header 2
### Header 3
This is a **bold** statement.
This is an *italic* statement.
[Link](http://example.com)
A simple paragraph.
"#;

    let elements = parse_markdown(markdown);
    let html = convert_to_html(elements);
    println!("{}", html);
}

fn parse_markdown(input: &str) -> Vec<HtmlElement> {
    let mut elements = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        if line.starts_with("# ") {
            elements.push(HtmlElement::Header(1, line[2..].to_string()));
        } else if line.starts_with("## ") {
            elements.push(HtmlElement::Header(2, line[3..].to_string()));
        } else if line.starts_with("### ") {
            elements.push(HtmlElement::Header(3, line[4..].to_string()));
        } else if let Some((text, url)) = parse_link(line) {
            elements.push(HtmlElement::Link(text, url));
        } else if let Some(text) = parse_bold(line) {
            elements.push(HtmlElement::Bold(text));
        } else if let Some(text) = parse_italic(line) {
            elements.push(HtmlElement::Italic(text));
        } else {
            elements.push(HtmlElement::Paragraph(line.to_string()));
        }
    }

    elements
}

fn parse_link(input: &str) -> Option<(String, String)> {
    let link_pattern = r"\[([^\]]+)\]\(([^)]+)\)";
    if let Some(captures) = regex::Regex::new(link_pattern).unwrap().captures(input) {
        return Some((
            captures.get(1).map(|m| m.as_str()).unwrap().to_string(),
            captures.get(2).map(|m| m.as_str()).unwrap().to_string(),
        ));
    }
    None
}

fn parse_bold(input: &str) -> Option<String> {
    let bold_pattern = r"\*\*(.*?)\*\*";
    if let Some(captures) = regex::Regex::new(bold_pattern).unwrap().captures(input) {
        return Some(captures.get(1).map(|m| m.as_str()).unwrap().to_string());
    }
    None
}

fn parse_italic(input: &str) -> Option<String> {
    let italic_pattern = r"\*(.*?)\*";
    if let Some(captures) = regex::Regex::new(italic_pattern).unwrap().captures(input) {
        return Some(captures.get(1).map(|m| m.as_str()).unwrap().to_string());
    }
    None
}

fn convert_to_html(elements: Vec<HtmlElement>) -> String {
    let mut html = String::new();

    for element in elements {
        match element {
            HtmlElement::Header(level, text) => {
                html.push_str(&format!("<h{0}>{1}</h{0}>", level, text));
            }
            HtmlElement::Paragraph(text) => {
                html.push_str(&format!("<p>{}</p>", text));
            }
            HtmlElement::Bold(text) => {
                html.push_str(&format!("<strong>{}</strong>", text));
            }
            HtmlElement::Italic(text) => {
                html.push_str(&format!("<em>{}</em>", text));
            }
            HtmlElement::Link(text, url) => {
                html.push_str(&format!(r#"<a href="{1}">{0}</a>"#, text, url));
            }
        }
    }

    html
}
