#[cfg(feature = "ssr")]
use super::markdown;

use futures::TryStreamExt;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    io,
    path::{Path, PathBuf},
};
#[cfg(feature = "ssr")]
use tokio::fs;

/// An entry in the blog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogEntry {
    pub slug: String,
    pub html: String,
    pub first_line: String,
    pub metadata: BlogMetadata,
}

/// Front-matter of blog.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde()]
pub struct BlogMetadata {
    pub title: String,
    pub date: jiff::civil::Date,
    pub topics: Vec<String>,
    pub draft: Option<bool>,
    pub language: Option<String>, // TODO: This should probs be enum
}

fn base_path() -> PathBuf {
    // This could be a `LazyLock`, but it's nice to have it here to be able to change the value at runtime.
    // Maybe that is unecessary tho.
    let path_str = std::env::var("ODILF_BLOG_PATH")
        .expect("ODILF_BLOG_PATH needs to be set to a path with markdown files");

    PathBuf::from(path_str)
}

/// Finds the slug of a blog post from a file path
fn get_slug_from_path(path: &Path) -> Option<&str> {
    path.file_name().map(|name| {
        name.to_str()
            .expect("File names should be valid UTF-8")
            .trim_end_matches(".md")
    })
}

/// Parses markdown and returns an `(html, first_line)` tuple.
#[cfg(feature = "ssr")]
impl BlogEntry {
    /// Returns a blog entry from the file path and the slug.
    ///
    /// Returns `Ok(None)` if the blog post doesn't exist.
    /// Returns `Err` if something goes wrong when loading the file (other than the file not existing).
    async fn from_file_and_slug(path: &Path, slug: String) -> io::Result<Option<Self>> {
        let content = match fs::read_to_string(&path).await {
            Ok(content) => content,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err),
        };

        // TODO: Now we ignore invalid frontmatter, maybe we should reject it.
        let Ok(metadata) = markdown::parse_metadata(&content) else {
            return Ok(None);
        };

        // if metadata.draft != Some(false) {
        //     return Ok(None);
        // }
        //
        let (html, first_line) = markdown::to_html(&content);

        Ok(Some(Self {
            slug,
            html,
            first_line,
            metadata,
        }))
    }

    /// Reads a blog post from a file. Returns `Err` if there was an error reading the file,
    /// `Ok(None)` if everything went correctly but the
    pub async fn from_file(path: &Path) -> io::Result<Option<Self>> {
        let Some(slug) = get_slug_from_path(path) else {
            return Ok(None);
        };

        Self::from_file_and_slug(path, slug.to_string()).await
    }

    pub async fn from_slug(slug: String) -> io::Result<Option<Self>> {
        let mut path = base_path();
        path.push(&slug);
        path.set_extension("md");
        Self::from_file_and_slug(&path, slug).await
    }
}

#[server]
pub async fn blog_entry_from_slug_server(slug: String) -> Result<Option<BlogEntry>, ServerFnError> {
    BlogEntry::from_slug(slug)
        .await
        .map_err(ServerFnError::from)
}

#[server]
pub async fn list_entries() -> Result<Vec<BlogEntry>, ServerFnError> {
    list_entries_impl(&base_path())
        .await
        .map_err(ServerFnError::from)
}

#[cfg(feature = "ssr")]
async fn list_entries_impl(base_path: &Path) -> io::Result<Vec<BlogEntry>> {
    use std::cmp::Reverse;

    use tokio_stream::wrappers::ReadDirStream;

    // TODO: This seems inneficient...
    let files = ReadDirStream::new(fs::read_dir(base_path).await?);
    let mut blog_entries = files
        .try_filter_map::<_, _, Vec<BlogEntry>>(|entry| async move {
            let result = if entry.path().is_dir() {
                Box::pin(list_entries_impl(&entry.path())).await?
            } else {
                let Some(entry) = BlogEntry::from_file(&entry.path()).await? else {
                    return Ok(None);
                };
                vec![entry]
            };

            Ok(Some(result))
        }) // This is `impl Stream<Item = Result<Vec<BlogEntry>>>`
        .try_collect::<Vec<Vec<BlogEntry>>>()
        .await
        .map(|vss| vss.into_iter().flatten().collect::<Vec<_>>())?;

    blog_entries.sort_by_key(|entry| Reverse(entry.metadata.date));

    Ok(blog_entries)
}
