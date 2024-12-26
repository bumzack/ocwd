pub fn markdown_to_html(markdown: &str) -> Vec<String> {
    let mut html_elements = Vec::new();
    let lines = markdown.lines();

    for line in lines {
        if line.starts_with('#') {
            // Heading
            let level = line.chars().filter(|c| *c == '#').count();
            let text = line.trim_start_matches('#').trim();
            html_elements.push(format!(r#"<h{}>{}</h{}>"#, level, text, level));
        } else if line.starts_with(">") {
            // Blockquote (not strictly part of the requested subset but included for completeness)
            let text = line.trim_start_matches('>').trim();
            html_elements.push(format!(r#"<blockquote>{}</blockquote>"#, text));
        } else if line.is_empty() {
            // Empty line (creates a new paragraph)
            html_elements.push("<p></p>".to_string());
        } else {
            // Paragraph with potential formatting
            let mut formatted_line = String::new();
            let words: Vec<&str> = line.split_whitespace().collect();

            for word in words {
                if word.starts_with('*') && word.ends_with('*') {
                    // Italic
                    formatted_line += &format!(r#"<em>{}</em>"#, &word[1..word.len() - 1]);
                } else if word.starts_with("* * ") && word.ends_with("* * ") {
                    // Bold
                    formatted_line += &format!(r#"<strong>{}</strong>"#, &word[2..word.len() - 2]);
                } else if word.starts_with('[') && word.contains("](") {
                    // Inline link
                    let parts: Vec<&str> = word.splitn(2, ']').collect();
                    if parts.len() == 2 {
                        let link_text = &parts[0][1..];
                        let url = &parts[1][2..parts[1].len() - 1];
                        formatted_line += &format!(r#"<a href="{}">{}</a>"#, url, link_text);
                    }
                } else {
                    formatted_line += word;
                }
                formatted_line.push(' ');
            }
            html_elements.push(format!("<p>{}</p>", formatted_line.trim()));
        }
    }

    html_elements
}
