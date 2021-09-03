use crate::state::global_count;
use seed::{prelude::*, *};
use seed_hooks::*;

#[topo::nested]
pub fn button<Ms: 'static>() -> Node<Ms> {
    div![
        button!["- ", global_count().on_click(|c| *c -= 1)],
        span![global_count().to_string()],
        button![" +", global_count().on_click(|c| *c += 1)],
    ]
}
