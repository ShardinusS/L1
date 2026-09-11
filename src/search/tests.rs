//! Tests du moteur de recherche, exécutés sans navigateur.

use super::*;
use crate::course::Course;

/// Index synthétique : cinq libellés qui commencent tous par « alpha » et une
/// entrée dont seul le sous-titre correspond, pour éprouver le seuil.
const A: IndexEntry = IndexEntry::new(Course::Sf, "sf-logique", "alpha un", "premier");
const B: IndexEntry = IndexEntry::new(Course::Sf, "sf-logique", "alpha deux", "deuxième");
const C: IndexEntry = IndexEntry::new(Course::Sf, "sf-logique", "alpha trois", "troisième");
const D: IndexEntry = IndexEntry::new(Course::Sf, "sf-logique", "alpha quatre", "quatrième");
const E: IndexEntry = IndexEntry::new(Course::Sf, "sf-logique", "alpha cinq", "cinquième");
const F: IndexEntry = IndexEntry::new(Course::Sf, "sf-logique", "alpha six", "sixième");
const SECOND: IndexEntry = IndexEntry::new(Course::Os, "o-os", "sans rapport", "alpha caché");

const CINQ: &[IndexEntry] = &[A, B, C, D, E, SECOND];
const SIX: &[IndexEntry] = &[A, B, C, D, E, F, SECOND];
const TABLE_CINQ: &[&[IndexEntry]] = &[CINQ];
const TABLE_SIX: &[&[IndexEntry]] = &[SIX];

fn labels(entries: &[&IndexEntry]) -> Vec<&'static str> {
    entries.iter().map(|e| e.label).collect()
}

#[test]
fn normalisation_des_accents() {
    assert_eq!(norm("Représentation"), "representation");
    assert_eq!(norm("SYSTÈMES d'Exploitation"), "systemes d'exploitation");
    assert_eq!(norm("Dénombrement"), "denombrement");
    assert_eq!(norm("ÉÀÙÎÔÇ"), "eauioc");
}

#[test]
fn requete_vide_ne_renvoie_rien() {
    assert!(search(index(), "").is_empty());
    assert!(search(index(), "   ").is_empty());
}

#[test]
fn correspondance_sur_le_libelle_d_abord() {
    let found = search(index(), "ensembles");
    let first = found.first().expect("au moins un résultat");
    assert_eq!(first.anchor, "sf-ensembles");
    assert_eq!(first.course, Course::Sf);
}

#[test]
fn recherche_insensible_aux_accents_et_a_la_casse() {
    let a = labels(&search(index(), "representation"));
    let b = labels(&search(index(), "REPRÉSENTATION"));
    assert_eq!(a, b);
    assert!(!a.is_empty());
}

#[test]
fn les_commandes_shell_sont_trouvees() {
    let found = search(index(), "uniq");
    let entry = found.first().expect("uniq est dans l'aide-mémoire");
    assert!(entry.mono, "une commande s'affiche en chasse fixe");
    assert_eq!(entry.course, Course::Os);
}

#[test]
fn le_second_rang_apparait_quand_le_premier_est_maigre() {
    let found = search(TABLE_CINQ, "alpha");
    assert_eq!(
        labels(&found),
        vec![
            "alpha un",
            "alpha deux",
            "alpha trois",
            "alpha quatre",
            "alpha cinq",
            "sans rapport",
        ]
    );
}

#[test]
fn le_second_rang_disparait_au_dela_du_seuil() {
    let found = search(TABLE_SIX, "alpha");
    assert_eq!(found.len(), SECONDARY_CUTOFF);
    assert!(!labels(&found).contains(&"sans rapport"));
}

#[test]
fn dedoublonnage_par_matiere_et_libelle() {
    const DOUBLON: &[IndexEntry] = &[
        IndexEntry::new(Course::Sf, "sf-logique", "même titre", "un"),
        IndexEntry::new(Course::Sf, "sf-ensembles", "même titre", "deux"),
        IndexEntry::new(Course::Os, "o-os", "même titre", "trois"),
    ];
    let found = search(&[DOUBLON], "même titre");
    assert_eq!(found.len(), 2, "un par matière");
    assert_eq!(found[0].anchor, "sf-logique");
    assert_eq!(found[1].course, Course::Os);
}

#[test]
fn plafond_a_quatorze_resultats() {
    let found = search(index(), "e");
    assert!(found.len() <= MAX_RESULTS);
    assert_eq!(found.len(), MAX_RESULTS, "« e » est partout");
}

#[test]
fn requete_sans_resultat() {
    assert!(search(index(), "zzzzzz").is_empty());
}

#[test]
fn lien_d_une_entree() {
    let found = search(index(), "aide-mémoire");
    let entry = found.first().expect("l'aide-mémoire est indexé");
    assert_eq!(entry.href(), "/systemes#o-recap");
}

#[test]
fn troncature_du_sous_titre() {
    let court = "trois mots courts";
    assert_eq!(truncate_sub(court), court);

    let long = "é".repeat(100);
    let coupe = truncate_sub(&long);
    assert_eq!(coupe.chars().count(), 79, "78 caractères plus l'ellipse");
    assert!(coupe.ends_with('…'));
}

#[test]
fn l_index_reel_n_est_pas_vide() {
    let total: usize = index().iter().map(|s| s.len()).sum();
    assert!(total > 200, "index anormalement court : {total}");
}
