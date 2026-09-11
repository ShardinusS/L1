//! Index de recherche typé et moteur de correspondance.
//!
//! La version HTML d'origine reconstruisait son index en parcourant le DOM au
//! chargement (sélecteurs `.def dt`, `.cmd .c`, `.an h4`…). Ici chaque page
//! déclare ses entrées à côté du contenu qu'elle rend, et un test vérifie que
//! chaque ancre citée existe réellement. Le classement, lui, reprend à
//! l'identique la logique de l'original.

use crate::course::Course;
use unicode_normalization::UnicodeNormalization;

#[cfg(test)]
mod tests;

/// Nombre de correspondances sur le libellé au-delà duquel on n'affiche plus
/// les correspondances de second rang (celles trouvées sur le sous-titre).
const SECONDARY_CUTOFF: usize = 6;

/// Nombre maximum de résultats affichés.
pub const MAX_RESULTS: usize = 14;

pub const EMPTY_MESSAGE: &str = "Rien trouvé dans les cinq matières.";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IndexEntry {
    pub course: Course,
    /// Ancre de la section visée, sans `#`.
    pub anchor: &'static str,
    /// Texte principal du résultat.
    pub label: &'static str,
    /// Ligne secondaire : section parente, glose d'une commande, année d'annale…
    pub sub: &'static str,
    /// Affiche le libellé en chasse fixe (commandes shell, symboles).
    pub mono: bool,
}

impl IndexEntry {
    pub const fn new(
        course: Course,
        anchor: &'static str,
        label: &'static str,
        sub: &'static str,
    ) -> Self {
        Self {
            course,
            anchor,
            label,
            sub,
            mono: false,
        }
    }

    pub const fn mono(
        course: Course,
        anchor: &'static str,
        label: &'static str,
        sub: &'static str,
    ) -> Self {
        Self {
            course,
            anchor,
            label,
            sub,
            mono: true,
        }
    }

    /// Lien complet vers l'entrée : route de la matière + ancre.
    pub fn href(&self) -> String {
        format!("{}#{}", self.course.route(), self.anchor)
    }
}

/// Index complet, une tranche par matière, dans l'ordre d'apparition.
pub fn index() -> &'static [&'static [IndexEntry]] {
    crate::pages::INDEXES
}

/// Minuscules, décomposition NFD puis retrait des diacritiques.
///
/// Équivalent Rust de `s.toLowerCase().normalize('NFD').replace(/[̀-ͯ]/g,'')`.
pub fn norm(s: &str) -> String {
    s.to_lowercase()
        .nfd()
        .filter(|c| !matches!(*c, '\u{0300}'..='\u{036f}'))
        .collect()
}

/// Renvoie les entrées à afficher pour `query`, dans l'ordre d'affichage.
///
/// Reprend la sémantique de l'original : correspondance sur le libellé d'abord,
/// dédoublonnage par `(matière, libellé)`, puis correspondances sur le
/// sous-titre uniquement si les premières sont peu nombreuses.
pub fn search(slices: &'static [&'static [IndexEntry]], query: &str) -> Vec<&'static IndexEntry> {
    let needle = norm(query.trim());
    if needle.is_empty() {
        return Vec::new();
    }

    let mut seen: Vec<(Course, &'static str)> = Vec::new();
    let mut primary: Vec<&'static IndexEntry> = Vec::new();
    let mut secondary: Vec<&'static IndexEntry> = Vec::new();

    for entry in slices.iter().flat_map(|s| s.iter()) {
        let key = (entry.course, entry.label);
        if seen.contains(&key) {
            continue;
        }
        if norm(entry.label).contains(&needle) {
            seen.push(key);
            primary.push(entry);
        } else if norm(entry.sub).contains(&needle) {
            secondary.push(entry);
        }
    }

    let mut results = primary;
    if results.len() < SECONDARY_CUTOFF {
        for entry in secondary {
            let key = (entry.course, entry.label);
            if seen.contains(&key) {
                continue;
            }
            seen.push(key);
            results.push(entry);
        }
    }

    results.truncate(MAX_RESULTS);
    results
}

/// Tronque un sous-titre trop long, comme le faisait l'affichage d'origine.
pub fn truncate_sub(sub: &str) -> String {
    const LIMIT: usize = 78;
    if sub.chars().count() > LIMIT {
        let head: String = sub.chars().take(LIMIT).collect();
        format!("{head}…")
    } else {
        sub.to_string()
    }
}
