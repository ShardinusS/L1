//! Page « Prompts » : un prompt d'apprentissage par chapitre, prêt à copier.
//!
//! Écrite à la main : `tools/generate.py` ne touche pas à ce fichier.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::course::{Course, COURSES};
use crate::prompts::{prompts, total, ChapterPrompt, PREAMBLE};

/// Texte réellement copié : le préambule commun, puis le corps du prompt.
fn full_text(p: &ChapterPrompt) -> String {
    format!(
        "{PREAMBLE}\n\nMatière : {} — chapitre {} « {} ».\n\n{}",
        p.course.label(),
        p.num(),
        p.title,
        p.body
    )
}

fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(text);
    }
}

#[component]
fn PromptCard(entry: &'static ChapterPrompt) -> impl IntoView {
    let copied = RwSignal::new(false);
    let text = full_text(entry);
    let to_copy = text.clone();

    view! {
        <article class=format!("pr-card c-{}", entry.course.key())>
            <header class="pr-head">
                <span class="pr-num">{entry.num()}</span>
                <div class="pr-title">
                    <h3>{entry.title}</h3>
                    <span class="pr-course">{entry.course.label()}</span>
                </div>
                <span class=format!("pr-level lv-{}", entry.level.key())>
                    {entry.level.label()}
                </span>
            </header>
            <p class="pr-evidence">
                <span class="cat">"Ce que disent les sujets"</span>
                {entry.evidence}
            </p>
            <details class="pr-body">
                <summary>"Lire le prompt"</summary>
                <pre class="pr-text">{text}</pre>
            </details>
            <footer class="pr-actions">
                <button
                    class="btn"
                    type="button"
                    on:click=move |_| {
                        copy_to_clipboard(&to_copy);
                        copied.set(true);
                    }
                >
                    {move || if copied.get() { "Copié ✓" } else { "Copier le prompt" }}
                </button>
                <a class="pr-link" href=entry.href()>
                    "Revoir la fiche"
                </a>
                <A href=entry.practice_href() scroll=false attr:class="pr-link">
                    "S'entraîner"
                </A>
            </footer>
        </article>
    }
}

fn course_block(course: Course) -> impl IntoView {
    let list = prompts(course);
    view! {
        <section id=format!("pr-{}", course.key())>
            <div class="sec-head">
                <span class="num">{course.short()}</span>
                <h2>{course.label()}</h2>
            </div>
            <div class="body">
                <p>
                    {format!(
                        "{} chapitres, {} prompts. Chaque prompt borne explicitement le programme\u{a0}: c'est ce qui empêche une séance de réviser de dériver vers du hors-sujet.",
                        list.len(),
                        list.len(),
                    )}
                </p>
            </div>
            <div class="pr-grid">
                {list.iter().map(|p| view! { <PromptCard entry=p/> }).collect_view()}
            </div>
        </section>
    }
}

fn masthead() -> impl IntoView {
    view! {
        <header class="hero">
            <div class="hero-inner">
                <div class="eyebrow">"Réviser avec un assistant"</div>
                <h1>"Un prompt par chapitre"</h1>
                <p class="lede">
                    "Trente-cinq prompts sur mesure, un par chapitre des cinq matières. Chacun dit à l'assistant ce qu'il doit faire travailler, "
                    <em>"jusqu'où aller"</em>
                    ", et comment corriger. Le calibrage vient des annales quand elles existent, du volume des TD sinon."
                </p>
                <div class="meta">
                    <span><span class="mono">{total()}</span>" prompts"</span>
                    <span><span class="mono">"5"</span>" sujets d'examen dépouillés"</span>
                    <span>"Copier, coller, répondre"</span>
                </div>
            </div>
        </header>
    }
}

fn sec_methode() -> impl IntoView {
    view! {
        <section id="pr-methode">
            <div class="sec-head">
                <span class="num">"—"</span>
                <h2>"Comment ces prompts ont été calibrés"</h2>
            </div>
            <div class="body">
                <p>
                    "Les cinq sujets disponibles depuis la réforme — partiels 2024 et 2025, examens finals 2024 et 2025, rattrapage de juin 2025 — ont été dépouillés exercice par exercice. Pour chaque chapitre, le prompt indique combien de fois il est tombé et sous quelle forme. Deux conclusions gouvernent tout le reste."
                </p>
            </div>
            <div class="note warn" style="margin-top:16px">
                <span class="title">"Deux chapitres n'apparaissent dans aucun sujet"</span>
                <p>
                    <strong>"Dénombrement"</strong>" et "<strong>"nombres complexes"</strong>
                    " sont au programme de Structures fondamentales, le TD 3 est même entièrement consacré aux complexes — et pourtant aucun des cinq sujets n'en contient le moindre exercice. Le partiel s'arrête aux suites, l'examen final commence à l'arithmétique modulaire\u{a0}: ces deux chapitres tombent exactement entre les deux."
                </p>
                <p>
                    "Leurs prompts le disent en ouverture et bornent la séance à une demi-heure. Ils ne sont pas supprimés — ils restent au programme et peuvent tomber — mais ils ne méritent pas les heures qu'on leur consacre spontanément parce qu'ils « ont l'air » difficiles."
                </p>
            </div>
            <div class="note method" style="margin-top:16px">
                <span class="title">"Le partiel et l'examen ne portent pas sur le même programme"</span>
                <p>
                    "Partiel\u{a0}: logique, ensembles, applications, suites. Examen final\u{a0}: arithmétique modulaire, groupes, permutations, plus une limite par ε en ouverture. Réviser l'un ne prépare pas à l'autre, et chaque prompt indique à laquelle des deux épreuves son chapitre appartient."
                </p>
            </div>
            <div class="note plan" style="margin-top:16px">
                <span class="title">"Comment s'en servir"</span>
                <ol>
                    <li>"Copier le prompt du chapitre, le coller dans l'assistant, et répondre sans rien ouvrir d'autre."</li>
                    <li>
                        "Les prompts imposent tous la même règle\u{a0}: une question à la fois, pas de réponse avant d'avoir essayé, correction de la rédaction avant le fond."
                    </li>
                    <li>
                        "Quand une notion résiste, ouvrir la fiche du chapitre par le lien « Revoir la fiche », puis reprendre."
                    </li>
                    <li>
                        "Enchaîner sur "<A href="/entrainement" scroll=false>"l'entraînement"</A>
                        " du même chapitre\u{a0}: les exercices y sont corrigés automatiquement et tirés au hasard."
                    </li>
                </ol>
            </div>
            <div class="note" style="margin-top:16px">
                <span class="title">"Le préambule ajouté à chaque copie"</span>
                <pre class="pr-text">{PREAMBLE}</pre>
            </div>
        </section>
    }
}

/// Page complète.
pub fn page() -> impl IntoView {
    view! {
        <div class="course">
            {masthead()}
            <div class="shell wide">
                <main>
                    <>
                        {sec_methode()}
                    </>
                    <>{COURSES.iter().map(|c| course_block(*c)).collect_view()}</>
                </main>
            </div>
        </div>
    }
}
