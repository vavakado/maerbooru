use crate::components::modal::Modal;
use crate::schemes::tag::get_paginated_tags;
use leptos::*;
use web_sys::window;
use web_sys::SubmitEvent;

#[component]
pub fn TagTable() -> impl IntoView {
    let (page, set_page) = create_signal(1u32);
    let (search_term, set_search_term) = create_signal(String::new());
    let (current_search, set_current_search) = create_signal(String::new());
    let per_page = 40u32;

    let (dark_mode, set_dark_mode) = create_signal(false);

    // Load the initial dark mode preference from localStorage
    create_effect(move |_| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(preference)) = storage.get_item("dark_mode") {
                    set_dark_mode.set(preference == "true");
                }
            }
        }
    });

    // Update localStorage when dark mode changes
    create_effect(move |_| {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("dark_mode", &dark_mode.get().to_string());
            }
        }
    });

    let toggle_dark_mode = move |_| {
        set_dark_mode.update(|dm| *dm = !*dm);
    };

    let tags = create_resource(
        move || (page.get(), current_search.get()),
        move |(current_page, search)| async move {
            let search_option = if search.is_empty() {
                None
            } else {
                Some(search)
            };
            get_paginated_tags(current_page, per_page, search_option)
                .await
                .unwrap_or_else(|_| vec![])
        },
    );

    let handle_search = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_page.set(1); // Reset to first page when searching
        set_current_search.set(search_term.get());
        tags.refetch();
    };

    let (show_modal, set_show_modal) = create_signal(false);

    let open_modal = move |_| set_show_modal(true);
    let close_modal = move |_| set_show_modal(false);

    view! {
        <div class=move || {
            format!(
                "flex flex-col justify-center py-6 min-h-screen {} sm:py-12",
                if dark_mode() { "bg-gray-900" } else { "bg-white" },
            )
        }>
            <div class=move || {
                format!(
                    "container mx-auto px-4 sm:px-8 {}",
                    if dark_mode() { "bg-gray-900 text-white" } else { "bg-white text-black" },
                )
            }>
                <div class="py-8">
                    <div class="mb-4">
                        <form on:submit=handle_search class="flex">
                            <input
                                type="text"
                                placeholder="Search tags (e.g., lain*, *girls, *hello*)"
                                on:input=move |ev| set_search_term.set(event_target_value(&ev))
                                prop:value=search_term
                                class=move || {
                                    format!(
                                        "flex-grow px-3 py-2 text-sm leading-tight border rounded-l focus:outline-none focus:shadow-outline {}",
                                        if dark_mode() {
                                            "bg-gray-800 text-white border-gray-700"
                                        } else {
                                            "bg-white text-gray-700 border-gray-300"
                                        },
                                    )
                                }
                            />
                            <button
                                type="submit"
                                class=move || {
                                    format!(
                                        "px-4 transition duration-300 ease-in-out py-2 font-bold text-white bg-blue-500 rounded-r hover:bg-blue-700 focus:outline-none focus:shadow-outline {}",
                                        if dark_mode() {
                                            "hover:bg-blue-600"
                                        } else {
                                            "hover:bg-blue-500"
                                        },
                                    )
                                }
                            >
                                "Search"
                            </button>
                        </form>
                    </div>
                    <div class="overflow-x-auto py-4 px-4 -mx-4 sm:px-8 sm:-mx-8">
                        <div class=move || {
                            format!(
                                "inline-block min-w-full shadow rounded-lg overflow-hidden {}",
                                if dark_mode() { "bg-gray-800" } else { "bg-white" },
                            )
                        }>
                            <Suspense fallback=move || {
                                view! {
                                    <p class=move || {
                                        if dark_mode() { "text-white" } else { "text-black" }
                                    }>"Loading..."</p>
                                }
                            }>
                                {move || {
                                    tags.get()
                                        .map(|tags| {
                                            view! {
                                                <table class="min-w-full leading-normal">
                                                    <thead>
                                                        <tr>
                                                            <th class=move || {
                                                                format!(
                                                                    "px-5 py-3 border-b-2 text-left text-xs font-semibold uppercase tracking-wider {}",
                                                                    if dark_mode() {
                                                                        "bg-gray-700 text-gray-300 border-gray-600"
                                                                    } else {
                                                                        "bg-gray-100 text-gray-600 border-gray-200"
                                                                    },
                                                                )
                                                            }>"Name"</th>
                                                            <th class=move || {
                                                                format!(
                                                                    "px-5 py-3 border-b-2 text-left text-xs font-semibold uppercase tracking-wider {}",
                                                                    if dark_mode() {
                                                                        "bg-gray-700 text-gray-300 border-gray-600"
                                                                    } else {
                                                                        "bg-gray-100 text-gray-600 border-gray-200"
                                                                    },
                                                                )
                                                            }>"Category"</th>
                                                            <th class=move || {
                                                                format!(
                                                                    "px-5 py-3 border-b-2 text-left text-xs font-semibold uppercase tracking-wider {}",
                                                                    if dark_mode() {
                                                                        "bg-gray-700 text-gray-300 border-gray-600"
                                                                    } else {
                                                                        "bg-gray-100 text-gray-600 border-gray-200"
                                                                    },
                                                                )
                                                            }>"Amount of Uses"</th>
                                                        </tr>
                                                    </thead>
                                                    <tbody>
                                                        {tags
                                                            .into_iter()
                                                            .map(|tag| {
                                                                view! {
                                                                    <tr>
                                                                        <td class=move || {
                                                                            format!(
                                                                                "px-5 py-5 border-b text-sm {}",
                                                                                if dark_mode() {
                                                                                    "border-gray-700 text-gray-300"
                                                                                } else {
                                                                                    "border-gray-200 text-gray-900"
                                                                                },
                                                                            )
                                                                        }>
                                                                            <p class="whitespace-no-wrap">{&tag.name}</p>
                                                                        </td>
                                                                        <td class=move || {
                                                                            format!(
                                                                                "px-5 py-5 border-b text-sm {}",
                                                                                if dark_mode() {
                                                                                    "border-gray-700 text-gray-300"
                                                                                } else {
                                                                                    "border-gray-200 text-gray-900"
                                                                                },
                                                                            )
                                                                        }>
                                                                            <p class="whitespace-no-wrap">{tag.category}</p>
                                                                        </td>
                                                                        <td class=move || {
                                                                            format!(
                                                                                "px-5 py-5 border-b text-sm {}",
                                                                                if dark_mode() {
                                                                                    "border-gray-700 text-gray-300"
                                                                                } else {
                                                                                    "border-gray-200 text-gray-900"
                                                                                },
                                                                            )
                                                                        }>
                                                                            <p class="whitespace-no-wrap">{tag.use_count}</p>
                                                                        </td>
                                                                    </tr>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </tbody>
                                                </table>
                                            }
                                        })
                                }}
                            </Suspense>
                            <div class=move || {
                                format!(
                                    "px-5 py-5 border-t flex flex-col xs:flex-row items-center xs:justify-between {}",
                                    if dark_mode() {
                                        "bg-gray-800 border-gray-700"
                                    } else {
                                        "bg-white border-gray-200"
                                    },
                                )
                            }>
                                <div class="inline-flex mt-2 xs:mt-0">
                                    <button
                                        class=move || {
                                            format!(
                                                "text-sm font-semibold py-2 px-4 rounded-l transition duration-300 ease-in-out {}",
                                                if dark_mode() {
                                                    "bg-gray-700 text-white hover:bg-gray-600"
                                                } else {
                                                    "bg-gray-300 text-gray-800 hover:bg-gray-400"
                                                },
                                            )
                                        }
                                        on:click=move |_| {
                                            set_page
                                                .update(|p| {
                                                    if *p > 1 {
                                                        *p -= 1;
                                                    }
                                                });
                                            tags.refetch();
                                        }
                                    >
                                        "Prev"
                                    </button>
                                    <button
                                        class=move || {
                                            format!(
                                                "text-sm font-semibold py-2 px-4 rounded-r transition duration-300 ease-in-out {}",
                                                if dark_mode() {
                                                    "bg-gray-700 text-white hover:bg-gray-600"
                                                } else {
                                                    "bg-gray-300 text-gray-800 hover:bg-gray-400"
                                                },
                                            )
                                        }
                                        on:click=move |_| {
                                            set_page.update(|p| *p += 1);
                                            tags.refetch();
                                        }
                                    >
                                        "Next"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <button
                        on:click=open_modal
                        class=move || {
                            format!(
                                "mt-4 px-4 py-2 rounded-lg transition duration-300 ease-in-out mr-6 {}",
                                if dark_mode() {
                                    "bg-green-900 text-white hover:bg-green-700"
                                } else {
                                    "bg-green-300 text-gray-800 hover:bg-green-400"
                                },
                            )
                        }
                    >
                        "Add a Tag"
                    </button>

                    <Modal is_open=show_modal on_close=close_modal dark_mode=dark_mode>
                        <crate::components::tag::AddTagForm dark_mode=dark_mode />
                    </Modal>

                    <button
                        on:click=toggle_dark_mode
                        class=move || {
                            format!(
                                "mt-4 px-4 py-2 rounded-lg transition duration-300 ease-in-out {}",
                                if dark_mode() {
                                    "bg-gray-700 text-white hover:bg-gray-600"
                                } else {
                                    "bg-gray-200 text-gray-800 hover:bg-gray-300"
                                },
                            )
                        }
                    >
                        {move || if dark_mode() { "Light Mode" } else { "Dark Mode" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
