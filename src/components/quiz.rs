//! Séance d'entraînement : tirage d'une série, correction immédiate,
//! bilan, et statistiques par chapitre.
//!
//! Le tirage et la correction vivent dans [`crate::practice`], testés sans
//! navigateur; ce composant ne fait que relier l'état de la séance au DOM.
//!
//! Clavier : `1` à `9` choisissent une option; `Entrée` valide une réponse
//! tapée, puis passe à la suite (le bouton reçoit le focus); sur une carte,
//! `Entrée` la retourne, `1` « je savais », `2` « à revoir ».

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::search_box::is_editable_target;
use crate::course::{Course, COURSES};
use crate::practice::progress::{answers_label, Progress, Tally};
use crate::practice::{chapters, draw, Answer, Filter, Question, Response, Rng};

const LENGTHS: [usize; 3] = [5, 10, 20];
const DEFAULT_LENGTH: usize = 10;

/// État d'une séance. Tous les champs sont des signaux : la structure se
/// copie librement dans les gestionnaires d'évènements.
#[derive(Clone, Copy)]
struct Session {
    questions: RwSignal<Vec<Question>>,
    pos: RwSignal<usize>,
    /// Résultat de chaque question de la série; `None` tant qu'elle n'a pas
    /// reçu de réponse.
    outcomes: RwSignal<Vec<Option<bool>>>,
    text: RwSignal<String>,
    picked: RwSignal<Option<usize>>,
    flipped: RwSignal<bool>,
    filter: RwSignal<Filter>,
    length: RwSignal<usize>,
    progress: RwSignal<Progress>,
    rng: StoredValue<Rng>,
}

impl Session {
    fn new(filter: Filter) -> Self {
        Self {
            questions: RwSignal::new(Vec::new()),
            pos: RwSignal::new(0),
            outcomes: RwSignal::new(Vec::new()),
            text: RwSignal::new(String::new()),
            picked: RwSignal::new(None),
            flipped: RwSignal::new(false),
            filter: RwSignal::new(filter),
            length: RwSignal::new(DEFAULT_LENGTH),
            progress: RwSignal::new(Progress::load()),
            rng: StoredValue::new(Rng::from_entropy()),
        }
    }

    fn clear_inputs(self) {
        self.text.set(String::new());
        self.picked.set(None);
        self.flipped.set(false);
    }

    fn load(self, questions: Vec<Question>) {
        self.outcomes.set(vec![None; questions.len()]);
        self.questions.set(questions);
        self.pos.set(0);
        self.clear_inputs();
    }

    /// Tire une nouvelle série selon le filtre et la longueur choisis.
    fn start(self) {
        let filter = self.filter.get_untracked();
        let count = self.length.get_untracked();
        let mut drawn = Vec::new();
        self.rng
            .update_value(|rng| drawn = draw(filter, count, rng));
        self.load(drawn);
    }

    fn current(self) -> Option<Question> {
        let i = self.pos.get();
        self.questions.with(|qs| qs.get(i).cloned())
    }

    fn outcome(self) -> Option<bool> {
        let i = self.pos.get();
        self.outcomes.with(|o| o.get(i).copied().flatten())
    }

    fn revealed(self) -> bool {
        self.outcome().is_some()
    }

    fn finished(self) -> bool {
        let n = self.questions.with(Vec::len);
        n > 0 && self.pos.get() >= n
    }

    /// Note la question courante, une seule fois, et met à jour les
    /// statistiques conservées.
    fn record(self, correct: bool) {
        let i = self.pos.get_untracked();
        let Some(q) = self.questions.with_untracked(|qs| qs.get(i).cloned()) else {
            return;
        };
        if self
            .outcomes
            .with_untracked(|o| o.get(i).copied().flatten().is_some())
        {
            return;
        }
        self.outcomes.update(|o| o[i] = Some(correct));
        self.progress
            .update(|p| p.record(q.course, q.chapter_id, correct));
        self.progress.with_untracked(Progress::save);
    }

    fn submit(self, response: Response) {
        let i = self.pos.get_untracked();
        let ok = self
            .questions
            .with_untracked(|qs| qs.get(i).map(|q| q.exercise.answer.grade(&response)));
        if let Some(ok) = ok {
            self.record(ok);
        }
    }

    fn next(self) {
        self.clear_inputs();
        self.pos.update(|p| *p += 1);
    }

    fn mistakes(self) -> Vec<Question> {
        self.questions.with_untracked(|qs| {
            self.outcomes.with_untracked(|o| {
                qs.iter()
                    .zip(o)
                    .filter(|(_, r)| **r == Some(false))
                    .map(|(q, _)| q.clone())
                    .collect()
            })
        })
    }

    fn retry_mistakes(self) {
        let mistakes = self.mistakes();
        if !mistakes.is_empty() {
            self.load(mistakes);
        }
    }
}

fn tally(p: &Progress, filter: Filter) -> Tally {
    match filter {
        Filter::All => p.total(),
        Filter::Course(c) => p.course(c),
        Filter::Chapter(c, id) => p.chapter(c, id),
    }
}

fn rate_label(t: Tally) -> String {
    t.rate().map(|r| format!("{r} %")).unwrap_or_default()
}

/// Parties proposées dans le sélecteur : tout, puis chaque chapitre (ou
/// chaque matière pour une série mélangée).
fn filter_options(scope: Option<Course>) -> Vec<(String, Filter)> {
    match scope {
        Some(c) => std::iter::once(("Tout le cours".to_string(), Filter::Course(c)))
            .chain(
                chapters(c)
                    .iter()
                    .map(move |ch| (ch.title.to_string(), Filter::Chapter(c, ch.id))),
            )
            .collect(),
        None => std::iter::once(("Les cinq matières".to_string(), Filter::All))
            .chain(
                COURSES
                    .iter()
                    .map(|c| (c.label().to_string(), Filter::Course(*c))),
            )
            .collect(),
    }
}

#[component]
pub fn Quiz(scope: Option<Course>) -> impl IntoView {
    let s = Session::new(scope.map_or(Filter::All, Filter::Course));
    s.start();

    let input_ref = NodeRef::<html::Input>::new();
    let next_ref = NodeRef::<html::Button>::new();

    // Focus : le champ à chaque nouvelle question tapée, le bouton « suivante »
    // après correction. Pas au premier rendu, pour ne pas faire défiler la page.
    Effect::new(move |previous: Option<()>| {
        let revealed = s.revealed();
        s.questions.track();
        if previous.is_none() {
            return;
        }
        request_animation_frame(move || {
            if revealed {
                if let Some(b) = next_ref.get_untracked() {
                    let _ = b.focus();
                }
            } else if let Some(i) = input_ref.get_untracked() {
                let _ = i.focus();
            }
        });
    });

    {
        let handle = window_event_listener(ev::keydown, move |ev: ev::KeyboardEvent| {
            if ev.ctrl_key() || ev.meta_key() || ev.alt_key() || is_editable_target(ev.target()) {
                return;
            }
            let i = s.pos.get_untracked();
            let Some(q) = s.questions.with_untracked(|qs| qs.get(i).cloned()) else {
                return;
            };
            if s.outcomes
                .with_untracked(|o| o.get(i).copied().flatten().is_some())
            {
                return;
            }
            let key = ev.key();
            match &q.exercise.answer {
                Answer::Choice { options, .. } => {
                    let Some(n) = key
                        .parse::<usize>()
                        .ok()
                        .filter(|n| (1..=options.len()).contains(n))
                    else {
                        return;
                    };
                    ev.prevent_default();
                    s.picked.set(Some(n - 1));
                    s.submit(Response::Choice(n - 1));
                }
                Answer::Card { .. } => match (key.as_str(), s.flipped.get_untracked()) {
                    ("Enter", false) => {
                        ev.prevent_default();
                        s.flipped.set(true);
                    }
                    ("1", true) => s.submit(Response::Card(true)),
                    ("2", true) => s.submit(Response::Card(false)),
                    _ => {}
                },
                _ => {}
            }
        });
        on_cleanup(move || handle.remove());
    }

    view! {
        <div class="qz">
            <aside class="qz-side">
                {filters_view(s, scope)}
                {length_view(s)}
                {stats_view(s, scope)}
            </aside>
            <div class="qz-main">
                {move || progress_bar(s)}
                {move || {
                    if s.finished() {
                        summary_view(s).into_any()
                    } else if let Some(q) = s.current() {
                        question_view(s, q, input_ref, next_ref).into_any()
                    } else {
                        view! { <p class="empty">"Aucun exercice pour cette partie."</p> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

fn filters_view(s: Session, scope: Option<Course>) -> impl IntoView {
    view! {
        <div class="qz-block">
            <div class="qz-label">"Sur quoi ?"</div>
            <div class="qz-chips">
                {filter_options(scope)
                    .into_iter()
                    .map(|(label, filter)| {
                        view! {
                            <button
                                type="button"
                                class="qz-chip"
                                aria-pressed=move || (s.filter.get() == filter).to_string()
                                on:click=move |_| {
                                    s.filter.set(filter);
                                    s.start();
                                }
                            >
                                <span>{label}</span>
                                <span class="qz-rate">
                                    {move || s.progress.with(|p| rate_label(tally(p, filter)))}
                                </span>
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

fn length_view(s: Session) -> impl IntoView {
    view! {
        <div class="qz-block">
            <div class="qz-label">"Longueur de la série"</div>
            <div class="qz-seg" role="group" aria-label="Nombre de questions">
                {LENGTHS
                    .iter()
                    .map(|&n| {
                        view! {
                            <button
                                type="button"
                                aria-pressed=move || (s.length.get() == n).to_string()
                                on:click=move |_| {
                                    s.length.set(n);
                                    s.start();
                                }
                            >
                                {n}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

fn stats_view(s: Session, scope: Option<Course>) -> impl IntoView {
    let rows = filter_options(scope)
        .into_iter()
        .skip(1)
        .collect::<Vec<_>>();
    let confirm = RwSignal::new(false);
    let total = move || {
        s.progress
            .with(|p| tally(p, scope.map_or(Filter::All, Filter::Course)))
    };
    view! {
        <div class="qz-block">
            <div class="qz-label">"Mes statistiques"</div>
            <p class="qz-total">
                {move || {
                    let t = total();
                    match t.rate() {
                        Some(r) => format!("{} · {r} % de réussite", answers_label(t.asked)),
                        None => "Pas encore de réponse enregistrée.".to_string(),
                    }
                }}
            </p>
            {move || {
                let course = scope?;
                let (chapter, rate) = s.progress.with(|p| p.weakest(course))?;
                (rate < 80)
                    .then(|| {
                        view! {
                            <button
                                type="button"
                                class="qz-weak"
                                on:click=move |_| {
                                    s.filter.set(Filter::Chapter(course, chapter.id));
                                    s.start();
                                }
                            >
                                <span class="k">"Point faible"</span>
                                <span>{format!("{} — {rate} %", chapter.title)}</span>
                                <span class="go">"S'y entraîner →"</span>
                            </button>
                        }
                    })
            }}
            <ul class="qz-stats">
                {rows
                    .into_iter()
                    .map(|(label, filter)| {
                        let t = move || s.progress.with(|p| tally(p, filter));
                        view! {
                            <li>
                                <span class="lab">{label}</span>
                                <span class="track">
                                    <span
                                        class="fill"
                                        style=move || format!("width:{}%", t().rate().unwrap_or(0))
                                    ></span>
                                </span>
                                <span class="n">
                                    {move || {
                                        let t = t();
                                        if t.asked == 0 { "—".to_string() } else { format!("{}/{}", t.correct, t.asked) }
                                    }}
                                </span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            <button
                type="button"
                class="qz-reset"
                on:click=move |_| {
                    if confirm.get_untracked() {
                        s.progress.update(|p| p.reset(scope));
                        s.progress.with_untracked(Progress::save);
                        confirm.set(false);
                    } else {
                        confirm.set(true);
                    }
                }
                on:blur=move |_| confirm.set(false)
            >
                {move || {
                    if confirm.get() { "Confirmer l'effacement" } else { "Effacer mes statistiques" }
                }}
            </button>
        </div>
    }
}

fn progress_bar(s: Session) -> impl IntoView {
    let pos = s.pos.get();
    s.outcomes.with(|o| {
        view! {
            <ol class="qz-bar" aria-hidden="true">
                {o
                    .iter()
                    .enumerate()
                    .map(|(i, r)| {
                        let class = match r {
                            Some(true) => "ok",
                            Some(false) => "ko",
                            None if i == pos => "now",
                            None => "",
                        };
                        view! { <li class=class></li> }
                    })
                    .collect_view()}
            </ol>
        }
    })
}

fn question_view(
    s: Session,
    q: Question,
    input_ref: NodeRef<html::Input>,
    next_ref: NodeRef<html::Button>,
) -> impl IntoView {
    let total = s.questions.with_untracked(Vec::len);
    let index = s.pos.get_untracked();
    let area = match q.exercise.answer.clone() {
        Answer::Choice { options, correct } => choice_view(s, options, correct).into_any(),
        Answer::Card { back } => card_view(s, back).into_any(),
        typed => typed_view(s, typed.placeholder(), input_ref).into_any(),
    };
    let for_feedback = q.clone();
    view! {
        <article class=format!("qz-card c-{}", q.course.key())>
            <header class="qz-head">
                <span>{format!("Question {} / {total}", index + 1)}</span>
                <span class=format!("badge {}", q.course.key())>{q.chapter_title}</span>
            </header>
            <div class="qz-body">
                <p class="qz-prompt">{q.exercise.prompt.clone()}</p>
                {q.exercise.code.clone().map(|c| view! { <pre class="qz-code">{c}</pre> })}
                {area}
                {move || s.outcome().map(|correct| feedback_view(s, &for_feedback, correct, next_ref))}
            </div>
        </article>
    }
}

fn choice_view(s: Session, options: Vec<String>, correct: usize) -> impl IntoView {
    view! {
        <div class="qz-options" role="group" aria-label="Réponses proposées">
            {options
                .into_iter()
                .enumerate()
                .map(|(i, text)| {
                    view! {
                        <button
                            type="button"
                            class="qz-opt"
                            class:picked=move || s.picked.get() == Some(i)
                            class:right=move || s.revealed() && i == correct
                            class:wrong=move || {
                                s.revealed() && s.picked.get() == Some(i) && i != correct
                            }
                            disabled=move || s.revealed()
                            on:click=move |_| {
                                s.picked.set(Some(i));
                                s.submit(Response::Choice(i));
                            }
                        >
                            <span class="qz-key">{i + 1}</span>
                            <span class="qz-opt-text">{text}</span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}

fn typed_view(
    s: Session,
    placeholder: &'static str,
    input_ref: NodeRef<html::Input>,
) -> impl IntoView {
    view! {
        <form
            class="qz-form"
            on:submit=move |ev: ev::SubmitEvent| {
                ev.prevent_default();
                let text = s.text.get_untracked();
                if !text.trim().is_empty() {
                    s.submit(Response::Text(text));
                }
            }
        >
            <input
                node_ref=input_ref
                class="qz-input"
                type="text"
                autocomplete="off"
                autocapitalize="off"
                spellcheck="false"
                aria-label="Ta réponse"
                placeholder=placeholder
                prop:value=move || s.text.get()
                on:input=move |ev| s.text.set(event_target_value(&ev))
                disabled=move || s.revealed()
            />
            <button
                type="submit"
                class="btn primary"
                disabled=move || s.revealed() || s.text.with(|t| t.trim().is_empty())
            >
                "Vérifier"
            </button>
            <button type="button" class="btn" disabled=move || s.revealed() on:click=move |_| s.record(false)>
                "Je ne sais pas"
            </button>
        </form>
    }
}

fn card_view(s: Session, back: String) -> impl IntoView {
    move || {
        if s.flipped.get() {
            view! {
                <div class="qz-back">{back.clone()}</div>
                <div class="qz-actions" hidden=move || s.revealed()>
                    <button type="button" class="btn primary" on:click=move |_| s.submit(Response::Card(true))>
                        "Je savais" <kbd>"1"</kbd>
                    </button>
                    <button type="button" class="btn" on:click=move |_| s.submit(Response::Card(false))>
                        "À revoir" <kbd>"2"</kbd>
                    </button>
                </div>
            }
            .into_any()
        } else {
            view! {
                <div class="qz-actions">
                    <button type="button" class="btn primary" on:click=move |_| s.flipped.set(true)>
                        "Retourner la carte" <kbd>"Entrée"</kbd>
                    </button>
                </div>
            }
            .into_any()
        }
    }
}

fn feedback_view(
    s: Session,
    q: &Question,
    correct: bool,
    next_ref: NodeRef<html::Button>,
) -> impl IntoView {
    let answer = &q.exercise.answer;
    let is_card = matches!(answer, Answer::Card { .. });
    let verdict = match (is_card, correct) {
        (true, true) => "✓ Acquis",
        (true, false) => "↻ À revoir",
        (false, true) => "✓ Juste",
        (false, false) => "✗ Pas tout à fait",
    };
    let expected = (!is_card && (!correct || answer.is_typed())).then(|| answer.expected());
    let explain = (!q.exercise.explain.is_empty()).then(|| q.exercise.explain.clone());
    let last = s.pos.get_untracked() + 1 >= s.questions.with_untracked(Vec::len);
    view! {
        <div class="qz-feedback" class:ok=correct class:ko=!correct role="status">
            <p class="qz-verdict">{verdict}</p>
            {expected
                .map(|e| {
                    view! {
                        <p class="qz-expected">
                            "Réponse attendue : " <span class="mono">{e}</span>
                        </p>
                    }
                })}
            {explain.map(|e| view! { <p class="qz-explain">{e}</p> })}
            <A href=q.href() scroll=false attr:class="qz-link">
                "Revoir la fiche du cours →"
            </A>
        </div>
        <div class="qz-actions">
            <button type="button" class="btn primary" node_ref=next_ref on:click=move |_| s.next()>
                {if last { "Voir le bilan" } else { "Question suivante →" }}
            </button>
        </div>
    }
}

fn summary_view(s: Session) -> impl IntoView {
    let (correct, total) = s
        .outcomes
        .with_untracked(|o| (o.iter().filter(|r| **r == Some(true)).count(), o.len()));
    let pct = (correct * 100 + total / 2) / total.max(1);
    let message = match pct {
        90.. => "Excellent. Passe à un autre chapitre, ou allonge la série.",
        70..=89 => "Solide. Refais tes erreurs pour consolider.",
        50..=69 => "À moitié acquis : relis la fiche des questions manquées, puis recommence.",
        _ => "Reprends le cours de ce chapitre avant de retenter : chaque erreur ci-dessous renvoie à sa fiche.",
    };
    let mistakes = s.mistakes();
    let retry_label = match mistakes.len() {
        0 => None,
        1 => Some("Refaire mon erreur".to_string()),
        n => Some(format!("Refaire mes {n} erreurs")),
    };
    let review = (!mistakes.is_empty()).then(|| {
        view! {
            <h3>"À revoir"</h3>
            <ol class="qz-miss">
                {mistakes
                    .iter()
                    .map(|q| {
                        view! {
                            <li>
                                <span class="qz-miss-q">{q.exercise.prompt.clone()}</span>
                                {q.exercise.code.clone().map(|c| view! { <pre class="qz-code">{c}</pre> })}
                                <span class="qz-miss-a">
                                    "Réponse : " <span class="mono">{q.exercise.answer.expected()}</span>
                                </span>
                                <A href=q.href() scroll=false attr:class="qz-link">
                                    "Revoir la fiche →"
                                </A>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        }
    });
    view! {
        <section class="qz-summary">
            <div class="kpis">
                <div class="kpi">
                    <span class="v">{format!("{correct}/{total}")}</span>
                    <span class="k">"bonnes réponses"</span>
                </div>
                <div class="kpi">
                    <span class="v">{format!("{pct} %")}</span>
                    <span class="k">"de réussite"</span>
                </div>
            </div>
            <p class="qz-message">{message}</p>
            {review}
            <div class="qz-actions">
                <button type="button" class="btn primary" on:click=move |_| s.start()>
                    "Nouvelle série"
                </button>
                {retry_label
                    .map(|label| {
                        view! {
                            <button type="button" class="btn" on:click=move |_| s.retry_mistakes()>
                                {label}
                            </button>
                        }
                    })}
            </div>
        </section>
    }
}
