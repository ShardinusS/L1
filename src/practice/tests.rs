//! Cohérence des banques d'exercices : liens vers le cours, forme des
//! questions, et surtout accord entre chaque réponse attendue et le
//! correcteur, sur des centaines de tirages.

use super::*;
use crate::pages::sections;

/// Tirages par générateur.
const SEEDS: u64 = 400;

fn all_chapters() -> impl Iterator<Item = (Course, &'static Chapter)> {
    COURSES
        .iter()
        .flat_map(|c| chapters(*c).iter().map(move |ch| (*c, ch)))
}

/// Vérifie qu'un exercice est posable et que sa réponse attendue passe
/// le correcteur.
fn check(ex: &Exercise, origin: &str) {
    assert!(!ex.prompt.trim().is_empty(), "{origin} : énoncé vide");
    match &ex.answer {
        Answer::Choice { options, correct } => {
            assert!(options.len() >= 2, "{origin} : moins de deux options");
            assert!(
                *correct < options.len(),
                "{origin} : bonne réponse hors liste"
            );
            let mut sorted = options.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(
                sorted.len(),
                options.len(),
                "{origin} : options en double {options:?}"
            );
            assert!(!ex.explain.is_empty(), "{origin} : pas d'explication");
        }
        Answer::Card { back } => assert!(!back.is_empty(), "{origin} : carte sans verso"),
        answer => {
            let expected = answer.expected();
            assert!(
                answer.grade(&Response::Text(expected.clone())),
                "{origin} : la réponse attendue « {expected} » est refusée ({})",
                ex.prompt
            );
            assert!(!ex.explain.is_empty(), "{origin} : pas d'explication");
        }
    }
}

#[test]
fn chaque_chapitre_renvoie_a_une_section_du_cours() {
    for (course, ch) in all_chapters() {
        assert!(
            sections(course).contains(&ch.anchor),
            "{course:?}/{} : ancre inconnue « {} »",
            ch.id,
            ch.anchor
        );
    }
}

#[test]
fn identifiants_uniques_et_chapitres_non_vides() {
    for course in COURSES {
        let ids: Vec<&str> = chapters(course).iter().map(|c| c.id).collect();
        let mut dedup = ids.clone();
        dedup.sort_unstable();
        dedup.dedup();
        assert_eq!(
            ids.len(),
            dedup.len(),
            "{course:?} : identifiant de chapitre en double"
        );
        assert!(ids.len() >= 3, "{course:?} : programme trop maigre");
        for ch in chapters(course) {
            assert!(
                !ch.fixed.is_empty() || !ch.generators.is_empty(),
                "{course:?}/{} : aucun exercice",
                ch.id
            );
            assert!(!ch.id.contains(char::is_whitespace));
        }
    }
}

#[test]
fn questions_fixes_bien_formees() {
    let mut rng = Rng::new(3);
    for (course, ch) in all_chapters() {
        for (i, f) in ch.fixed.iter().enumerate() {
            let origin = format!("{course:?}/{} n°{i}", ch.id);
            check(&f.to_exercise(&mut rng), &origin);
        }
    }
}

#[test]
fn generateurs_coherents_sur_des_centaines_de_tirages() {
    for (course, ch) in all_chapters() {
        for (g, generator) in ch.generators.iter().enumerate() {
            for seed in 0..SEEDS {
                let ex = generator(&mut Rng::new(seed));
                check(
                    &ex,
                    &format!("{course:?}/{} générateur {g} graine {seed}", ch.id),
                );
            }
        }
    }
}

#[test]
fn une_serie_respecte_le_filtre_et_le_nombre() {
    let mut rng = Rng::new(11);
    let qs = draw(Filter::Course(Course::Info), 10, &mut rng);
    assert_eq!(qs.len(), 10);
    assert!(qs.iter().all(|q| q.course == Course::Info));

    let qs = draw(Filter::Chapter(Course::Sf, "permutations"), 8, &mut rng);
    assert_eq!(qs.len(), 8);
    assert!(qs.iter().all(|q| q.chapter_id == "permutations"));

    let qs = draw(Filter::All, 30, &mut rng);
    let mut courses: Vec<Course> = qs.iter().map(|q| q.course).collect();
    courses.dedup();
    assert!(
        courses.len() > 1,
        "une série mélangée doit toucher plusieurs matières"
    );
}

#[test]
fn une_serie_ne_repete_pas_une_question() {
    for seed in 0..20 {
        let qs = draw(Filter::All, 20, &mut Rng::new(seed));
        for (i, a) in qs.iter().enumerate() {
            for b in &qs[i + 1..] {
                assert!(
                    a.exercise.prompt != b.exercise.prompt || a.exercise.code != b.exercise.code,
                    "doublon : {}",
                    a.exercise.prompt
                );
            }
        }
    }
}

#[test]
fn filtre_inconnu_renvoie_une_serie_vide() {
    assert!(draw(
        Filter::Chapter(Course::Os, "inexistant"),
        5,
        &mut Rng::new(0)
    )
    .is_empty());
}

#[test]
fn exposants_et_indices() {
    assert_eq!(sup(12), "¹²");
    assert_eq!(sub(16), "₁₆");
}
