use crate::state;
use seed::{prelude::*, *};
use seed_hooks::*;

pub const NAMESPACE: &str = "report";

const DAILY: &str = "daily";
const WEEKLY: &str = "weekly";

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url, model: &mut Option<Model>) -> Option<()> {
    let model = model.get_or_insert_with(|| Model {
        base_url: url.to_base_url(),
        frequency: Frequency::Daily,
    });

    model.frequency = match url.remaining_path_parts().as_slice() {
        [] => {
            match model.frequency {
                Frequency::Daily => Urls::new(&model.base_url).daily().go_and_push(),
                Frequency::Weekly => Urls::new(&model.base_url).weekly().go_and_push(),
            }
            model.frequency
        }
        [DAILY] => Frequency::Daily,
        [WEEKLY] => Frequency::Weekly,
        _ => None?,
    };
    Some(())
}

// ------ ------
//     Model
// ------ ------
#[derive(Clone)]
pub struct Model {
    base_url: Url,
    frequency: Frequency,
}

// ------ Frequency ------

#[derive(Copy, Clone)]
enum Frequency {
    Daily,
    Weekly,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
    pub fn daily(self) -> Url {
        self.base_url().add_path_part(DAILY)
    }
    pub fn weekly(self) -> Url {
        self.base_url().add_path_part(WEEKLY)
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(model: Model) -> Node<Ms> {
    let (frequency, link) = match &model.frequency {
        Frequency::Daily => (
            "daily",
            a![
                "Switch to weekly",
                attrs! {
                    At::Href => Urls::new(&model.base_url).weekly()
                }
            ],
        ),
        Frequency::Weekly => (
            "weekly",
            a![
                "Switch to daily",
                attrs! {
                    At::Href => Urls::new(&model.base_url).daily()
                }
            ],
        ),
    };
    div![
        format!(
            "Hello {}! This is your {} report for number: {}.",
            "DUMMY",
            frequency,
            state::global_count()
        ),
        link,
        div![
            button!["- ", state::global_count().on_click(|c| *c -= 1)],
            span![state::global_count().to_string()],
            button![" +", state::global_count().on_click(|c| *c += 1)],
        ]
    ]
}
