use leptos::*;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn FileUpload() -> impl IntoView {
    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        async move { file_length(data.into()).await }
    });

    view! {
        <div class="flex flex-col justify-center py-6 min-h-screen bg-gray-100 sm:py-12">
            <div class="relative py-3 sm:mx-auto sm:max-w-xl">
                <div class="absolute inset-0 bg-gradient-to-r from-blue-400 to-blue-600 shadow-lg transform -skew-y-6 sm:rounded-3xl sm:-rotate-6 sm:skew-y-0"></div>
                <div class="relative py-10 px-4 bg-white shadow-lg sm:p-20 sm:rounded-3xl">
                    <div class="mx-auto max-w-md">
                        <h3 class="mb-4 text-2xl font-semibold text-gray-900">File Upload</h3>
                        <p class="mb-6 text-gray-600">
                            Uploading files is fairly easy using multipart form data.
                        </p>
                        <form
                            on:submit=move |ev: SubmitEvent| {
                                ev.prevent_default();
                                let target = wasm_bindgen::JsCast::unchecked_into::<
                                    HtmlFormElement,
                                >(ev.target().unwrap());
                                let form_data = FormData::new_with_form(&target).unwrap();
                                upload_action.dispatch(form_data);
                            }
                            class="mb-6"
                        >
                            <div class="mb-4">
                                <input
                                    type="file"
                                    name="file_to_upload"
                                    class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
                                />
                            </div>
                            <input
                                type="submit"
                                value="Upload"
                                class="py-2 px-4 w-full font-bold text-white bg-blue-500 rounded-lg transition duration-300 ease-in-out cursor-pointer hover:bg-blue-600"
                            />
                        </form>
                        <p class="text-center text-gray-600">
                            {move || {
                                if upload_action.input().get().is_none()
                                    && upload_action.value().get().is_none()
                                {
                                    "Upload a file.".to_string()
                                } else if upload_action.pending().get() {
                                    "Uploading...".to_string()
                                } else if let Some(Ok(value)) = upload_action.value().get() {
                                    value.to_string()
                                } else {
                                    format!("{:?}", upload_action.value().get())
                                }
                            }}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// A simple file upload function, which does just returns the length of the file.
///
/// On the server, this uses the `multer` crate, which provides a streaming API.
#[server(input = server_fn::codec::MultipartFormData)]
pub async fn file_length(data: server_fn::codec::MultipartData) -> Result<usize, ServerFnError> {
    let mut data = data.into_inner().unwrap();
    let mut count = 0;

    while let Ok(Some(mut field)) = data.next_field().await {
        let file_name = format!("./uploads/{}", field.file_name().unwrap_or_default());

        if !file_name.is_empty() {
            match tokio::fs::create_dir("./uploads/").await {
                Ok(()) => (),
                Err(_error) => (),
            };
            let mut file = tokio::fs::File::create(&file_name).await?;

            while let Ok(Some(chunk)) = field.chunk().await {
                let len = chunk.len();
                count += len;
                tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
            }

            println!("File '{}' saved successfully.", file_name);
        }
    }

    Ok(count)
}
