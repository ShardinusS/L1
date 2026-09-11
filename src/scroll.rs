//! Défilement vers une section et surbrillance passagère.
//!
//! Équivalent de `goTo()` dans la version d'origine : on fait défiler jusqu'à
//! la section, puis on relance l'animation `.flash` sur son titre. Le décalage
//! sous la barre collante est assuré par `section{scroll-margin-top:104px}`
//! dans la feuille de style.

use web_sys::wasm_bindgen::JsCast;

/// Fait défiler jusqu'à l'ancre et rejoue la surbrillance. Sans effet si
/// l'élément n'existe pas (encore).
pub fn scroll_to_anchor(anchor: &str) -> bool {
    let Some(document) = web_sys::window().and_then(|w| w.document()) else {
        return false;
    };
    let Some(el) = document.get_element_by_id(anchor) else {
        return false;
    };

    let options = web_sys::ScrollIntoViewOptions::new();
    options.set_behavior(web_sys::ScrollBehavior::Smooth);
    options.set_block(web_sys::ScrollLogicalPosition::Start);
    el.scroll_into_view_with_scroll_into_view_options(&options);

    let head = el
        .query_selector(".sec-head")
        .ok()
        .flatten()
        .unwrap_or_else(|| el.clone());
    flash(&head);
    true
}

/// Délais successifs de réessai, en millisecondes.
///
/// Changer de matière remplace tout le contenu : au moment où l'URL change,
/// la section visée n'est pas encore dans le document. On retente jusqu'à ce
/// qu'elle apparaisse, puis on abandonne — une ancre inconnue ne doit pas
/// faire boucler la page.
const RETRIES: [i32; 5] = [0, 16, 48, 120, 300];

/// Fait défiler jusqu'à l'ancre dès qu'elle existe.
pub fn scroll_to_anchor_when_ready(anchor: String) {
    if scroll_to_anchor(&anchor) {
        return;
    }
    retry(anchor, 0);
}

fn retry(anchor: String, attempt: usize) {
    let Some(delay) = RETRIES.get(attempt).copied() else {
        return;
    };
    leptos::prelude::set_timeout(
        move || {
            if !scroll_to_anchor(&anchor) {
                retry(anchor, attempt + 1);
            }
        },
        std::time::Duration::from_millis(delay as u64),
    );
}

/// Remonte en haut de page, sans animation, comme au changement d'onglet.
pub fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        let options = web_sys::ScrollToOptions::new();
        options.set_top(0.0);
        options.set_behavior(web_sys::ScrollBehavior::Auto);
        window.scroll_to_with_scroll_to_options(&options);
    }
}

/// Relance l'animation en retirant la classe puis en forçant un reflow.
fn flash(el: &web_sys::Element) {
    let list = el.class_list();
    let _ = list.remove_1("flash");
    if let Some(html) = el.dyn_ref::<web_sys::HtmlElement>() {
        let _ = html.offset_width();
    }
    let _ = list.add_1("flash");
}
