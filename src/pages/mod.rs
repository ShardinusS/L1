//! Pages du classeur, une par matière plus l'accueil.
//!
//! Les modules sont produits à partir du classeur HTML d'origine : le contenu
//! y est repris tel quel, et l'index de recherche est déclaré à côté des
//! sections qu'il désigne.

pub mod algo;
pub mod entrainement;
pub mod home;
pub mod info;
pub mod mtc;
pub mod os;
pub mod prompts;
pub mod sf;

use crate::components::toc::TocItem;
use crate::course::Course;
use crate::search::IndexEntry;

/// Index de recherche, une tranche par matière, dans l'ordre des onglets.
pub const INDEXES: &[&[IndexEntry]] = &[sf::INDEX, mtc::INDEX, algo::INDEX, info::INDEX, os::INDEX];

/// Ancres des sections de l'accueil, qui n'est pas une matière.
pub const HOME_SECTIONS: &[&str] = home::SECTIONS;

/// Sommaire d'une matière.
pub fn toc_of(course: Course) -> &'static [TocItem] {
    match course {
        Course::Sf => sf::TOC,
        Course::Mtc => mtc::TOC,
        Course::Algo => algo::TOC,
        Course::Info => info::TOC,
        Course::Os => os::TOC,
    }
}

/// Ancres de section déclarées par une matière.
pub fn sections(course: Course) -> &'static [&'static str] {
    match course {
        Course::Sf => sf::SECTIONS,
        Course::Mtc => mtc::SECTIONS,
        Course::Algo => algo::SECTIONS,
        Course::Info => info::SECTIONS,
        Course::Os => os::SECTIONS,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::COURSES;

    fn toc(course: Course) -> &'static [TocItem] {
        toc_of(course)
    }

    fn index_of(course: Course) -> &'static [IndexEntry] {
        INDEXES[COURSES.iter().position(|c| *c == course).unwrap()]
    }

    /// Préfixe attendu des ancres, matière par matière.
    fn prefix(course: Course) -> &'static str {
        match course {
            Course::Sf => "sf-",
            Course::Mtc => "m-",
            Course::Algo => "al-",
            Course::Info => "i-",
            Course::Os => "o-",
        }
    }

    #[test]
    fn chaque_entree_vise_une_section_existante() {
        for course in COURSES {
            let known = sections(course);
            for entry in index_of(course) {
                assert_eq!(
                    entry.course, course,
                    "entrée rangée dans la mauvaise matière"
                );
                assert!(
                    known.contains(&entry.anchor),
                    "{course:?} : l'entrée « {} » vise l'ancre inconnue « {} »",
                    entry.label,
                    entry.anchor
                );
            }
        }
    }

    #[test]
    fn chaque_lien_du_sommaire_vise_une_section_existante() {
        for course in COURSES {
            let known = sections(course);
            for item in toc(course) {
                assert!(
                    known.contains(&item.anchor),
                    "{course:?} : le sommaire renvoie à l'ancre inconnue « {} »",
                    item.anchor
                );
            }
        }
    }

    #[test]
    fn chaque_section_est_au_sommaire() {
        for course in COURSES {
            let listed: Vec<&str> = toc(course).iter().map(|i| i.anchor).collect();
            for anchor in sections(course) {
                assert!(
                    listed.contains(anchor),
                    "{course:?} : la section « {anchor} » manque au sommaire"
                );
            }
        }
    }

    #[test]
    fn les_ancres_portent_le_prefixe_de_leur_matiere() {
        for course in COURSES {
            for anchor in sections(course) {
                assert!(
                    anchor.starts_with(prefix(course)),
                    "{anchor} ne commence pas par {}",
                    prefix(course)
                );
            }
        }
    }

    #[test]
    fn aucune_ancre_en_double() {
        let mut all: Vec<&str> = HOME_SECTIONS.to_vec();
        all.extend(COURSES.iter().flat_map(|c| sections(*c)).copied());
        let count = all.len();
        all.sort_unstable();
        all.dedup();
        assert_eq!(count, all.len(), "deux sections partagent la même ancre");
    }

    #[test]
    fn l_index_couvre_toutes_les_matieres() {
        assert_eq!(INDEXES.len(), COURSES.len());
        for slice in INDEXES {
            assert!(!slice.is_empty());
        }
    }

    #[test]
    fn le_classeur_compte_cinquante_neuf_sections() {
        let total = HOME_SECTIONS.len() + COURSES.iter().map(|c| sections(*c).len()).sum::<usize>();
        assert_eq!(total, 59, "une section a disparu ou a été ajoutée");
    }
}
