use leptos::prelude::*;
use leptos_router::components::A;

mod button;

pub use button::Button;

#[component]
pub fn Back() -> impl IntoView {
    view! {
        <span class="faint"> <A href=".." > "<-- (back)" </A> </span>
    }
}
