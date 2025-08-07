//! Blog of odilf.com

pub mod feed;
mod markdown;

use crate::components::{self, back};
use color_eyre::eyre;
use maud::{html, Markup, PreEscaped, Render};
use serde::{Deserialize, Serialize};

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
        p."pb-4 faint" {
            "some thoughts, stories and reflections from throughout the years."
        }

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
            return Ok(None);
        }

        let (html, summary) = markdown::to_html(&content, referenced_links);

        Ok(Some(Self {
            slug: slug.into(),
            html,
            summary,
            metadata,
        }))
    }

    pub fn render_summary(&self) -> Markup {
        let mut topic_classes = String::from("blog-entry");
        for topic in &self.metadata.topics {
            topic_classes.push_str(" ");
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
                        (self.metadata.date.strftime("%d %b, %Y").to_string())
                    }
                }

                ."flex gap-2 text-primary" {

                    ."flex-1 text-sm no-underline opacity-50 line-clamp-2 text-ellipsis"
                        style="text-decoration: none"
                    {
                        (self.summary)
                    }

                    ."flex flex-wrap gap-1 justify-evenly no-underline max-w-[30%] w-[15%]" {
                        @for topic in &self.metadata.topics {
                            (tag(topic))
                        }
                    }
                }
            }
        }
    }
}

fn tag(topic: &str) -> Markup {
    html! {
        ."content-center px-1 text-xs rounded opacity-80 w-fit h-fit outline-1 outline-primary text-primary py-[1px]" {
            (topic)
        }
    }
}

impl Render for BlogEntry {
    fn render(&self) -> Markup {
        html! {
            (components::back())
            h1 { (self.metadata.title) }
            ."flex gap-2 mb-6" {
                ."font-light text-primary" {
                    (self.metadata.date.strftime("%d %b, %Y").to_string())
                }
                ."flex-1" {}
                @for topic in &self.metadata.topics {
                    (tag(topic))
                }
            }
            ."prose pb-8" lang=(self.metadata.lang.html_name()) {
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
pub struct BlogMetadata {
    pub title: String,
    pub date: jiff::civil::Date,
    pub draft: Option<bool>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub lang: Language,
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
