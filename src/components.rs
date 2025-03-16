use leptos::prelude::*;
use leptos::prelude::Callback;
use wasm_bindgen::prelude::*;
use web_sys::{Event, MouseEvent};

#[component]
pub fn ColumnMappingList(
    pre_selected_columns: Vec<(String, String)>, // (name, value)
    column_headers: RwSignal<Vec<String>>,
    update_column_mapping: Callback<(usize, String)>,
) -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; margin-top: 20px;">
            {pre_selected_columns.into_iter().enumerate().map(|(index, (name, value))| {
                view! {
                    <MappingComponent
                        name=name.clone()
                        value=value.clone()
                        index=index
                        column_headers=column_headers
                        update_column_mapping=update_column_mapping.clone()
                    />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn MappingComponent(
    name: String,
    value: String,
    index: usize,
    column_headers: RwSignal<Vec<String>>,
    update_column_mapping: Callback<(usize, String)>,
) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: space-between; margin-bottom: 10px;">
            <div style="flex: 1; margin-right: 10px;">
                <label style="color: #e0e0e0;">{name.clone()}</label>
                <input
                    type="text"
                    value=value.clone()
                    readonly=true
                    style="width: 100%; padding: 8px; border: 1px solid #424242; border-radius: 4px; background-color: #303030; color: #e0e0e0;"
                />
            </div>
            <div style="flex: 1;">
                <label style="color: #e0e0e0;">{"Mapped "}{name.clone()}</label>
                <select
                    on:change=move |ev| update_column_mapping.run((index, event_target_value(&ev)))
                    style="width: 100%; padding: 8px; border: 1px solid #424242; border-radius: 4px; background-color: #303030; color: #e0e0e0;"
                >
                    {column_headers.get().iter().map(|header| {
                        view! {
                            <option value=header.clone()>{header.clone()}</option>
                        }
                    }).collect_view()}
                </select>
            </div>
        </div>
    }
}


#[component]
pub fn RadioButtonList(
    options: Vec<(String, bool)>, // (label, is_selected)
    on_select: Callback<usize>,
) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: center; margin-bottom: 25px;">
            {options
                .into_iter()
                .enumerate()
                .map(|(index, (label, is_selected))| {
                    view! {
                        <label style="margin-right: 20px; display: flex; align-items: center; cursor: pointer;">
                            <div
                                on:click=move |_| on_select.run(index)
                                style="width: 20px; height: 20px; border: 2px solid #bdbdbd; border-radius: 50%; display: flex; justify-content: center; align-items: center; margin-right: 8px;"
                            >
                                {move || if is_selected {
                                    view! { <div style="width: 10px; height: 10px; background-color: #64b5f6; border-radius: 50%;"></div> }
                                } else {
                                    view! { <div style="width: 10px; height: 10px; background-color: #1e1e1e; border-radius: 50%;"></div> }
                                }}
                            </div>
                            {label}
                        </label>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn FileUploadLabel(
    on_file_change: Callback<JsValue>,
) -> impl IntoView {
    view! {
        <label
            for="file-upload"
            style="display: flex; align-items: center; justify-content: center; padding: 12px 20px; border: 1px dashed #5c5c5c; border-radius: 8px; cursor: pointer; margin-bottom: 10px; transition: border-color 0.3s; &:hover { border-color: #64b5f6; } color: #bdbdbd;"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6 mr-3"
                fill="none"
                viewBox="0 0 24 24"
                stroke="#bdbdbd"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M7 16a4 4 0 01-.88-7.907A5 5 0 1115.9 6.193M12 17v-5m0 0L10.3 12.393"
                ></path>
            </svg>
            "Choose File"
            <input
                type="file"
                id="file-upload"
                accept=".csv"
                on:change=move |ev| on_file_change.run(ev.into())
                style="display: none;"
            />
        </label>
    }
}

#[component]
pub fn FileDisplaySection(
    file_signal: RwSignal<Option<Vec<u8>>>,
    file_name: RwSignal<String>,
    clear_selected_file: Callback<MouseEvent>,
) -> impl IntoView {
    move || {
        file_signal.get().map(|_| {
            view! {
                <FileDisplay file_name=file_name on_clear=clear_selected_file />
            }
        })
    }
}

#[component]
pub fn FileDisplay(
    file_name: RwSignal<String>,
    on_clear: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; justify-content: center; margin-bottom: 15px;">
            <span style="color: #333; background-color: #e0e0e0; padding: 5px 8px; border-radius: 4px;">{file_name.get()}</span>
            <button
                on:click=move |ev| on_clear.run(ev)
                style="background-color: #f0f0f0; border: 1px solid #aaa; color: #333; cursor: pointer; padding: 5px 8px; border-radius: 4px; margin-left: 5px;"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                >
                    <path
                        fill-rule="evenodd"
                        d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                        clip-rule="evenodd"
                    />
                </svg>
            </button>
        </div>
    }
}

#[component]
pub fn VendorForm(
    is_new_vendor: ReadSignal<bool>,
    vendor_name: RwSignal<String>,
    password: RwSignal<String>,
    vendor_key: RwSignal<String>,
    update_vendor_name: Callback<Event>,
    update_password: Callback<Event>,
    update_file_key: Callback<Event>,
) -> impl IntoView {
    view! {
        <Show when=move || is_new_vendor.get() fallback=|| view! { <div style="display: none;"></div> }>
            <div style="margin-bottom: 25px;">
                <TextInput 
                    placeholder="Vendor Name".to_string() 
                    value=vendor_name 
                    on_input=update_vendor_name 
                />
                <TextInput 
                    placeholder="Password".to_string() 
                    value=password 
                    on_input=update_password 
                />
                <TextInput 
                    placeholder="Vendor Key".to_string() 
                    value=vendor_key 
                    on_input=update_file_key 
                />
            </div>
        </Show>
    }
}

#[component]
pub fn TextInput(
    placeholder: String,
    value: RwSignal<String>,
    on_input: Callback<Event>,
) -> impl IntoView {
    view! {
        <input
            type="text"
            placeholder=placeholder
            value=value
            on:input=move |ev| on_input.run(ev.into())
            style="padding: 12px; border: 1px solid #424242; border-radius: 8px; width: 100%; margin-bottom: 15px; box-sizing: border-box; transition: border-color 0.3s; background-color: #303030; color: #e0e0e0; &:focus { border-color: #64b5f6; }"
        />
    }
}

#[component]
pub fn SubmitButton(
    on_submit: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |ev: MouseEvent| on_submit.run(ev)
            style="background-color: #64b5f6; color: #1e1e1e; padding: 12px 20px; border: none; border-radius: 8px; cursor: pointer; width: 100%; transition: background-color 0.3s; &:hover { background-color: #42a5f5; }"
        >
            "Upload File"
        </button>
    }
}