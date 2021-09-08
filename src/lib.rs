// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod button;
mod label;
mod page;
mod routes;
mod state;

use crate::routes::Page;
use seed::{prelude::*, *};
use seed_hooks::*;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Main {
    log!(url);
    orders.subscribe(Msg::UrlChanged);

    Main {
        base_url: url.to_hash_base_url(),
        page: Page::init(url),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Main {
    base_url: Url,
    page: Page,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    UrlChanged(subs::UrlChanged),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Main, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
#[topo::nested]
fn view(model: &Main) -> Node<Msg> {
    let base_url = model.base_url.clone();
    match model.page {
        Page::Home => page::home(base_url),
        Page::Admin => page::admin(base_url),
        Page::NotFound => page::not_found(),
    }
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
