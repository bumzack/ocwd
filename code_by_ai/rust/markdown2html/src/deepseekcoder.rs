//deepseek-coder-v2:236b / 235.7B


use std::fmt;

#[derive(Debug)]
enum Node {
    Header(u8, String), // u8 for header level, String for content
    Paragraph(String),  // String for paragraph content
    Bold(String),       // String for bold text
    Italic(String),     // String for italic text
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Header(level, content) => write!(f, "<h{0}>{1}</h{0}>", level, content),
            Node::Paragraph(content) => write!(f, "<p>{}</p>", content),
            Node::Bold(content) => write!(f, "<strong>{}</strong>", content),
            Node::Italic(content) => write!(f, "<em>{}</em>", content),
        }
    }
}

pub fn parse_markdown(input: &str) -> Vec<Node> {
    let mut nodes = vec![];
    // for line in input.lines() {
    //     if line.starts_with('#') {
    //         let level = (line.chars().take_while(|c| *c == '#').count()) as u8;
    //         let content = &line[level..].trim();
    //         nodes.push(Node::Header(level, content.to_string()));
    //     } else if line.starts_with("**") && line.ends_with("**") {
    //         let content = &line[2..line.len() - 2];
    //         nodes.push(Node::Bold(content.to_string()));
    //     } else if line.starts_with('*') && line.ends_with('*') {
    //         let content = &line[1..line.len() - 1];
    //         nodes.push(Node::Italic(content.to_string()));
    //     } else {
    //         nodes.push(Node::Paragraph(line.trim().to_string()));
    //     }
    // }
    nodes
}
