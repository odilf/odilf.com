pub mod fetch;

use jiff::Timestamp;
use maud::{Markup, html};
use serde::{Deserialize, Serialize};
use serde_with::{Map, serde_as};

use crate::components::{self, tag};

pub const DESC: &str = "most of my personal coding projects.";

pub fn home(projects: Projects) -> Markup {
    let link = |url, arrow, name| {
        html! {
            a."text-inherit group/link transition-[translate] -translate-x-[1.5ch] hover:-translate-x-[1ch] w-full inline-block"
                href=(url) target="_blank" rel="noopener noreferrer"
            {
                span."opacity-80 group-hover/link:opacity-100 transition-opacity" { (arrow) " " } (name)
            }
        }
    };

    html! {
        (components::back())
        h1 { "programming projects" }

        p."pb-2" { (DESC) }
        p {
            "Feel free to check out my "
            a href="https://github.com/odilf" target="_blank" rel="noopener noreferrer" { "GitHub"}
            " and my "
            a href="https://git.odilf.com" target="_blank" rel="noopener noreferrer" { "personal git forge "}
            " for a more comprehensive list."
        }

        ul."grid grid-cols-1 gap-3 mt-8" {
            @for (name, project) in projects.iter() {
                li."text-primary-soft opacity-90 hover:opacity-100 transition flex justify-between gap-2 px-2 py-6 rounded-sm"
                style=(format!("
                    background-image: linear-gradient(to right, rgba(0,0,0, 0.2) 0 100%), url({});
                    background-position: 50% 40%;
                    background-size: cover;
                    background-repeat: no-repeat;
                ", project.image_url.as_ref().map(|s| s.as_str()).unwrap_or(""))) {
                    div."flex-1" {
                        h2."text-xl font-bold text-secondary pt-0 text-balance w-full" {
                            a href=(project.main_link()) target="_blank" rel="noopener noreferrer" { span."opacity-50" { "> " } (name) }
                        }

                        p."text-lg text-primary text-balance" { (project.description) }

                        ul."text-tertiary" {
                            li {
                                @if let Some(weburl) = &project.website_url {
                                    (link(weburl, "->>", "Website"))
                                }
                            }
                            li {
                                (link(&project.source_code_url, "~>>", "Source code"))
                            }
                            li {
                                @if let Some(docurl) = &project.documentation_url {
                                    (link(docurl, "-|>", "Documentation"))
                                }
                            }
                        }
                    }

                    // Min width to fit 3ch for month + 1ch space + 4ch year + 9ch for "Updated: " = 17ch
                    div."text-right min-w-[17ch] flex flex-col items-end" {
                        @if let Some(lang) = &project.language { p."text-primary-intense" { (lang) } }
                        p."text-primary-faint faint" { "Created: " (project.creation_date.strftime("%b %Y")) }
                        p."text-primary-faint faint" { "Updated: " (project.last_update.strftime("%b %Y")) }
                        ul."grid grid-rows-2 grid-flow-col place-items-center w-fit gap-1 justify-end" {
                            @for topic in &project.topics {
                                li."w-fit" {
                                    (tag(topic))
                                }
                            }
                        }
                    }
                }
            }
        }

        p."faint text-xs mt-8" { "Last updated: " (projects.last_fetched.strftime("%d %b, %Y")) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Project {
    pub description: String,
    pub source_code_url: String,
    pub website_url: Option<String>,
    pub documentation_url: Option<String>,
    pub creation_date: Timestamp,
    pub last_update: Timestamp,
    pub image_url: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
}

impl Project {
    pub fn main_link(&self) -> &str {
        self.website_url.as_ref().unwrap_or(&self.source_code_url)
    }
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Projects {
    #[serde_as(as = "Map<_, _>")]
    projects: Vec<(String, Project)>,
    last_fetched: Timestamp,
}

impl Projects {
    pub fn new(projects: Vec<(String, Project)>) -> Self {
        Self {
            projects,
            last_fetched: Timestamp::now(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, Project)> {
        self.projects.iter()
    }
}
