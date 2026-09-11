//! Entraînement : l'accueil des séries et une page par matière.
//!
//! Écrit à la main, contrairement aux pages de matière transposées du
//! classeur HTML : `tools/generate.py` ne touche pas à ce fichier.

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::quiz::Quiz;
use crate::course::{Course, COURSES};
use crate::practice::chapters;
use crate::practice::progress::{answers_label, Progress};

/// Segment d'URL de la série qui mélange toutes les matières.
const MIXED: &str = "melange";

/// Numéro de la matière, comme sur les cartes de l'accueil.
fn number(course: Course) -> &'static str {
    match course {
        Course::Sf => "01",
        Course::Mtc => "02",
        Course::Algo => "03",
        Course::Info => "04",
        Course::Os => "05",
    }
}

fn course_card(course: Course, progress: &Progress) -> impl IntoView {
    let tally = progress.course(course);
    let stats = match tally.rate() {
        Some(r) => format!("{} · {r} %", answers_label(tally.asked)),
        None => "jamais pratiqué".to_string(),
    };
    let weak = progress
        .weakest(course)
        .filter(|(_, rate)| *rate < 80)
        .map(|(ch, _)| format!("à revoir : {}", ch.title));
    let titles: Vec<&str> = chapters(course).iter().map(|ch| ch.title).collect();
    view! {
        <A href=course.practice_route() scroll=false attr:class=format!("mat-card c-{}", course.key())>
            <span class="bar"></span>
            <span class="in">
                <span class="k">
                    {format!("Matière {} · {} chapitres", number(course), titles.len())}
                </span>
                <h3>{course.label()}</h3>
                <p>{titles.join(" · ")}</p>
                <span class="facts">
                    <span>{stats}</span>
                    {weak.map(|w| view! { <span>{w}</span> })}
                </span>
            </span>
        </A>
    }
}

/// Accueil de l'entraînement : une carte par matière et la série mélangée.
pub fn hub() -> impl IntoView {
    let progress = Progress::load();
    let all = COURSES.iter().flat_map(|c| chapters(*c));
    let fixed: usize = all.clone().map(|ch| ch.fixed.len()).sum();
    let generators: usize = all.map(|ch| ch.generators.len()).sum();
    let total = progress.total();
    view! {
        <div class="course">
            <header class="hero">
                <div class="hero-inner">
                    <div class="eyebrow">"Entraînement"</div>
                    <h1>"S'exercer, matière par matière"</h1>
                    <p class="lede">
                        "Des séries courtes, corrigées sur-le-champ, où chaque erreur renvoie à la fiche du cours qui l'explique. Conversions, traces, restes modulo, permutations\u{a0}: la plupart des exercices sont tirés au hasard, il y en a toujours de nouveaux."
                    </p>
                    <div class="meta">
                        <span>
                            <span class="mono">{fixed}</span>
                            " questions rédigées"
                        </span>
                        <span>
                            <span class="mono">{generators}</span>
                            " générateurs d'exercices"
                        </span>
                        <span>
                            {match total.rate() {
                                Some(r) => format!(
                                    "{} {} · {r} % de réussite",
                                    answers_label(total.asked),
                                    if total.asked > 1 { "enregistrées" } else { "enregistrée" },
                                ),
                                None => "Aucune réponse enregistrée pour l'instant".to_string(),
                            }}
                        </span>
                    </div>
                </div>
            </header>
            <div class="shell wide">
                <main>
                    <section>
                        <div class="sec-head">
                            <span class="num">"—"</span>
                            <h2>"Choisir une matière"</h2>
                        </div>
                        <div class="mat-grid">
                            {COURSES.iter().map(|c| course_card(*c, &progress)).collect_view()}
                            <A href=format!("/entrainement/{MIXED}") scroll=false attr:class="mat-card">
                                <span class="bar"></span>
                                <span class="in">
                                    <span class="k">"Révision express"</span>
                                    <h3>"Série mélangée"</h3>
                                    <p>
                                        "Les cinq matières dans une même série\u{a0}: cinq minutes avant un TD, ou pour vérifier que rien ne s'est effacé."
                                    </p>
                                    <span class="facts">
                                        <span>"5 matières"</span>
                                    </span>
                                </span>
                            </A>
                        </div>
                    </section>
                    <section>
                        <div class="sec-head">
                            <span class="num">"—"</span>
                            <h2>"Mode d'emploi"</h2>
                        </div>
                        <div class="body">
                            <p>
                                "Choisis un chapitre, ou garde tout le cours, et la longueur de la série. Une réponse tapée est corrigée avec indulgence sur la forme — majuscules, accents, espaces, zéros de tête, ordre des éléments d'un ensemble, point de départ d'un cycle — jamais sur le fond."
                            </p>
                            <p>
                                "Les cartes de révision (définitions, rédactions types) se retournent\u{a0}: c'est toi qui dis si tu savais. Les statistiques restent dans ce navigateur et font ressortir ton point faible dans chaque matière."
                            </p>
                        </div>
                        <div class="note" style="margin-top:18px">
                            <span class="title">"Au clavier"</span>
                            <ul>
                                <li>
                                    <kbd>"1"</kbd>" à "<kbd>"4"</kbd>" choisissent une réponse proposée."
                                </li>
                                <li>
                                    <kbd>"Entrée"</kbd>" valide une réponse tapée, puis passe à la question suivante."
                                </li>
                                <li>
                                    "Sur une carte\u{a0}: "<kbd>"Entrée"</kbd>" la retourne, "<kbd>"1"</kbd>" je savais, "<kbd>"2"</kbd>" à revoir."
                                </li>
                            </ul>
                        </div>
                    </section>
                </main>
            </div>
        </div>
    }
}

fn practice_view(scope: Option<Course>) -> impl IntoView {
    let (class, eyebrow, title, lede) = match scope {
        Some(c) => (
            format!("course c-{}", c.key()),
            format!("Entraînement · Matière {}", number(c)),
            c.label(),
            "Choisis un chapitre ou garde tout le cours. Chaque correction renvoie à la fiche qui l'explique.",
        ),
        None => (
            "course".to_string(),
            "Entraînement · Révision express".to_string(),
            "Série mélangée",
            "Des questions des cinq matières, dans le désordre.",
        ),
    };
    view! {
        <div class=class>
            <header class="masthead">
                <div class="masthead-inner">
                    <div class="eyebrow">{eyebrow}</div>
                    <h1>{title}</h1>
                    <p class="lede">{lede}</p>
                    <div class="meta">
                        <A href="/entrainement" scroll=false>"← Tous les entraînements"</A>
                        {scope.map(|c| view! { <A href=c.route() scroll=false>"Ouvrir le cours"</A> })}
                    </div>
                </div>
            </header>
            <div class="shell wide">
                <main>
                    <Quiz scope=scope/>
                </main>
            </div>
        </div>
    }
}

/// Page `/entrainement/:matiere`; une matière inconnue ramène à l'accueil.
pub fn course_page() -> impl IntoView {
    let params = use_params_map();
    move || {
        let slug = params.with(|p| p.get("matiere")).unwrap_or_default();
        if slug == MIXED {
            return practice_view(None).into_any();
        }
        match Course::from_slug(&slug) {
            Some(course) => practice_view(Some(course)).into_any(),
            None => hub().into_any(),
        }
    }
}
