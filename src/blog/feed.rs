//! Generation of RSS and Atom feeds.
//!
//! Technically, I'm using the [`html!`] macro even though these feeds are XML,
//! not HTML. I think it's fine in practice.
//!
//! Checked for validity with https://validator.w3.org/feed/check.cgi

use color_eyre::eyre;
use jiff::{Timestamp, Zoned, fmt::rfc2822, tz::TimeZone};
use maud::{Markup, PreEscaped, html};

use crate::blog::BlogEntry;

impl BlogEntry {
    pub fn rss(&self) -> eyre::Result<Markup> {
        let url = format!("https://odilf.com/blog/{}", self.slug);

        Ok(html! {
            item {
                title { (self.metadata.title) }
                link { (url) }
                description { (self.summary) }
                @if let Some(date) = self.metadata.date {
                    pubDate { (rfc2822::to_string(&date.to_zoned(TimeZone::system())?)?) }
                }
                guid isPermaLink="true" { (url) }
            }
        })
    }

    fn zoned_date(&self) -> eyre::Result<Option<Zoned>> {
        // This is a funny dance to strip out the timezone identifier.
        let tz = TimeZone::system();
        let Some(blog_date) = self.metadata.date else {
            return Ok(None);
        };
        let date = blog_date.to_zoned(tz)?;
        let offset = TimeZone::system().to_offset(date.timestamp());
        let date = blog_date.to_zoned(TimeZone::fixed(offset))?;
        Ok(Some(date))
    }

    pub fn atom(&self) -> eyre::Result<Markup> {
        let url = format!("https://odilf.com/blog/{}", self.slug);

        Ok(html! {
            entry {
                title { (self.metadata.title) }
                link href=(url) {}
                id { (url) }
                @if let Some(date) = self.zoned_date()? { updated { (date) } }
                summary { (self.summary) }
                content type="html" {
                    (PreEscaped("<![CDATA["))
                    (PreEscaped(&self.html))
                    (PreEscaped("]]>"))
                }
            }
        })
    }
}

pub fn rss<'a>(entries: impl Iterator<Item = &'a BlogEntry>) -> eyre::Result<Markup> {
    Ok(html! {
        (PreEscaped(r#"<?xml version="1.0" encoding="UTF-8"?>"#))
        rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" {
          channel {
            title { "Odilf's blog" }
            link { "https://odilf.com/blog" }
            description { "Odilf's personal blog." }
            language { "en" }
            webMaster { "odysseas.maheras@gmail.com (Odilf)" }
            lastBuildDate { (rfc2822::to_string(&Zoned::now())?) }
            generator { "Custom Generator at https://github.com/odilf/odilf.com" }
            atom:link href="https://odilf.com/blog/rss.xml" rel="self" type="application/rss+xml" {}

            @for entry in entries {
                (entry.rss()?)
            }
          }
        }
    })
}

pub fn atom<'a>(entries: impl Iterator<Item = &'a BlogEntry>) -> eyre::Result<Markup> {
    Ok(html! {
        (PreEscaped(r#"<?xml version="1.0" encoding="UTF-8"?>"#))
        feed xmlns="http://www.w3.org/2005/Atom" {
            title { "Odilf's blog" }
            link href="https://odilf.com/blog" {}
            link href="https://odilf.com/blog/atom.xml" rel="self" {}
            id { "https://odilf.com/blog" }
            author {
                name { "Odilf" }
                email { "odysseas.maheras@gmail.com" }
            }
            subtitle { "Odilf's personal blog." }
            updated { (Timestamp::now()) }
            generator uri="https://github.com/odilf/odilf.com" { "Custom Generator" }

            @for entry in entries {
                (entry.atom()?)
            }
        }
    })
}
