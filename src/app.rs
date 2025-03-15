use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    SsrMode,
    components::{A, Route, Router, Routes},
    path,
    static_routes::StaticRoute,
};

use crate::blog;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/odilf-site.css"/>

        // TODO: Maybe change this?
        <Title text="odilf.com"/>

        <Router>
            <main class="bg-neutral text-primary h-screen max-h-screen overflow-y-scroll">
                // TODO: Make nicer 404 page.
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=Home/>
                    <blog::Routes/>
                    <Route ssr=SsrMode::Static(StaticRoute::new()) path=path!("/about") view=About />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let links = [("/blog", "blog"), ("/about", "about")];

    view! {
        <div class="content main">
            <h1>"hi, i'm Ody 👋"</h1>
            <p class="faint mb-4">"go ahead and take a look at what's here ^^"</p>

            <ol>
                {links.map(|(href, display)| view! {
                    <li class="text-secondary text-xl hover:underline">
                        <A href={href}> "> " {display} </A>
                    </li>
                }).collect_view()}
            </ol>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="content main prose">
            <h1>"about"</h1>
            <p> "hi, I'm Odysseas, I like making computers do stuff tastefully, usually using Rust." </p>
            <p> "I have a master's from the Carlos III University of Madrid, a bachelor for TU Delft in the Netherlands, and the highschool IB and Spanish Bachillerato" </p>
            <p> "I also like to make and play music; mainly with the bass, the keyboard and the computer" </p>
        </div>
    }
}
