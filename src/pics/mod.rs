pub mod immich;

use maud::Markup;
use maud::PreEscaped;
use maud::html;

use crate::components::back;
use immich::Photo;

pub const DESC: &str = "pictures I've taken.";

pub fn home<'a>(photos: impl Iterator<Item = &'a Photo>, all_ids: &[String]) -> Markup {
    html! {
        (back())

        h1 { "pics" }
        p."pb-4 faint" { (DESC) }

        button #big-random ."block mx-auto text-center py-4 px-6 rounded-sm text-2xl font-black" { "random" }

        div #gallery ."grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 p-4" {
            @for photo in photos {
                a href=(format!("/pics/{}/", photo.id)) ."block aspect-square overflow-hidden" {
                    img src=(photo.thumb_path()) alt=(photo.caption) loading="lazy" ."w-full h-full object-cover opacity-90 hover:opacity-100 transition-opacity duration-50" {}
                }
            }
        }

        script {
            (PreEscaped(r#"
                document.getElementById('big-random').addEventListener('click', (e) => {
                    e.preventDefault();
                    const ids = JSON.parse(document.getElementById('photo-ids').textContent);
                    const randomId = ids[Math.floor(Math.random() * ids.length)];
                    window.location.href = '/pics/' + randomId + '/';
                });
            "#))
        }
        script #photo-ids type="application/json" data-current=(all_ids.first().map(|s| s.as_str()).unwrap_or("")) { (maud::PreEscaped(serde_json::to_string(all_ids).unwrap())) }
    }
}

pub fn pic(photo: &Photo, index: usize, all_ids: &[String]) -> Markup {
    let id = &photo.id;
    let prev_id = (index > 0).then(|| all_ids[index - 1].as_str());
    let next_id = (index < all_ids.len() - 1).then(|| all_ids[index + 1].as_str());

    html! {
        (back())

        #photo-view data-id=(id) ."flex flex-col items-center" {
            nav."grid grid-cols-3 justify-center text-center" {
                @if let Some(id) = prev_id {
                    a href={(format!("/pics/{}/", id))}."opacity-70 hover:opacity-100" { "<- prev" }
                } @else {
                    div {}
                }
                a href="#" #random ."mx-4 opacity-70 hover:opacity-100" { "rand" }
                @if let Some(id) = next_id {
                    a href={(format!("/pics/{}/", id))}."opacity-70 hover:opacity-100" { "next ->" }
                }
            }
            
            img src=(photo.path()) alt=(photo.caption) ."max-h-[calc(100dvh-200px)] max-w-full mt-8" {}

            ."mt-4 text-center flex gap-6 justify-center items-center" {
                p."opacity-80 text-primary-intense" { (photo.caption) }
                p."text-sm opacity-50 text-secondary" { (format!("{} / {}", index + 1, all_ids.len())) }
            }
        }

        script {
            (PreEscaped(r#"
                document.getElementById('random').addEventListener('click', (e) => {
                    e.preventDefault();
                    const ids = JSON.parse(document.getElementById('photo-ids').textContent);
                    const currentId = document.getElementById('photo-view').dataset.id;
                    const filtered = ids.filter(id => id !== currentId);
                    const randomId = filtered[Math.floor(Math.random() * filtered.length)];
                    window.location.href = '/pics/' + randomId + '/';
                });
            "#))
        }
        script #photo-ids type="application/json" { (maud::PreEscaped(serde_json::to_string(all_ids).unwrap())) }
    }
}
