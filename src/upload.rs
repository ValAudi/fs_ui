use std::io::Cursor;
use csv::ReaderBuilder;
use leptos::task::spawn_local;
use leptos::prelude::*;
use leptos::prelude::Callback;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{console, FileList, HtmlInputElement, MouseEvent};
use gloo_file::futures::read_as_bytes;
use gloo_file:: Blob as GlooBlob;

use crate::components::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Debug)]
struct VendorInfo {
    vendor_name: String,
    password: String,
    file_mappings: String,
    file_bytes: Vec<u8>,
}

#[component]
pub fn Upload() -> impl IntoView {

    let (is_new_vendor, set_is_new_vendor) = signal(false);
    let selected_index = RwSignal::new(0 as usize);
    let vendor_name = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let file_name = RwSignal::new(String::new());
    let file_signal = RwSignal::new(None::<Vec<u8>>);
    let (column_mappings, set_column_mappings) = signal(String::new());
    let (column_headers, set_column_headers) = signal(Vec::<String>::new());
    let notifications = RwSignal::new(None::<String>);

    let on_select = Callback::new(move |index: usize| {
        if index == 1 {
            set_is_new_vendor.set(!is_new_vendor.get()); // This will toggle between true and false
        } else {
            set_is_new_vendor.set(false); // Optionally handle other indexes
        }
    });

    let options = vec![
        "Existing Vendor".to_string(),
        "New Vendor".to_string(),
    ];

    let update_vendor_name = Callback::new(move |ev:web_sys::Event | {
        let v = event_target_value(&ev);
        vendor_name.set(v);
    });

    let update_password = Callback::new(move |ev: web_sys::Event| {
        let v = event_target_value(&ev);
        password.set(v);
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

    let update_column_mapping = Callback::new(move |(index, ev): (usize, String)| {
        let pre_selected_columns = vec![
            "File Key".to_string(),
            "Catalog".to_string(),
            "Brand".to_string(),
            "Description".to_string(),
            "Quantity".to_string(),
            "Ecommerce".to_string(),
        ];
        set_column_mappings.update(move |mappings_string| {
            let column_name = pre_selected_columns[index].clone();

            let mut updated_mappings = mappings_string
                .split(", ")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 2 {
                        (parts[0].to_string(), parts[1].to_string())
                    } else {
                        ("".to_string(), "".to_string())
                    }
                })
                .collect::<std::collections::HashMap<String, String>>();

            updated_mappings.insert(column_name, ev);

            let new_mappings_string = updated_mappings
                .iter()
                .map(|(key, value)| format!("{}:{}", key, value))
                .collect::<Vec<String>>()
                .join(", ");

            *mappings_string = new_mappings_string;
        });
    });

    let on_submit = Callback::new(move |_: MouseEvent| {
        spawn_local(async move {   
            if let Some(file_bytes) = file_signal.get_untracked() {
                let vendor_info = VendorInfo {
                    vendor_name: vendor_name.get_untracked(),
                    password: password.get_untracked(),
                    file_mappings: column_mappings.get_untracked(),
                    file_bytes,
                }; 
                let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                    "payload": vendor_info  // Wrap inside payload key
                }))
                .unwrap();
                console::log_1(&format!("Serialized Args: {:?}", args).into()); 
                let result = invoke("setup_new_user", args).await;  
                console::log_1(&result);
                vendor_name.set("".to_string());
                password.set("".to_string());
                file_name.set("".to_string());
                file_signal.set(None);
                set_column_headers.set(Vec::new());

                notifications.set(Some(result.as_string().unwrap()));

                spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(3000).await;
                    notifications.set(None);
                });
            } else {
                eprintln!("No file selected!");
            }           
        });      
    });

    view! {
        <div class="upload-container">
            <h1>"New Vendor Setup"</h1>
            <RadioButtonList
                options=options
                selected_index=selected_index
                set_is_new_vendor=set_is_new_vendor
                on_select=on_select
            />
            <VendorForm
                is_new_vendor=is_new_vendor
                vendor_name=vendor_name
                password=password
                update_vendor_name=update_vendor_name
                update_password=update_password
            />
            <FileUploadLabel on_file_change=on_file_change />
            <FileDisplaySection
                file_signal=file_signal
                file_name=file_name
                clear_selected_file=clear_selected_file
            />
            <ColumnMappingList
                is_new_vendor=is_new_vendor
                file_signal=file_signal
                column_headers=column_headers
                update_column_mapping=update_column_mapping
            />
            <SubmitButton on_submit=on_submit />
            <NotifyPopup notification=notifications />
        </div>
    }
}
