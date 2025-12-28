use crate::components::back;
use color_eyre::eyre::{self, Context};
use comrak::{ExtensionOptions, Options, RenderOptions};
use jiff::civil::Date as JiffDate;
use maud::{Markup, PreEscaped, Render, html};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Write};
use url::Url;

mod data;
mod markdown;

pub const DESC: &str = "logging and reviews of books, movies and videogames.";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MediaLog {
    pub title: String,
    // Slug is set after parsing.
    #[serde(skip)]
    pub slug: String,
    #[serde(rename = "type")]
    pub typ: MediaType,
    pub rating: Rating,
    // #[serde(flatten)]
    pub date: Date,
    pub urls: Vec<Url>,
    pub review: Option<String>,
    // Image URL is set after parsing.
    #[serde(skip)]
    pub image_url: String,
}

pub fn home<'a>(entries: impl Iterator<Item = &'a MediaLog>) -> Markup {
    html! {
        (back())

        h1 { "media log" }
        p."pb-4 faint" { (DESC) }

        ."flex mb-4 gap-2" {
            button #all-tab disabled="true" { "all" }
            button #books-tab { "books" }
            button #movies-tab { "movies" }
            button #videogames-tab { "videogames" }
        }

        ul {
            @for entry in entries {
                li."mb-4" { (entry.render_summary()) }
            }
        }

        script {
            (PreEscaped(include_str!("./index.js")))
        }
    }
}

impl Render for MediaLog {
    fn render(&self) -> maud::Markup {
        html! {
            (back())
            h1 { (self.title) }
            ."flex gap-2" {
                ."flex-1" {
                    ."flex justify-between" {
                        ."text-primary text-2xl" {
                            (self.rating)
                        }

                        ."text-tertiary faint" {
                            (self.date)
                        }
                    }

                    ."font-light text-primary" {
                    }

                    ." text-primary faint" {
                        (PreEscaped(self.review.as_ref().unwrap()))
                    }
                }

                img."w-[30%]" src=(self.image_url) alt=(self.title) {}
            }
        }
    }
}

impl MediaLog {
    pub fn from_slug_and_content(slug: impl Into<String>, content: &str) -> eyre::Result<Self> {
        let mut log = markdown::parse_media_log(content).wrap_err("Invalid frontmatter")?;

        // From blog again, might be unecessary.
        // TODO: Factor out common configuration.
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

        log.review = (!content.is_empty()).then(|| comrak::markdown_to_html(content, &options));
        log.slug = slug.into();
        log.image_url = data::get_image(log.urls.iter(), &log.slug)?;

        Ok(log)
    }

    pub fn render_summary(&self) -> Markup {
        let class = format!("media-log-entry topic-{}", self.typ);

        html! {
            a href=(format!("/media-log/{}", self.slug)).(class) {
                ."flex gap-2" {

                    ."flex-1" {
                        ."flex" {
                            ."text-primary pr-[1ch] text-xl" { ">" }
                            ."flex-1 font-bold text-lg text-xl" { (self.title) }

                            ."font-light text-primary" {
                                (self.date)
                            }
                        }

                        ."flex justify-between" {
                            ."text-primary text-2xl" {
                                (self.rating)
                            }

                            ."text-tertiary faint" {
                                "(" (self.typ) ")"
                            }
                        }

                        ." text-primary faint" {
                            ."flex-1 text-sm no-underline opacity-50 line-clamp-2 text-ellipsis"
                                style="text-decoration: none"
                            {
                                (PreEscaped(self.review.as_ref().unwrap()))
                            }
                        }
                    }

                    img."w-[30%]" src=(self.image_url) alt=(self.title) {}
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MediaType {
    Book,
    Movie,
    Videogame,
    Music,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Book => "book",
            Self::Movie => "movie",
            Self::Videogame => "videogame",
            Self::Music => "music",
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum Date {
    Single(JiffDate),
    Range(JiffDate, JiffDate),
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(date) => write!(f, "{}", date.strftime("%d %b, %Y")),
            Self::Range(start, end) => write!(
                f,
                "{} - {}",
                start.strftime("%d %b, %Y"),
                end.strftime("%d %b, %Y")
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating(f32);

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..(self.0 as u8) {
            f.write_char('★')?;
        }

        if self.0 % 1.0 != 0.0 {
            f.write_char('⯨')?;
        }

        Ok(())
    }
}
