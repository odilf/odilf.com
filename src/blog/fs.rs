pub fn generate_blog() {
    let blog_path = PathBuf::from(
        std::env::var("ODILF_BLOG_PATH")
            .wrap_err("Couldn't get `ODILF_BLOG_PATH` env variable.")?,
    );

    let blog_output = output.join("blog");
    fs::create_dir_all(&blog_output)
        .wrap_err_with(|| format!("Couldn't create blog path at {blog_output:?}"))?;

    tracing::info!(?blog_output, ?blog_path);

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
                format!("blog/{}/index.html", entry.slug),
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
