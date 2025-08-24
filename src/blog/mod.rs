//! Blog of odilf.com

pub mod feed;
mod markdown;

use std::borrow::Cow;

use crate::{
    blog::markdown::MarkdownData,
    components::{self, back, tag},
};
use color_eyre::eyre;
use maud::{Markup, PreEscaped, Render, html};
use serde::{Deserialize, Serialize};

pub const DESC: &str = "some thoughts, stories and reflections from throughout the years.";

/// Blog home page, with the blog entries.
pub fn home<'a>(entries: impl Iterator<Item = &'a BlogEntry>) -> Markup {
    // Show the drafts with less opacity on development.
    #[cfg(debug_assertions)]
    const STYLE_IF_DEBUG: &str = r"<style> .draft-post { opacity: 50%; } </style>";
    #[cfg(not(debug_assertions))]
    const STYLE_IF_DEBUG: &str = "";

    html! {
        (back())

        h1 { "blog" }
        p."pb-4 faint" { (DESC) }

        ."flex mb-4 gap-2" {
            button #all-tab disabled="true" { "all" }
            button #development-tab { "development" }
            button #personal-tab { "personal" }
        }

        ul {
            @for entry in entries {
                li."mb-4" { (entry.render_summary()) }
            }
        }

        script {
            (PreEscaped(include_str!("./index.js")))
        }

        (PreEscaped(STYLE_IF_DEBUG))
    }
}

/// An entry in the blog.
#[derive(Debug, Clone)]
pub struct BlogEntry {
    pub slug: String,
    pub html: String,
    pub summary: String,
    pub word_count: u32,
    pub metadata: BlogMetadata,
}

impl BlogEntry {
    pub fn from_slug_and_content(
        slug: impl Into<String>,
        content: &str,
        referenced_links: &mut Vec<String>,
    ) -> eyre::Result<Option<Self>> {
        let Ok(metadata) = markdown::parse_metadata(content) else {
            return Ok(None);
        };

        #[cfg(not(debug_assertions))]
        if metadata.draft != Some(false) {
            tracing::debug!("Skipped draft post");
            return Ok(None);
        }

        let MarkdownData {
            html,
            summary,
            word_count,
        } = markdown::parse(content, referenced_links);

        Ok(Some(Self {
            slug: slug.into(),
            html,
            summary,
            word_count,
            metadata,
        }))
    }

    pub fn tags(&self) -> impl Iterator<Item = Cow<'_, str>> {
        use std::iter::once;
        self.metadata
            .topics
            .iter()
            .map(|tag| Cow::Borrowed(tag.as_str()))
            .chain(once(Cow::Owned(format!(
                "{:.1}k words",
                self.word_count as f32 / 1000.0
            ))))
    }
    pub fn num_tags(&self) -> usize {
        self.metadata.topics.len() + 1
    }

    pub fn render_summary(&self) -> Markup {
        let mut topic_classes = String::from("blog-entry");
        for topic in &self.metadata.topics {
            topic_classes.push(' ');
            topic_classes.push_str("topic-");
            topic_classes.push_str(topic);
        }

        #[cfg(debug_assertions)]
        if self.metadata.draft != Some(false) {
            topic_classes.push_str(" draft-post")
        }

        html! {
            a href=(format!("/blog/{}", self.slug)).(topic_classes) {
                ."flex" {
                    ."text-primary pr-[1ch]" { ">" }
                    ."flex-1 font-bold text-lg" { (self.metadata.title) }

                    ."font-light text-primary" {
                        (self.metadata.date.strftime("%d %b, %Y"))
                    }
                }

                ."flex gap-2 text-primary" {
                    ."flex-1 text-sm no-underline opacity-50 line-clamp-2 text-ellipsis"
                        style="text-decoration: none"
                    {
                        (self.summary)
                    }

                    ."no-underline grid gap-1" {
                        ."flex gap-1 justify-evenly" {
                            @for tag_text in self.tags().take(self.num_tags().div_ceil(2)) {
                                (tag(tag_text))
                            }
                        }
                        ."flex gap-1 justify-evenly" {
                            @for tag_text in self.tags().skip(self.num_tags().div_ceil(2)) {
                                (tag(tag_text))
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Render for BlogEntry {
    fn render(&self) -> Markup {
        let numbered_headings = if self.metadata.numbered_headings.unwrap_or(true) {
            "numbered-headings"
        } else {
            ""
        };

        html! {
            (components::back())
            h1 { (self.metadata.title) }
            ."flex gap-2 mb-6" {
                ."font-light text-primary" {
                    (self.metadata.date.strftime("%d %b, %Y"))
                }
                ."flex-1" {}
                @for tag_text in self.tags() {
                    (tag(tag_text))
                }
            }
            .(format!("prose pb-8 {numbered_headings}")) lang=(self.metadata.lang.html_name()) {
                (PreEscaped(&self.html))
            }

            link
                rel="stylesheet"
                href="https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css"
                integrity="sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib"
                crossorigin="anonymous"
            {}
        }
    }
}

/// Front-matter of blog.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BlogMetadata {
    pub title: String,
    pub date: jiff::civil::Date,
    pub draft: Option<bool>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub lang: Language,
    #[serde(default)]
    pub numbered_headings: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum Language {
    #[default]
    English,
    Spanish,
}

impl Language {
    pub fn html_name(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
        }
    }
}
