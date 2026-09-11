//! Recherche globale : champ, popover de résultats et navigation au clavier.
//!
//! Comportement repris de l'original : `↑` `↓` pour parcourir, `Entrée` pour
//! ouvrir (le premier résultat si rien n'est surligné), `Échap` pour fermer,
//! `/` pour prendre le focus depuis n'importe où hors champ de saisie, et
//! fermeture au clic à l'extérieur.

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::JsCast;

use crate::search::{index, search, truncate_sub, EMPTY_MESSAGE};

#[component]
pub fn SearchBox() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let open = RwSignal::new(false);
    let cursor = RwSignal::new(-1i32);
    let input_ref = NodeRef::<html::Input>::new();
    let navigate = use_navigate();

    let results = Memo::new(move |_| search(index(), &query.get()));

    let close = move || {
        open.set(false);
        cursor.set(-1);
    };

    let blur_input = move || {
        if let Some(el) = input_ref.get_untracked() {
            let _ = el.blur();
        }
    };

    let go = {
        let navigate = navigate.clone();
        move |href: String| {
            close();
            blur_input();
            navigate(&href, Default::default());
        }
    };

    // Raccourci « / » : capté au niveau du document, ignoré dans les champs.
    {
        let handle = window_event_listener(ev::keydown, move |ev: ev::KeyboardEvent| {
            if ev.key() != "/" {
                return;
            }
            if is_editable_target(ev.target()) {
                return;
            }
            ev.prevent_default();
            if let Some(el) = input_ref.get_untracked() {
                let _ = el.focus();
                el.select();
            }
        });
        on_cleanup(move || handle.remove());
    }

    // Clic hors du bloc de recherche : on referme.
    {
        let handle = window_event_listener(ev::click, move |ev: ev::MouseEvent| {
            if !open.get_untracked() {
                return;
            }
            let inside = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .and_then(|el| el.closest(".gsearch").ok().flatten())
                .is_some();
            if !inside {
                close();
            }
        });
        on_cleanup(move || handle.remove());
    }

    let on_keydown = {
        let go = go.clone();
        move |ev: ev::KeyboardEvent| match ev.key().as_str() {
            "Escape" => {
                close();
                blur_input();
            }
            "ArrowDown" | "ArrowUp" | "Enter" => {
                let len = results.with(|r| r.len()) as i32;
                if !open.get_untracked() || len == 0 {
                    return;
                }
                ev.prevent_default();
                match ev.key().as_str() {
                    "ArrowDown" => cursor.set((cursor.get_untracked() + 1).rem_euclid(len)),
                    "ArrowUp" => cursor.set((cursor.get_untracked() - 1).rem_euclid(len)),
                    _ => {
                        let i = cursor.get_untracked().max(0) as usize;
                        if let Some(href) = results.with_untracked(|r| r.get(i).map(|e| e.href())) {
                            go(href);
                        }
                    }
                }
            }
            _ => {}
        }
    };

    view! {
        <div class="gsearch">
            <span class="icon" aria-hidden="true">"⌕"</span>
            <input
                node_ref=input_ref
                id="gq"
                type="search"
                placeholder="Chercher dans les 5 matières…"
                autocomplete="off"
                spellcheck="false"
                aria-label="Chercher dans toutes les matières"
                aria-expanded=move || open.get().to_string()
                aria-controls="gres"
                prop:value=move || query.get()
                on:input=move |ev| {
                    query.set(event_target_value(&ev));
                    cursor.set(-1);
                    open.set(!query.get_untracked().trim().is_empty());
                }
                on:focus=move |_| {
                    if !query.get_untracked().trim().is_empty() {
                        open.set(true);
                    }
                }
                on:keydown=on_keydown
            />
            <kbd>"/"</kbd>
            <div class="results" id="gres" role="listbox" hidden=move || !open.get()>
                <Show when=move || results.with(|r| r.is_empty())>
                    <p class="none">{EMPTY_MESSAGE}</p>
                </Show>
                <For
                    each=move || results.get().into_iter().enumerate()
                    key=|(i, entry)| (*i, entry.course, entry.label)
                    let((i, entry))
                >
                    {
                        let href = entry.href();
                        let go = go.clone();
                        let i = i as i32;
                        view! {
                            <button
                                type="button"
                                role="option"
                                aria-selected=move || (cursor.get() == i).to_string()
                                class:on=move || cursor.get() == i
                                on:click=move |_| go(href.clone())
                                on:mouseenter=move |_| cursor.set(i)
                            >
                                <span class="lab">
                                    <span class:k=entry.mono>{entry.label}</span>
                                    <Show when=move || !entry.sub.is_empty()>
                                        <span class="sub">{truncate_sub(entry.sub)}</span>
                                    </Show>
                                </span>
                                <span class=format!(
                                    "badge {}",
                                    entry.course.key(),
                                )>{entry.course.short()}</span>
                            </button>
                        }
                    }
                </For>
            </div>
        </div>
    }
}

/// Vrai si l'évènement vient d'un champ de saisie : le raccourci `/` doit y
/// rester inerte, comme ceux de l'entraînement.
pub fn is_editable_target(target: Option<web_sys::EventTarget>) -> bool {
    target
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .map(|el| {
            matches!(el.tag_name().as_str(), "INPUT" | "SELECT" | "TEXTAREA")
                || el
                    .dyn_ref::<web_sys::HtmlElement>()
                    .is_some_and(|h| h.is_content_editable())
        })
        .unwrap_or(false)
}
