#![allow(clippy::must_use_candidate)]

use seed::{prelude::*, *};

mod page;
mod state;

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let base_url = url.to_base_url();
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url));

    Model {
        base_url,
        page_id: None,
        admin_model: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    base_url: Url,
    page_id: Option<PageId>,
    admin_model: Option<page::admin::Model>,
}

// ------ Context ------

pub struct Context {}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq)]
enum PageId {
    Home,
    Admin,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn admin_urls(self) -> page::admin::Urls<'a> {
        page::admin::Urls::new(self.base_url().add_path_part(page::admin::NAMESPACE))
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.page_id = match url.next_path_part() {
                None => Some(PageId::Home),
                Some(page::admin::NAMESPACE) => {
                    page::admin::init(url, &mut model.admin_model).map(|_| PageId::Admin)
                }
                Some(_) => None,
            };
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    let base_url = model.base_url.clone();

    vec![
        header(&model.base_url),
        match model.page_id {
            Some(PageId::Home) => div![format!(
                "Welcome home! The count is: {}",
                state::global_count()
            )],
            Some(PageId::Admin) => {
                page::admin::view(model.admin_model.as_ref().expect("admin model"))
            }
            None => page::not_found::view(base_url),
        },
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    ul![
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).admin_urls().report_urls().root() },
            "Report",
        ]],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
