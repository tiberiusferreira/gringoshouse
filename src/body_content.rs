use crate::Msg;
use seed::*;
use seed::prelude::*;
use super::*;
use about_content::about_body_content;
pub fn body_content(model: &Model) -> El<Msg> {
    match model.current_page {
        Page::Fotos => fotos_content::fotos_body_content(),
        Page::Vagas => {
            div![
                attrs!{At::Class => "vagas-body-container"},
                div![
                    attrs!{At::Class => "vagas-body-container-side"},
                    h1![attrs!{At::Class => "vagas-body-container-main-h1"},
                        "Temos 3 vagas abertas!"
                        ],
                    h2![attrs!{At::Class => "vagas-body-container-main-h2"},
                    "Benefícios de morar conosco"
                    ],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Casa grande, 10 quartos, 6 banheiros e sala."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "3 minutos a pé da entrada do IB e da FEA e 5 minutos do RU."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Internet 200Mb/s (Vivo fibra óptica) e WiFi com suporte a 200Mb/s e cobertura excelente."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Máquina de Espresso e moedor pros amantes de café."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Faxineira 2x por semana."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Contratamos o segurança da rua."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Alguns produtos de limpeza e alimentícios já estão inclusos no valor fixo (sal, açucar, óleo, etc)."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "1 cão da república e 1 cão de um morador."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Piscina, churrasqueira e área externa ampla."
                    ],
                    br![],
                    h2![attrs!{At::Class => "vagas-body-container-main-h2"},
                    "Procuramos pessoas"
                    ],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Responsáveis e proativas para resolver o problemas do dia-dia de uma república."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Tranquilas, que saibam equilibrar estudos e diversão."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Pessoas sem preconceitos."
                    ],
                    br![],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Pessoas que gostem que socializar com os moradores. Somos uma rep unida e não \"cada um no seu canto\"."
                    ],
                    br![],
                    h2![attrs!{At::Class => "vagas-body-container-main-h2"},
                    "Tem interesse na vaga? Fale conosco!"
                    ],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    a![attrs!{At::Href => "https://telegram.me/TiberioFerreira";
                     At::Target => "_blank"; At::Class => "vagas-body-container-main-link"}, "Telegram @TiberioFerreira"]
                    ],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    "Whatsapp (19) 98197-4942"
                    ],
                    p![attrs!{At::Class => "vagas-body-container-main-text"},
                    a![attrs!{At::Href =>  "https://www.facebook.com/GringosHouse/";
                     At::Target => "_blank"; At::Class => "vagas-body-container-main-link"}, "Facebook"]
                    ],
                ],
            ]
        },
        Page::Sobre => about_body_content(),
    }
}

/*
Vagas na Gringo's House! (República Mista)

Vaga em quarto para dividir com mais uma pessoa, para entrada imediata!
- Casa grande, 10 quartos, 6 banheiros e sala
- 3 minutos a pé da entrada do IB e da FEA e 5 minutos do RU.
- Internet 200Mb/s (Vivo fibra óptica) e WiFi com suporte a 200Mb/s e cobertura excelente
- Máquina de Espresso e moedor pros amantes de café
Valor fixo: R$600,00, com todas as contas inclusas
- Faxineira 2x por semana.
- Segurança da rua.
- Alguns produtos de limpeza e alimentícios já estão inclusos no valor fixo (sal, açucar, óleo, etc)
- 3 cães
- Piscina, churrasqueira e área externa ampla.

Tentamos tornar a casa um ambiente agradável de morar e estudar. Por isso procuramos pessoas que se integrem bem com os moradores da casa e saibam equilibrar estudo e diversão respeitando o espaço um do outro.

Somos majoritariamente de exatas, mas estamos abertos a alunos de qualquer curso da Unicamp, PucCamp, e Facamp, que estejam dispostos a investir tempo e esforço para não só manter a república, mas torná-la cada vez melhor.

Contato inbox ou Telegram @TiberioFerreira / Whatsapp (19) 98197-4942
*/