//! Cohérence des prompts : chaque prompt vise une section réelle du cours,
//! porte le bon numéro de sommaire, et ne laisse aucun champ vide.

use super::*;
use crate::pages::sections;

#[test]
fn chaque_prompt_vise_une_section_du_cours() {
    for course in COURSES {
        for p in prompts(course) {
            assert_eq!(p.course, course, "prompt rangé dans la mauvaise matière");
            assert!(
                sections(course).contains(&p.anchor),
                "{course:?} : le prompt « {} » vise l'ancre inconnue « {} »",
                p.title,
                p.anchor
            );
        }
    }
}

#[test]
fn le_numero_vient_du_sommaire() {
    // Un chapitre absent du sommaire retomberait sur « — » : les seuls tirets
    // admis sont ceux que le sommaire porte lui-même.
    assert_eq!(SF[0].num(), "01");
    assert_eq!(MTC[8].num(), "09");
    assert_eq!(ALGO[0].num(), "02");
    for course in COURSES {
        for p in prompts(course) {
            let listed = crate::pages::toc_of(course)
                .iter()
                .any(|i| i.anchor == p.anchor && i.num == p.num());
            assert!(listed, "{course:?} : « {} » absent du sommaire", p.anchor);
        }
    }
}

#[test]
fn aucun_champ_vide_et_pas_de_doublon() {
    for course in COURSES {
        let mut seen: Vec<&str> = Vec::new();
        for p in prompts(course) {
            assert!(!p.title.trim().is_empty());
            assert!(!p.evidence.trim().is_empty(), "{} sans calibrage", p.title);
            assert!(
                p.body.len() > 400,
                "{} : prompt trop court pour être utile",
                p.title
            );
            assert!(
                !seen.contains(&p.anchor),
                "{course:?} : deux prompts pour « {} »",
                p.anchor
            );
            seen.push(p.anchor);
        }
        assert!(
            prompts(course).len() >= 4,
            "{course:?} : trop peu de prompts"
        );
    }
}

#[test]
fn les_liens_sont_bien_formes() {
    let p = &SF[0];
    assert_eq!(p.href(), "/structures-fondamentales#sf-logique");
    assert_eq!(p.practice_href(), "/entrainement/structures-fondamentales");
    assert_eq!(MTC[8].href(), "/methodes-calcul#m-primitives");
}

#[test]
fn le_classeur_compte_trente_cinq_prompts() {
    assert_eq!(total(), 35, "un prompt a disparu ou a été ajouté");
}

#[test]
fn les_niveaux_sont_distincts() {
    for lv in [
        Level::Acquis,
        Level::Central,
        Level::Exigeant,
        Level::Marginal,
    ] {
        assert!(!lv.label().is_empty());
        assert!(!lv.key().is_empty());
    }
    // Le dépouillement des annales doit ressortir quelque part.
    let marginaux = COURSES
        .iter()
        .flat_map(|c| prompts(*c))
        .filter(|p| p.level == Level::Marginal)
        .count();
    assert!(marginaux >= 2, "aucun chapitre signalé comme jamais tombé");
}
