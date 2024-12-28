use std::collections::HashMap;

#[derive(Debug)]
enum MarkdownElement {
    Heading(u8, String),
    Paragraph(String),
    Bold(String),
    Italic(String),
    Link(String, String),
    Image(String, String),
    ListItem(String),
    Blockquote(String),
    CodeBlock(String),
}

pub  fn mistral_largest2() -> String {
    let markdown = r#"
# Heading 1
## Heading 2
### Heading 3

This is a paragraph with **bold** and *italic* text.

* List item 1
* List item 2

> This is a blockquote.

This is a code block.


[Link](http://example.com)

![Image](http://example.com/image.png)
"#;

    let elements = parse_markdown(markdown);
    let html = convert_to_html(elements);
    println!("{}", html);
}




fn parse_markdown(input: &str) -> Vec<MarkdownElement> {
    let mut elements = vec![];
    let lines: Vec<&str> = input.lines().collect();
    let mut current_paragraph = String::new();

    for line in lines {
        if line.starts_with("#") {
            if !current_paragraph.is_empty() {
                elements.push(MarkdownElement::Paragraph(current_paragraph.clone()));
                current_paragraph.clear();
            }
            let heading_level = line.chars().take_while(|c| *c == '#').count() as u8;
            let content = line[heading_level..].trim().to_string();
            elements.push(MarkdownElement::Heading(heading_level, content));
        } else if line.starts_with("* ") || line.starts_with("- ") {
            if !current_paragraph.is_empty() {
                elements.push(MarkdownElement::Paragraph(current_paragraph.clone()));
                current_paragraph.clear();
            }
            let content = line[2..].trim().to_string();
            elements.push(MarkdownElement::ListItem(content));
        } else if line.starts_with("> ") {
            if !current_paragraph.is_empty() {
                elements.push(MarkdownElement::Paragraph(current_paragraph.clone()));
                current_paragraph.clear();
            }
            let content = line[2..].trim().to_string();
            elements.push(MarkdownElement::Blockquote(content));
        } else if line.starts_with("```") {
            if !current_paragraph.is_empty() {
                elements.push(MarkdownElement::Paragraph(current_paragraph.clone()));
                current_paragraph.clear();
            }
            let mut code_block = String::new();
            for l in lines.iter().skip_while(|&&l| l != line).skip(1) {
                if l == "```" {
                    break;
                }
                code_block.push_str(l);
                code_block.push('\n');
            }
            elements.push(MarkdownElement::CodeBlock(code_block));
        } else if line.starts_with("!") {
            let parts: Vec<&str> = line[1..].split(']').collect();
            if parts.len() == 2 {
                let alt_text = parts[0][1..].trim().to_string();
                let url = parts[1][1..parts[1].len() - 1].trim().to_string();
                elements.push(MarkdownElement::Image(alt_text, url));
            }
        } else if line.starts_with("[") {
            let parts: Vec<&str> = line.split(']').collect();
            if parts.len() == 2 {
                let text = parts[0][1..].trim().to_string();
                let url = parts[1][1..parts[1].len() - 1].trim().to_string();
                elements.push(MarkdownElement::Link(text, url));
            }
        } else {
            current_paragraph.push_str(line);
            current_paragraph.push('\n');
        }
    }

    if !current_paragraph.is_empty() {
        elements.push(MarkdownElement::Paragraph(current_paragraph));
    }

    elements
}

fn convert_to_html(elements: Vec<MarkdownElement>) -> String {
    let mut html = String::new();

    for element in elements {
        match element {
            MarkdownElement::Heading(level, content) => {
                html.push_str(&format!("<h{0}>{1}</h{0}>", level, content));
            }
            MarkdownElement::Paragraph(content) => {
                let mut paragraph = String::new();
                for line in content.lines() {
                    if line.starts_with("**") && line.ends_with("**") {
                        paragraph.push_str(&format!("<strong>{}</strong>", &line[2..line.len() - 2]));
                    } else if line.starts_with("*") && line.ends_with("*") {
                        paragraph.push_str(&format!("<em>{}</em>", &line[1..line.len() - 1]));
                    } else {
                        paragraph.push_str(line);
                    }
                }
                html.push_str(&format!("<p>{}</p>", paragraph));
            }
            MarkdownElement::Bold(content) => {
                html.push_str(&format!("<strong>{}</strong>", content));
            }
            MarkdownElement::Italic(content) => {
                html.push_str(&format!("<em>{}</em>", content));
            }
            MarkdownElement::Link(text, url) => {
                html.push_str(&format!("<a href=\"{}\">{}</a>", url, text));
            }
            MarkdownElement::Image(alt_text, url) => {
                html.push_str(&format!("<img src=\"{}\" alt=\"{}\"/>", url, alt_text));
            }
            MarkdownElement::ListItem(content) => {
                html.push_str(&format!("<li>{}</li>", content));
            }
            MarkdownElement::Blockquote(content) => {
                html.push_str(&format!("<blockquote>{}</blockquote>", content));
            }
            MarkdownElement::CodeBlock(content) => {
                html.push_str(&format!(r#"<textarea readonly="true">{}</textarea>"#, content));
            }
        }
    }

    html
}
