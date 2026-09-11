//! Sommaire latéral, avec suivi de la section visible.
//!
//! Reprend le comportement de l'`IntersectionObserver` d'origine, y compris sa
//! marge de déclenchement `-15% 0px -75% 0px` : une section devient active
//! lorsqu'elle entre dans le quart haut de la fenêtre.

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::course::Course;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TocItem {
    pub anchor: &'static str,
    /// Numéro affiché à gauche du titre; « — » pour les sections annexes.
    pub num: &'static str,
    pub title: &'static str,
}

const ROOT_MARGIN: &str = "-15% 0px -75% 0px";

#[component]
pub fn Toc(course: Course, items: &'static [TocItem], label: &'static str) -> impl IntoView {
    let active = RwSignal::new(String::new());
    let observer: StoredValue<Option<web_sys::IntersectionObserver>> = StoredValue::new(None);
    let navigate = use_navigate();

    // Les sections doivent être montées avant d'être observées : un effet,
    // qui s'exécute après le premier rendu, convient.
    Effect::new(move |_| {
        if observer.with_value(|o| o.is_some()) {
            return;
        }
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let Some(obs) = build_observer(active) else {
            return;
        };
        for item in items {
            if let Some(el) = document.get_element_by_id(item.anchor) {
                obs.observe(&el);
            }
        }
        observer.set_value(Some(obs));
    });

    on_cleanup(move || {
        observer.update_value(|slot| {
            if let Some(obs) = slot.take() {
                obs.disconnect();
            }
        });
    });

    view! {
        <nav class="toc" aria-label=label>
            <div class="toc-label">"Sommaire"</div>
            <ol>
                {items
                    .iter()
                    .map(|item| {
                        let anchor = item.anchor;
                        let href = format!("{}#{}", course.route(), anchor);
                        let target = href.clone();
                        let navigate = navigate.clone();
                        view! {
                            <li>
                                <a
                                    href=href
                                    class:active=move || active.get() == anchor
                                    on:click=move |ev: leptos::ev::MouseEvent| {
                                        if ev.ctrl_key() || ev.meta_key() || ev.shift_key() {
                                            return;
                                        }
                                        ev.prevent_default();
                                        navigate(&target, Default::default());
                                    }
                                >
                                    <span class="num">{item.num}</span>
                                    <span>{item.title}</span>
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </nav>
    }
}

fn build_observer(active: RwSignal<String>) -> Option<web_sys::IntersectionObserver> {
    let callback = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
        move |entries: js_sys::Array, _obs: web_sys::IntersectionObserver| {
            for entry in entries.iter() {
                let Ok(entry) = entry.dyn_into::<web_sys::IntersectionObserverEntry>() else {
                    continue;
                };
                if entry.is_intersecting() {
                    active.set(entry.target().id());
                }
            }
        },
    );

    let options = web_sys::IntersectionObserverInit::new();
    options.set_root_margin(ROOT_MARGIN);
    let observer = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    )
    .ok()?;
    // Le rappel doit survivre au composant : l'observateur le référence.
    callback.forget();
    Some(observer)
}
