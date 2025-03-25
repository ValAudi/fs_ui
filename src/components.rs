use leptos::prelude::*;
use leptos::prelude::Callback;
use wasm_bindgen::prelude::*;
use web_sys::{Event, MouseEvent};

#[component]
pub fn ColumnMappingList(
    is_new_vendor: ReadSignal<bool>,
    file_signal: RwSignal<Option<Vec<u8>>>,
    column_headers: ReadSignal<Vec<String>>,
    update_column_mapping: Callback<(usize, String)>,
) -> impl IntoView {
    let pre_selected_columns = vec![
        "File Key".to_string(),
        "Catalog".to_string(),
        "Brand".to_string(),
        "Description".to_string(),
        "Quantity".to_string(),
        "Ecommerce".to_string(),  
    ];
    view! {
        {move || {
            if file_signal.get().is_some() && !column_headers.get().is_empty()
                && is_new_vendor.get()
            {
                Some(
                    view! {
                        <div class="column-mappings">
                            // Clone the vector
                            {pre_selected_columns
                                .clone()
                                .into_iter()
                                .enumerate()
                                .map(|(index, name)| {
                                    view! {
                                        <MappingComponent
                                            name=name.clone()
                                            index=index
                                            column_headers=column_headers
                                            update_column_mapping=update_column_mapping.clone()
                                        />
                                    }
                                })
                                .collect_view()}
                        </div>
                    },
                )
            } else {
                None
            }
        }}
    }
}

#[component]
pub fn MappingComponent(
    name: String,
    index: usize,
    column_headers: ReadSignal<Vec<String>>,
    update_column_mapping: Callback<(usize, String)>,
) -> impl IntoView {
    let selected_header = RwSignal::new("Select a Column".to_string());
    view! {
        <div class="column-mapping-item">
            <div class="mapping-content">
                <input
                    type="text"
                    value=name.clone()
                    readonly=true
                />
                // <CustomSelect options=column_headers selected=selected_header />
                <select
                    on:change=move |ev| {
                        let selected_value = event_target_value(&ev);
                        selected_header.set(selected_value.clone());
                        update_column_mapping.run((index, selected_value));
                    }
                >
                    <option value="">"Select a Column"</option>
                    {move || column_headers
                        .get()
                        .iter()
                        .map(|header| {
                            view! {
                                <option value=header.clone()>
                                    {header.clone()}
                                </option>
                            }
                        })
                        .collect_view()}
                </select>
            </div>
        </div>
    }
}

// #[component]
// pub fn CustomSelect(
//     options: ReadSignal<Vec<String>>, 
//     selected: RwSignal<String>,
// ) -> impl IntoView {
//     let is_open = RwSignal::new(false); // Track dropdown open/close state

//     view! {
//         <div class="custom-select" on:click=move |_| is_open.set(!is_open.get())>
//             <div class="selected-option">
//                 {move || selected.get()}
//             </div>
//             <Show when=move || is_open.get()>
//                 <ul class="dropdown-options">
//                     {options.get_untracked().iter().map(|option| {
//                         let option = option.clone();
//                         view! {
//                             <li on:click=move |_| {
//                                 selected.set(option.clone());
//                                 is_open.set(false);
//                             }>
//                                 {option.clone()}
//                             </li>
//                         }
//                     }).collect_view()}
//                 </ul>
//             </Show>
//         </div>
//     }
// }

#[component]
pub fn RadioButtonList(
    options: Vec<String>,  // List of labels
    selected_index: RwSignal<usize>,  // Reactive signal to track selected index
    on_select: Callback<usize>,
    set_is_new_vendor: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div style="display: flex; justify-content: center; margin-bottom: 25px;">
            {options
                .into_iter()
                .enumerate()
                .map(|(index, label)| {
                    view! {
                        <RadioButton
                            label=label
                            index=index
                            selected_index=selected_index
                            set_is_new_vendor=set_is_new_vendor
                            on_select=on_select.clone()
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn RadioButton(
    label: String,
    index: usize,
    selected_index: RwSignal<usize>,
    set_is_new_vendor: WriteSignal<bool>,
    on_select: Callback<usize>,
) -> impl IntoView {
    let is_selected = move || selected_index.get() == index;
    let label_clone = label.clone();
    view! {
        <label class="radio-button-label">
            <div
                on:click=move |_| {
                    selected_index.set(index);
                    on_select.run(index);
                    if label_clone == "New Vendor" {
                        set_is_new_vendor.set(true);
                    } else {
                        set_is_new_vendor.set(false);
                    }
                }
                style=move || {
                    format!(
                        "width: 20px; height: 20px; border: 2px solid #bdbdbd; border-radius: 50%;
                    display: flex; justify-content: center; align-items: center; margin-right: 8px; 
                    background-color: {};",
                        if is_selected() { "#64b5f6" } else { "#1e1e1e" },
                    )
                }
            >
                <div style=move || {
                    format!(
                        "width: 10px; height: 10px; border-radius: 50%; background-color: {}",
                        if is_selected() { "#64b5f6" } else { "transparent" },
                    )
                }></div>
            </div>
            {label}
        </label>
    }
}


#[component]
pub fn FileUploadLabel(
    on_file_change: Callback<JsValue>,
) -> impl IntoView {
    view! {
        <label class="file-upload-label" for="file-upload">
            <span class="file-upload-text">"Choose File"</span>
            <input
                type="file"
                id="file-upload"
                accept=".csv"
                on:change=move |ev| on_file_change.run(ev.into())
                class="file-upload-input"
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
            view! { <FileDisplay file_name=file_name on_clear=clear_selected_file /> }
        })
    }
}

#[component]
pub fn FileDisplay(file_name: RwSignal<String>, on_clear: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <div class="file-display">
            <span class="file-name">{move || file_name.get()}</span>
            <CloseButton on_click=on_clear/>
        </div>
    }
}

#[component]
pub fn CloseButton(on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button class="close-button" on:click=move |ev| on_click.run(ev)>
            <img src="public/close_icon.png" alt="Close" class="close-icon"/>
        </button>
    }
}

#[component]
pub fn VendorForm(
    is_new_vendor: ReadSignal<bool>,
    vendor_name: RwSignal<String>,
    password: RwSignal<String>,
    update_vendor_name: Callback<Event>,
    update_password: Callback<Event>,
) -> impl IntoView {
    view! {
        <Show
            when=move || is_new_vendor.get()
            fallback=|| view! { <div style="display: none;"></div> }
        >
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
            value=move || value.get()
            on:input=move |ev| on_input.run(ev.into())
            class="text-input"
        />
    }
}

#[component]
pub fn SubmitButton(
    on_submit: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button class="submit-button" on:click=move |ev: MouseEvent| on_submit.run(ev)>
            "Upload File"
        </button>
    }
}

#[component]
pub fn NotifyPopup(notification: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        {move || notification.get().map(|msg| view! {
            <div class="notification">
                {msg}
            </div>
        })}
    }
}