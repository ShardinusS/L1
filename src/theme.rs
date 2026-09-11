//! Thème clair / sombre / automatique, persisté dans `localStorage`.
//!
//! La feuille de style gère déjà trois cas : `:root` (clair), la requête
//! `prefers-color-scheme: dark` filtrée par `:root:not([data-theme="light"])`,
//! et `:root[data-theme="dark"]`. Ce module se contente donc de poser ou de
//! retirer l'attribut `data-theme` sur `<html>`.

use leptos::prelude::*;

const STORAGE_KEY: &str = "classeur-theme";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Theme {
    pub fn from_storage_value(raw: &str) -> Self {
        match raw {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::Auto,
        }
    }

    /// Valeur écrite dans `data-theme`; `None` pour le mode automatique,
    /// qui laisse la main à `prefers-color-scheme`.
    pub fn attribute(self) -> Option<&'static str> {
        match self {
            Theme::Light => Some("light"),
            Theme::Dark => Some("dark"),
            Theme::Auto => None,
        }
    }

    pub fn storage_value(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::Auto => "auto",
        }
    }

    /// Cycle du bouton : automatique → clair → sombre → automatique.
    pub fn next(self) -> Self {
        match self {
            Theme::Auto => Theme::Light,
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Auto,
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Theme::Light => "☀",
            Theme::Dark => "☾",
            Theme::Auto => "◐",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Theme::Light => "Thème clair",
            Theme::Dark => "Thème sombre",
            Theme::Auto => "Thème automatique",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeState(pub RwSignal<Theme>);

/// Installe le signal de thème dans le contexte et synchronise le DOM.
pub fn provide_theme() {
    let stored = storage()
        .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
        .map(|raw| Theme::from_storage_value(&raw))
        .unwrap_or(Theme::Auto);

    let theme = RwSignal::new(stored);

    Effect::new(move |_| {
        let value = theme.get();
        if let Some(root) = document_element() {
            match value.attribute() {
                Some(attr) => {
                    let _ = root.set_attribute("data-theme", attr);
                }
                None => {
                    let _ = root.remove_attribute("data-theme");
                }
            }
        }
        if let Some(storage) = storage() {
            let _ = storage.set_item(STORAGE_KEY, value.storage_value());
        }
    });

    provide_context(ThemeState(theme));
}

pub fn use_theme() -> RwSignal<Theme> {
    use_context::<ThemeState>()
        .expect("provide_theme() doit être appelé au montage de l'application")
        .0
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn document_element() -> Option<web_sys::Element> {
    web_sys::window()?.document()?.document_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_du_bouton() {
        assert_eq!(Theme::Auto.next(), Theme::Light);
        assert_eq!(Theme::Light.next(), Theme::Dark);
        assert_eq!(Theme::Dark.next(), Theme::Auto);
    }

    #[test]
    fn auto_ne_pose_pas_data_theme() {
        assert_eq!(Theme::Auto.attribute(), None);
        assert_eq!(Theme::Light.attribute(), Some("light"));
        assert_eq!(Theme::Dark.attribute(), Some("dark"));
    }

    #[test]
    fn valeur_stockee_relue_a_l_identique() {
        for t in [Theme::Light, Theme::Dark, Theme::Auto] {
            assert_eq!(Theme::from_storage_value(t.storage_value()), t);
        }
    }

    #[test]
    fn valeur_inconnue_retombe_sur_auto() {
        assert_eq!(Theme::from_storage_value("bleu"), Theme::Auto);
        assert_eq!(Theme::from_storage_value(""), Theme::Auto);
    }
}
