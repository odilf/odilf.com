use color_eyre::eyre::{self, ContextCompat, WrapErr as _};
use maud::{Markup, Render};
use odilf_site::{
    about,
    blog::{self, BlogEntry},
    home, shell,
};
use std::{
    cmp::Reverse,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let output = {
        #[cfg(debug_assertions)]
        const DEFAULT_PATH: &str = "./target/debug/site";
        #[cfg(not(debug_assertions))]
        const DEFAULT_PATH: &str = "./target/release/site";

        let relative =
            std::env::var("ODILF_OUTPUT_PATH").unwrap_or_else(|_| DEFAULT_PATH.to_string());

        std::env::current_dir()?.join(relative)
    };

    tracing::info!(?output);

    save_page("index.html", home(), &output)?;
    save_page("about/index.html", about(), &output)?;
    generate_blog(&output)?;
    copy_public_to_static(&output)?;
    generate_tailwind("static/app.css", &output)?;

    Ok(())
}

fn save_page_no_shell(path: impl AsRef<Path>, page: Markup, output: &Path) -> eyre::Result<()> {
    let path = path.as_ref();
    fs::create_dir_all(output.join(path.parent().wrap_err("Couldn't get parent of path")?))?;
    fs::write(output.join(path), page.0)
        .wrap_err_with(|| format!("Couldn't write to page {path:?}"))
}

fn save_page(path: impl AsRef<Path>, page: Markup, output: &Path) -> eyre::Result<()> {
    save_page_no_shell(path, shell(page), output)
}

fn generate_blog(output: &Path) -> eyre::Result<()> {
    let blog_path = PathBuf::from(
        std::env::var("ODILF_BLOG_PATH")
            .wrap_err("Couldn't get `ODILF_BLOG_PATH` env variable.")?,
    );

    let blog_output = output.join("blog");
    fs::create_dir_all(&blog_output)
        .wrap_err_with(|| format!("Couldn't create blog path at {blog_output:?}"))?;

    let _span = tracing::info!(?blog_output, ?blog_path);

    let mut referenced_urls = Vec::new();
    let mut blog_entries = fs::read_dir(&blog_path)?
        .map(|entry| {
            let entry = entry?;
            let mut path = entry.path();
            if path.is_dir() {
                tracing::debug!("Skipping directory");
                return eyre::Ok(None);
            }

            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                tracing::info!("Skipping non `.md` file");
                return Ok(None);
            }

            tracing::debug!(?path, "Reading blog entry");

            let post_content = fs::read_to_string(&path).wrap_err("Couldn't read blog post")?;

            path.set_extension("");
            let slug = path
                .file_name()
                .and_then(|name| name.to_str())
                .wrap_err("Couldn't get file name")?;

            let Some(entry) =
                BlogEntry::from_slug_and_content(slug, &post_content, &mut referenced_urls)?
            else {
                return Ok(None);
            };

            tracing::info!(?path, "Generating blog page");

            // TODO: This shouldn't need to allocate
            save_page(
                &format!("blog/{}/index.html", entry.slug),
                entry.render(),
                output,
            )?;

            Ok(Some(entry))
        })
        .flat_map(|result| match result {
            Ok(Some(entry)) => Some(entry),
            Ok(None) => None,
            Err(err) => {
                tracing::error!(?err);
                None
            }
        })
        .collect::<Vec<_>>();

    blog_entries.sort_by_key(|blog| Reverse(blog.metadata.date));
    save_page("blog/index.html", blog::home(blog_entries.iter()), output)?;
    save_page_no_shell(
        "blog/rss.xml",
        blog::feed::rss(blog_entries.iter())?,
        output,
    )?;
    save_page_no_shell(
        "blog/atom.xml",
        blog::feed::atom(blog_entries.iter())?,
        output,
    )?;

    for url in referenced_urls {
        let src = blog_path.join(&url);
        let dst = blog_output.join(&url);
        fs::copy(src, dst).wrap_err_with(|| format!("Couldn't copy referenced url ({url})"))?;
    }

    Ok(())
}

fn copy_public_to_static(output: &Path) -> eyre::Result<()> {
    // From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
    fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.join(entry.file_name()))?;
            }
        }

        Ok(())
    }

    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public");
    let dst = output.join("static");

    copy_dir_all(&src, &dst).wrap_err("Failed to copy directory from {src:?} to {dst?:}")?;

    Ok(())
}

fn generate_tailwind(path: impl AsRef<Path>, output: &Path) -> eyre::Result<()> {
    // let path = path.as_ref();

    let output = Command::new("tailwindcss")
        .args([
            "--input",
            "public/app.css",
            "--output",
            output.join(path).to_str().unwrap(),
        ])
        .output()
        .wrap_err("Failed to run tailwind cli")?;

    tracing::debug!(?output);

    tracing::info!("Succesfully generated tailwind.");

    Ok(())
}
