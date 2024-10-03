use crate::components::file_upload::FileUpload;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/maerbooru.css" />

        // sets the document title
        <Title text="Welcome to Maerbooru" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=FileUpload />
                    <Route path="/tags" view=crate::components::tag_table::TagTable />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="flex flex-col min-h-screen font-mono text-white bg-gradient-to-tl to-blue-900 from-[#a50000]">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button
                        on:click=on_click
                        class="py-2 px-3 m-1 text-white bg-blue-700 rounded border-l-2 border-b-4 border-blue-800 shadow-lg"
                    >
                        "Click number "
                        {count}
                    </button>
                </div>
            </div>
        </main>
    }
}
