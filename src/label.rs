use seed::{prelude::*, *};
use crate::state::global_count;

pub fn label<Ms: 'static>() -> Node<Ms> {
    div![global_count().to_string()]
}