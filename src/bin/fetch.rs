use color_eyre::eyre;
use odilf_site::projects::{self, Projects};

const PERSONAL_PROJECTS: &[&str] = &[
    "identity-crisis",
    "norcina",
    "dotfiles",
    "churri",
    "odilf.com",
    "FAM",
    "smt-guidance-experiments",
    "bachelor-thesis",
    "chessagon",
    "sentouki",
    "quantum-key-exchange",
    "indecision",
    "hext-boards",
    "incipit",
    "elvish",
    "pink",
    "barbarosa",
    "modern-docs-rs",
    "fosbury",
    "daedalus",
    "Pistol",
    "Dveco",
    "Minigames",
    "spasmodic-consonizer",
    "rummy",
    "baselga-timer",
];

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let projects = PERSONAL_PROJECTS
        .iter()
        .map(|project_name| projects::fetch::get_github_single(project_name))
        .collect::<eyre::Result<_>>()?;

    // Write to projects.toml
    let toml = toml::to_string_pretty(&Projects::new(projects))?;
    std::fs::write("projects-fetched.toml", toml)?;

    Ok(())
}
