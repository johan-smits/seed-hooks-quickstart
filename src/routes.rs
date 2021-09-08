use seed::{prelude::*, *};

pub const ADMIN: &str = "/admin";

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn admin_urls(self) -> Url {
        self.base_url().add_hash_path_part(ADMIN)
    }
}

pub enum Page {
    Home,
    Admin,
    NotFound,
}

impl Page {
    pub(crate) fn init(mut url: Url) -> Self {
        match url.next_hash_path_part() {
            None => Self::Home,
            Some(ADMIN) => Self::Admin,
            Some(_) => Self::NotFound,
        }
    }
}
