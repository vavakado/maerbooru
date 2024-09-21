use leptos::*;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

#[component]
pub fn FileUpload() -> impl IntoView {
    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        async move { file_length(data.into()).await }
    });

    view! {
        <h3>File Upload</h3>
        <p>Uploading files is fairly easy using multipart form data.</p>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = wasm_bindgen::JsCast::unchecked_into::<
                HtmlFormElement,
            >(ev.target().unwrap());
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch(form_data);
        }>
            <input type="file" name="file_to_upload" />
            <input
                type="submit"
                class="py-2 px-4 font-bold text-white bg-blue-500 rounded transition duration-300 ease-in-out cursor-pointer hover:bg-blue-600"
            />

        </form>
        <p>
            {move || {
                if upload_action.input().get().is_none() && upload_action.value().get().is_none() {
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
