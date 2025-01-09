use regex::Regex;

#[derive(Debug, PartialEq)]
enum HtmlElement {
    Text(String),
    CodeBlock(String),
    // You can add more elements as needed, e.g., Paragraph, Heading, etc.
}

pub  fn aya_expanse_test() {
    let markdown = "This is a paragraph.\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```\n\nAnd here is some more text.";
    let html_elements = markdown_to_html(markdown);

    for element in html_elements {
        match element {
            HtmlElement::Text(text) => println!("<p>{}</p>", text),
            HtmlElement::CodeBlock(code) => println!("<textarea>{}</textarea>", code),
        }
    }
}

fn markdown_to_html(markdown: &str) -> Vec<HtmlElement> {
    let mut elements = Vec::new();
    let code_block_regex = Regex::new(r"```(.*?)```").unwrap();

    let lines: Vec<&str> = markdown.lines().collect();
    for line in lines {
        if line.starts_with("```") {
            let captures = code_block_regex.captures(line).unwrap();
            let code = captures.get(1).unwrap().as_str().trim();
            elements.push(HtmlElement::CodeBlock(code.to_string()));
        } else {
            elements.push(HtmlElement::Text(line.to_string()));
        }
    }

    elements
}
