use leptos::*;
use leptos_router::{components::{Outlet, Route, Router, Routes, A}, *};
use leptos::prelude::*;
use leptos::prelude::Callback;
use wasm_bindgen::prelude::*;
use web_sys::{Event, MouseEvent};

use crate::upload::Upload;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Landingpage() -> impl IntoView {
    view! {
        <Router>
            <div class="home-page">
                <TopBar/>
                <Body/>
            </div>
        </Router>
    }
}

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div class="top-bar">
            Top Bar
        </div>
    }
}

#[component]
pub fn Body() -> impl IntoView {
    view! {
        <div class="main-content">
            <Sidebar/>
            <ContentArea/>
        </div>
    }
}

#[component]
pub fn ContentArea() -> impl IntoView {
    view! {
        <div class="content-area">
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=Upload/>
                <Route path=path!("/apps") view=AppsPageContent/>
            </Routes>
        </div>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar">
            <button>
                + New Vendor
            </button>
            <div class="separator"></div>
            <A href="/">
                <div class="menu-item link">
                    <span>HOME</span>
                    <span>Home</span>
                </div>
            </A>
            <A href="/apps">
                <div class="menu-item link">
                    <span>ICON</span>
                    <span>Apps</span>
                </div>
            </A>        
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 - Page Not Found"</h1>
            <p>"Oops! This page doesn't exist."</p>
        </div>
    }
}

#[component]
fn HomePageContent() -> impl IntoView {
    view! { 
        <h1>"Home Page"</h1>
        <p>"Welcome to the Home Page!"</p>
    }
}

#[component]
fn AppsPageContent() -> impl IntoView {
    view! {
        <h1>"Apps Page"</h1>
        <p>"Here are your Apps!"</p>    
    }
}