pub mod immich;

use maud::Markup;
use maud::html;

use crate::components::back;
use immich::Photo;

pub const DESC: &str = "pictures I've taken";

pub fn home<'a>(photos: impl Iterator<Item = &'a Photo>) -> Markup {
    html! {
        (back())

        h1 { "pictures" }

        div #gallery {
            @for photo in photos {
                ."photo-item" {
                    img src=(photo.image_path) alt=(photo.caption) loading="lazy" {}
                    p."caption" { (photo.caption) }
                }
            }
        }
    }
}
