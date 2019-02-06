use crate::Msg;
use seed::*;
use seed::prelude::*;
use super::*;

pub fn header(model: &Model) -> El<Msg> {
    let fotos_attr = if model.current_page == Page::Fotos {
        attrs!{At::Class => "header-nav-item-button header-nav-item-button-active"}
    } else{
        attrs!{At::Class => "header-nav-item-button"}
    };
    let sobre_attr = if model.current_page == Page::Sobre {
        attrs!{At::Class => "header-nav-item-button header-nav-item-button-active"}
    } else{
        attrs!{At::Class => "header-nav-item-button"}
    };
    let contato_attr = if model.current_page == Page::Contato {
        attrs!{At::Class => "header-nav-item-button header-nav-item-button-active"}
    } else{
        attrs!{At::Class => "header-nav-item-button"}
    };
    let vagas_attr = if model.current_page == Page::Vagas {
        attrs!{At::Class => "header-nav-item-button header-nav-item-button-active"}
    } else{
        attrs!{At::Class => "header-nav-item-button"}
    };
    div![ attrs!{At::Class => "header-container"},
                ul![attrs!{At::Class => "header-nav-container"},
                        li![
                            attrs!{At::Class => "header-nav-item"},
                            a![simple_ev("click", Msg::ChangePage(Page::Fotos)), fotos_attr, "Fotos"]
                            ],
                        li![
                            attrs!{At::Class => "header-nav-item"},
                            a![simple_ev("click", Msg::ChangePage(Page::Sobre)), sobre_attr, "Sobre"]
                            ],
                        li![
                            attrs!{At::Class => "header-nav-item"},
                            a![simple_ev("click", Msg::ChangePage(Page::Contato)), contato_attr, "Contato"]
                            ],
                        li![
                            attrs!{At::Class => "header-nav-item"},
                            a![simple_ev("click", Msg::ChangePage(Page::Vagas)), vagas_attr, "Vagas"]
                            ],
                ],
                div![ attrs!{At::Class => "logo-container"}, span!["Gringos House"]],
        ]
}