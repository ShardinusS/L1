//! Racine de l'application : routes, barre supérieure, pied de page.

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

use crate::components::topbar::Topbar;
use crate::course::COURSES;
use crate::scroll;
use crate::theme::provide_theme;

/// Ancien fragment d'URL -> route correspondante, pour que les liens et les
/// favoris de la version HTML continuent de fonctionner.
fn legacy_route(hash: &str) -> Option<&'static str> {
    let hash = hash.trim_start_matches('#');
    if hash.is_empty() {
        return None;
    }
    if let Some(pane) = hash.strip_prefix("pane-") {
        return COURSES
            .iter()
            .find(|c| c.key() == pane)
            .map(|c| c.route())
            .or(if pane == "home" { Some("/") } else { None });
    }
    if crate::pages::HOME_SECTIONS.contains(&hash) {
        return Some("/");
    }
    COURSES
        .iter()
        .find(|c| crate::pages::sections(**c).contains(&hash))
        .map(|c| c.route())
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme();

    view! {
        <Title text="Classeur d'informatique"/>
        <Router>
            <Topbar/>
            <Main/>
            <Footer/>
        </Router>
    }
}

#[component]
fn Main() -> impl IntoView {
    let location = use_location();

    // Un lien profond (`/systemes#o-recap`) ou un résultat de recherche change
    // le fragment : on fait défiler une fois la page rendue. Sans fragment, on
    // revient en haut, comme le faisait le changement d'onglet.
    Effect::new(move |previous: Option<String>| {
        let path = location.pathname.get();
        let hash = location.hash.get();
        let anchor = hash.trim_start_matches('#').to_string();

        if anchor.is_empty() {
            if previous.as_deref() != Some(path.as_str()) {
                scroll::scroll_to_top();
            }
        } else {
            scroll::scroll_to_anchor_when_ready(anchor);
        }
        path
    });

    // Redirection des anciens liens `#pane-sf` / `#sf-logique`, pour que les
    // favoris de la version HTML tombent sur la bonne route.
    let navigate = leptos_router::hooks::use_navigate();
    Effect::new(move |_| {
        if location.pathname.get_untracked() != "/" {
            return;
        }
        let hash = location.hash.get();
        let Some(route) = legacy_route(&hash) else {
            return;
        };
        if route == "/" {
            return;
        }
        let anchor = hash.trim_start_matches('#');
        let target = if anchor.starts_with("pane-") {
            route.to_string()
        } else {
            format!("{route}#{anchor}")
        };
        navigate(&target, Default::default());
    });

    view! {
        <Routes fallback=|| crate::pages::home::page()>
            <Route path=path!("/") view=crate::pages::home::page/>
            <Route
                path=path!("/structures-fondamentales")
                view=crate::pages::sf::page
            />
            <Route path=path!("/methodes-calcul") view=crate::pages::mtc::page/>
            <Route path=path!("/algorithmique") view=crate::pages::algo::page/>
            <Route path=path!("/information") view=crate::pages::info::page/>
            <Route path=path!("/systemes") view=crate::pages::os::page/>
            <Route path=path!("/prompts") view=crate::pages::prompts::page/>
            <Route path=path!("/entrainement") view=crate::pages::entrainement::hub/>
            <Route
                path=path!("/entrainement/:matiere")
                view=crate::pages::entrainement::course_page
            />
        </Routes>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <span>"Classeur d'informatique — Kenzo Metgy, L1 2026-2027"</span>
            <span>"Structures fondamentales · Méthodes et techniques de calcul · F. Durand, UPJV"</span>
            <span>"Algorithmique 1 · J. Caracotte et L. Robert"</span>
            <span>
                "Représentation de l'information · Systèmes d'exploitation · notes personnelles"
            </span>
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::Course;

    #[test]
    fn anciens_liens_de_panneau() {
        assert_eq!(legacy_route("#pane-sf"), Some(Course::Sf.route()));
        assert_eq!(legacy_route("#pane-os"), Some(Course::Os.route()));
        assert_eq!(legacy_route("#pane-home"), Some("/"));
    }

    #[test]
    fn anciens_liens_de_section() {
        assert_eq!(legacy_route("#sf-logique"), Some(Course::Sf.route()));
        assert_eq!(legacy_route("#m-derivees"), Some(Course::Mtc.route()));
        assert_eq!(legacy_route("#o-recap"), Some(Course::Os.route()));
        assert_eq!(legacy_route("#i-conversion"), Some(Course::Info.route()));
        assert_eq!(legacy_route("#al-orga"), Some(Course::Algo.route()));
    }

    #[test]
    fn fragment_inconnu_ou_vide() {
        assert_eq!(legacy_route(""), None);
        assert_eq!(legacy_route("#"), None);
        assert_eq!(legacy_route("#inconnu"), None);
    }

    #[test]
    fn toutes_les_sections_sont_redirigeables() {
        for course in COURSES {
            for anchor in crate::pages::sections(course) {
                assert_eq!(
                    legacy_route(&format!("#{anchor}")),
                    Some(course.route()),
                    "ancre {anchor}"
                );
            }
        }
    }
}
