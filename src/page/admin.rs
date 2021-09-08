use seed::{prelude::*, *};

pub const NAMESPACE: &str = "admin";

mod report;

// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url, model: &mut Option<Model>) -> Option<()> {
    let model = model.get_or_insert_with(Model::default);
    model.base_url = Some(url.to_base_url());

    model.page_id.replace(match url.next_path_part() {
        Some(report::NAMESPACE) => {
            report::init(url, &mut model.report_model).map(|_| PageId::Report)?
        }
        _ => None?,
    });

    Some(())
}

// ------ ------
//     Model
// ------ ------

#[derive(Default, Clone)]
pub struct Model {
    base_url: Option<Url>,
    page_id: Option<PageId>,
    report_model: Option<report::Model>,
}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq)]
enum PageId {
    Report,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn report_urls(self) -> report::Urls<'a> {
        report::Urls::new(self.base_url().add_path_part(report::NAMESPACE))
    }
}

// ------ ------
//     View
// ------ ------

#[allow(clippy::single_match_else)]
pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    let base_url = model.base_url.clone();
    let report_model = model.report_model.as_ref().expect("report model").clone();
    match model.page_id {
        Some(PageId::Report) => report::view(report_model),
        None => crate::page::not_found::view(base_url.expect("Base url")),
    }
}
