use crate::blog::BlogMetadata;
use color_eyre::eyre::{self, ContextCompat, WrapErr as _};
use comrak::{
    ExtensionOptions, Options, RenderOptions,
    html::{ChildRendering, Context, format_document_with_formatter, format_node_default},
    nodes::{AstNode, NodeValue},
    parse_document,
};
use gray_matter::{Matter, engine::YAML};
use std::{
    io::{self, Write},
    path::PathBuf,
};

pub fn parse_metadata(content: &str) -> eyre::Result<BlogMetadata> {
    let frontmatter_parser = Matter::<YAML>::new();

    let metadata = frontmatter_parser
        .parse(&content)
        .wrap_err("Couldn't parse frontmatter")?;

    metadata.data.wrap_err("Frontmatter not found")
}

pub struct MarkdownData {
    pub html: String,
    pub summary: String,
    pub word_count: u32,
}

/// Parses the input into markdown and returns an `(html, summary)` tuple.
pub fn parse(input: &str, referenced_links: &mut Vec<String>) -> MarkdownData {
    // TODO: Pass this arena from above.
    let arena = comrak::Arena::new();

    let options = Options {
        extension: ExtensionOptions {
            front_matter_delimiter: Some("---".into()),
            math_dollars: true,
            footnotes: true,
            ..Default::default()
        },
        render: RenderOptions {
            figure_with_caption: true,
            unsafe_: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let root = parse_document(&arena, input, &options);

    // Get summary
    let summary = {
        const FIRST_LINE_MIN_LENGTH: usize = 250;

        let mut summary = String::new();
        let mut push_text = |text: &str| {
            if summary.len() + text.len() >= FIRST_LINE_MIN_LENGTH {
                summary.push_str(&text[..FIRST_LINE_MIN_LENGTH - summary.len()]);
            } else {
                summary.push_str(text);
            }

            if summary.len() >= FIRST_LINE_MIN_LENGTH - 1 {
                return false;
            }

            true
        };

        for node in root.descendants() {
            let cont = match &node.data.borrow().value {
                NodeValue::Text(text) => push_text(text),
                NodeValue::Math(math) => push_text(&math.literal),
                // TODO: This might be poorly handled, especially we should collapse spaces.
                _ => push_text(" "),
            };

            if !cont {
                break;
            }
        }

        summary
    };

    let mut word_count = 0;
    for node in root.descendants() {
        match &mut node.data.borrow_mut().value {
            // Increase the levels of all heading by one, since the title is going to be the first.
            NodeValue::Heading(heading) => heading.level += 1,
            NodeValue::Image(img) => {
                referenced_links.push(img.url.clone());
                img.url = PathBuf::from("/blog")
                    .join(&img.url)
                    .to_str()
                    .expect("All are UTF-8 strings originally")
                    .to_string();
            }
            _ => (),
        }

        if let Some(text) = node.data.borrow().value.text() {
            word_count += text.split_whitespace().count() as u32;
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

    MarkdownData {
        html,
        summary,
        word_count,
    }
}

#[inline]
fn format_node_custom<'a>(
    context: &mut Context,
    node: &'a AstNode<'a>,
    entering: bool,
) -> io::Result<ChildRendering> {
    match node.data.borrow().value {
        NodeValue::Math(_) => render_math(context, node, entering),
        _ => format_node_default(context, node, entering),
    }
}

fn render_math<'a>(
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
