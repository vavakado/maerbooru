use leptos::*;

#[component]
pub fn Modal(
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] dark_mode: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let base_classes = "fixed inset-0 z-50 overflow-auto flex items-center justify-center";
                let visibility = if is_open() { "" } else { "hidden" };
                let bg_color = if dark_mode() {
                    "bg-black bg-opacity-50"
                } else {
                    "bg-gray-500 bg-opacity-50"
                };
                format!("{} {} {}", base_classes, visibility, bg_color)
            }
            on:click=move |_| Callable::call(&on_close, ())
        >
            <div
                class=move || {
                    let base_classes = "relative p-6 w-full max-w-md m-auto flex-col flex rounded-lg";
                    let bg_color = if dark_mode() {
                        "bg-gray-800 text-white"
                    } else {
                        "bg-white text-black"
                    };
                    format!("{} {}", base_classes, bg_color)
                }
                on:click=|ev| ev.stop_propagation()
            >
                <button
                    class=move || {
                        let base_classes = "absolute top-2 right-2 p-1 rounded-full";
                        let color_classes = if dark_mode() {
                            "text-gray-300 hover:text-white hover:bg-gray-700"
                        } else {
                            "text-gray-500 hover:text-gray-700 hover:bg-gray-100"
                        };
                        format!("{} {}", base_classes, color_classes)
                    }
                    on:click=move |_| Callable::call(&on_close, ())
                >
                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                        <path
                            fill-rule="evenodd"
                            d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                            clip-rule="evenodd"
                        ></path>
                    </svg>
                </button>
                <div class="mt-4">{children()}</div>
            </div>
        </div>
    }
}
