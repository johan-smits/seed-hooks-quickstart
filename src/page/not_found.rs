use crate::Urls;
use seed::{prelude::*, *};

pub fn view<Ms>(base_url: Url) -> Node<Ms> {
    div![
        "404 go ",
        a![attrs! { At::Href => Urls::new(base_url).home() }, "home",]
    ]
}
