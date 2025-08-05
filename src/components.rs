use maud::{Markup, html};

pub fn back() -> Markup {
    html! {
        ."faint sticky absolute top-0 left-0 right-0 z-10 bg-neutral" {
            a href=".." { "<-- (back)" }
        }
    }
}
