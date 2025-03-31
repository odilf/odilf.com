mod data;

#[cfg(feature = "ssr")]
pub mod markdown;

use data::{BlogEntry, blog_entry_from_slug_server};
use leptos::prelude::*;
use leptos_meta::Link;
use leptos_router::{
    MatchNestedRoutes,
    components::{A, Outlet, ParentRoute, Route},
    hooks::use_params,
    params::Params,
    path,
};

use crate::components;

#[component(transparent)]
pub fn Routes() -> impl MatchNestedRoutes + Clone {
    view! {
      <ParentRoute path=path!("/blog") view=BlogLayout>
        <Route path=path!("/") view=BlogHome/>
        <Route path=path!("/:slug") view=BlogEntry/>
      </ParentRoute>
    }
    .into_inner()
}

#[component]
fn BlogLayout() -> impl IntoView {
    view! {
        <div class="content main">
            <Outlet/>
        </div>
    }
}

#[component]
fn BlogHome() -> impl IntoView {
    #[component]
    fn EntryList(entries: Vec<BlogEntry>) -> impl IntoView {
        view! {
            <ul>
                <For each=move || entries.clone() key=|entry| entry.slug.clone() let:entry>
                    <li class="mb-4">
                        <Entry entry/>
                    </li>
                </For>
            </ul>
        }
    }

    #[component]
    fn Entry(entry: BlogEntry) -> impl IntoView {
        view! {
            <A href=entry.slug>
                <div class="flex">
                    <div class="text-primary pr-[1ch]"> ">" </div>
                    <div class="font-bold flex-1"> {entry.metadata.title} </div>

                    <div class="text-primary font-light"> {entry.metadata.date.strftime("%d %b, %Y").to_string()} </div>
                </div>

                <div class="text-primary flex gap-2">
                    <div style="text-decoration: none" class="text-sm line-clamp-2 opacity-50 text-ellipsis flex-1 no-underline">
                        {entry.first_line}
                    </div>
                    <div class="flex flex-wrap gap-1 justify-evenly max-w-[30%] w-[15%] no-underline">
                        <For each=move || entry.metadata.topics.clone() key=|topic| topic.clone() let:topic>
                            <Tag topic/>
                        </For>
                    </div>

                </div>
            </A>
        }
    }

    #[component]
    fn Tag(topic: String) -> impl IntoView {
        view! {
            <div class="w-fit h-fit outline-1 outline-primary text-primary opacity-80 rounded py-[2px] px-1 text-xs content-center">
                {topic}
            </div>
        }
    }

    let entries = Resource::new(|| (), |_| data::list_entries());
    view! {
        <components::Back/>
        <h1> "blog" </h1>
        <p class="faint pb-4"> "some thoughts, stories and reflections from throughout the years." </p>
        <Suspense fallback=move || view! { <p> "Loading" </p> }>
            {move || entries.get().map(|result| view! {
                // TODO: This error seems bad.
                <ErrorBoundary fallback=move |errors| view! {
                    {format!("Something went wrong! ({:?})", errors.get())}
                }>
                    {result.map(|entries| view! { <EntryList entries /> })}
                </ErrorBoundary>
            })}
        </Suspense>
    }
}

#[component]
fn BlogEntry() -> impl IntoView {
    #[derive(Params, Clone, Debug, PartialEq, Eq)]
    pub struct BlogParams {
        slug: Option<String>,
    }

    let query_params = use_params::<BlogParams>();
    let slug = move || {
        query_params
            .get()
            .map(|q| q.slug.expect("TODO: When does this trigger?"))
    };

    let post_resource = Resource::new_blocking(slug, |slug| async move {
        match slug {
            Err(e) => panic!("TODO: When does this trigger? (Error was {e})"),
            Ok(slug) => blog_entry_from_slug_server(slug).await.map_err(|_| ()),
        }
    });

    view! {
        <components::Back/>
        <Suspense fallback=move || view! { <p> "Loading..." </p> } >
        {move || post_resource.get().map(|entry| {
            let entry = entry.expect("Filesystem should be stable and caught at build time.");
            let Some(entry) = entry else {
                return view! { <p> "Blog entry `" {slug} "` doesn't seem to exist" </p> }.into_any();
            };

            view! {
                <h1> {entry.metadata.title} </h1>
                <div class="prose" inner_html=entry.html/>

                <Link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.21/dist/katex.min.css" integrity="sha384-zh0CIslj+VczCZtlzBcjt5ppRcsAmDnRem7ESsYwWwg3m/OaJ2l4x7YBZl9Kxxib" crossorigin="anonymous"/>

                // TODO: Change html lang when having lang
                // <Html
                //     {..}
                //     lang="he"
                //     dir="rtl"
                //     data-theme="dark"
                // />

            }.into_any()
        })}

        </Suspense>
    }
}

// #[allow(unused)] // path is not used in non-SSR
// fn watch_path(path: &Path) -> impl Stream<Item = ()> {
//     #[allow(unused)]
//     let (mut tx, rx) = mpsc::channel(0);

//     #[cfg(feature = "ssr")]
//     {
//         use notify::RecursiveMode;
//         use notify::Watcher;

//         let mut watcher = notify::recommended_watcher(move |res: Result<_, _>| {
//             if res.is_ok() {
//                 // if this fails, it's because the buffer is full
//                 // this means we've already notified before it's regenerated,
//                 // so this page will be queued for regeneration already
//                 _ = tx.try_send(());
//             }
//         })
//         .expect("could not create watcher");

//         // Add a path to be watched. All files and directories at that path and
//         // below will be monitored for changes.
//         watcher
//             .watch(path, RecursiveMode::NonRecursive)
//             .expect("could not watch path");

//         // we want this to run as long as the server is alive
//         std::mem::forget(watcher);
//     }

//     rx
// }
