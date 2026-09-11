//! Barre supérieure : marque, onglets de matière, recherche et bouton de thème.
//!
//! Les onglets étaient des `<button role="tab">` pilotant l'affichage des
//! panneaux; ce sont maintenant de vrais liens de navigation. L'état courant
//! est porté par `aria-current="page"`, que la feuille de style utilise à la
//! place de l'ancien `aria-selected`.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::search_box::SearchBox;
use crate::course::COURSES;
use crate::theme::use_theme;

#[component]
pub fn Topbar() -> impl IntoView {
    let theme = use_theme();

    view! {
        <div class="topbar">
            <div class="topbar-inner">
                <A href="/" exact=true scroll=false attr:class="brand">
                    "Classeur"
                    <span>"Kenzo Metgy · L1 · 2026-2027"</span>
                </A>
                <nav class="tabs" aria-label="Matières">
                    <A href="/" exact=true scroll=false attr:class="tab tab-home">
                        <span class="dot"></span>
                        "Accueil"
                    </A>
                    {COURSES
                        .iter()
                        .map(|course| {
                            let course = *course;
                            view! {
                                <A
                                    href=course.route()
                                    exact=true
                                    scroll=false
                                    attr:class=format!("tab tab-{}", course.key())
                                >
                                    <span class="dot"></span>
                                    {course.label()}
                                </A>
                            }
                        })
                        .collect_view()}
                    <A href="/prompts" exact=true scroll=false attr:class="tab tab-prompt">
                        <span class="dot"></span>
                        "Prompts"
                    </A>
                    // Pas `exact` : l'onglet reste actif sur /entrainement/… .
                    <A href="/entrainement" scroll=false attr:class="tab tab-train">
                        <span class="dot"></span>
                        "Entraînement"
                    </A>
                </nav>
                <SearchBox/>
                <button
                    class="theme-btn"
                    type="button"
                    aria-label=move || theme.get().label()
                    title=move || theme.get().label()
                    on:click=move |_| theme.update(|t| *t = t.next())
                >
                    {move || theme.get().icon()}
                </button>
            </div>
        </div>
    }
}
