use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="app-container">
            <main class="mobile-frame">
                // Header
                <div class="header">
                    <img src="public/logo.svg" alt="Gidigroove" class="logo-img-small" />
                </div>

                // Content
                <div class="content">
                    {children()}
                </div>

                // Footer
                <div class="footer">
                    "Powered by Gidigroove"
                </div>
            </main>
        </div>
    }
}
