use color_eyre::eyre::{self, Context as _, ContextCompat as _};
use gray_matter::{Matter, engine::YAML};

use crate::media::MediaLog;

pub fn parse_media_log(content: &str) -> eyre::Result<MediaLog> {
    let frontmatter_parser = Matter::<YAML>::new();

    let media_log = frontmatter_parser
        .parse::<MediaLog>(content)
        .wrap_err("Couldn't parse frontmatter")?;

    let media_log = media_log.data.wrap_err("Frontmatter not found")?;
    if media_log.review.is_some() {
        eyre::bail!("`review` field present in frontmatter");
    }

    Ok(media_log)
}
