pub mod immich;

use maud::Markup;
use maud::html;

use immich::Photo;

pub const DESC: &str = "pictures from my immich album";

pub fn home(photos: &[Photo]) -> Markup {
    html! {
        h1 { "pictures" }

        div id="gallery" {
            @for photo in photos {
                div."photo-item" {
                    img src=(photo.image_path) alt=(photo.caption) loading="lazy" {}
                    @if !photo.caption.is_empty() {
                        p."caption" { (photo.caption) }
                    }
                }
            }
        }
    }
}
