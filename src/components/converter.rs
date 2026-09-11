//! Convertisseur de bases de la matière 03.
//!
//! Le calcul et les messages vivent dans [`crate::convert`], qui est testé
//! sans navigateur; ce composant ne fait que relier la saisie à l'affichage.
//! Valeur de départ, libellés et ordre des bases repris de l'original.

use leptos::prelude::*;

use crate::convert::convert;

/// Bases proposées, dans l'ordre du sélecteur d'origine.
const CHOICES: [(u32, &str); 4] = [
    (10, "10 — décimal"),
    (2, "2 — binaire"),
    (8, "8 — octal"),
    (16, "16 — hexadécimal"),
];

const DEFAULT_INPUT: &str = "235";
const DEFAULT_BASE: u32 = 10;

#[component]
pub fn Converter() -> impl IntoView {
    let input = RwSignal::new(DEFAULT_INPUT.to_string());
    let base = RwSignal::new(DEFAULT_BASE);

    let result = Memo::new(move |_| convert(&input.get(), base.get()));

    view! {
        <div class="conv">
            <div class="conv-row">
                <div class="field">
                    <label for="cv-in">"Nombre"</label>
                    <input
                        id="cv-in"
                        spellcheck="false"
                        autocomplete="off"
                        prop:value=move || input.get()
                        on:input=move |ev| input.set(event_target_value(&ev))
                    />
                </div>
                <div class="field">
                    <label for="cv-base">"Base de départ"</label>
                    <select
                        id="cv-base"
                        prop:value=move || base.get().to_string()
                        on:change=move |ev| {
                            if let Ok(b) = event_target_value(&ev).parse::<u32>() {
                                base.set(b);
                            }
                        }
                    >
                        {CHOICES
                            .iter()
                            .map(|(value, label)| {
                                view! { <option value=value.to_string()>{*label}</option> }
                            })
                            .collect_view()}
                    </select>
                </div>
            </div>
            <div class="out">
                <div>
                    <span class="k">"Binaire"</span>
                    <span class="v" id="cv-2">{move || result.get().bin}</span>
                </div>
                <div>
                    <span class="k">"Octal"</span>
                    <span class="v" id="cv-8">{move || result.get().oct}</span>
                </div>
                <div>
                    <span class="k">"Décimal"</span>
                    <span class="v" id="cv-10">{move || result.get().dec}</span>
                </div>
                <div>
                    <span class="k">"Hexadécimal"</span>
                    <span class="v" id="cv-16">{move || result.get().hex}</span>
                </div>
            </div>
            <p class="msg" class:warn=move || result.get().warn id="cv-msg" aria-live="polite">
                {move || result.get().message}
            </p>
        </div>
    }
}
