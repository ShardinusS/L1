//! Entraînement : exercices corrigés automatiquement, matière par matière.
//!
//! Chaque matière découpe son programme en chapitres. Un chapitre renvoie à
//! une section du cours (pour « revoir la fiche ») et réunit deux sortes
//! d'exercices :
//!
//! - des questions fixes, écrites à partir du cours et des TD;
//! - des générateurs, qui tirent de nouvelles valeurs à chaque appel : une
//!   conversion, une trace, une permutation ne se ressemblent jamais deux fois.
//!
//! Tout ce module est indépendant du DOM et se teste sur la machine hôte.

mod algo;
pub mod answer;
mod info;
pub mod math;
mod mtc;
mod os;
pub mod progress;
mod rng;
mod sf;
mod sf_gen;

#[cfg(test)]
mod tests;

pub use answer::{Answer, Response};
pub use rng::Rng;

use crate::course::{Course, COURSES};

/// Fabrique un exercice neuf à chaque appel.
pub type Generator = fn(&mut Rng) -> Exercise;

/// Nombre de fois qu'un générateur figure dans la pioche : il vaut plusieurs
/// questions fixes puisqu'il ne s'épuise pas.
const GENERATOR_WEIGHT: usize = 3;

#[derive(Debug)]
pub struct Chapter {
    /// Clé des statistiques, stable : ne pas renommer.
    pub id: &'static str,
    pub title: &'static str,
    /// Section du cours à revoir.
    pub anchor: &'static str,
    pub fixed: &'static [Fixed],
    pub generators: &'static [Generator],
}

/// Question écrite à la main.
#[derive(Debug)]
pub struct Fixed {
    pub prompt: &'static str,
    /// Bloc en chasse fixe affiché sous l'énoncé; vide s'il n'y en a pas.
    pub code: &'static str,
    pub answer: FixedAnswer,
    pub explain: &'static str,
}

#[derive(Debug)]
pub enum FixedAnswer {
    /// Choix multiple : la **première** option est la bonne, l'ordre est
    /// mélangé au tirage.
    Choice(&'static [&'static str]),
    TrueFalse(bool),
    Text(&'static [&'static str]),
    Number(i64),
    Set(&'static [i64]),
    Card(&'static str),
}

pub const fn qcm(
    prompt: &'static str,
    options: &'static [&'static str],
    explain: &'static str,
) -> Fixed {
    Fixed {
        prompt,
        code: "",
        answer: FixedAnswer::Choice(options),
        explain,
    }
}

pub const fn vf(prompt: &'static str, truth: bool, explain: &'static str) -> Fixed {
    Fixed {
        prompt,
        code: "",
        answer: FixedAnswer::TrueFalse(truth),
        explain,
    }
}

pub const fn txt(
    prompt: &'static str,
    accepted: &'static [&'static str],
    explain: &'static str,
) -> Fixed {
    Fixed {
        prompt,
        code: "",
        answer: FixedAnswer::Text(accepted),
        explain,
    }
}

pub const fn num(prompt: &'static str, value: i64, explain: &'static str) -> Fixed {
    Fixed {
        prompt,
        code: "",
        answer: FixedAnswer::Number(value),
        explain,
    }
}

pub const fn set(prompt: &'static str, elems: &'static [i64], explain: &'static str) -> Fixed {
    Fixed {
        prompt,
        code: "",
        answer: FixedAnswer::Set(elems),
        explain,
    }
}

/// Carte de révision : recto, verso.
pub const fn card(prompt: &'static str, back: &'static str) -> Fixed {
    Fixed {
        prompt,
        code: "",
        answer: FixedAnswer::Card(back),
        explain: "",
    }
}

impl Fixed {
    pub const fn code(self, code: &'static str) -> Self {
        Fixed { code, ..self }
    }

    fn to_exercise(&self, rng: &mut Rng) -> Exercise {
        let answer = match self.answer {
            FixedAnswer::Choice(options) => shuffled_choice(
                rng,
                options[0].to_string(),
                options[1..].iter().map(|o| o.to_string()),
            ),
            FixedAnswer::TrueFalse(truth) => true_false(truth),
            FixedAnswer::Text(accepted) => Answer::Text {
                accepted: accepted.iter().map(|a| a.to_string()).collect(),
            },
            FixedAnswer::Number(value) => Answer::Number { value, radix: 10 },
            FixedAnswer::Set(elems) => Answer::Set {
                elems: elems.to_vec(),
            },
            FixedAnswer::Card(back) => Answer::Card {
                back: back.to_string(),
            },
        };
        let ex = Exercise::new(self.prompt, answer, self.explain);
        if self.code.is_empty() {
            ex
        } else {
            ex.with_code(self.code)
        }
    }
}

/// Un exercice prêt à poser.
#[derive(Clone, Debug, PartialEq)]
pub struct Exercise {
    pub prompt: String,
    pub code: Option<String>,
    pub answer: Answer,
    pub explain: String,
}

impl Exercise {
    pub fn new(prompt: impl Into<String>, answer: Answer, explain: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            code: None,
            answer,
            explain: explain.into(),
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

/// Choix multiple mélangé; les leurres identiques à la bonne réponse ou
/// entre eux sont écartés.
pub fn shuffled_choice(
    rng: &mut Rng,
    right: String,
    wrong: impl IntoIterator<Item = String>,
) -> Answer {
    let mut options = vec![right.clone()];
    for w in wrong {
        if !options.contains(&w) {
            options.push(w);
        }
    }
    rng.shuffle(&mut options);
    let correct = options.iter().position(|o| *o == right).unwrap_or(0);
    Answer::Choice { options, correct }
}

pub fn true_false(truth: bool) -> Answer {
    Answer::Choice {
        options: vec!["Vrai".into(), "Faux".into()],
        correct: if truth { 0 } else { 1 },
    }
}

pub fn yes_no(yes: bool) -> Answer {
    Answer::Choice {
        options: vec!["Oui".into(), "Non".into()],
        correct: if yes { 0 } else { 1 },
    }
}

/// Exposant typographique : `sup(12)` = `¹²`.
pub fn sup(n: u32) -> String {
    const DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    n.to_string()
        .bytes()
        .map(|b| DIGITS[(b - b'0') as usize])
        .collect()
}

/// Indice typographique : `sub(16)` = `₁₆`.
pub fn sub(n: u32) -> String {
    const DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];
    n.to_string()
        .bytes()
        .map(|b| DIGITS[(b - b'0') as usize])
        .collect()
}

/// Une question tirée, avec de quoi la ranger et renvoyer au cours.
#[derive(Clone, Debug, PartialEq)]
pub struct Question {
    pub course: Course,
    pub chapter_id: &'static str,
    pub chapter_title: &'static str,
    pub anchor: &'static str,
    pub exercise: Exercise,
}

impl Question {
    /// Lien vers la section du cours correspondante.
    pub fn href(&self) -> String {
        format!("{}#{}", self.course.route(), self.anchor)
    }
}

pub fn chapters(course: Course) -> &'static [Chapter] {
    match course {
        Course::Sf => sf::CHAPTERS,
        Course::Mtc => mtc::CHAPTERS,
        Course::Algo => algo::CHAPTERS,
        Course::Info => info::CHAPTERS,
        Course::Os => os::CHAPTERS,
    }
}

/// Partie du programme sur laquelle porte une série.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Filter {
    /// Toutes les matières mélangées.
    All,
    Course(Course),
    Chapter(Course, &'static str),
}

impl Filter {
    fn selected(self) -> Vec<(Course, &'static Chapter)> {
        let courses: Vec<Course> = match self {
            Filter::All => COURSES.to_vec(),
            Filter::Course(c) | Filter::Chapter(c, _) => vec![c],
        };
        courses
            .into_iter()
            .flat_map(|c| chapters(c).iter().map(move |ch| (c, ch)))
            .filter(|(_, ch)| match self {
                Filter::Chapter(_, id) => ch.id == id,
                _ => true,
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
enum Source {
    Fixed(&'static Fixed),
    Generated(Generator),
}

/// Tire une série de `count` questions distinctes dans la partie choisie.
///
/// Les questions fixes ne sortent qu'une fois par tour de pioche; les
/// générateurs, plus lourds dans la pioche, complètent la série.
pub fn draw(filter: Filter, count: usize, rng: &mut Rng) -> Vec<Question> {
    let mut pool: Vec<(Course, &'static Chapter, Source)> = Vec::new();
    for (course, chapter) in filter.selected() {
        for f in chapter.fixed {
            pool.push((course, chapter, Source::Fixed(f)));
        }
        for g in chapter.generators {
            for _ in 0..GENERATOR_WEIGHT {
                pool.push((course, chapter, Source::Generated(*g)));
            }
        }
    }
    if pool.is_empty() {
        return Vec::new();
    }
    rng.shuffle(&mut pool);

    let mut out: Vec<Question> = Vec::with_capacity(count);
    for attempt in 0..count * 20 {
        if out.len() == count {
            break;
        }
        let (course, chapter, source) = pool[attempt % pool.len()];
        let exercise = match source {
            Source::Fixed(f) => f.to_exercise(rng),
            Source::Generated(g) => g(rng),
        };
        let duplicate = out
            .iter()
            .any(|q| q.exercise.prompt == exercise.prompt && q.exercise.code == exercise.code);
        if duplicate {
            continue;
        }
        out.push(Question {
            course,
            chapter_id: chapter.id,
            chapter_title: chapter.title,
            anchor: chapter.anchor,
            exercise,
        });
    }
    out
}
