use maud::{DOCTYPE, Markup, html};

pub mod blog;
pub mod components;

pub fn shell(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="UTF-8" {}
                meta name="viewport" content="width=device-width, initial-scale=1.0" {}
                title { "odilf's site" }
                link href="/static/app.css" rel="stylesheet" {}

                link rel="alternate" type="application/rss+xml" title="RSS Feed" href="/blog/rss.xml" {}
                link rel="alternate" type="application/atom+xml" title="Atom Feed" href="/blog/atom.xml" {}
            }

            body { main."font-mono py-4 content" { (content) } }
        }
    }
}

pub fn home() -> Markup {
    let links = [("/blog", "blog"), ("/about", "about")];

    html! {
        h1 { "hi, i'm Ody 👋" }
        p."mb-4 faint" { "go ahead and take a look at what's here ^^" }

        ol {
            @for (href, display) in links {
                li."text-xl hover:underline text-secondary" {
                    a href=(href) { "> " (display) }
                }
            }
        }
    }
}

pub fn about() -> Markup {
    html! {
        (components::back())
        h1 { "about" }
        ."prose" {
            p {
                "hi, I'm Odysseas, I like making computers do stuff tastefully, usually using Rust."
            }
            p {
                "I have a master's from the Carlos III University of Madrid, a bachelor for TU Delft in the Netherlands, and the highschool IB and Spanish Bachillerato"
            }
            p {
                "I also like to make and play music; mainly with the bass, the keyboard and the computer"
            }
        }
    }
}
