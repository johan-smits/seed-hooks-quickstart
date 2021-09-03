use crate::state::global_count;
use seed::{prelude::*, *};

pub fn label<Ms: 'static>() -> Node<Ms> {
    div![global_count().to_string()]
}
