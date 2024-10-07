use crate::api::tags::AddNewTag;
use leptos::*;

#[component]
pub fn AddTagForm(#[prop(into)] dark_mode: Signal<bool>) -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    //let (description, set_description) = create_signal(String::new());
    //let (category, set_category) = create_signal(0u8);

    let add_tag = create_server_action::<AddNewTag>();

    view! {
        <div class="relative py-3 sm:mx-auto sm:max-w-xl">
            <div class=move || {
                format!(
                    "relative p-1 {} shadow-lg sm:p-20 sm:rounded-3xl",
                    if dark_mode() { "bg-gray-800" } else { "bg-white" },
                )
            }>
                <div class="mx-auto max-w-md">
                    <h3 class=move || {
                        format!(
                            "mb-4 text-2xl font-semibold {}",
                            if dark_mode() { "text-gray-100" } else { "text-gray-900" },
                        )
                    }>"Add New Tag"</h3>
                    <p class=move || {
                        format!(
                            "mb-6 {}",
                            if dark_mode() { "text-gray-300" } else { "text-gray-600" },
                        )
                    }>"Create a new tag for your booru system."</p>
                    <form
                        on:submit=move |ev| {
                            ev.prevent_default();
                            add_tag.dispatch(AddNewTag { name: name.get() })
                        }
                        class="mb-6"
                    >
                        <div class="mb-4">
                            <input
                                type="text"
                                placeholder="Tag Name"
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                prop:value=name
                                class=move || {
                                    format!(
                                        "w-full px-3 py-2 text-sm leading-tight {} border rounded appearance-none focus:outline-none focus:shadow-outline",
                                        if dark_mode() {
                                            "text-gray-300 bg-gray-700 border-gray-600"
                                        } else {
                                            "text-gray-700 bg-white border-gray-300"
                                        },
                                    )
                                }
                            />
                        </div>
                        <input
                            type="submit"
                            value="Add Tag"
                            class="py-2 px-4 w-full font-bold text-white bg-blue-500 rounded-lg transition duration-300 ease-in-out cursor-pointer hover:bg-blue-600"
                        />
                    </form>
                    <p class=move || {
                        format!(
                            "text-center {}",
                            if dark_mode() { "text-gray-300" } else { "text-gray-600" },
                        )
                    }>
                        {move || {
                            if add_tag.pending().get() {
                                "Adding tag...".to_string()
                            } else if let Some(Ok(custom_id)) = add_tag.value().get() {
                                format!("Tag added successfully with ID: {}", custom_id)
                            } else if let Some(Err(e)) = add_tag.value().get() {
                                format!("Error: {}", e)
                            } else {
                                "Add a new tag.".to_string()
                            }
                        }}
                    </p>
                </div>
            </div>
        </div>
    }
}
