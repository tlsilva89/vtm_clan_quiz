use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use slint::{Model, SharedString, VecModel};

slint::include_modules!();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Clan {
    Brujah, Gangrel, Malkavian, Nosferatu, Toreador, Tremere, Ventrue,
    Lasombra, Tzimisce, Hecata, BanuHaqim, TheMinistry, Caitiff,
}

impl Clan {
    fn get_quote(&self) -> &'static str {
        match self {
            Clan::Brujah => "A revolução não pede permissão. A ira é nossa virtude, a liberdade nossa causa.",
            Clan::Gangrel => "A civilização é uma ilusão. A verdade está no sangue, nas garras, na natureza selvagem.",
            Clan::Malkavian => "A loucura é clareza. Vemos o que outros temem olhar, sabemos o que outros negam.",
            Clan::Nosferatu => "Nas sombras, somos reis. Informação é poder, e nós vemos tudo que você esconde.",
            Clan::Toreador => "A beleza é eterna, a arte é divina. Somos os guardiões da paixão em um mundo morto.",
            Clan::Tremere => "O conhecimento é supremacia. Dominamos o sangue através da vontade e do ritual.",
            Clan::Ventrue => "Governar é nossa natureza. A ordem é nossa dádiva, a liderança nosso fardo sagrado.",
            Clan::Lasombra => "Das trevas ascendemos. Somos sombras que devoram a luz, mestres do poder absoluto.",
            Clan::Tzimisce => "A carne é argila, a vontade é o escultor. Transcendemos as limitações da forma mortal.",
            Clan::Hecata => "A morte não é o fim, é apenas o começo. Caminhamos entre mundos, senhores da mortalidade.",
            Clan::BanuHaqim => "Justiça através do sangue. Somos a espada que pune os transgressores da lei eterna.",
            Clan::TheMinistry => "Liberte-se das correntes. Mostramos o caminho para os verdadeiros desejos ocultos.",
            Clan::Caitiff => "Sem legado, sem correntes. Nossa força está na sobrevivência, nossa liberdade no abandono.",
        }
    }

    fn get_name(&self) -> &'static str {
        match self {
            Clan::Brujah => "Brujah",
            Clan::Gangrel => "Gangrel",
            Clan::Malkavian => "Malkavian",
            Clan::Nosferatu => "Nosferatu",
            Clan::Toreador => "Toreador",
            Clan::Tremere => "Tremere",
            Clan::Ventrue => "Ventrue",
            Clan::Lasombra => "Lasombra",
            Clan::Tzimisce => "Tzimisce",
            Clan::Hecata => "Hecata",
            Clan::BanuHaqim => "Banu Haqim",
            Clan::TheMinistry => "Ministry",
            Clan::Caitiff => "Caitiff",
        }
    }
}

struct Question {
    text: &'static str,
    answers: Vec<(&'static str, Clan)>,
}

struct QuizState {
    questions: Vec<Question>,
    current_question_index: usize,
    scores: HashMap<Clan, i32>,
}

fn get_quiz_questions() -> Vec<Question> {
    vec![
        Question { text: "Uma injustiça grave acontece na sua frente. Como você reage?", answers: vec![ ("Intervenho com a força que for necessária. Opressão não fica sem resposta.", Clan::Brujah), ("Observo das sombras, aprendo quem são os envolvidos para usar depois.", Clan::Nosferatu), ("Analiso os riscos e ajo para garantir a ordem e o melhor resultado a longo prazo.", Clan::Ventrue), ("Manipulo a situação sutilmente para que os próprios envolvidos se destruam.", Clan::Lasombra), ] },
        Question { text: "Você se encontra em uma cidade nova. Qual é sua primeira prioridade?", answers: vec![ ("Encontrar os lugares mais isolados e escondidos.", Clan::Nosferatu), ("Descobrir quem está no poder e como posso subir na estrutura social.", Clan::Ventrue), ("Explorar, sentir o pulso da cidade, especialmente as áreas mais selvagens.", Clan::Gangrel), ("Procurar os lugares de arte, cultura e beleza. Onde a alma da cidade reside.", Clan::Toreador), ] },
        Question { text: "O que significa 'poder' para você?", answers: vec![ ("Autoridade e liderança. A capacidade de governar e manter a ordem.", Clan::Ventrue), ("Liberdade. O poder de quebrar correntes e viver pelas minhas próprias regras.", Clan::Brujah), ("Conhecimento. Saber o que ninguém mais sabe.", Clan::Tremere), ("Domínio. O controle absoluto sobre meu ambiente e aqueles que o habitam.", Clan::Tzimisce), ] },
        Question { text: "Um rival te humilha publicamente. Como você planeja sua vingança?", answers: vec![ ("Desmonto a vida dele peça por peça, social e financeiramente. A vingança é um prato frio.", Clan::Lasombra), ("Eu o confronto diretamente e o destruo na frente de todos, para que sirva de exemplo.", Clan::Brujah), ("Eu o transformo em uma piada. Uso minha perspicácia para espalhar uma verdade inconveniente sobre ele.", Clan::Malkavian), ("Agressão direta é vulgar. Eu vou possuir tudo o que ele ama.", Clan::Tzimisce), ] },
        Question { text: "Como você vê a humanidade?", answers: vec![ ("Um rebanho a ser guiado e protegido, para o bem de todos.", Clan::Ventrue), ("Fonte de inspiração, prazer e, claro, sustento.", Clan::Toreador), ("Peças em um tabuleiro de xadrez, a serem movidas para meus próprios fins.", Clan::Lasombra), ("Um sistema corrupto e opressor que precisa ser derrubado.", Clan::Brujah), ] },
        Question { text: "O que você teme mais?", answers: vec![ ("Perder o controle para meus impulsos mais primitivos.", Clan::Brujah), ("Ser esquecido e insignificante.", Clan::Toreador), ("Ser preso, seja fisicamente, socialmente ou espiritualmente.", Clan::TheMinistry), ("Ser exposto à luz, ter meus segredos revelados a todos.", Clan::Nosferatu), ] },
        Question { text: "Você precisa obter uma informação. Qual método você prefere?", answers: vec![ ("Ofereço algo que a pessoa deseja desesperadamente, criando uma dívida.", Clan::TheMinistry), ("Intimidação sutil e controle mental. A pessoa me dará a informação e acreditará que a ideia foi dela.", Clan::Lasombra), ("Uso de rituais e conhecimento arcano para extrair a verdade.", Clan::Tremere), ("Eu já sei a informação. As pessoas falam quando acham que ninguém está ouvindo.", Clan::Nosferatu), ] },
        Question { text: "Qual ambiente te deixa mais confortável?", answers: vec![ ("Uma biblioteca antiga, cheia de conhecimento proibido.", Clan::Tremere), ("Uma floresta densa e escura, longe da civilização.", Clan::Gangrel), ("Uma cobertura de luxo, com vista para a cidade que eu controlo.", Clan::Ventrue), ("Meu laboratório/oficina, onde posso moldar a carne e o osso.", Clan::Tzimisce), ] },
        Question { text: "Como você enxerga a loucura?", answers: vec![ ("É uma forma de clareza, uma janela para uma verdade maior.", Clan::Malkavian), ("É uma fraqueza perigosa que deve ser controlada ou eliminada.", Clan::Ventrue), ("É uma ferramenta útil para desestabilizar os outros.", Clan::Lasombra), ("É uma condição trágica que aflige os fracos.", Clan::Caitiff), ] },
        Question { text: "A morte para você é...", answers: vec![ ("Apenas o começo de uma nova forma de negócio.", Clan::Hecata), ("Um enigma a ser estudado e, talvez, dominado.", Clan::Tremere), ("Uma barreira a ser transcendida, uma limitação a ser superada.", Clan::Tzimisce), ("Uma ferramenta para impor justiça e punir os transgressores.", Clan::BanuHaqim), ] },
        Question { text: "Sua lealdade pertence a quê, acima de tudo?", answers: vec![ ("À minha família e aos negócios que mantemos juntos.", Clan::Hecata), ("A um código de leis ou a uma fé que guia minhas ações.", Clan::BanuHaqim), ("A mim mesmo e à minha sobrevivência. O resto é secundário.", Clan::Gangrel), ("À hierarquia e à estrutura que nos mantém fortes e seguros.", Clan::Tremere), ] },
        Question { text: "Qual é a sua relação com a beleza e a arte?", answers: vec![ ("É a única coisa que torna a existência suportável. Eu a busco obsessivamente.", Clan::Toreador), ("É uma ferramenta de influência e status.", Clan::Ventrue), ("É uma distração frívola dos assuntos mais importantes.", Clan::BanuHaqim), ("A verdadeira beleza está na singularidade, mesmo que seja grotesca.", Clan::Tzimisce), ] },
        Question { text: "Como você lida com regras impostas a você?", answers: vec![ ("Eu as quebro. Regras foram feitas para serem desafiadas.", Clan::Brujah), ("Eu as sigo, pois a estrutura é essencial para a sobrevivência de todos.", Clan::Ventrue), ("Eu aprendo a contorná-las. As melhores regras são as que ninguém sabe que existem.", Clan::TheMinistry), ("Regras são para os outros. Eu existo fora delas.", Clan::Gangrel), ] },
        Question { text: "O que você busca em um refúgio?", answers: vec![ ("Segurança e total isolamento do mundo exterior.", Clan::Nosferatu), ("Um lugar que reflita meu poder e meu status.", Clan::Ventrue), ("Um local que seja intrinsecamente meu, ligado à minha própria essência.", Clan::Tzimisce), ("Não preciso de um. Meu refúgio é a estrada aberta.", Clan::Gangrel), ] },
        Question { text: "Você descobre uma verdade chocante sobre o mundo. O que você faz?", answers: vec![ ("Compartilho com todos, de forma enigmática. O caos que se segue é a prova da verdade.", Clan::Malkavian), ("Vendo a informação para quem pagar mais.", Clan::Nosferatu), ("Uso-a para fortalecer minha posição e minar meus inimigos.", Clan::Lasombra), ("Guardo-a para estudo. Conhecimento é algo a ser acumulado, não desperdiçado.", Clan::Tremere), ] },
        Question { text: "Qual sua opinião sobre fé e religião?", answers: vec![ ("É uma ferramenta poderosa para controlar as massas e obter poder.", Clan::Lasombra), ("É uma corrente da qual as pessoas precisam ser libertadas, para abraçar seus verdadeiros desejos.", Clan::TheMinistry), ("É a base de um código moral que separa a justiça da anarquia.", Clan::BanuHaqim), ("É uma ilusão reconfortante para os fracos.", Clan::Brujah), ] },
        Question { text: "Como você prefere que seus inimigos te vejam?", answers: vec![ ("Eles não deveriam me ver de forma alguma.", Clan::Nosferatu), ("Como uma força da natureza, inevitável e implacável.", Clan::Gangrel), ("Como seu superior em todos os aspectos.", Clan::Ventrue), ("Como a personificação de seus piores pesadelos.", Clan::Tzimisce), ] },
        Question { text: "Seu relacionamento mais importante é com...", answers: vec![ ("...minha arte/musa.", Clan::Toreador), ("...meus ancestrais/espíritos.", Clan::Hecata), ("...meus animais/a natureza.", Clan::Gangrel), ("...minhas ideias/minha causa.", Clan::Brujah), ] },
        Question { text: "O que é mais importante em um julgamento?", answers: vec![ ("A aplicação estrita da lei, sem exceções.", Clan::BanuHaqim), ("O resultado que mais beneficia a estabilidade da comunidade.", Clan::Ventrue), ("A verdade, não importa quão inconveniente ou perturbadora ela seja.", Clan::Malkavian), ("A oportunidade de adquirir influência sobre os envolvidos.", Clan::Lasombra), ] },
        Question { text: "Você está sozinho. O que você sente?", answers: vec![ ("Paz. É meu estado natural.", Clan::Gangrel), ("Tédio. Preciso de estímulo, de beleza, de interação.", Clan::Toreador), ("Poder. A solidão é o laboratório da mente.", Clan::Tremere), ("Vulnerabilidade. Há força nos números e na estrutura.", Clan::Caitiff), ] },
    ]
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    let quiz_state = Rc::new(RefCell::new(QuizState {
        questions: get_quiz_questions(),
        current_question_index: 0,
        scores: HashMap::new(),
    }));

    fn update_question(app_handle: &AppWindow, state: &QuizState) {
        let question = &state.questions[state.current_question_index];
        app_handle.set_question_text(question.text.into());

        let answer_model = Rc::new(VecModel::from(
            question
                .answers
                .iter()
                .map(|(text, _)| SharedString::from(*text))
                .collect::<Vec<_>>(),
        ));
        app_handle.set_answers(answer_model.into());
    }

    update_question(&app, &quiz_state.borrow());

    let quiz_state_clone = quiz_state.clone();
    let app_handle = app.as_weak();

    app.on_answer_selected(move |index| {
        let app = app_handle.upgrade().unwrap();
        let mut state = quiz_state_clone.borrow_mut();

        let chosen_answer = &state.questions[state.current_question_index].answers[index as usize];
        let clan_to_score = chosen_answer.1;

        *state.scores.entry(clan_to_score).or_insert(0) += 1;
        state.current_question_index += 1;

        if state.current_question_index < state.questions.len() {
            update_question(&app, &state);
        } else {
            let final_clan = state.scores
                .iter()
                .max_by_key(|&(_, score)| score)
                .map(|(clan, _)| *clan)
                .unwrap_or(Clan::Caitiff);

            let result_text = format!("Clã {}", final_clan.get_name());
            
            app.set_question_text(result_text.into());
            app.set_clan_quote(final_clan.get_quote().into());
            let empty_answers = Rc::new(VecModel::from(Vec::<SharedString>::new()));
            app.set_answers(empty_answers.into());
            app.set_quiz_active(false);
        }
    });

    let quiz_state_clone2 = quiz_state.clone();
    let app_handle2 = app.as_weak();
    app.on_restart_quiz(move || {
        let app = app_handle2.upgrade().unwrap();
        let mut state = quiz_state_clone2.borrow_mut();

        state.current_question_index = 0;
        state.scores.clear();
        
        update_question(&app, &state);
        app.set_quiz_active(true);
    });

    app.run()
}
