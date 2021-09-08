use crate::button::button;
use crate::label::label;
use crate::routes::Urls;
use seed::{prelude::*, *};
use seed_hooks::*;

#[topo::nested]
pub fn admin<Ms: 'static>(base_url: Url) -> Node<Ms> {
    div![
        "ADMIN",
        button(),
        button![
            "Home",
            ev(Ev::Click, move |_| Urls::new(base_url)
                .home()
                .go_and_load())
        ]
    ]
}

#[topo::nested]
pub fn home<Ms: 'static>(base_url: Url) -> Node<Ms> {
    div![
        div![
            "This is a counter: ",
            hr![],
            "Which contains data of:",
            label()
        ],
        button![
            "Admin",
            ev(Ev::Click, |_| Urls::new(base_url)
                .admin_urls()
                .go_and_load())
        ]
    ]
}

#[topo::nested]
pub fn not_found<Ms>() -> Node<Ms> {
    div!["404"]
}
