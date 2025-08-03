use maud::{Markup, html};

pub fn back() -> Markup {
    html! {
        span class="faint" {
            a href=".." { "<-- (back)" }
        }
    }
}
