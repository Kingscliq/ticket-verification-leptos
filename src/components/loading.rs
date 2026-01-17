use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="loading-container">
            <div class="spinner"></div>
            <span>"Loading..."</span>
        </div>
    }
}
