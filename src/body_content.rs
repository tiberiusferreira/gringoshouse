use crate::Msg;
use seed::*;
use seed::prelude::*;
use super::*;

pub fn body_content(model: &Model) -> El<Msg> {
    let vagas_attr = if model.current_page == Page::Vagas {
        attrs!{At::Class => "header-nav-item-button header-nav-item-button-active"}
    } else{
        attrs!{At::Class => "header-nav-item-button"}
    };
    div![
            attrs!{At::Class => "about-body-container"},
            div![
                attrs!{At::Class => "about-body-container-main"},
            ],
            div![
                attrs!{At::Class => "about-body-container-side"},
                h1![attrs!{At::Class => "about-body-container-side-h1"},
                    "Hello"
                    ],
                p![attrs!{At::Class => "about-body-container-side-para"},
                    "777 Nameless Ave
                    New York, NY 00000


                    template.placeholder@gmail.com

                    +1 555 555 5782"
                ]
            ]
        ]
}