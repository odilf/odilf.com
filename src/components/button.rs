use leptos::prelude::*;

#[component]
pub fn Button(children: ChildrenFn) -> impl IntoView {
    view! {
        <button class="m-[-2] px-2 bg-secondary text-neutral rounded">
            {children()}
        </button>
    }
}
