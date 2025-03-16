use std::io::Cursor;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use csv::ReaderBuilder;
use leptos::task::spawn_local;
use leptos::prelude::*;
use leptos::prelude::Callback;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::*;
use web_sys::{FileList, HtmlInputElement, MouseEvent};
use gloo_file::futures::read_as_bytes;
use gloo_file:: Blob as GlooBlob;
use gloo_utils::format::JsValueSerdeExt;

use crate::components::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct VendorInfo<'a> {
    vendor_name: &'a str,
    password: &'a str,
    file_key: &'a str,
    file_mappings: &'a str,
}

#[component]
pub fn Upload() -> impl IntoView {

    let (is_new_vendor, set_is_new_vendor) = signal(false);
    let vendor_name = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let file_name = RwSignal::new(String::new());
    let file_signal = RwSignal::new(None::<Vec<u8>>);
    let file_key = RwSignal::new(String::new());
    let (column_mappings, set_column_mappings) = signal(json!({}));
    let (column_headers, set_column_headers) = signal(Vec::<String>::new());

    let on_select = Callback::new(move |index: usize| {
        if index == 1 {
            set_is_new_vendor.set(!is_new_vendor.get()); // This will toggle between true and false
        } else {
            set_is_new_vendor.set(false); // Optionally handle other indexes
        }
    });

    let options = vec![
        ("Existing Vendor".to_string(), !is_new_vendor.get()),
        ("New Vendor".to_string(), is_new_vendor.get()),
    ];

    let update_vendor_name = Callback::new(move |ev| {
        let v = event_target_value(&ev);
        vendor_name.set(v);
    });

    let update_password = Callback::new(move |ev| {
        let v = event_target_value(&ev);
        password.set(v);
    });

    let update_file_key = Callback::new(move |ev| {
        let v = event_target_value(&ev);
        file_key.set(v);
    });

    let clear_selected_file = Callback::new(move |_: MouseEvent| {
        file_signal.set(None);
        file_name.set("".to_string());
    });

    let on_file_change = Callback::new(move |ev: JsValue| {
        let ev: web_sys::Event = ev.into();
        let input: HtmlInputElement = event_target(&ev);
        let files: FileList = input.files().unwrap();
        if let Some(f) = files.get(0) {
            file_name.set(f.name());
            let gloo_blob = GlooBlob::from(f); 
            spawn_local(async move {
                match read_as_bytes(&gloo_blob).await {
                    Ok(bytes) => {
                        file_signal.set(Some(bytes.clone()));
                        // Read CSV headers
                        let cursor = Cursor::new(bytes.clone());
                        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(cursor);
                        if let Ok(headers) = rdr.headers() {
                            set_column_headers.set(headers.iter().map(|h| h.to_string()).collect());
                        } else {
                            eprintln!("Failed to read CSV headers");
                        }
                    }
                    Err(err) => eprintln!("File read error: {:?}", err),
                }
            });
        } else {
            file_name.set("".to_string());
            file_signal.set(None);
            set_column_headers.set(Vec::new());
        }
    });

    let update_column_mapping = move |index: usize, ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        set_column_mappings.update(move |mappings| {
            let mut updated_mappings = mappings.as_object().unwrap().clone();
            updated_mappings.insert(column_headers.get()[index].clone(), json!(value));
            *mappings = json!(updated_mappings);
        });
    };

    let on_submit = Callback::new(move |_: MouseEvent| {
        if let Some(file_bytes) = file_signal.get() {
            let payload = json!({
                "isNewVendor": is_new_vendor.get(),
                "vendorName": vendor_name.get(),
                "password": password.get(),
                "fileKey": file_key.get(),
                "tableMappings": column_mappings.get(),
                "fileData": BASE64_STANDARD.encode(file_bytes),
            });
            spawn_local(async move {
                let result = invoke("upload_file", JsValue::from_serde(&payload).unwrap()).await;
                println!("{:?}", result);
            });
        } else {
            eprintln!("No file selected!");
        }
    });

    view! {
        <div style="display: flex; justify-content: center; align-items: center; min-height: 100vh; background-color: #121212;">
            <div style="width: 400px; padding: 30px; border-radius: 12px; background-color: #1e1e1e; box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);">
                <h1 style="text-align: center; margin-bottom: 30px; color: #e0e0e0;">"New Vendor Setup"</h1>
                <RadioButtonList options=options on_select=on_select/>
                <VendorForm 
                    is_new_vendor=is_new_vendor 
                    vendor_name=vendor_name 
                    password=password 
                    vendor_key=file_key 
                    update_vendor_name=update_vendor_name 
                    update_password=update_password 
                    update_file_key=update_file_key 
                />
                <FileUploadLabel on_file_change=on_file_change/>
                <FileDisplaySection
                    file_signal=file_signal
                    file_name=file_name
                    clear_selected_file=clear_selected_file
                />     
                <SubmitButton on_submit=on_submit />             
            </div>
        </div>
    }
}
