use crate::Msg;
use seed::*;
use seed::prelude::*;
use super::*;

pub fn about_body_content() -> El<Msg> {

    let title = "A Gringos de Hoje";
    let paragraph_1 = p![attrs!{At::Class => "about-body-container-main-text"},
                    "República Mista tradicional localizada em Barão Geraldo. \
                    Composta majoritariamente por estudantes da Unicamp, \
                    mas possuímos também estudantes da FACAMP, \
                    PUC-Campinas e pré-vestibulandos. Procuramos \
                    ser pessoas mente aberta e livre de preconceitos. \
                    Fazemos parte da ", a![attrs!{At::Href => "http://republicasunicamp.com.br/";
                     At::Target => "_blank"; At::Class => "about-body-container-main-link"}, "ARU"],
                      " - Associação de Repúblicas da UNICAMP."
                ];
    let paragraph_2 = "O nosso objetivo principal é ser um lugar agradável e propício aos estudos. \
    Como alunos, ajudamos uns aos outros com as disciplinas da universidade, estudando juntos na sala sempre que possível.";
    let paragraph_3 = "Temos conciência que a vida universitária vai além da sala de aula. \
    Dessa forma valorizamos discussões enriquecedoras, principalmente sobre temas polêmicos.";
    let paragraph_4 = "Devido a reclamações de vizinhos quanto ao barulho ocasionado por festas \
    e principalmente devido à rotina dos moradores não fazemos festas ou alugamos a casa para eventos.";
    let paragraph_5 = "Isto não impede de irmos em festas, bares, ou mesmo tomarmos uma cerveja \
    ou outra ao redor da piscina.";
        div![
            attrs!{At::Class => "about-body-container"},
            div![
                h1![attrs!{At::Class => "about-body-container-main-h1"},
                    title
                    ],
                h2![attrs!{At::Class => "about-body-container-main-h2"},
                "Quem somos"
                ],
                paragraph_1,
                br![],
                h2![attrs!{At::Class => "about-body-container-main-h2"},
                "Nossa Filosofia"
                ],
                p![attrs!{At::Class => "about-body-container-main-text"},
                    paragraph_2
                ],
                br![],
                p![attrs!{At::Class => "about-body-container-main-text"},
                    paragraph_3
                ],
                h2![attrs!{At::Class => "about-body-container-main-h2"},
                "Festas e Eventos"
                ],
                p![attrs!{At::Class => "about-body-container-main-text"},
                    paragraph_4
                ],
                p![attrs!{At::Class => "about-body-container-main-text"},
                    paragraph_5
                ],
            ],
        ]
}


// mente aberta // livres de preconceitos // dona helena nossa mãe // ARU