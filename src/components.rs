use maud::{Markup, html};

pub fn back() -> Markup {
    html! {
        ."faint sticky absolute top-0 left-0 right-0 z-10 bg-neutral" {
            a href=".." { "<-- (back)" }
        }
    }
}

pub fn tag(topic: impl AsRef<str>) -> Markup {
    html! {
        ."content-center text-center whitespace-nowrap px-1 text-xs rounded-xs
        opacity-80 w-fit h-fit outline-1 outline-primary/50 text-primary py-[1px]" {
            (topic.as_ref())
        }
    }
}
