use super::data::BlogMetadata;
use comrak::{
    ExtensionOptions, Options,
    html::{ChildRendering, Context, format_document_with_formatter, format_node_default},
    nodes::{AstNode, NodeValue},
    parse_document,
};
use gray_matter::{Matter, engine::YAML};
// use latex2mathml::{DisplayStyle, latex_to_mathml};
use std::io::{self, Write};

pub fn parse_metadata(content: &str) -> io::Result<BlogMetadata> {
    let frontmatter_parser = Matter::<YAML>::new();

    let Some(metadata) = frontmatter_parser.parse_with_struct(&content) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "frontmatter is not valid.",
        ));
    };

    Ok(metadata.data)
}

/// Parses the input into markdown and returns an `(html, first_line)` tuple.
pub fn to_html(input: &str) -> (String, String) {
    // TODO: Pass this arena from above.
    let arena = comrak::Arena::new();

    let options = Options {
        extension: ExtensionOptions {
            front_matter_delimiter: Some("---".into()),
            math_dollars: true,
            footnotes: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let root = parse_document(&arena, input, &options);

    const FIRST_LINE_MIN_LENGTH: usize = 180;

    let mut first_line = String::new();
    let mut push_text = |text: &str| {
        if first_line.len() + text.len() >= FIRST_LINE_MIN_LENGTH {
            first_line.push_str(&text[..FIRST_LINE_MIN_LENGTH - first_line.len()]);
        } else {
            first_line.push_str(text);
        }

        if first_line.len() >= FIRST_LINE_MIN_LENGTH - 1 {
            return false;
        }

        first_line.push(' ');
        true
    };

    'outer: for child in root.children() {
        if let NodeValue::Paragraph = child.data.borrow().value {
            for node in child.children() {
                let cont = match &node.data.borrow().value {
                    NodeValue::Text(text) => push_text(text),
                    NodeValue::Math(math) => push_text(&math.literal),
                    _ => true,
                };

                if !cont {
                    break 'outer;
                }
            }
        }
    }

    for node in root.descendants() {
        match &mut node.data.borrow_mut().value {
            // Increase the levels of all heading by one, since the title is going to be the first.
            NodeValue::Heading(heading_node) => heading_node.level += 1,
            _ => (),
        }
    }

    let mut html = vec![];
    format_document_with_formatter(
        root,
        &options,
        &mut html,
        &comrak::Plugins::default(),
        format_node_custom,
        (),
    )
    .expect("Markdown should be well-formed.");
    let html = String::from_utf8(html).expect("Parsing should generate valid UTF-8");

    (html, first_line)
}

#[inline]
pub fn format_node_custom<'a>(
    context: &mut Context,
    node: &'a AstNode<'a>,
    entering: bool,
) -> io::Result<ChildRendering> {
    match node.data.borrow().value {
        NodeValue::Math(_) => render_math(context, node, entering),
        _ => format_node_default(context, node, entering),
    }
}

pub fn render_math<'a>(
    context: &mut Context,
    node: &'a AstNode<'a>,
    entering: bool,
) -> io::Result<ChildRendering> {
    use comrak::nodes::NodeMath;

    let NodeValue::Math(NodeMath {
        ref literal,
        display_math,
        dollar_math: _,
        ..
    }) = node.data.borrow().value
    else {
        panic!()
    };

    if entering {
        // TODO: Handle errors.
        let opts = katex::Opts::builder()
            .display_mode(display_math)
            .build()
            .unwrap();
        let html = katex::render_with_opts(&literal, &opts).unwrap();

        // TODO: This sometimes overflows uglily
        write!(context, "{}", html)?;
    }

    Ok(ChildRendering::HTML)
}
