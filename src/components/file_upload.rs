use leptos::*;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn FileUpload() -> impl IntoView {
    let (dark_mode, set_dark_mode) = create_signal(false);

    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        async move { upload_post(data.into()).await }
    });

    let toggle_dark_mode = move |_| set_dark_mode.update(|dm| *dm = !*dm);

    view! {
        <div class=move || {
            format!(
                "flex flex-col justify-center py-6 min-h-screen {} sm:py-12",
                if dark_mode() { "bg-gray-900" } else { "bg-gray-100" },
            )
        }>
            <div class="relative py-3 sm:mx-auto sm:max-w-xl">
                <div class="absolute inset-0 bg-gradient-to-r from-blue-400 to-blue-600 shadow-lg transform -skew-y-6 sm:rounded-3xl sm:-rotate-6 sm:skew-y-0"></div>
                <div class=move || {
                    format!(
                        "relative py-10 px-4 {} shadow-lg sm:p-20 sm:rounded-3xl",
                        if dark_mode() { "bg-gray-800" } else { "bg-white" },
                    )
                }>
                    <div class="mx-auto max-w-md">
                        <h3 class=move || {
                            format!(
                                "mb-4 text-2xl font-semibold {}",
                                if dark_mode() { "text-gray-100" } else { "text-gray-900" },
                            )
                        }>"File Upload"</h3>
                        <p class=move || {
                            format!(
                                "mb-6 {}",
                                if dark_mode() { "text-gray-300" } else { "text-gray-600" },
                            )
                        }>"Uploading files is fairly easy using multipart form data."</p>
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
                                    class=move || {
                                        format!(
                                            "block w-full text-sm {} file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100",
                                            if dark_mode() { "text-gray-300" } else { "text-gray-500" },
                                        )
                                    }
                                />
                            </div>
                            <input
                                type="submit"
                                value="Upload"
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
                                if upload_action.input().get().is_none()
                                    && upload_action.value().get().is_none()
                                {
                                    "Upload a file.".to_string()
                                } else if upload_action.pending().get() {
                                    "Uploading...".to_string()
                                } else if let Some(Ok(value)) = upload_action.value().get() {
                                    value.to_string()
                                } else {
                                    format!("Server error: {:?}", upload_action.value().get())
                                }
                            }}
                        </p>
                        <button
                            on:click=toggle_dark_mode
                            class=move || {
                                format!(
                                    "mt-4 py-2 px-4 rounded-lg transition duration-300 ease-in-out {}",
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
        </div>
    }
}

/// A simple file upload function, which does just returns the length of the file.
///
/// On the server, this uses the `multer` crate, which provides a streaming API.
#[server(input = server_fn::codec::MultipartFormData)]
pub async fn upload_post(data: server_fn::codec::MultipartData) -> Result<usize, ServerFnError> {
    let mut data = data.into_inner().unwrap();
    let mut count = 0;
    let acceptable_extensions = ["png", "webp", "avif", "jpg", "jpeg"];

    while let Ok(Some(mut field)) = data.next_field().await {
        let file_name = field.file_name().unwrap_or_default().to_string();
        let file_extension = std::path::Path::new(field.file_name().unwrap_or_default())
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_default()
            .to_string();
        if !acceptable_extensions.contains(&file_extension.as_str()) {
            return Err(ServerFnError::Args(
                "Invalid file extension. Upload a image.".to_string(),
            ));
        }

        if !file_name.is_empty() {
            match tokio::fs::create_dir("./uploads/").await {
                Ok(()) => (),
                Err(_error) => (),
            };
            let mut total_file: Vec<bytes::Bytes> = vec![];

            while let Ok(Some(chunk)) = field.chunk().await {
                let len = chunk.len();
                count += len;
                total_file.push(chunk.clone());
            }

            let file_name = format!(
                "./uploads/{}.{}",
                hash_bytes_vec(&total_file)
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .join(""),
                file_extension
            )
            .to_lowercase();

            let mut file = tokio::fs::File::create(&file_name).await?;
            for byte in total_file {
                tokio::io::AsyncWriteExt::write_all(&mut file, &byte).await?;
            }

            println!("File '{}' saved successfully.", file_name);
        }
    }

    Ok(count)
}
#[cfg(feature = "ssr")]
fn hash_bytes_vec(vec: &Vec<bytes::Bytes>) -> [u8; 32] {
    use sha2::Digest;
    use sha2::Sha256;
    // Create a hasher
    let mut hasher = Sha256::new();

    // Update the hasher with each Bytes object
    for bytes in vec {
        hasher.update(bytes);
    }

    // Finalize and return the hash
    hasher.finalize().into()
}
