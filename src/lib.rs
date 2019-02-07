use seed::*;
use seed::prelude::*;
use strum_macros::*;
mod header;
use header::*;
mod body_content;
use body_content::*;
mod about_content;
use std::str::FromStr;

#[derive(Clone)]
pub struct Model {
    current_page: Page
}

#[derive(Clone, PartialEq, Display, EnumString)]
pub enum Page {
    Fotos,
    Vagas,
    Sobre,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            current_page: Page::Sobre
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    ChangePageAndHistory(Page),
    ChangePage(Page),
}

/// The sole source of updating the model; returns a fresh one.
fn update(msg: Msg, model: Model) -> Update<Model> {
    match msg {
        // The change page pushes the history and THEN changes the page itself
        Msg::ChangePageAndHistory(page) => {
            // This just pushes the history
            seed::push_path(vec![&page.to_string()]);
            // Now we call the routing
            update(Msg::ChangePage(page), model)
        },

        // This is separate, because in-app naviation needs to call push_route,
        // but we don't want to call it from browser navigation. (eg back button)
        Msg::ChangePage(current_page) => Render (Model {current_page, ..model})
    }
}


// View


/// The top-level component we pass to the virtual dom.
fn view(state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
        div![header(model),
        body_content(model)]
}

fn routes(url: &seed::Url) -> Msg {
    if url.path.is_empty() {
        return Msg::ChangePage(Page::Sobre)
    }

    match Page::from_str(&url.path[0]){
        Ok(page) => return Msg::ChangePage(page),
        Err(_) => return Msg::ChangePage(Page::Sobre)
    };
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}