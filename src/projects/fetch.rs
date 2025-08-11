use color_eyre::eyre::{self, Context as _};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

use crate::projects::Project;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GithubResponse {
    id: u64,
    name: String,
    full_name: String,
    html_url: String,
    #[serde(default)]
    description: String,
    fork: bool,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    homepage: Option<String>,
    size: u64,
    language: String,
    topics: Vec<String>,
    open_issues: u64,
    default_branch: String,
}

pub fn get_github_single(project_name: &str) -> eyre::Result<(String, Project)> {
    let url = format!("https://api.github.com/repos/odilf/{project_name}");

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "rust-web-api-client")
        .header(
            AUTHORIZATION,
            format!(
                "Bearer {}",
                std::env::var("GITHUB_TOKEN").wrap_err("Github token not found")?
            ),
        )
        .send()
        .wrap_err("Failed to fetch project")?;

    tracing::debug!(?response, "Got response");
    let response: serde_json::Value = response.json().wrap_err("Failed to parse response")?;
    tracing::debug!(?response, "Got json");
    println!("{response}");
    let response: GithubResponse =
        serde_json::from_value(response).wrap_err("Failed to deserialize response")?;
    // let readme = reqwest::blocking::get(&format!("{}/raw/{}/README.md", project.html_url, project.default_branch))?.text()?;

    let project = Project {
        description: response.description,
        source_code_url: response.html_url,
        website_url: response.homepage,
        documentation_url: None,
        creation_date: response
            .created_at
            .parse()
            .wrap_err("Can't parse creation date")?,
        last_update: response
            .pushed_at
            .parse()
            .wrap_err("Can't parse updated date")?,
        image_url: None,
        language: Some(response.language),
        topics: response.topics,
    };

    Ok((response.name, project))
}
