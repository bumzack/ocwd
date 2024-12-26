use crate::aya_expanse::markdown_to_html;
use crate::aya_expanse2::aya_expanse_test;

mod aya_expanse;
mod aya_expanse2;
mod deepseekcoder;
mod mistral_largest;

fn main() {
    let markdown = r#"
# Heading 1
This is a **bold** and *italic* paragraph with [a link](https://example.com).

> Blockquote

# Heading 2
    "#;

    // no code blocks, that ain't it
    let html_elements = markdown_to_html(markdown);
    for element in html_elements {
        println!("aya_expanse  {}", element);
    }

    // naaa - thats no good at all
    // let html_elements = parse_markdown(markdown);
    // for element in html_elements {
    //     println!("deepseekcoder   {}", element);
    // }

    // tbe .lines() break the regex which is looking for the closing ```
    println!("aya_expanse2 ");
    aya_expanse_test();
}
