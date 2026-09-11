//! Matiere 01 - Structures fondamentales.
//!
//! Contenu transposé tel quel depuis le classeur HTML d'origine :
//! mêmes textes, mêmes formules, mêmes exemples.

use leptos::prelude::*;

use crate::components::toc::{Toc, TocItem};
use crate::course::Course;
use crate::search::IndexEntry;

/// Ancres des sections de la page, dans l'ordre d'affichage.
#[rustfmt::skip]
pub const SECTIONS: &[&str] = &[
    "sf-logique",
    "sf-ensembles",
    "sf-applications",
    "sf-denombrement",
    "sf-nombres",
    "sf-complexes",
    "sf-algebre",
    "sf-td1",
    "sf-td2",
    "sf-td3",
    "sf-td4",
    "sf-recurrent",
    "sf-annales",
    "sf-notes",
    "sf-outils",
];

/// Sommaire de la matière : (ancre, numéro, titre).
#[rustfmt::skip]
pub const TOC: &[TocItem] = &[
    TocItem { anchor: "sf-logique", num: "01", title: "Logique et raisonnements" },
    TocItem { anchor: "sf-ensembles", num: "02", title: "Ensembles" },
    TocItem { anchor: "sf-applications", num: "03", title: "Relations et applications" },
    TocItem { anchor: "sf-denombrement", num: "04", title: "Cardinaux et dénombrement" },
    TocItem { anchor: "sf-nombres", num: "05", title: "Les ensembles de nombres" },
    TocItem { anchor: "sf-complexes", num: "06", title: "Nombres complexes" },
    TocItem { anchor: "sf-algebre", num: "07", title: "Structures algébriques" },
    TocItem { anchor: "sf-td1", num: "TD1", title: "Logique" },
    TocItem { anchor: "sf-td2", num: "TD2", title: "Ensembles et applications" },
    TocItem { anchor: "sf-td3", num: "TD3", title: "Nombres complexes" },
    TocItem { anchor: "sf-td4", num: "TD4", title: "Groupes et permutations" },
    TocItem { anchor: "sf-recurrent", num: "★", title: "Questions récurrentes" },
    TocItem { anchor: "sf-annales", num: "—", title: "Annales et corrigés" },
    TocItem { anchor: "sf-notes", num: "—", title: "Le niveau réel au partiel" },
    TocItem { anchor: "sf-outils", num: "—", title: "Boîte à outils" },
];

/// Libellé accessible du sommaire.
pub const TOC_LABEL: &str = "Sommaire de Structures fondamentales";

/// Entrées de recherche de la matière, dans l'ordre du cours.
#[rustfmt::skip]
pub const INDEX: &[IndexEntry] = &[
    IndexEntry::new(Course::Sf, "sf-logique", "Logique et raisonnements", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-logique", "Les connecteurs", "Logique et raisonnements"),
    IndexEntry::new(Course::Sf, "sf-logique", "Les équivalences à connaître", "Logique et raisonnements"),
    IndexEntry::new(Course::Sf, "sf-logique", "Les quantificateurs", "Logique et raisonnements"),
    IndexEntry::new(Course::Sf, "sf-logique", "La récurrence", "Logique et raisonnements"),
    IndexEntry::new(Course::Sf, "sf-logique", "Les cinq raisonnements classiques", "Logique et raisonnements"),
    IndexEntry::new(Course::Sf, "sf-logique", "⇒ n'est pas «\u{a0}donc\u{a0}»", "Logique et raisonnements"),
    IndexEntry::new(Course::Sf, "sf-logique", "Ce qui est vraiment évalué", "Logique et raisonnements"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Ensembles", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Les ensembles usuels", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Appartenance, inclusion, égalité", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Les opérations", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Les règles de calcul", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Appartenance x ∈ E", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Inclusion A ⊂ B", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Égalité A = B", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "∈ et ⊂ ne se remplacent pas", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-ensembles", "Méthode — prouver une égalité d'ensembles", "Ensembles"),
    IndexEntry::new(Course::Sf, "sf-applications", "Relations et applications", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-applications", "Relations binaires", "Relations et applications"),
    IndexEntry::new(Course::Sf, "sf-applications", "Fonction et application", "Relations et applications"),
    IndexEntry::new(Course::Sf, "sf-applications", "Injection, surjection, bijection", "Relations et applications"),
    IndexEntry::new(Course::Sf, "sf-applications", "Composition", "Relations et applications"),
    IndexEntry::new(Course::Sf, "sf-applications", "Image directe, image réciproque", "Relations et applications"),
    IndexEntry::new(Course::Sf, "sf-applications", "Les deux réciproques partielles", "Relations et applications"),
    IndexEntry::new(Course::Sf, "sf-applications", "Méthode — montrer qu'une application est bijective", "Relations et applications"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Cardinaux et dénombrement", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Ce que le cardinal impose", "Cardinaux et dénombrement"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Cantor", "Cardinaux et dénombrement"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Les formules de comptage", "Cardinaux et dénombrement"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Les identités binomiales du TD", "Cardinaux et dénombrement"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Équipotence", "Cardinaux et dénombrement"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Ensemble fini, cardinal", "Cardinaux et dénombrement"),
    IndexEntry::new(Course::Sf, "sf-denombrement", "Dénombrable", "Cardinaux et dénombrement"),
    IndexEntry::new(Course::Sf, "sf-nombres", "Les ensembles de nombres", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-nombres", "Ce qui est exigible", "Les ensembles de nombres"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Nombres complexes", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Définition et forme algébrique", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Conjugué et module", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Forme trigonométrique et exponentielle", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Linéarisation", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Équations polynomiales", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Racines n-ièmes de l'unité", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Méthode — mettre un quotient sous forme algébrique", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Méthode — linéariser cos(x)^p · sin(x)^q", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-complexes", "Le second degré dans ℂ", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Structures algébriques", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Lois de composition interne", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Groupes", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Anneaux et corps", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Morphismes", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Divisibilité dans ℤ", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Congruences et ℤ/nℤ", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Permutations", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Groupe", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Sous-groupe", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Ordre d'un élément", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Groupe symétrique", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Support et points fixes", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Cycle", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Transposition", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Méthode — reste de a^N modulo m", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-algebre", "Méthode — la question type sur une permutation", "Structures algébriques"),
    IndexEntry::new(Course::Sf, "sf-td1", "Logique", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-td1", "Ces énoncés sont-ils des assertions\u{a0}?", "Logique · ex. 1"),
    IndexEntry::new(Course::Sf, "sf-td1", "Écrire en français la négation d'énoncés courants.", "Logique · ex. 2"),
    IndexEntry::new(Course::Sf, "sf-td1", "Comparer des assertions composées.", "Logique · ex. 3"),
    IndexEntry::new(Course::Sf, "sf-td1", "Ensembles de vérité, implications, conditions suffisantes.", "Logique · ex. 4-6"),
    IndexEntry::new(Course::Sf, "sf-td1", "Variables libres et variables muettes.", "Logique · ex. 7"),
    IndexEntry::new(Course::Sf, "sf-td1", "Nier en poussant ¬ jusqu'aux assertions élémentaires.", "Logique · ex. 8"),
    IndexEntry::new(Course::Sf, "sf-td1", "Traduire des propriétés de fonctions en quantificateurs — et l'inverse.", "Logique · ex. 9"),
    IndexEntry::new(Course::Sf, "sf-td1", "Absurde et contraposée.", "Logique · ex. 10-13"),
    IndexEntry::new(Course::Sf, "sf-td1", "Récurrence.", "Logique · ex. 14-15"),
    IndexEntry::new(Course::Sf, "sf-td1", "QCM de logique appliquée (test de positionnement).", "Logique · ex. 16"),
    IndexEntry::new(Course::Sf, "sf-td2", "Ensembles, relations, applications, dénombrement", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-td2", "Écritures correctes et parties d'un ensemble.", "Ensembles, relations, applications, dénombrement · ex. 1-3"),
    IndexEntry::new(Course::Sf, "sf-td2", "Simplifier et comparer.", "Ensembles, relations, applications, dénombrement · ex. 4-6"),
    IndexEntry::new(Course::Sf, "sf-td2", "A ∩ B = A ∩ C entraîne-t-il B = C\u{a0}?", "Ensembles, relations, applications, dénombrement · ex. 7"),
    IndexEntry::new(Course::Sf, "sf-td2", "Identités et complémentaires.", "Ensembles, relations, applications, dénombrement · ex. 8-13"),
    IndexEntry::new(Course::Sf, "sf-td2", "Relations binaires et ordres.", "Ensembles, relations, applications, dénombrement · ex. 14-17"),
    IndexEntry::new(Course::Sf, "sf-td2", "Fonction ou application\u{a0}? injective, surjective, bijective\u{a0}?", "Ensembles, relations, applications, dénombrement · ex. 18-19"),
    IndexEntry::new(Course::Sf, "sf-td2", "Composition et bijections.", "Ensembles, relations, applications, dénombrement · ex. 20-23"),
    IndexEntry::new(Course::Sf, "sf-td2", "Fonction indicatrice 1_A.", "Ensembles, relations, applications, dénombrement · ex. 24"),
    IndexEntry::new(Course::Sf, "sf-td2", "Images directes et réciproques.", "Ensembles, relations, applications, dénombrement · ex. 25-27"),
    IndexEntry::new(Course::Sf, "sf-td2", "Dénombrement.", "Ensembles, relations, applications, dénombrement · ex. 28-35"),
    IndexEntry::new(Course::Sf, "sf-td3", "Nombres complexes", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-td3", "Parties réelle et imaginaire, formes algébrique et trigonométrique.", "Nombres complexes · ex. 1-4"),
    IndexEntry::new(Course::Sf, "sf-td3", "Modules et arguments.", "Nombres complexes · ex. 5-9"),
    IndexEntry::new(Course::Sf, "sf-td3", "Équations en Z + 1/Z, formules de duplication, linéarisation.", "Nombres complexes · ex. 10-12"),
    IndexEntry::new(Course::Sf, "sf-td3", "Équations dans ℂ.", "Nombres complexes · ex. 13-14"),
    IndexEntry::new(Course::Sf, "sf-td3", "Racines de l'unité.", "Nombres complexes · ex. 15-21"),
    IndexEntry::new(Course::Sf, "sf-td3", "Méthode — les équations en (z+1)/(z−1)", "Nombres complexes"),
    IndexEntry::new(Course::Sf, "sf-td4", "Arithmétique modulaire, groupes, permutations", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-td4", "Partie 1 — Le groupe ℤ/nℤ (ex. 1 à 17)", "Arithmétique modulaire, groupes, permutations"),
    IndexEntry::new(Course::Sf, "sf-td4", "Partie 2 — Groupes (ex. 18 à 30)", "Arithmétique modulaire, groupes, permutations"),
    IndexEntry::new(Course::Sf, "sf-td4", "Partie 3 — Permutations (ex. 31 à 49)", "Arithmétique modulaire, groupes, permutations"),
    IndexEntry::new(Course::Sf, "sf-td4", "Division euclidienne dans la vraie vie.", "Arithmétique modulaire, groupes, permutations · ex. 1-2"),
    IndexEntry::new(Course::Sf, "sf-td4", "Restes et classes d'équivalence.", "Arithmétique modulaire, groupes, permutations · ex. 3-5"),
    IndexEntry::new(Course::Sf, "sf-td4", "Structure de ℤ/7Z et ℤ/12ℤ.", "Arithmétique modulaire, groupes, permutations · ex. 6-8"),
    IndexEntry::new(Course::Sf, "sf-td4", "Bijectivité de x ↦ ax + b dans ℤ/nℤ.", "Arithmétique modulaire, groupes, permutations · ex. 7"),
    IndexEntry::new(Course::Sf, "sf-td4", "Équations et critères de divisibilité.", "Arithmétique modulaire, groupes, permutations · ex. 9-16"),
    IndexEntry::new(Course::Sf, "sf-td4", "Reste de 40071235 par 13.", "Arithmétique modulaire, groupes, permutations · ex. 17"),
    IndexEntry::new(Course::Sf, "sf-td4", "Sous-groupes et produits.", "Arithmétique modulaire, groupes, permutations · ex. 18-21"),
    IndexEntry::new(Course::Sf, "sf-td4", "Est-ce un groupe\u{a0}?", "Arithmétique modulaire, groupes, permutations · ex. 22-23"),
    IndexEntry::new(Course::Sf, "sf-td4", "Propriétés générales.", "Arithmétique modulaire, groupes, permutations · ex. 24-28"),
    IndexEntry::new(Course::Sf, "sf-td4", "Deux groupes à construire.", "Arithmétique modulaire, groupes, permutations · ex. 29-30"),
    IndexEntry::new(Course::Sf, "sf-td4", "La batterie de calculs.", "Arithmétique modulaire, groupes, permutations · ex. 31-35"),
    IndexEntry::new(Course::Sf, "sf-td4", "Groupe de Klein et petits résultats.", "Arithmétique modulaire, groupes, permutations · ex. 36-38"),
    IndexEntry::new(Course::Sf, "sf-td4", "Théorie.", "Arithmétique modulaire, groupes, permutations · ex. 39-45"),
    IndexEntry::new(Course::Sf, "sf-td4", "Trois sujets d'examen recopiés dans la fiche.", "Arithmétique modulaire, groupes, permutations · ex. 46-48"),
    IndexEntry::new(Course::Sf, "sf-td4", "Le mélange de cartes.", "Arithmétique modulaire, groupes, permutations · ex. 49"),
    IndexEntry::new(Course::Sf, "sf-recurrent", "Les questions qui reviennent", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-recurrent", "Le programme du partiel n'est pas celui de l'examen", "Les questions qui reviennent"),
    IndexEntry::new(Course::Sf, "sf-annales", "Annales et corrigés", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-annales", "Logique, ensembles, suites", "Annales et corrigés"),
    IndexEntry::new(Course::Sf, "sf-annales", "Modulo, ensembles, permutations", "Annales et corrigés"),
    IndexEntry::new(Course::Sf, "sf-annales", "Logique, applications, indicatrices", "Annales et corrigés"),
    IndexEntry::new(Course::Sf, "sf-annales", "Le plus long, le plus varié", "Annales et corrigés"),
    IndexEntry::new(Course::Sf, "sf-annales", "Corrigé intégré au sujet", "Annales et corrigés"),
    IndexEntry::new(Course::Sf, "sf-annales", "Logique, ensembles, suites", "Partiel · 2024-2025 · 26 pts"),
    IndexEntry::new(Course::Sf, "sf-annales", "Modulo, ensembles, permutations", "Examen final · 2024-2025"),
    IndexEntry::new(Course::Sf, "sf-annales", "Logique, applications, indicatrices", "Partiel · 2025-2026 · 28 pts"),
    IndexEntry::new(Course::Sf, "sf-annales", "Le plus long, le plus varié", "Examen final · 2025-2026 · 31,5 pts"),
    IndexEntry::new(Course::Sf, "sf-annales", "Corrigé intégré au sujet", "Rattrapage · juin 2025 · 31 pts"),
    IndexEntry::new(Course::Sf, "sf-annales", "Méthode — la limite par la définition, quatre fois sur cinq", "Annales et corrigés"),
    IndexEntry::new(Course::Sf, "sf-notes", "Le niveau réel au partiel", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-notes", "Répartition des 304 copies notées", "Le niveau réel au partiel"),
    IndexEntry::new(Course::Sf, "sf-notes", "Le calcul de la note finale de l'UE", "Le niveau réel au partiel"),
    IndexEntry::new(Course::Sf, "sf-outils", "Boîte à outils", "Structures fondamentales"),
    IndexEntry::new(Course::Sf, "sf-outils", "Coefficients de Bezout par l'algorithme d'Euclide", "Boîte à outils"),
    IndexEntry::new(Course::Sf, "sf-outils", "Résoudre x² + bx + c = 0 dans ℤ/nℤ avec n = pq", "Boîte à outils"),
    IndexEntry::new(Course::Sf, "sf-outils", "Les réflexes de rédaction", "Boîte à outils"),
    IndexEntry::new(Course::Sf, "sf-outils", "Ce qui coûte des points sans erreur de maths", "Boîte à outils"),
];

fn masthead() -> impl IntoView {
    view! {
        <header class="masthead">
            <div class="masthead-inner">
                <div class="eyebrow">
                    "Matière 01 · Mathématiques"
                </div>
                <h1>
                    "Structures fondamentales"
                </h1>
                <p class="lede">
                    "Le langage de base des mathématiques\u{a0}: raisonner et rédiger une preuve, manipuler ensembles et applications, compter, calculer avec les complexes, et découvrir les structures algébriques jusqu'aux permutations."
                </p>
                <div class="meta">
                    <span>
                        "Cours de Fabien Durand · UPJV"
                    </span>
                    <span>
                        "L1, semestre 1 · 2026-2027"
                    </span>
                    <span>
                        <span class="mono">
                            "7"
                        </span>
                        " chapitres"
                    </span>
                    <span>
                        <span class="mono">
                            "4"
                        </span>
                        " TD"
                    </span>
                    <span>
                        <span class="mono">
                            "5"
                        </span>
                        " annales"
                    </span>
                </div>
            </div>
        </header>
    }
}

fn sec_sf_logique() -> impl IntoView {
    view! {
        <section id="sf-logique">
            <div class="sec-head">
                <span class="num">
                    "01"
                </span>
                <h2>
                    "Logique et raisonnements"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Une "
                    <strong>
                        "assertion"
                    </strong>
                    " (ou proposition) est un énoncé mathématique susceptible d'être vrai ou faux — et rien d'autre\u{a0}: «\u{a0}2 + 2 = 4\u{a0}» est une assertion, «\u{a0}Que Dieu nous protège\u{a0}!\u{a0}» n'en est pas une."
                </p>
            </div>
            <h3>
                "Les connecteurs"
            </h3>
            <div class="tw" style="max-width:68ch">
                <table class="truth compact">
                    <thead>
                        <tr>
                            <th>
                                "P"
                            </th>
                            <th>
                                "Q"
                            </th>
                            <th>
                                "¬P"
                            </th>
                            <th>
                                "P ∧ Q"
                            </th>
                            <th>
                                "P ∨ Q"
                            </th>
                            <th>
                                "P ⇒ Q"
                            </th>
                            <th>
                                "P ⇔ Q"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                "V"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "V"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "V"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "F"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "F"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "F"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "F"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "F"
                            </td>
                            <td>
                                "V"
                            </td>
                            <td>
                                "V"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="body" style="margin-top:14px">
                <p>
                    "Le «\u{a0}ou\u{a0}» mathématique est "
                    <strong>
                        "inclusif"
                    </strong>
                    ". L'implication est "
                    <em>
                        "définie"
                    </em>
                    " par "
                    <span class="fi">
                        "P ⇒ Q ≡ Q ∨ ¬P"
                    </span>
                    "\u{a0}: elle est donc vraie dès que P est fausse. "
                    <span class="fi">
                        "Q ⇒ P"
                    </span>
                    " est la "
                    <strong>
                        "réciproque"
                    </strong>
                    " de "
                    <span class="fi">
                        "P ⇒ Q"
                    </span>
                    ", et "
                    <span class="fi">
                        "P ⇔ Q"
                    </span>
                    " abrège "
                    <span class="fi">
                        "(P ⇒ Q) ∧ (Q ⇒ P)"
                    </span>
                    "."
                </p>
            </div>
            <div class="note warn" style="margin-top:18px">
                <span class="title">
                    "⇒ n'est pas «\u{a0}donc\u{a0}»"
                </span>
                <p>
                    <span class="fi">
                        "(1 = 2) ⇒ (6 > 7)"
                    </span>
                    " est une assertion "
                    <em>
                        "vraie"
                    </em>
                    ". «\u{a0}1 = 2 donc 6 > 7\u{a0}» énonce deux faussetés et affirme une causalité. De même, «\u{a0}f est dérivable ⇒ f est continue\u{a0}» n'affirme ni l'une ni l'autre, alors que «\u{a0}f est dérivable donc f est continue\u{a0}» affirme les deux."
                </p>
                <p>
                    "Conséquence pratique\u{a0}: dans une copie, on écrit "
                    <em>
                        "donc"
                    </em>
                    ", "
                    <em>
                        "or"
                    </em>
                    ", "
                    <em>
                        "car"
                    </em>
                    ", "
                    <em>
                        "c'est-à-dire"
                    </em>
                    " — pas les symboles logiques, et jamais les quantificateurs comme abréviations de français."
                </p>
            </div>
            <h3>
                "Les équivalences à connaître"
            </h3>
            <span class="f">
                "¬(P ∧ Q)  ⇔  (¬P) ∨ (¬Q)          lois de De Morgan\n¬(P ∨ Q)  ⇔  (¬P) ∧ (¬Q)\nP         ⇔  ¬(¬P)\n(P ⇒ Q)   ⇔  (¬Q ⇒ ¬P)               contraposition\n((¬P) ⇒ (Q ∧ ¬Q))  ⇒  P              raisonnement par l'absurde"
            </span>
            <h3>
                "Les quantificateurs"
            </h3>
            <div class="body">
                <p>
                    "Une proposition peut dépendre d'une variable\u{a0}: "
                    <span class="fi">
                        "P(x)"
                    </span>
                    " est alors une fonction de E dans {Vrai, Faux}. On la quantifie\u{a0}:"
                </p>
            </div>
            <span class="f">
                "∃x ∈ E, P(x)     il existe x dans E tel que P(x) est vraie\n∀x ∈ E, P(x)     P(x) est vraie pour tout x de E\n\n¬(∃x ∈ E, P(x))  ⇔  ∀x ∈ E, ¬P(x)\n¬(∀x ∈ E, P(x))  ⇔  ∃x ∈ E, ¬P(x)"
            </span>
            <div class="body" style="margin-top:14px">
                <p>
                    "Deux pièges classiques\u{a0}: si E est "
                    <strong>
                        "vide"
                    </strong>
                    ", "
                    <span class="fi">
                        "∀x ∈ E, P(x)"
                    </span>
                    " est toujours vraie\u{a0}; et l'"
                    <strong>
                        "ordre"
                    </strong>
                    " des quantificateurs change tout — "
                    <span class="fi">
                        "∃x, ∀y, x + y ≥ 0"
                    </span>
                    " est fausse dans ℝ alors que "
                    <span class="fi">
                        "∀x, ∃y, x + y ≥ 0"
                    </span>
                    " est vraie "
                    <span class="tag hot">
                        "annale"
                    </span>
                    "."
                </p>
            </div>
            <span class="f">
                "(∀x, P(x) ∧ Q(x))  ⇔  (∀x, P(x)) ∧ (∀x, Q(x))     ∀ se distribue sur ∧\n(∃x, P(x) ∨ Q(x))  ⇔  (∃x, P(x)) ∨ (∃x, Q(x))     ∃ se distribue sur ∨\n\n(∃x, P(x) ∧ Q(x))  ⇒  (∃x, P(x)) ∧ (∃x, Q(x))     seulement une implication\n(∀x, P(x)) ∨ (∀x, Q(x))  ⇒  (∀x, P(x) ∨ Q(x))     seulement une implication"
            </span>
            <h3>
                "La récurrence"
            </h3>
            <div class="body">
                <p>
                    "Pour prouver "
                    <span class="fi">
                        "∀n ≥ n₀, P(n)"
                    </span>
                    ", il suffit de prouver "
                    <span class="fi">
                        "P(n₀)"
                    </span>
                    " (initialisation) et "
                    <span class="fi">
                        "∀n ≥ n₀, P(n) ⇒ P(n+1)"
                    </span>
                    " (hérédité). En "
                    <strong>
                        "récurrence forte"
                    </strong>
                    ", on peut supposer P vraie à tous les rangs de n₀ à n pour démontrer P(n+1)."
                </p>
                <p>
                    "Le principe repose sur une propriété de ℕ\u{a0}: "
                    <strong>
                        "toute partie non vide de ℕ admet un plus petit élément"
                    </strong>
                    " — on dit que ℕ est bien ordonné. Rien de tel dans ℝ\u{a0}: ]0,1[ est non vide et borné, sans plus petit ni plus grand élément."
                </p>
            </div>
            <div class="ex" style="margin-top:14px">
                <div class="ex-head">
                    <span>
                        "Rédaction type"
                    </span>
                    <span class="mono">
                        "Σ n² "
                    </span>
                </div>
                <div class="ex-body">
                    <p>
                        "Considérons P(N)\u{a0}: "
                        <span class="fi">
                            "Σ"
                            <sub>
                                "n=0..N"
                            </sub>
                            " n² = N(N+1)(2N+1)/6"
                        </span>
                        "."
                    </p>
                    <p>
                        <strong>
                            "Initialisation."
                        </strong>
                        " P(0) est vraie\u{a0}: les deux membres valent 0."
                    </p>
                    <p>
                        <strong>
                            "Hérédité."
                        </strong>
                        " Soit N ∈ ℕ, supposons P(N). Alors"
                    </p>
                    <span class="mono">
                        "Σ"
                        <sub>
                            "n=0..N+1"
                        </sub>
                        " n² = N(N+1)(2N+1)/6 + (N+1)² = (2N³ + 9N² + 13N + 6)/6"
                    </span>
                    <p>
                        "et par ailleurs (N+1)(N+2)(2N+3)/6 = (2N³ + 9N² + 13N + 6)/6. Les deux membres coïncident, donc P(N+1) est vraie."
                    </p>
                    <p>
                        <strong>
                            "Conclusion."
                        </strong>
                        " P(N) est vraie pour tout N ∈ ℕ."
                    </p>
                </div>
            </div>
            <h3>
                "Les cinq raisonnements classiques"
            </h3>
            <div class="tw">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Raisonnement"
                            </th>
                            <th>
                                "Quand"
                            </th>
                            <th>
                                "Principe"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <strong>
                                    "Direct"
                                </strong>
                            </td>
                            <td>
                                "Prouver P ⇒ Q"
                            </td>
                            <td>
                                "«\u{a0}Supposons P\u{a0}», puis on établit Q en utilisant cette information."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Par l'absurde"
                                </strong>
                            </td>
                            <td>
                                "Prouver P"
                            </td>
                            <td>
                                "On suppose ¬P, on en déduit une contradiction (une assertion Q avec Q et ¬Q), on conclut P."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Par contraposée"
                                </strong>
                            </td>
                            <td>
                                "Prouver P ⇒ Q"
                            </td>
                            <td>
                                "On prouve ¬Q ⇒ ¬P, qui lui est équivalente. Typique\u{a0}: «\u{a0}n² pair ⇒ n pair\u{a0}»."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Par contre-exemple"
                                </strong>
                            </td>
                            <td>
                                "Réfuter ∀x, P(x)"
                            </td>
                            <td>
                                "On exhibe un x₀ tel que P(x₀) est fausse. Un seul suffit."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Par récurrence"
                                </strong>
                            </td>
                            <td>
                                "Prouver ∀n, P(n)"
                            </td>
                            <td>
                                "Initialisation + hérédité. Toujours annoncer la proposition P(n) que l'on récurre."
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note method" style="margin-top:18px">
                <span class="title">
                    "Ce qui est vraiment évalué"
                </span>
                <p>
                    "L'objectif affiché du cours\u{a0}: rédiger des preuves "
                    <em>
                        "lisibles et convaincantes"
                    </em>
                    ". Une solution comporte toujours au moins un sujet et un verbe\u{a0}; le seul résultat, même juste, ne vaut pas la solution de l'exercice."
                </p>
            </div>
        </section>
    }
}

fn sec_sf_ensembles() -> impl IntoView {
    view! {
        <section id="sf-ensembles">
            <div class="sec-head">
                <span class="num">
                    "02"
                </span>
                <h2>
                    "Ensembles"
                </h2>
            </div>
            <h3>
                "Les ensembles usuels"
            </h3>
            <span class="f">
                "ℕ = {0, 1, 2, …}                    ℕ* = ℕ \\ {0}\nℕ_n = {0, 1, …, n}                  ℕ*_n = {1, …, n}   (donc ℕ*₀ = ∅)\nℤ = ℕ ∪ (−ℕ)                        ℚ = { p/q | p ∈ ℤ, q ∈ ℕ* }\nℝ                                    ℝ \\ ℚ = les irrationnels\nℂ = { x + iy | x, y ∈ ℝ }            ℝ̄ = ℝ ∪ {−∞, +∞}"
            </span>
            <div class="body" style="margin-top:14px">
                <p>
                    "Attention aux objets un peu piégeux mais parfaitement corrects\u{a0}: "
                    <span class="fi">
                        "{∅} ≠ ∅"
                    </span>
                    " (l'un a un élément, l'autre aucun), "
                    <span class="fi">
                        "{∅, {∅}}"
                    </span>
                    ", et "
                    <span class="fi">
                        "{1, 2, 3} = {3, 1, 2} = {1, 3, 1, 2}"
                    </span>
                    ", de cardinal 3."
                </p>
            </div>
            <h3>
                "Appartenance, inclusion, égalité"
            </h3>
            <dl class="deflist">
                <div class="def">
                    <dt>
                        "Appartenance "
                        <span style="font-weight:400;font-size:14px;color:var(--ink-soft)">
                            "x ∈ E"
                        </span>
                    </dt>
                    <dd>
                        "Relation entre un "
                        <em>
                            "élément"
                        </em>
                        " et un ensemble."
                    </dd>
                </div>
                <div class="def">
                    <dt>
                        "Inclusion "
                        <span style="font-weight:400;font-size:14px;color:var(--ink-soft)">
                            "A ⊂ B"
                        </span>
                    </dt>
                    <dd>
                        "Relation entre deux "
                        <em>
                            "ensembles"
                        </em>
                        "\u{a0}: "
                        <span class="fi">
                            "∀x ∈ A, x ∈ B"
                        </span>
                        ". C'est une relation d'ordre (réflexive, transitive, antisymétrique) mais "
                        <strong>
                            "non totale"
                        </strong>
                        "\u{a0}: {0,1} et {1,2} ne sont pas comparables."
                    </dd>
                </div>
                <div class="def">
                    <dt>
                        "Égalité "
                        <span style="font-weight:400;font-size:14px;color:var(--ink-soft)">
                            "A = B"
                        </span>
                    </dt>
                    <dd>
                        <span class="fi">
                            "(A ⊂ B) et (B ⊂ A)"
                        </span>
                        ". C'est la formulation à utiliser en pratique pour prouver une égalité d'ensembles\u{a0}: on montre les deux inclusions."
                    </dd>
                </div>
            </dl>
            <div class="note warn" style="margin-top:18px">
                <span class="title">
                    "∈ et ⊂ ne se remplacent pas"
                </span>
                <p>
                    "On a "
                    <span class="fi">
                        "1 ∈ [0,2]"
                    </span>
                    " car 1 est un élément du segment, mais "
                    <span class="fi">
                        "{1} ⊂ [0,2]"
                    </span>
                    " car {1} est un ensemble. Une question de TD entière porte là-dessus."
                </p>
            </div>
            <h3>
                "Les opérations"
            </h3>
            <span class="f">
                "A ∩ B = { x | x ∈ A et x ∈ B }        ⋂ᵢ Aᵢ = { x | ∀i ∈ I, x ∈ Aᵢ }\nA ∪ B = { x | x ∈ A ou x ∈ B }        ⋃ᵢ Aᵢ = { x | ∃i ∈ I, x ∈ Aᵢ }\nA \\ B = { x ∈ A | x ∉ B }             (B n'a pas à être inclus dans A)\nC_E(A) = E \\ A = Aᶜ                   complémentaire de A dans E\nP(E)  = { A | A ⊂ E }                 famille des parties de E\nA × B = { (a,b) | a ∈ A, b ∈ B }      produit cartésien"
            </span>
            <div class="body" style="margin-top:14px">
                <p>
                    "Exemple\u{a0}: pour E = {1,2,3}, "
                    <span class="fi">
                        "P(E) = {∅, {1}, {2}, {3}, {1,2}, {1,3}, {2,3}, {1,2,3}}"
                    </span>
                    " — huit parties, soit 2³."
                </p>
                <p>
                    "Deux couples sont égaux quand leurs coordonnées le sont\u{a0}: "
                    <span class="fi">
                        "(a,b) = (a′,b′) ⇔ (a = a′ et b = b′)"
                    </span>
                    "."
                </p>
            </div>
            <h3>
                "Les règles de calcul"
            </h3>
            <span class="f">
                "A ∪ (B ∪ C) = (A ∪ B) ∪ C            associativité\nA ∩ (B ∩ C) = (A ∩ B) ∩ C\nA ∪ B = B ∪ A,  A ∩ B = B ∩ A        commutativité\nA ∩ (B ∪ C) = (A ∩ B) ∪ (A ∩ C)      distributivité"
            </span>
            <div class="body" style="margin-top:14px">
                <p>
                    "La différence "
                    <span class="fi">
                        "A \\ B"
                    </span>
                    ", elle, se comporte beaucoup moins bien\u{a0}: pas d'associativité, pas de commutativité. Les identités qui la concernent se démontrent toujours par double inclusion, élément par élément."
                </p>
            </div>
            <div class="note method" style="margin-top:18px">
                <span class="title">
                    "Méthode — prouver une égalité d'ensembles"
                </span>
                <ul>
                    <li>
                        "Prendre "
                        <span class="fi">
                            "x ∈"
                        </span>
                        " membre de gauche, traduire l'appartenance en assertion logique."
                    </li>
                    <li>
                        "Manipuler l'assertion avec les règles logiques (De Morgan, distributivité)."
                    </li>
                    <li>
                        "Relire l'assertion obtenue comme une appartenance au membre de droite\u{a0}: cela donne ⊂."
                    </li>
                    <li>
                        "Refaire dans l'autre sens — ou justifier que toutes les étapes étaient des équivalences."
                    </li>
                </ul>
            </div>
        </section>
    }
}

fn sec_sf_applications() -> impl IntoView {
    view! {
        <section id="sf-applications">
            <>
                <div class="sec-head">
                    <span class="num">
                        "03"
                    </span>
                    <h2>
                        "Relations et applications"
                    </h2>
                </div>
                <h3>
                    "Relations binaires"
                </h3>
                <div class="body">
                    <p>
                        "Une "
                        <strong>
                            "relation binaire"
                        </strong>
                        " R de E vers F est définie par une partie "
                        <span class="fi">
                            "Gr(R) ⊂ E × F"
                        </span>
                        ", son "
                        <em>
                            "graphe"
                        </em>
                        "\u{a0}: on note "
                        <span class="fi">
                            "xRy"
                        </span>
                        " quand "
                        <span class="fi">
                            "(x,y) ∈ Gr(R)"
                        </span>
                        ". Si E = F, la relation est dite interne et peut être\u{a0}:"
                    </p>
                </div>
                <span class="f">
                    "réflexive       ∀x, xRx\nsymétrique      ∀x, y, (xRy ⇔ yRx)\nantisymétrique  ∀x, y, ((xRy et yRx) ⇒ x = y)\ntransitive      ∀x, y, z, ((xRy et yRz) ⇒ xRz)"
                </span>
                <div class="tw" style="margin-top:16px;max-width:68ch">
                    <table>
                        <thead>
                            <tr>
                                <th>
                                    "Type"
                                </th>
                                <th>
                                    "Propriétés requises"
                                </th>
                                <th>
                                    "Exemples"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    <strong>
                                        "Relation d'équivalence"
                                    </strong>
                                </td>
                                <td>
                                    "réflexive + symétrique + transitive"
                                </td>
                                <td>
                                    "l'équipotence, la congruence modulo n"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <strong>
                                        "Relation d'ordre"
                                    </strong>
                                </td>
                                <td>
                                    "réflexive + antisymétrique + transitive"
                                </td>
                                <td>
                                    "≤ sur ℝ, ⊂ sur P(E)"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <strong>
                                        "Ordre total"
                                    </strong>
                                </td>
                                <td>
                                    "ordre + "
                                    <span class="fi">
                                        "∀x,y, (x⪯y ou y⪯x)"
                                    </span>
                                </td>
                                <td>
                                    "≤ sur ℝ (mais pas ⊂)"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <strong>
                                        "Ordre strict"
                                    </strong>
                                </td>
                                <td>
                                    <span class="fi">
                                        "xRy ⇒ x ≠ y"
                                    </span>
                                </td>
                                <td>
                                    "<, >"
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <h3>
                    "Fonction et application"
                </h3>
                <div class="body">
                    <p>
                        "Une "
                        <strong>
                            "fonction"
                        </strong>
                        " "
                        <span class="fi">
                            "f : E → F"
                        </span>
                        " associe à tout x de E "
                        <em>
                            "au plus"
                        </em>
                        " un élément de F. L'ensemble des x qui ont une image est le "
                        <strong>
                            "domaine de définition"
                        </strong>
                        " "
                        <span class="fi">
                            "Dom(f)"
                        </span>
                        ". Une "
                        <strong>
                            "application"
                        </strong>
                        " est une fonction telle que "
                        <span class="fi">
                            "Dom(f) = E"
                        </span>
                        " — tout élément de départ a une image."
                    </p>
                    <p>
                        "Formellement, deux applications sont égales lorsqu'elles ont même ensemble de départ, même ensemble d'arrivée, et même valeur en chaque point\u{a0}: "
                        <span class="fi">
                            "x ↦ √x"
                        </span>
                        " de ℝ dans ℝ et de ℝ⁺ dans ℝ ne sont donc pas la même chose."
                    </p>
                </div>
                <h3>
                    "Injection, surjection, bijection"
                </h3>
                <div class="body">
                    <p>
                        "Tout se lit sur l'équation "
                        <span class="fi">
                            "f(x) = y"
                        </span>
                        ", d'inconnue x\u{a0}:"
                    </p>
                </div>
                <div class="tw" style="max-width:68ch">
                    <table>
                        <thead>
                            <tr>
                                <th>
                                    "f est…"
                                </th>
                                <th>
                                    "… si tout y de F admet"
                                </th>
                                <th>
                                    "Caractérisation utile"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    <strong>
                                        "injective"
                                    </strong>
                                </td>
                                <td>
                                    <em>
                                        "au plus"
                                    </em>
                                    " un antécédent"
                                </td>
                                <td class="mono">
                                    "f(x) = f(x′) ⇒ x = x′"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <strong>
                                        "surjective"
                                    </strong>
                                </td>
                                <td>
                                    <em>
                                        "au moins"
                                    </em>
                                    " un antécédent"
                                </td>
                                <td class="mono">
                                    "Im f = F"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    <strong>
                                        "bijective"
                                    </strong>
                                </td>
                                <td>
                                    <em>
                                        "un et un seul"
                                    </em>
                                    " antécédent"
                                </td>
                                <td class="mono">
                                    "injective et surjective"
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div class="body" style="margin-top:14px">
                    <p>
                        "Si f est bijective, l'unique antécédent de y se note "
                        <span class="fi">
                            "f⁻¹(y)"
                        </span>
                        " et définit la "
                        <strong>
                            "bijection réciproque"
                        </strong>
                        " "
                        <span class="fi">
                            "f⁻¹ : F → E"
                        </span>
                        ", avec "
                        <span class="fi">
                            "f⁻¹ ∘ f = Id_E"
                        </span>
                        " et "
                        <span class="fi">
                            "f ∘ f⁻¹ = Id_F"
                        </span>
                        "."
                    </p>
                </div>
                <h3>
                    "Composition"
                </h3>
                <span class="f">
                    "(g ∘ f)(x) = g(f(x))                 h ∘ (g ∘ f) = (h ∘ g) ∘ f\n\nf et g injectives  ⇒  g ∘ f injective\nf et g surjectives ⇒  g ∘ f surjective\nf et g bijectives  ⇒  g ∘ f bijective, et (g ∘ f)⁻¹ = f⁻¹ ∘ g⁻¹"
                </span>
                <div class="note" style="margin-top:16px">
                    <span class="title">
                        "Les deux réciproques partielles "
                        <span class="tag hot">
                            "annale"
                        </span>
                    </span>
                    <p>
                        "Si "
                        <span class="fi">
                            "g ∘ f"
                        </span>
                        " est "
                        <strong>
                            "injective"
                        </strong>
                        ", alors "
                        <strong>
                            "f"
                        </strong>
                        " est injective. Si "
                        <span class="fi">
                            "g ∘ f"
                        </span>
                        " est "
                        <strong>
                            "surjective"
                        </strong>
                        ", alors "
                        <strong>
                            "g"
                        </strong>
                        " est surjective. Retenir le sens\u{a0}: l'information remonte vers la "
                        <em>
                            "première"
                        </em>
                        " appliquée pour l'injectivité, vers la "
                        <em>
                            "dernière"
                        </em>
                        " pour la surjectivité."
                    </p>
                    <p>
                        "Et le critère pratique\u{a0}: si "
                        <span class="fi">
                            "g ∘ f = Id_E"
                        </span>
                        " et "
                        <span class="fi">
                            "f ∘ g = Id_F"
                        </span>
                        ", alors f est bijective, de réciproque g."
                    </p>
                </div>
                <h3>
                    "Image directe, image réciproque"
                </h3>
                <span class="f">
                    "f(A)    = { f(a) | a ∈ A }       = { y ∈ F | ∃x ∈ A, f(x) = y }\nf⁻¹(B)  = { x ∈ E | f(x) ∈ B }\nIm f    = f(E)                   f surjective ⇔ Im f = F"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        "Les quatre relations à connaître, avec leurs cas d'égalité — c'est le cœur des exercices d'examen\u{a0}:"
                    </p>
                </div>
                <div class="tw" style="max-width:68ch;margin-top:10px">
                    <table>
                        <thead>
                            <tr>
                                <th>
                                    "Relation"
                                </th>
                                <th>
                                    "Toujours vraie"
                                </th>
                                <th>
                                    "Égalité si et seulement si"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="mono">
                                    "f(A₁ ∪ A₂)"
                                </td>
                                <td class="mono">
                                    "= f(A₁) ∪ f(A₂)"
                                </td>
                                <td>
                                    "toujours"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "f(A₁ ∩ A₂)"
                                </td>
                                <td class="mono">
                                    "⊂ f(A₁) ∩ f(A₂)"
                                </td>
                                <td>
                                    "f injective"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "A ⊂ f⁻¹(f(A))"
                                </td>
                                <td class="mono">
                                    "⊂"
                                </td>
                                <td>
                                    "f injective"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "f(f⁻¹(B)) ⊂ B"
                                </td>
                                <td class="mono">
                                    "⊂"
                                </td>
                                <td>
                                    "f surjective"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "f⁻¹(B₁ ∪ B₂)"
                                </td>
                                <td class="mono">
                                    "= f⁻¹(B₁) ∪ f⁻¹(B₂)"
                                </td>
                                <td>
                                    "toujours"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "f⁻¹(C_F B)"
                                </td>
                                <td class="mono">
                                    "= C_E f⁻¹(B)"
                                </td>
                                <td>
                                    "toujours"
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div class="body" style="margin-top:12px">
                    <p>
                        "À retenir\u{a0}: l'image "
                        <em>
                            "réciproque"
                        </em>
                        " se comporte bien avec toutes les opérations ensemblistes\u{a0}; l'image "
                        <em>
                            "directe"
                        </em>
                        " ne respecte que la réunion."
                    </p>
                </div>
                <div class="note method" style="margin-top:18px">
                    <span class="title">
                        "Méthode — montrer qu'une application est bijective"
                    </span>
                    <ul>
                        <li>
                            <strong>
                                "Voie 1."
                            </strong>
                            " Injectivité\u{a0}: on part de "
                            <span class="fi">
                                "f(x) = f(x′)"
                            </span>
                            " et on aboutit à "
                            <span class="fi">
                                "x = x′"
                            </span>
                            ". Surjectivité\u{a0}: on se donne y et on "
                            <em>
                                "exhibe"
                            </em>
                            " un antécédent (le brouillon sert à résoudre "
                            <span class="fi">
                                "f(x) = y"
                            </span>
                            ", la copie ne montre que la vérification)."
                        </li>
                        <li>
                            <strong>
                                "Voie 2."
                            </strong>
                            " On devine la réciproque g et on vérifie "
                            <span class="fi">
                                "g ∘ f = Id"
                            </span>
                            " et "
                            <span class="fi">
                                "f ∘ g = Id"
                            </span>
                            ". Plus rapide quand on sait résoudre le système."
                        </li>
                        <li>
                            <strong>
                                "Cas fini."
                            </strong>
                            " Si "
                            <span class="fi">
                                "card E = card F"
                            </span>
                            " et E fini, alors injective ⇔ surjective ⇔ bijective\u{a0}: une seule des deux suffit."
                        </li>
                    </ul>
                </div>
            </>
            <>
                <div class="ex" style="margin-top:16px">
                    <div class="ex-head">
                        <span>
                            "Exemple type "
                            <span class="tag hot" style="vertical-align:0">
                                "annale"
                            </span>
                        </span>
                        <span class="mono">
                            "f(x,y) = (x + 3y, x + y)"
                        </span>
                    </div>
                    <div class="ex-body">
                        <p>
                            <strong>
                                "Injectivité."
                            </strong>
                            " Si "
                            <span class="fi">
                                "f(x,y) = f(x′,y′)"
                            </span>
                            " alors x + 3y = x′ + 3y′ et x + y = x′ + y′. En soustrayant, 2y = 2y′ donc y = y′, puis x = x′."
                        </p>
                        <p>
                            <strong>
                                "Surjectivité."
                            </strong>
                            " Soit (a,b) ∈ ℝ². Le système x + 3y = a, x + y = b donne y = (a − b)/2 et x = (3b − a)/2. On vérifie que "
                            <span class="fi">
                                "f((3b−a)/2, (a−b)/2) = (a,b)"
                            </span>
                            "."
                        </p>
                        <p>
                            <strong>
                                "Réciproque."
                            </strong>
                            " "
                            <span class="res">
                                "f⁻¹(a,b) = ((3b − a)/2, (a − b)/2)"
                            </span>
                            "."
                        </p>
                    </div>
                </div>
            </>
        </section>
    }
}

fn sec_sf_denombrement() -> impl IntoView {
    view! {
        <section id="sf-denombrement">
            <div class="sec-head">
                <span class="num">
                    "04"
                </span>
                <h2>
                    "Cardinaux et dénombrement"
                </h2>
            </div>
            <dl class="deflist">
                <div class="def">
                    <dt>
                        "Équipotence"
                    </dt>
                    <dd>
                        "E et F sont équipotents ("
                        <span class="fi">
                            "E ∼ F"
                        </span>
                        ") s'il existe une bijection de E sur F. C'est une relation d'équivalence entre ensembles."
                    </dd>
                </div>
                <div class="def">
                    <dt>
                        "Ensemble fini, cardinal"
                    </dt>
                    <dd>
                        "E est fini s'il est vide ou équipotent à "
                        <span class="fi">
                            "{1, …, n}"
                        </span>
                        "\u{a0}; cet entier n, unique, est le "
                        <strong>
                            "cardinal"
                        </strong>
                        " "
                        <span class="fi">
                            "card E"
                        </span>
                        ". Sinon E est infini."
                    </dd>
                </div>
                <div class="def">
                    <dt>
                        "Dénombrable"
                    </dt>
                    <dd>
                        "Fini, ou équipotent à ℕ. Tout sous-ensemble de ℕ est dénombrable\u{a0}; ℤ, ℕ² et ℚ le sont aussi."
                    </dd>
                </div>
            </dl>
            <h3>
                "Ce que le cardinal impose"
            </h3>
            <span class="f">
                "E fini, f : E → F\n  f injective   ⇒  card E ≤ card F\n  f surjective  ⇒  card E ≥ card F\n  si card E = card F :  bijective ⇔ injective ⇔ surjective\n\nE ⊊ F  ⇒  card E < card F        (E fini)"
            </span>
            <h3>
                "Cantor"
            </h3>
            <div class="body">
                <p>
                    <strong>
                        "Théorème (Cantor)."
                    </strong>
                    " Pour tout ensemble E, il n'existe "
                    <em>
                        "pas"
                    </em>
                    " de surjection "
                    <span class="fi">
                        "E → P(E)"
                    </span>
                    ". La preuve tient en une ligne\u{a0}: si f en était une, on pose "
                    <span class="fi">
                        "P = { x ∈ E | x ∉ f(x) }"
                    </span>
                    ", et l'égalité "
                    <span class="fi">
                        "P = f(a)"
                    </span>
                    " donne l'absurdité "
                    <span class="fi">
                        "a ∈ P ⇔ a ∉ P"
                    </span>
                    "."
                </p>
                <p>
                    <strong>
                        "Corollaire."
                    </strong>
                    " ℝ n'est pas dénombrable (argument de la diagonale). Donc "
                    <span class="fi">
                        "card ℕ < card ℝ"
                    </span>
                    ", et "
                    <span class="fi">
                        "card E < card P(E)"
                    </span>
                    " pour tout E\u{a0}: il n'existe pas de plus grand cardinal."
                </p>
            </div>
            <h3>
                "Les formules de comptage"
            </h3>
            <div class="tw" style="max-width:68ch">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Objet compté"
                            </th>
                            <th>
                                "Formule"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                "Réunion disjointe"
                            </td>
                            <td class="mono">
                                "card(A₁ ∪ … ∪ Aₙ) = card A₁ + … + card Aₙ"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Réunion quelconque"
                            </td>
                            <td class="mono">
                                "card(E ∪ F) + card(E ∩ F) = card E + card F"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Produit cartésien"
                            </td>
                            <td class="mono">
                                "card(A₁ × … × Aₙ) = card A₁ × … × card Aₙ"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Applications de E dans F"
                            </td>
                            <td class="mono">
                                "card F(E,F) = m"
                                <sup>
                                    "n"
                                </sup>
                                " (n = card E, m = card F)"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "p-uplets d'éléments de E"
                            </td>
                            <td class="mono">
                                "n"
                                <sup>
                                    "p"
                                </sup>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Parties de E"
                            </td>
                            <td class="mono">
                                "card P(E) = 2"
                                <sup>
                                    "n"
                                </sup>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Arrangements (p-uplets distincts)"
                            </td>
                            <td class="mono">
                                "A"
                                <sup>
                                    "p"
                                </sup>
                                <sub>
                                    "n"
                                </sub>
                                " = n! / (n − p)!"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Permutations de E"
                            </td>
                            <td class="mono">
                                "n! = A"
                                <sup>
                                    "n"
                                </sup>
                                <sub>
                                    "n"
                                </sub>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Combinaisons (parties à p éléments)"
                            </td>
                            <td class="mono">
                                "C(n,p) = n! / (p!(n − p)!)"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="body" style="margin-top:12px">
                <p>
                    "Avec la convention "
                    <span class="fi">
                        "0! = 1! = 1"
                    </span>
                    "."
                </p>
            </div>
            <h3>
                "Les identités binomiales du TD"
            </h3>
            <span class="f">
                "C(n,k) = C(n, n−k)                          symétrie\nC(n,k) = C(n−1, k−1) + C(n−1, k)            formule de Pascal\n(a + b)ⁿ = Σ_{k=0..n} C(n,k) aᵏ bⁿ⁻ᵏ         binôme de Newton\nΣ_{k=0..r} C(m,k)·C(n, r−k) = C(m+n, r)     formule de Vandermonde"
            </span>
        </section>
    }
}

fn sec_sf_nombres() -> impl IntoView {
    view! {
        <section id="sf-nombres">
            <div class="sec-head">
                <span class="num">
                    "05"
                </span>
                <h2>
                    "Les ensembles de nombres"
                </h2>
                <span class="pill">
                    "culture — non exigible"
                </span>
            </div>
            <div class="body">
                <p>
                    "Le cours précise que ces constructions ne sont pas exigibles\u{a0}: elles sont là pour montrer qu'un même procédé, le "
                    <strong>
                        "passage au quotient"
                    </strong>
                    " par une relation d'équivalence, fabrique chaque nouvel ensemble de nombres à partir du précédent."
                </p>
            </div>
            <div class="tw" style="margin-top:16px">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Ensemble"
                            </th>
                            <th>
                                "Construit à partir de"
                            </th>
                            <th>
                                "Relation d'équivalence"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">
                                "ℕ"
                            </td>
                            <td>
                                "l'ensemble vide et l'axiome de l'infini"
                            </td>
                            <td>
                                "0 = ∅, 1 = {0}, 2 = {0,1}, …"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "ℤ"
                            </td>
                            <td class="mono">
                                "ℕ × ℕ"
                            </td>
                            <td class="mono">
                                "(n,m) R (n′,m′) ⇔ n + m′ = n′ + m"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "ℚ"
                            </td>
                            <td class="mono">
                                "ℤ × ℕ*"
                            </td>
                            <td class="mono">
                                "(a,b) R (c,d) ⇔ ad = cb"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "ℝ"
                            </td>
                            <td>
                                "les suites de Cauchy de ℚ"
                            </td>
                            <td class="mono">
                                "(uₙ) R (vₙ) ⇔ lim(uₙ − vₙ) = 0"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="body" style="margin-top:14px">
                <p>
                    "L'idée à chaque étape\u{a0}: le couple "
                    <span class="fi">
                        "(n,m)"
                    </span>
                    " «\u{a0}représente\u{a0}» "
                    <span class="fi">
                        "n − m"
                    </span>
                    ", mais plusieurs couples représentent le même entier — on quotiente donc pour n'en garder qu'un représentant."
                </p>
            </div>
            <h3>
                "Ce qui est exigible"
            </h3>
            <div class="tw">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Propriété"
                            </th>
                            <th>
                                "Énoncé"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <strong>
                                    "Bon ordre de ℕ"
                                </strong>
                            </td>
                            <td>
                                "Toute partie non vide de ℕ admet un plus petit élément — c'est le fondement de la récurrence."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Archimède"
                                </strong>
                            </td>
                            <td class="mono">
                                "∀ε > 0, ∀y ∈ ℝ, ∃n ∈ ℕ, y ≤ nε"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Partie entière"
                                </strong>
                            </td>
                            <td class="mono">
                                "E(x) = ⌊x⌋ = l'unique n ∈ ℤ avec n ≤ x < n + 1. Ex : E(π) = 3, E(−π) = −4."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Densité"
                                </strong>
                            </td>
                            <td>
                                "Tout intervalle ouvert non vide de ℝ contient un rationnel "
                                <em>
                                    "et"
                                </em>
                                " un irrationnel. Preuve avec "
                                <span class="fi">
                                    "rₙ = E(nx)/n"
                                </span>
                                ", qui vérifie "
                                <span class="fi">
                                    "0 ≤ x − rₙ < 1/n"
                                </span>
                                "."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Irrationalité de √2"
                                </strong>
                            </td>
                            <td>
                                "Par l'absurde, ou par les valuations\u{a0}: "
                                <span class="fi">
                                    "√a ∈ ℚ ⇔ ∀p premier, val_p(a) est pair"
                                </span>
                                "."
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </section>
    }
}

fn sec_sf_complexes() -> impl IntoView {
    view! {
        <section id="sf-complexes">
            <>
                <div class="sec-head">
                    <span class="num">
                        "06"
                    </span>
                    <h2>
                        "Nombres complexes"
                    </h2>
                </div>
                <h3>
                    "Définition et forme algébrique"
                </h3>
                <div class="body">
                    <p>
                        "ℂ est l'ensemble des couples "
                        <span class="fi">
                            "(x,y)"
                        </span>
                        " de réels munis de\u{a0}:"
                    </p>
                </div>
                <span class="f">
                    "(x, y) + (x′, y′) = (x + x′, y + y′)\n(x, y) · (x′, y′) = (xx′ − yy′, xy′ + yx′)"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        "En posant "
                        <span class="fi">
                            "i = (0,1)"
                        </span>
                        " et "
                        <span class="fi">
                            "1 = (1,0)"
                        </span>
                        ", on obtient "
                        <span class="fi">
                            "i² = (−1,0) = −1"
                        </span>
                        " et l'écriture "
                        <strong>
                            "algébrique"
                        </strong>
                        " "
                        <span class="fi">
                            "z = x + iy"
                        </span>
                        ", avec "
                        <span class="fi">
                            "x = Re(z)"
                        </span>
                        " et "
                        <span class="fi">
                            "y = Im(z)"
                        </span>
                        ". Muni de ces deux lois, ℂ est un "
                        <strong>
                            "corps"
                        </strong>
                        "."
                    </p>
                </div>
                <h3>
                    "Conjugué et module"
                </h3>
                <span class="f">
                    "z̄ = x − iy                         |z| = √(x² + y²) = √(z z̄)\n\nz̄̄ = z                              Re(z) = (z + z̄)/2      Im(z) = (z − z̄)/2i\nconj(z + z′) = z̄ + z̄′               conj(z z′) = z̄ · z̄′     conj(zⁿ) = (z̄)ⁿ\nz z̄ = x² + y² ∈ ℝ⁺                   z⁻¹ = z̄ / |z|²"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        "Propriétés du module\u{a0}: "
                        <span class="fi">
                            "|z̄| = |z|"
                        </span>
                        "\u{a0}; "
                        <span class="fi">
                            "|z| = 0 ⇔ z = 0"
                        </span>
                        "\u{a0}; "
                        <span class="fi">
                            "|zz′| = |z||z′|"
                        </span>
                        "\u{a0}; "
                        <span class="fi">
                            "|z/z′| = |z|/|z′|"
                        </span>
                        "\u{a0}; "
                        <span class="fi">
                            "|zⁿ| = |z|ⁿ"
                        </span>
                        "\u{a0}; "
                        <span class="fi">
                            "|Re z| ≤ |z|"
                        </span>
                        " et "
                        <span class="fi">
                            "|Im z| ≤ |z|"
                        </span>
                        "\u{a0}; et l'"
                        <strong>
                            "inégalité triangulaire"
                        </strong>
                        " "
                        <span class="fi">
                            "||z| − |z′|| ≤ |z − z′| ≤ |z| + |z′|"
                        </span>
                        "."
                    </p>
                    <p>
                        "Géométriquement, "
                        <span class="fi">
                            "|z|"
                        </span>
                        " est la distance de l'origine au point d'affixe z."
                    </p>
                </div>
                <div class="note method" style="margin-top:16px">
                    <span class="title">
                        "Méthode — mettre un quotient sous forme algébrique"
                    </span>
                    <p>
                        "On multiplie haut et bas par le conjugué du dénominateur\u{a0}: "
                        <span class="fi">
                            "(2+i)/(1−i) = (2+i)(1+i)/((1−i)(1+i)) = (1 + 3i)/2"
                        </span>
                        "."
                    </p>
                </div>
                <h3>
                    "Forme trigonométrique et exponentielle"
                </h3>
                <span class="f">
                    "z = |z|(cos θ + i sin θ) = r e^{iθ}      r = |z| unique, θ = arg(z) à 2π près\n\ne^{iθ} = cos θ + i sin θ                e^{a+ib} = eᵃ(cos b + i sin b)\ne^{z+z′} = eᶻ · e^{z′}                   conj(e^{iθ}) = e^{−iθ}\n\ncos θ = (e^{iθ} + e^{−iθ})/2            formules d'Euler\nsin θ = (e^{iθ} − e^{−iθ})/2i\n\narg(z z′) = arg z + arg z′              arg(z/z′) = arg z − arg z′   [2π]"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        <strong>
                            "Formule de De Moivre"
                        </strong>
                        "\u{a0}: "
                        <span class="fi">
                            "(cos θ + i sin θ)ⁿ = cos(nθ) + i sin(nθ)"
                        </span>
                        ", conséquence immédiate de "
                        <span class="fi">
                            "(e^{iθ})ⁿ = e^{inθ}"
                        </span>
                        "."
                    </p>
                </div>
                <div class="tw" style="max-width:60ch;margin-top:16px">
                    <table class="truth compact">
                        <thead>
                            <tr>
                                <th>
                                    "Radians"
                                </th>
                                <th>
                                    "0"
                                </th>
                                <th>
                                    "π/6"
                                </th>
                                <th>
                                    "π/4"
                                </th>
                                <th>
                                    "π/3"
                                </th>
                                <th>
                                    "π/2"
                                </th>
                                <th>
                                    "π"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>
                                    "sin"
                                </td>
                                <td>
                                    "0"
                                </td>
                                <td>
                                    "1/2"
                                </td>
                                <td>
                                    "√2/2"
                                </td>
                                <td>
                                    "√3/2"
                                </td>
                                <td>
                                    "1"
                                </td>
                                <td>
                                    "0"
                                </td>
                            </tr>
                            <tr>
                                <td>
                                    "cos"
                                </td>
                                <td>
                                    "1"
                                </td>
                                <td>
                                    "√3/2"
                                </td>
                                <td>
                                    "√2/2"
                                </td>
                                <td>
                                    "1/2"
                                </td>
                                <td>
                                    "0"
                                </td>
                                <td>
                                    "−1"
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <span class="f" style="margin-top:16px">
                    "cos(a ± b) = cos a cos b ∓ sin a sin b\nsin(a ± b) = sin a cos b ± cos a sin b\ncos a + cos b = 2 cos((a+b)/2) cos((a−b)/2)\ncos a − cos b = −2 sin((a+b)/2) sin((a−b)/2)\nsin a + sin b = 2 sin((a+b)/2) cos((a−b)/2)"
                </span>
                <h3>
                    "Linéarisation"
                </h3>
                <div class="note method">
                    <span class="title">
                        "Méthode — linéariser cos(x)^p · sin(x)^q"
                    </span>
                    <ul>
                        <li>
                            "Écrire "
                            <span class="fi">
                                "cos x = (e^{ix} + e^{−ix})/2"
                            </span>
                            " et "
                            <span class="fi">
                                "sin x = (e^{ix} − e^{−ix})/2i"
                            </span>
                            ", puis élever aux puissances."
                        </li>
                        <li>
                            "Développer chaque terme (binôme de Newton)."
                        </li>
                        <li>
                            "Simplifier avec "
                            <span class="fi">
                                "e^{ikx} e^{iℓx} = e^{i(k+ℓ)x}"
                            </span>
                            "."
                        </li>
                        <li>
                            "Regrouper avec "
                            <span class="fi">
                                "e^{inx} + e^{−inx} = 2cos(nx)"
                            </span>
                            " et "
                            <span class="fi">
                                "e^{inx} − e^{−inx} = 2i sin(nx)"
                            </span>
                            "."
                        </li>
                    </ul>
                </div>
                <span class="f" style="margin-top:14px">
                    "cos²x = (1/4)(e^{2ix} + 2 + e^{−2ix}) = cos(2x)/2 + 1/2\nsin³x = −sin(3x)/4 + 3sin(x)/4"
                </span>
                <h3>
                    "Équations polynomiales"
                </h3>
                <div class="body">
                    <p>
                        <strong>
                            "Théorème de d'Alembert-Gauss."
                        </strong>
                        " Tout polynôme non constant à coefficients complexes admet au moins une racine dans ℂ. "
                        <strong>
                            "Corollaire\u{a0}:"
                        </strong>
                        " un polynôme de degré n a exactement n racines dans ℂ, comptées avec leurs multiplicités."
                    </p>
                    <p>
                        "Galois a montré qu'à partir du degré 5, il n'existe en général pas de formule exprimant les racines par radicaux\u{a0}: l'existence n'est pas la calculabilité."
                    </p>
                </div>
                <div class="note" style="margin-top:16px">
                    <span class="title">
                        "Le second degré dans ℂ"
                    </span>
                    <p>
                        "Pour "
                        <span class="fi">
                            "az² + bz + c = 0"
                        </span>
                        " avec a ≠ 0, on pose "
                        <span class="fi">
                            "Δ = b² − 4ac"
                        </span>
                        ", qui est maintenant un "
                        <em>
                            "complexe"
                        </em>
                        " quelconque. La racine carrée usuelle n'existant que sur ℝ⁺, on cherche "
                        <strong>
                            "δ tel que δ² = Δ"
                        </strong>
                        "\u{a0}: si "
                        <span class="fi">
                            "Δ = r e^{iθ}"
                        </span>
                        ", alors "
                        <span class="fi">
                            "δ = √r · e^{iθ/2}"
                        </span>
                        " convient."
                    </p>
                    <span class="mono" style="display:block;margin-top:4px">
                        "z = (−b − δ)/2a et z = (−b + δ)/2a"
                    </span>
                    <p>
                        "Si Δ = 0, racine double "
                        <span class="fi">
                            "−b/2a"
                        </span>
                        ". Le cas Δ réel positif redonne la formule du lycée."
                    </p>
                </div>
            </>
            <>
                <h3>
                    "Racines n-ièmes de l'unité"
                </h3>
                <div class="body">
                    <p>
                        "Ce sont les solutions de "
                        <span class="fi">
                            "zⁿ = 1"
                        </span>
                        ". En écrivant "
                        <span class="fi">
                            "z = re^{iθ}"
                        </span>
                        ", on obtient "
                        <span class="fi">
                            "rⁿ = 1"
                        </span>
                        " donc r = 1, et "
                        <span class="fi">
                            "nθ = 2kπ"
                        </span>
                        "\u{a0}:"
                    </p>
                </div>
                <span class="f center">
                    "ω_k = e^{2ikπ/n} = cos(2kπ/n) + i sin(2kπ/n),   k = 0, 1, …, n − 1"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        "Elles sont toutes de module 1\u{a0}: ce sont les "
                        <strong>
                            "sommets d'un polygone régulier à n côtés"
                        </strong>
                        " inscrit dans le cercle unité, deux racines consécutives faisant un angle "
                        <span class="fi">
                            "2π/n"
                        </span>
                        "."
                    </p>
                </div>
                <span class="f">
                    "1 + ω + ω² + … + ω^{n−1} = 0            (somme géométrique, ω ≠ 1)\nXⁿ − 1 = Π_{k=0..n−1} (X − ω^k)\nΠ_{k=0..n−1} ω^k = (−1)^{n+1}\n\nn = 3 :  1,  −1/2 + i√3/2,  −1/2 − i√3/2       (triangle équilatéral)\nn = 4 :  1,  i,  −1,  −i"
                </span>
            </>
        </section>
    }
}

fn sec_sf_algebre() -> impl IntoView {
    view! {
        <section id="sf-algebre">
            <>
                <div class="sec-head">
                    <span class="num">
                        "07"
                    </span>
                    <h2>
                        "Structures algébriques"
                    </h2>
                </div>
                <h3>
                    "Lois de composition interne"
                </h3>
                <div class="body">
                    <p>
                        "Une "
                        <strong>
                            "opération"
                        </strong>
                        " sur E est une application "
                        <span class="fi">
                            "E × E → F"
                        </span>
                        "\u{a0}; quand F = E on parle de "
                        <strong>
                            "loi de composition interne"
                        </strong>
                        ". Elle peut être\u{a0}:"
                    </p>
                </div>
                <span class="f">
                    "commutative     ∀x, y,     x ⋆ y = y ⋆ x\nassociative     ∀x, y, z,  x ⋆ (y ⋆ z) = (x ⋆ y) ⋆ z\ndistributive    x ⋆ (y △ z) = (x ⋆ y) △ (x ⋆ z)     (par rapport à une autre loi △)\n\nélément neutre  e tel que  ∀x, e ⋆ x = x ⋆ e = x\nsymétrique de x  y tel que  x ⋆ y = y ⋆ x = e\nA ⊂ E stable    ∀x, y ∈ A, x ⋆ y ∈ A"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        "Deux résultats d'unicité qui servent tout le temps\u{a0}: une loi admet "
                        <strong>
                            "au plus un"
                        </strong>
                        " élément neutre\u{a0}; si elle est de plus associative, chaque élément admet "
                        <strong>
                            "au plus un"
                        </strong>
                        " symétrique, noté "
                        <span class="fi">
                            "x⁻¹"
                        </span>
                        " (ou "
                        <span class="fi">
                            "−x"
                        </span>
                        " en notation additive). On a alors "
                        <span class="fi">
                            "(x⁻¹)⁻¹ = x"
                        </span>
                        "."
                    </p>
                </div>
                <h3>
                    "Groupes"
                </h3>
                <dl class="deflist">
                    <div class="def">
                        <dt>
                            "Groupe"
                        </dt>
                        <dd>
                            "Un couple "
                            <span class="fi">
                                "(G, ⋆)"
                            </span>
                            " où ⋆ est une loi interne "
                            <strong>
                                "associative"
                            </strong>
                            ", admettant un "
                            <strong>
                                "élément neutre"
                            </strong>
                            ", et où "
                            <strong>
                                "tout élément a un symétrique"
                            </strong>
                            ". Il est "
                            <em>
                                "commutatif"
                            </em>
                            " (ou abélien) si ⋆ l'est."
                        </dd>
                    </div>
                    <div class="def">
                        <dt>
                            "Sous-groupe"
                        </dt>
                        <dd>
                            "Une partie A de G "
                            <strong>
                                "stable"
                            </strong>
                            " par ⋆, contenant le neutre et le symétrique de chacun de ses éléments. C'est alors un groupe pour la loi induite — c'est le critère à appliquer en exercice, plutôt que de revérifier les axiomes."
                        </dd>
                    </div>
                    <div class="def">
                        <dt>
                            "Ordre d'un élément"
                        </dt>
                        <dd>
                            "Le plus petit entier "
                            <span class="fi">
                                "p ≥ 1"
                            </span>
                            " tel que "
                            <span class="fi">
                                "xᵖ = e"
                            </span>
                            " s'il existe (0 sinon). L'ensemble des n vérifiant "
                            <span class="fi">
                                "xⁿ = e"
                            </span>
                            " est alors exactement "
                            <span class="fi">
                                "pℤ"
                            </span>
                            "."
                        </dd>
                    </div>
                </dl>
                <div class="body" style="margin-top:16px">
                    <p>
                        "Dans un groupe, on peut "
                        <strong>
                            "simplifier"
                        </strong>
                        "\u{a0}: "
                        <span class="fi">
                            "a⋆x = a⋆y ⇒ x = y"
                        </span>
                        ". Et attention à l'ordre dans l'inverse d'un produit\u{a0}: "
                        <span class="fi">
                            "(xy)⁻¹ = y⁻¹x⁻¹"
                        </span>
                        "."
                    </p>
                    <p>
                        <strong>
                            "Théorème de Lagrange."
                        </strong>
                        " Si G est un groupe fini et H un sous-groupe, alors "
                        <span class="fi">
                            "card H divise card G"
                        </span>
                        "."
                    </p>
                </div>
                <div class="tw" style="margin-top:16px;max-width:68ch">
                    <table>
                        <thead>
                            <tr>
                                <th>
                                    "Exemple"
                                </th>
                                <th>
                                    "Groupe\u{a0}?"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="mono">
                                    "(ℝ, +), (ℤ, +), (ℂ, +)"
                                </td>
                                <td>
                                    "oui, commutatifs"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "(ℝ, ·)"
                                </td>
                                <td>
                                    <span class="bad">
                                        "non"
                                    </span>
                                    " — 0 n'a pas d'inverse"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "(ℝ*, ·)"
                                </td>
                                <td>
                                    "oui"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "U = { z ∈ ℂ | |z| = 1 }"
                                </td>
                                <td>
                                    "oui, sous-groupe de (ℂ*, ·)"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "(Perm E, ∘)"
                                </td>
                                <td>
                                    "oui — neutre Id_E, symétrique σ⁻¹, non commutatif dès n ≥ 3"
                                </td>
                            </tr>
                            <tr>
                                <td class="mono">
                                    "(ℤ/nℤ, +)"
                                </td>
                                <td>
                                    "oui, commutatif, de cardinal n"
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <h3>
                    "Anneaux et corps"
                </h3>
                <div class="body">
                    <p>
                        "Un "
                        <strong>
                            "anneau"
                        </strong>
                        " "
                        <span class="fi">
                            "(A, +, ·)"
                        </span>
                        "\u{a0}: "
                        <span class="fi">
                            "(A,+)"
                        </span>
                        " est un groupe commutatif, la multiplication est associative, distributive sur +, et admet un neutre "
                        <span class="fi">
                            "1_A"
                        </span>
                        ". On note "
                        <span class="fi">
                            "Inv(A)"
                        </span>
                        " l'ensemble des inversibles\u{a0}; c'est un groupe."
                    </p>
                    <p>
                        "C'est un "
                        <strong>
                            "corps"
                        </strong>
                        " lorsque "
                        <span class="fi">
                            "A \\ {0} = Inv A"
                        </span>
                        "\u{a0}: tout élément non nul est inversible."
                    </p>
                </div>
                <span class="f">
                    "(ℤ, +, ·)   anneau commutatif, pas un corps :  Inv ℤ = {−1, 1}\n(ℚ, +, ·), (ℝ, +, ·), (ℂ, +, ·)   corps commutatifs\nMₙ(ℝ)       anneau non commutatif dès n ≥ 2\nℤ/pℤ        corps si et seulement si p est premier"
                </span>
                <h3>
                    "Morphismes"
                </h3>
                <div class="body">
                    <p>
                        <span class="fi">
                            "φ : (G,⋆) → (G′,·)"
                        </span>
                        " est un "
                        <strong>
                            "morphisme de groupes"
                        </strong>
                        " si "
                        <span class="fi">
                            "∀x,y, φ(x ⋆ y) = φ(x) · φ(y)"
                        </span>
                        ". Son "
                        <strong>
                            "noyau"
                        </strong>
                        " est "
                        <span class="fi">
                            "ker φ = φ⁻¹({e′})"
                        </span>
                        ". Un morphisme bijectif est un "
                        <strong>
                            "isomorphisme"
                        </strong>
                        "."
                    </p>
                    <p>
                        "Alors "
                        <span class="fi">
                            "φ(e) = e′"
                        </span>
                        ", "
                        <span class="fi">
                            "φ(g⁻¹) = φ(g)⁻¹"
                        </span>
                        ", "
                        <span class="fi">
                            "Im φ"
                        </span>
                        " est un sous-groupe de G′ et "
                        <span class="fi">
                            "ker φ"
                        </span>
                        " un sous-groupe de G. Exemples\u{a0}: "
                        <span class="fi">
                            "t ↦ eᵗ"
                        </span>
                        " de (ℝ,+) sur (ℝ⁺*,·)\u{a0}; "
                        <span class="fi">
                            "t ↦ e^{it}"
                        </span>
                        " de (ℝ,+) dans (ℂ*,·), d'image le cercle unité\u{a0}; "
                        <span class="fi">
                            "n ↦ gⁿ"
                        </span>
                        " de (ℤ,+) dans G, de noyau pℤ où p est l'ordre de g."
                    </p>
                </div>
                <h3>
                    "Divisibilité dans ℤ"
                </h3>
                <span class="f">
                    "a | b   ⇔  ∃c, ac = b                   a divise b\n\nDiv(a₁,…,aₙ) = diviseurs communs        Mult(a₁,…,aₙ) = multiples communs\nPGCD(a, b) = a ∧ b                      PPCM(a, b) = a ∨ b\na et b premiers entre eux  ⇔  a ∧ b = 1\nab = (a ∧ b)(a ∨ b)"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        <strong>
                            "Division euclidienne."
                        </strong>
                        " Pour "
                        <span class="fi">
                            "a ∈ ℤ"
                        </span>
                        " et "
                        <span class="fi">
                            "b ∈ ℕ*"
                        </span>
                        ", il existe un unique couple "
                        <span class="fi">
                            "(q, r)"
                        </span>
                        " avec "
                        <span class="fi">
                            "a = bq + r"
                        </span>
                        " et "
                        <span class="fi">
                            "0 ≤ r < b"
                        </span>
                        ". Le quotient est "
                        <span class="fi">
                            "⌊a/b⌋"
                        </span>
                        ", et "
                        <span class="fi">
                            "b | a ⇔ r = 0"
                        </span>
                        ". Attention aux négatifs\u{a0}: la division de −17 par 5 donne "
                        <span class="fi">
                            "(−4, 3)"
                        </span>
                        ", pas (−3, −2)."
                    </p>
                    <p>
                        <strong>
                            "Identité de Bezout."
                        </strong>
                        " "
                        <span class="fi">
                            "∃u, v ∈ ℤ, ua + vb = a ∧ b"
                        </span>
                        ". En particulier "
                        <span class="fi">
                            "a ∧ b = 1 ⇔ ∃u,v, ua + vb = 1"
                        </span>
                        " (théorème de Bezout). Les coefficients u, v ne sont pas uniques\u{a0}: on les obtient en remontant l'algorithme d'Euclide."
                    </p>
                    <p>
                        <strong>
                            "Lemme de Gauss."
                        </strong>
                        " Si a est premier avec chaque "
                        <span class="fi">
                            "bᵢ"
                        </span>
                        ", il est premier avec leur produit."
                    </p>
                    <p>
                        <strong>
                            "Factorialité."
                        </strong>
                        " Tout rationnel "
                        <span class="fi">
                            "a > 0"
                        </span>
                        " s'écrit de manière unique "
                        <span class="fi">
                            "a = p₁^{α₁} ⋯ pₙ^{αₙ}"
                        </span>
                        " avec les pᵢ premiers croissants et "
                        <span class="fi">
                            "αᵢ ∈ ℤ*"
                        </span>
                        "\u{a0}; "
                        <span class="fi">
                            "a"
                        </span>
                        " est entier si et seulement si tous les αᵢ sont positifs. L'exposant αᵢ est la "
                        <strong>
                            "valuation"
                        </strong>
                        " "
                        <span class="fi">
                            "val_{pᵢ}(a)"
                        </span>
                        ", et\u{a0}:"
                    </p>
                </div>
                <span class="f">
                    "val_p(ab) = val_p(a) + val_p(b)          val_p(a/b) = val_p(a) − val_p(b)\nval_p(a ∧ b) = min(val_p a, val_p b)     val_p(a ∨ b) = max(val_p a, val_p b)\na | b  ⇔  ∀p premier, val_p(a) ≤ val_p(b)\n√a ∈ ℚ ⇔ ∀p, val_p(a) ∈ 2ℤ               (d'où √2 ∉ ℚ)"
                </span>
                <div class="body" style="margin-top:12px">
                    <p>
                        "Et l'ensemble P des nombres premiers est "
                        <strong>
                            "infini"
                        </strong>
                        "\u{a0}: tout facteur premier de "
                        <span class="fi">
                            "p₁p₂⋯pₙ + 1"
                        </span>
                        " échappe à la liste des n premiers."
                    </p>
                </div>
                <h3>
                    "Congruences et ℤ/nℤ"
                </h3>
            </>
            <>
                <div class="body">
                    <p>
                        "Pour "
                        <span class="fi">
                            "a"
                        </span>
                        " fixé, "
                        <span class="fi">
                            "b ≡ c [a]"
                        </span>
                        " signifie "
                        <span class="fi">
                            "a | c − b"
                        </span>
                        "\u{a0}: c'est une relation d'équivalence, et le quotient "
                        <span class="fi">
                            "A/aA"
                        </span>
                        " est un anneau où\u{a0}:"
                    </p>
                </div>
                <span class="f">
                    "b̄ = c̄  ⇔  b ≡ c [a]           b̄ + c̄ = (b+c)‾           b̄ · c̄ = (bc)‾\n\ncard ℤ/nℤ = n                  ℤ/nℤ = { 0̄, 1̄, …, (n−1)‾ }\nā inversible dans ℤ/nℤ  ⇔  a ∧ n = 1\nℤ/nℤ est un corps       ⇔  n est premier"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        <strong>
                            "Théorème des restes chinois."
                        </strong>
                        " Si "
                        <span class="fi">
                            "a₁, …, a_N"
                        </span>
                        " sont deux à deux premiers entre eux et "
                        <span class="fi">
                            "a = a₁⋯a_N"
                        </span>
                        ", alors "
                        <span class="fi">
                            "n ↦ (n mod a₁, …, n mod a_N)"
                        </span>
                        " est une bijection de "
                        <span class="fi">
                            "{0, …, a−1}"
                        </span>
                        " sur "
                        <span class="fi">
                            "ℤ/a₁ℤ × … × ℤ/a_Nℤ"
                        </span>
                        " (et même un isomorphisme d'anneaux). Autrement dit un système de congruences à modules premiers entre eux a une solution, unique modulo le produit."
                    </p>
                </div>
                <div class="note method" style="margin-top:16px">
                    <span class="title">
                        "Méthode — reste de a^N modulo m "
                        <span class="tag hot" style="vertical-align:0">
                            "annale"
                        </span>
                    </span>
                    <ul>
                        <li>
                            "Réduire la base\u{a0}: "
                            <span class="fi">
                                "a ≡ a₀ [m]"
                            </span>
                            " avec a₀ petit, donc "
                            <span class="fi">
                                "a^N ≡ a₀^N"
                            </span>
                            "."
                        </li>
                        <li>
                            "Chercher le plus petit exposant k avec "
                            <span class="fi">
                                "a₀^k ≡ ±1 [m]"
                            </span>
                            " en calculant les puissances successives."
                        </li>
                        <li>
                            "Écrire la division euclidienne "
                            <span class="fi">
                                "N = kq + r"
                            </span>
                            " et conclure "
                            <span class="fi">
                                "a₀^N ≡ (a₀^k)^q · a₀^r"
                            </span>
                            "."
                        </li>
                    </ul>
                </div>
                <div class="ex" style="margin-top:14px">
                    <div class="ex-head">
                        <span>
                            "Examen 2024 — reste de 4007"
                            <sup>
                                "1235"
                            </sup>
                            " par 13"
                        </span>
                        <span class="mono">
                            "= 9"
                        </span>
                    </div>
                    <div class="ex-body">
                        <span class="mono">
                            "4007 ≡ 3 [13]           donc 4007^1235 ≡ 3^1235\n3³ = 27 = 2×13 + 1      donc 3³ ≡ 1 [13]\n1235 = 411 × 3 + 2\n3^1235 ≡ (3³)^411 · 3² ≡ 1 · 9 ≡ "
                            <span class="res">
                                "9"
                            </span>
                            " [13]"
                        </span>
                    </div>
                </div>
                <div class="ex" style="margin-top:14px">
                    <div class="ex-head">
                        <span>
                            "Rattrapage 2025 — reste de 3044"
                            <sup>
                                "3044"
                            </sup>
                            " par 13"
                        </span>
                        <span class="mono">
                            "= 9"
                        </span>
                    </div>
                    <div class="ex-body">
                        <span class="mono">
                            "3044 = 234 × 13 + 2      donc 3044 ≡ 2 [13]\n2⁴ ≡ 3,  2⁵ ≡ 6,  2⁶ ≡ 12 ≡ −1 [13]\n3044 = 6 × 507 + 2\n2^3044 ≡ (−1)^507 · 2² ≡ −4 ≡ "
                            <span class="res">
                                "9"
                            </span>
                            " [13]"
                        </span>
                    </div>
                </div>
                <h3>
                    "Permutations"
                </h3>
                <dl class="deflist">
                    <div class="def">
                        <dt>
                            "Groupe symétrique"
                        </dt>
                        <dd>
                            <span class="fi">
                                "Perm E"
                            </span>
                            " = les bijections de E dans E, muni de ∘. On note "
                            <span class="fi">
                                "Sₙ = Perm(ℕ*ₙ)"
                            </span>
                            ", de cardinal "
                            <span class="fi">
                                "n!"
                            </span>
                            "."
                        </dd>
                    </div>
                    <div class="def">
                        <dt>
                            "Support et points fixes"
                        </dt>
                        <dd>
                            <span class="fi">
                                "Supp σ = { x | σ(x) ≠ x }"
                            </span>
                            " et "
                            <span class="fi">
                                "Fix σ = E \\ Supp σ"
                            </span>
                            ". Deux permutations à supports disjoints "
                            <strong>
                                "commutent"
                            </strong>
                            "."
                        </dd>
                    </div>
                    <div class="def">
                        <dt>
                            "Cycle"
                        </dt>
                        <dd>
                            <span class="fi">
                                "(a₀, a₁, …, a_{P−1})"
                            </span>
                            " envoie chaque aₖ sur le suivant et le dernier sur a₀. Sa longueur P est aussi son ordre et le cardinal de son support. L'écriture n'est pas unique\u{a0}: on peut partir de n'importe quel élément du cycle."
                        </dd>
                    </div>
                    <div class="def">
                        <dt>
                            "Transposition"
                        </dt>
                        <dd>
                            "Un cycle de longueur 2. Elle est sa propre réciproque. Un cycle de longueur P se décompose en "
                            <span class="fi">
                                "P − 1"
                            </span>
                            " transpositions\u{a0}: "
                            <span class="fi">
                                "(a₀,…,a_{P−1}) = (a₀,a₁) ∘ … ∘ (a_{P−2},a_{P−1})"
                            </span>
                            "."
                        </dd>
                    </div>
                </dl>
                <div class="body" style="margin-top:18px">
                    <p>
                        <strong>
                            "Théorème de décomposition."
                        </strong>
                        " Toute permutation se décompose en un produit de cycles à supports deux à deux disjoints, "
                        <em>
                            "unique à l'ordre près"
                        </em>
                        ". Leurs supports sont les orbites non réduites à un point, où "
                        <span class="fi">
                            "Orb_σ(x) = { x, σ(x), …, σ^{P−1}(x) }"
                        </span>
                        "."
                    </p>
                </div>
                <span class="f">
                    "ordre(σ) = PPCM des longueurs des cycles de sa décomposition\nε(σ) = (−1)ⁿ  si σ est produit de n transpositions\n     = (−1)^{p₁−1} ⋯ (−1)^{p_q−1}  avec pᵢ les longueurs des cycles\nε(σ ∘ τ) = ε(σ) ε(τ)          ε(transposition) = −1        ε(Id) = 1"
                </span>
                <div class="body" style="margin-top:14px">
                    <p>
                        "σ est dite "
                        <strong>
                            "paire"
                        </strong>
                        " si ε(σ) = 1\u{a0}; les permutations paires forment le "
                        <strong>
                            "groupe alterné"
                        </strong>
                        ", sous-groupe de Perm E. Un carré "
                        <span class="fi">
                            "σ²"
                        </span>
                        " est toujours pair."
                    </p>
                </div>
                <div class="note method" style="margin-top:16px">
                    <span class="title">
                        "Méthode — la question type sur une permutation "
                        <span class="tag hot" style="vertical-align:0">
                            "annale"
                        </span>
                    </span>
                    <ul>
                        <li>
                            <strong>
                                "Décomposer\u{a0}:"
                            </strong>
                            " partir de 0, écrire 0 → σ(0) → σ²(0) … jusqu'au retour\u{a0}; fermer le cycle, reprendre avec le plus petit élément non encore traité."
                        </li>
                        <li>
                            <strong>
                                "Transpositions\u{a0}:"
                            </strong>
                            " chaque cycle de longueur P en donne P − 1, en chaîne."
                        </li>
                        <li>
                            <strong>
                                "Ordre\u{a0}:"
                            </strong>
                            " PPCM des longueurs. "
                            <strong>
                                "Signature\u{a0}:"
                            </strong>
                            " (−1) puissance le nombre total de transpositions."
                        </li>
                        <li>
                            <strong>
                                "σ⁻¹\u{a0}:"
                            </strong>
                            " on retourne chaque cycle."
                        </li>
                        <li>
                            <strong>
                                "σ^N\u{a0}:"
                            </strong>
                            " on réduit N modulo l'ordre, puis on calcule σ^r cycle par cycle."
                        </li>
                        <li>
                            <strong>
                                "Existe-t-il une permutation d'ordre k\u{a0}?"
                            </strong>
                            " Décomposer k en facteurs, chercher des longueurs de cycles disjoints dont le PPCM vaut k, et vérifier qu'il y a "
                            <em>
                                "assez d'éléments"
                            </em>
                            " — c'est presque toujours là que se joue la réponse négative."
                        </li>
                    </ul>
                </div>
                <div class="ex" style="margin-top:14px">
                    <div class="ex-head">
                        <span>
                            "Examen 2024 — σ sur ℕ₁₂ (13 éléments)"
                        </span>
                        <span class="mono">
                            "ordre 60"
                        </span>
                    </div>
                    <div class="ex-body">
                        <span class="mono">
                            "x     0  1  2  3  4  5  6  7  8  9 10 11 12\nσ(x) 12  4  0 11  6  5 10  9  7  3  1  8  2\n\nσ  = (0, 12, 2)(1, 4, 6, 10)(3, 11, 8, 7, 9)\n   = (0,12)(12,2)(1,4)(4,6)(6,10)(3,11)(11,8)(8,7)(7,9)      9 transpositions\nordre = ppcm(3, 4, 5) = "
                            <span class="res">
                                "60"
                            </span>
                            "          signature = (−1)⁹ = "
                            <span class="res">
                                "−1"
                            </span>
                            "\nσ⁻¹ = (2, 12, 0)(10, 6, 4, 1)(9, 7, 8, 11, 3)\n18723 ≡ 3 [60]   donc σ^18723 = σ³ = (1, 10, 6, 4)(8, 3, 7, 11, 9)"
                        </span>
                        <p style="margin-top:10px">
                            "Ordre 30\u{a0}: oui, "
                            <span class="fi">
                                "(0,1,2,3,4)(5,6,7,8,9,10)"
                            </span>
                            " — 5 + 6 = 11 éléments, ppcm 30. Ordre 26\u{a0}: non, il faudrait un cycle de longueur 2 et un de longueur 13 disjoints, soit 15 éléments\u{a0}; on n'en a que 13."
                        </p>
                    </div>
                </div>
                <div class="ex" style="margin-top:14px">
                    <div class="ex-head">
                        <span>
                            "Le groupe de Klein K = {e, u, v, w} ⊂ S₄"
                        </span>
                        <span class="mono">
                            "annale 2024 & rattrapage"
                        </span>
                    </div>
                    <div class="ex-body">
                        <span class="mono">
                            "u = (1,2)∘(3,4)    v = (1,3)∘(2,4)    w = (1,4)∘(2,3)\n\n ∘ │ e  u  v  w\n───┼────────────\n e │ e  u  v  w\n u │ u  e  w  v\n v │ v  w  e  u\n w │ w  v  u  e"
                        </span>
                        <p style="margin-top:10px">
                            "Chaque élément est son propre inverse, le groupe est commutatif, et il est isomorphe à "
                            <span class="fi">
                                "ℤ/2ℤ × ℤ/2ℤ"
                            </span>
                            ". C'est un sous-groupe de S₄ car il est stable, contient e et les inverses."
                        </p>
                        <p>
                            <strong>
                                "Attention\u{a0}:"
                            </strong>
                            " au rattrapage 2025, l'énoncé impose "
                            <span class="fi">
                                "u∗v = v∗u = e = w∗w"
                            </span>
                            ", ce qui donne une "
                            <em>
                                "autre"
                            </em>
                            " table (celle du groupe cyclique ℤ/4ℤ, avec "
                            <span class="fi">
                                "u∗u = w"
                            </span>
                            "). Lire la table imposée avant de recopier celle de Klein."
                        </p>
                    </div>
                </div>
            </>
        </section>
    }
}

fn sec_sf_td1() -> impl IntoView {
    view! {
        <section id="sf-td1">
            <div class="sec-head">
                <span class="num">
                    "TD 1"
                </span>
                <h2>
                    "Logique"
                </h2>
            </div>
            <div class="docmeta">
                <span class="pill on">
                    "Fiche n°1"
                </span>
                <span class="pill">
                    "16 exercices"
                </span>
                <span class="pill">
                    "assertions"
                </span>
                <span class="pill">
                    "négations"
                </span>
                <span class="pill">
                    "quantificateurs"
                </span>
                <span class="pill">
                    "absurde & contraposée"
                </span>
                <span class="pill">
                    "récurrence"
                </span>
            </div>
            <div class="exos">
                <div class="exo">
                    <span class="n">
                        "1"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Ces énoncés sont-ils des assertions\u{a0}?"
                        </span>
                        <span class="sub">
                            "«\u{a0}Les poules ont des dents\u{a0}», «\u{a0}Des étudiants si charmants\u{a0}!\u{a0}», «\u{a0}Quand les poules auront des dents\u{a0}», «\u{a0}Soit n un entier pair\u{a0}», «\u{a0}L'entier n est un diviseur de 5\u{a0}»… Le critère\u{a0}: peut-on lui attribuer une valeur de vérité\u{a0}?"
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "2"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Écrire en français la négation d'énoncés courants."
                        </span>
                        <span class="sub">
                            "«\u{a0}Cent pour cent des gagnants ont tenté leur chance\u{a0}», «\u{a0}Tout coupable aura la tête tranchée\u{a0}», «\u{a0}Toutes les anglaises sont rousses et grandes\u{a0}», "
                            <span class="mono">
                                "∀x ∈ ℝ, x² + 1 > 0 et x² ≥ 0"
                            </span>
                            "…"
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "3"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Comparer des assertions composées."
                        </span>
                        <span class="sub">
                            "Montrer que "
                            <span class="mono">
                                "(¬A ⇒ B) et (¬A ⇒ ¬B)"
                            </span>
                            " a même valeur de vérité que A\u{a0}; comparer "
                            <span class="mono">
                                "A ⇒ (B ou C)"
                            </span>
                            " et "
                            <span class="mono">
                                "(A ⇒ B) ou (A ⇒ C)"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "4-6"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Ensembles de vérité, implications, conditions suffisantes."
                        </span>
                        <span class="sub">
                            "Résoudre «\u{a0}A(x) ou B(x)\u{a0}» pour "
                            <span class="mono">
                                "A(x) : x ≤ 1 ou x ≥ 4"
                            </span>
                            " et "
                            <span class="mono">
                                "B(x) : −1 ≤ x ≤ 2"
                            </span>
                            "\u{a0}; déterminer la véracité de A ⇒ B (démonstration ou contre-exemple)\u{a0}; repérer les conditions suffisantes mais non nécessaires."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "7"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Variables libres et variables muettes."
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "∀x, A(x,y)"
                            </span>
                            " dépend-elle de x\u{a0}? de y\u{a0}? Et "
                            <span class="mono">
                                "Σ_{i=1..N} i²"
                            </span>
                            " dépend-elle de i\u{a0}? de N\u{a0}?"
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "8"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Nier en poussant ¬ jusqu'aux assertions élémentaires."
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "A ⇒ B"
                            </span>
                            "\u{a0}; "
                            <span class="mono">
                                "∃x / A ⇒ B(x)"
                            </span>
                            "\u{a0}; "
                            <span class="mono">
                                "∀x, ∃y / A(y) ⇒ B(y)"
                            </span>
                            "\u{a0}; "
                            <span class="mono">
                                "[∀x, A(x) ou B(x)] ⇒ [(∀x,A(x)) ou (∀x,B(x))]"
                            </span>
                            ". Rappel\u{a0}: "
                            <span class="mono">
                                "¬(A ⇒ B) ⇔ A et ¬B"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "9"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Traduire des propriétés de fonctions en quantificateurs — et l'inverse."
                        </span>
                        <span class="sub">
                            "f et g égales, f(x) = g(x) a une solution, f croissante, f constante… puis retraduire en français "
                            <span class="mono">
                                "∀x, ∃m, M / m ≤ f(x) ≤ M"
                            </span>
                            ", "
                            <span class="mono">
                                "∃a / ∀x, f(x) = a"
                            </span>
                            ", "
                            <span class="mono">
                                "∀x, ∃a / f(x) = a"
                            </span>
                            ". Le nerf de l'exercice\u{a0}: l'ordre des quantificateurs."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "10-13"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Absurde et contraposée."
                        </span>
                        <span class="sub">
                            "Si "
                            <span class="mono">
                                "a² + b² = c²"
                            </span>
                            " alors a ou b est pair\u{a0}; si "
                            <span class="mono">
                                "∀y > 0, x ≤ y"
                            </span>
                            " alors x = 0\u{a0}; √2 irrationnel\u{a0}; si 8 ∤ n²−1 alors n est pair\u{a0}; si "
                            <span class="mono">
                                "n² + n + 1"
                            </span>
                            " est pair alors n est pair\u{a0}; deux entiers impairs consécutifs ont une somme divisible par 4."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "14-15"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Récurrence."
                        </span>
                        <span class="sub">
                            "Si "
                            <span class="mono">
                                "10ⁿ + 7"
                            </span>
                            " est multiple de 9 alors "
                            <span class="mono">
                                "10^{n+1} + 7"
                            </span>
                            " aussi — que peut-on en déduire (attention\u{a0}: l'hérédité seule ne suffit pas)\u{a0}? Puis "
                            <span class="mono">
                                "(1 + x)ⁿ ≥ 1 + nx"
                            </span>
                            "\u{a0}: la récurrence porte-t-elle sur n, sur x, sur les deux\u{a0}?"
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "16"
                    </span>
                    <span class="c">
                        <span class="t">
                            "QCM de logique appliquée (test de positionnement)."
                        </span>
                        <span class="sub">
                            "Cinq questions sur des implications en langage courant (Dell / appareil photo / éditeur vi, règles de confidentialité d'un réseau social). Tout se résout avec la contraposée."
                        </span>
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_sf_td2() -> impl IntoView {
    view! {
        <section id="sf-td2">
            <div class="sec-head">
                <span class="num">
                    "TD 2"
                </span>
                <h2>
                    "Ensembles, relations, applications, dénombrement"
                </h2>
            </div>
            <div class="docmeta">
                <span class="pill on">
                    "Fiche n°2"
                </span>
                <span class="pill">
                    "35 exercices"
                </span>
                <span class="pill">
                    "2 parties"
                </span>
                <span class="pill hot">
                    "le plus rentable pour le partiel"
                </span>
            </div>
            <div class="exos">
                <div class="exo">
                    <span class="n">
                        "1-3"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Écritures correctes et parties d'un ensemble."
                        </span>
                        <span class="sub">
                            "Pour "
                            <span class="mono">
                                "E = {a,b,c}"
                            </span>
                            "\u{a0}: lesquelles de "
                            <span class="mono">
                                "a ∈ E, a ⊂ E, {a} ⊂ E, ∅ ∈ E, ∅ ⊂ E, {∅} ⊂ E"
                            </span>
                            " ont un sens\u{a0}? Décrire "
                            <span class="mono">
                                "P({a,b})"
                            </span>
                            ", "
                            <span class="mono">
                                "P(∅)"
                            </span>
                            ", "
                            <span class="mono">
                                "P({∅})"
                            </span>
                            ", "
                            <span class="mono">
                                "P(P({a}))"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "4-6"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Simplifier et comparer."
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "A ∩ (A ∪ B)"
                            </span>
                            ", "
                            <span class="mono">
                                "A \\ (A ∩ B)"
                            </span>
                            ", "
                            <span class="mono">
                                "A \\ (B \\ A)"
                            </span>
                            ", "
                            <span class="mono">
                                "(A\\B) ∩ (B\\A)"
                            </span>
                            "… puis dénombrer "
                            <span class="mono">
                                "{(a,b) ∈ ℕ² | a² + b² ≤ 9}"
                            </span>
                            " et ses variantes dans ℕ×ℤ et ℤ²."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "7"
                    </span>
                    <span class="c">
                        <span class="t">
                            "A ∩ B = A ∩ C entraîne-t-il B = C\u{a0}?"
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " Non\u{a0}: "
                            <span class="mono">
                                "A = {1}, B = ∅, C = {2}"
                            </span>
                            ". Avec l'union non plus. Mais les "
                            <em>
                                "deux"
                            </em>
                            " hypothèses ensemble donnent B = C. Tombé au partiel 2024, à l'examen 2024 et au rattrapage."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "8-13"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Identités et complémentaires."
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "A\\(B ∩ C) = (A\\B) ∪ (A\\C)"
                            </span>
                            "\u{a0}; "
                            <span class="mono">
                                "C_E A \\ C_E B = B\\A"
                            </span>
                            "\u{a0}; "
                            <span class="mono">
                                "(A ∩ B = A ∪ B) ⇒ A = B"
                            </span>
                            " "
                            <span class="tag hot">
                                "annale"
                            </span>
                            "\u{a0}; résoudre "
                            <span class="mono">
                                "A ∪ X = B"
                            </span>
                            " puis "
                            <span class="mono">
                                "A ∩ X = B"
                            </span>
                            " d'inconnue X\u{a0}; comparaisons d'inclusions avec produits cartésiens."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "14-17"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Relations binaires et ordres."
                        </span>
                        <span class="sub">
                            "Diagrammes sagittaux\u{a0}; propriétés de "
                            <span class="mono">
                                "xRy ⇔ xy ≤ 0"
                            </span>
                            ", "
                            <span class="mono">
                                "xy ≥ 0"
                            </span>
                            ", "
                            <span class="mono">
                                "xy > 0"
                            </span>
                            ", "
                            <span class="mono">
                                "sin x = sin y"
                            </span>
                            "\u{a0}; majorants, minorants, sup et inf de "
                            <span class="mono">
                                "[0,1["
                            </span>
                            ", "
                            <span class="mono">
                                "]0,+∞["
                            </span>
                            ", ℕ, ℤ\u{a0}; "
                            <span class="mono">
                                "(P(E), ⊂)"
                            </span>
                            " est-il totalement ordonné\u{a0}?"
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "18-19"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Fonction ou application\u{a0}? injective, surjective, bijective\u{a0}?"
                        </span>
                        <span class="sub">
                            "Neuf correspondances à classer ("
                            <span class="mono">
                                "x ↦ 2x+3"
                            </span>
                            ", "
                            <span class="mono">
                                "x ↦ 1/(x+1)"
                            </span>
                            ", "
                            <span class="mono">
                                "x ↦ x/(x+2)"
                            </span>
                            ", tan, cos selon l'ensemble d'arrivée…), puis "
                            <span class="mono">
                                "f(x) = (ax+b)/(cx−a)"
                            </span>
                            " avec calcul de "
                            <span class="mono">
                                "f ∘ f"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "20-23"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Composition et bijections."
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " Si "
                            <span class="mono">
                                "g∘f"
                            </span>
                            " injective et f surjective alors g injective\u{a0}; "
                            <span class="mono">
                                "(x,y) ↦ (2x+3y, x−y)"
                            </span>
                            " est bijective\u{a0}; la fonction de couplage "
                            <span class="mono">
                                "π(m,n) = (m+n)(m+n+1)/2 + n"
                            </span>
                            " est une bijection de ℕ² sur ℕ\u{a0}; images directes et réciproques de "
                            <span class="mono">
                                "n ↦ 2n+1"
                            </span>
                            " et "
                            <span class="mono">
                                "n ↦ 2n"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "24"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Fonction indicatrice 1_A."
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " "
                            <span class="mono">
                                "1_{Aᶜ} = 1 − 1_A"
                            </span>
                            ", "
                            <span class="mono">
                                "1_{A∩B} = 1_A·1_B"
                            </span>
                            ", "
                            <span class="mono">
                                "1_{A∪B} = 1_A + 1_B − 1_A·1_B"
                            </span>
                            ". Repris tel quel au partiel 2025 (4 points)."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "25-27"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Images directes et réciproques."
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " Les inclusions du tableau du chapitre 03, et leurs cas d'égalité\u{a0}: f surjective ⇔ "
                            <span class="mono">
                                "f(f⁻¹(B)) = B"
                            </span>
                            "\u{a0}; f injective ⇔ "
                            <span class="mono">
                                "f⁻¹(f(A)) = A"
                            </span>
                            " ⇔ "
                            <span class="mono">
                                "f(A ∩ B) = f(A) ∩ f(B)"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "28-35"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Dénombrement."
                        </span>
                        <span class="sub">
                            "Montrer que "
                            <span class="mono">
                                "2ℕ"
                            </span>
                            ", ℤ, ℕ², ℚ sont dénombrables et que "
                            <span class="mono">
                                "{0,1}^ℕ"
                            </span>
                            " ne l'est pas\u{a0}; symétrie des coefficients binomiaux, binôme de Newton, Vandermonde, formule de Pascal\u{a0}; compter "
                            <span class="mono">
                                "{(A,B) | A ⊂ B}"
                            </span>
                            " et "
                            <span class="mono">
                                "{(A,B) | A ∪ B = E}"
                            </span>
                            "\u{a0}; et, pour finir, deux pays voisins du même nombre de pays sur toute planète."
                        </span>
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_sf_td3() -> impl IntoView {
    view! {
        <section id="sf-td3">
            <div class="sec-head">
                <span class="num">
                    "TD 3"
                </span>
                <h2>
                    "Nombres complexes"
                </h2>
            </div>
            <div class="docmeta">
                <span class="pill on">
                    "Fiche n°3"
                </span>
                <span class="pill">
                    "21 exercices"
                </span>
                <span class="pill">
                    "calcul algébrique"
                </span>
                <span class="pill">
                    "forme trigonométrique"
                </span>
                <span class="pill">
                    "racines de l'unité"
                </span>
            </div>
            <div class="exos">
                <div class="exo">
                    <span class="n">
                        "1-4"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Parties réelle et imaginaire, formes algébrique et trigonométrique."
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "(2+3i)(1−4i)"
                            </span>
                            ", "
                            <span class="mono">
                                "(2−i)³"
                            </span>
                            ", "
                            <span class="mono">
                                "(1−2i)/(2+i)"
                            </span>
                            ", "
                            <span class="mono">
                                "(1+i)⁴/(2−i)²"
                            </span>
                            ", "
                            <span class="mono">
                                "e^{iπ/3}"
                            </span>
                            ", "
                            <span class="mono">
                                "e^{iπ/4} ± e^{−iπ/4}"
                            </span>
                            "\u{a0}; calculer "
                            <span class="mono">
                                "Σ_{k=0..p} i^k"
                            </span>
                            " selon p\u{a0}; forme trigonométrique de "
                            <span class="mono">
                                "1 + i√3"
                            </span>
                            " et "
                            <span class="mono">
                                "(1+i)/(1−i)"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "5-9"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Modules et arguments."
                        </span>
                        <span class="sub">
                            "Si "
                            <span class="mono">
                                "|a| = |b| = 1"
                            </span>
                            " et "
                            <span class="mono">
                                "a ≠ ±b"
                            </span>
                            ", alors "
                            <span class="mono">
                                "(a+b)/(1+ab)"
                            </span>
                            " est réel\u{a0}; forme exponentielle de "
                            <span class="mono">
                                "1/(1+i)"
                            </span>
                            " et parties réelle/imaginaire de "
                            <span class="mono">
                                "zⁿ"
                            </span>
                            "\u{a0}; résoudre "
                            <span class="mono">
                                "z² = 1 + i"
                            </span>
                            " et en déduire "
                            <span class="mono">
                                "cos(π/8)"
                            </span>
                            " et "
                            <span class="mono">
                                "sin(π/8)"
                            </span>
                            "\u{a0}; pour quels n "
                            <span class="mono">
                                "(√3 + i)ⁿ"
                            </span>
                            " est-il réel\u{a0}?"
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "10-12"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Équations en Z + 1/Z, formules de duplication, linéarisation."
                        </span>
                        <span class="sub">
                            "Résoudre "
                            <span class="mono">
                                "Z + 1/Z = 2cos x"
                            </span>
                            "\u{a0}; exprimer "
                            <span class="mono">
                                "cos 4a"
                            </span>
                            ", "
                            <span class="mono">
                                "sin 3a"
                            </span>
                            ", "
                            <span class="mono">
                                "tan 3a"
                            </span>
                            " en fonction de cos a et sin a\u{a0}; linéariser "
                            <span class="mono">
                                "sin³a"
                            </span>
                            ", "
                            <span class="mono">
                                "cos³a"
                            </span>
                            ", "
                            <span class="mono">
                                "sin⁴a"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "13-14"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Équations dans ℂ."
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "(2+i)z + (1+2i)² = −6+5i"
                            </span>
                            "\u{a0}; "
                            <span class="mono">
                                "4x² − 2x + 1 = 0"
                            </span>
                            "\u{a0}; puis les équations exponentielles "
                            <span class="mono">
                                "eᶻ + e^{−z} = 1"
                            </span>
                            " et "
                            <span class="mono">
                                "= 2i"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "15-21"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Racines de l'unité."
                        </span>
                        <span class="sub">
                            "Simplifications avec une racine septième\u{a0}; sommes de sinus\u{a0}; résoudre "
                            <span class="mono">
                                "z⁶ = 1"
                            </span>
                            " sous forme exponentielle puis algébrique\u{a0}; représenter les racines 4-, 6- et 8-ièmes\u{a0}; puissances de "
                            <span class="mono">
                                "ω = e^{2iπ/5}"
                            </span>
                            " et sommes "
                            <span class="mono">
                                "Σ ω^{3k}"
                            </span>
                            ", "
                            <span class="mono">
                                "Σ ω^{4k}"
                            </span>
                            "\u{a0}; et la famille d'équations "
                            <span class="mono">
                                "((z+1)/(z−1))ⁿ = 1"
                            </span>
                            ", puis "
                            <span class="mono">
                                "= e^{inθ}"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
            </div>
            <div class="note method" style="margin-top:20px">
                <span class="title">
                    "Méthode — les équations en (z+1)/(z−1)"
                </span>
                <p>
                    "On pose "
                    <span class="fi">
                        "Z = (z+1)/(z−1)"
                    </span>
                    ". L'équation devient "
                    <span class="fi">
                        "Zⁿ = 1"
                    </span>
                    ", dont on connaît les n solutions "
                    <span class="fi">
                        "ω_k"
                    </span>
                    "\u{a0}; on revient ensuite à z en résolvant "
                    <span class="fi">
                        "(z+1)/(z−1) = ω_k"
                    </span>
                    ", soit "
                    <span class="fi">
                        "z = (ω_k + 1)/(ω_k − 1)"
                    </span>
                    ", en écartant "
                    <span class="fi">
                        "ω_k = 1"
                    </span>
                    " qui ne donne pas de solution."
                </p>
            </div>
        </section>
    }
}

fn sec_sf_td4() -> impl IntoView {
    view! {
        <section id="sf-td4">
            <div class="sec-head">
                <span class="num">
                    "TD 4"
                </span>
                <h2>
                    "Arithmétique modulaire, groupes, permutations"
                </h2>
            </div>
            <div class="docmeta">
                <span class="pill on">
                    "Fiche n°4"
                </span>
                <span class="pill">
                    "49 exercices"
                </span>
                <span class="pill">
                    "3 parties"
                </span>
                <span class="pill hot">
                    "4 exercices repris tels quels en examen"
                </span>
            </div>
            <h3>
                "Partie 1 — Le groupe ℤ/nℤ "
                <span style="font-weight:400;font-size:15px;color:var(--ink-soft)">
                    "(ex. 1 à 17)"
                </span>
            </h3>
            <div class="exos">
                <div class="exo">
                    <span class="n">
                        "1-2"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Division euclidienne dans la vraie vie."
                        </span>
                        <span class="sub">
                            "Combien de photos de 2,1 Mo sur une clé de 16 Go, et combien reste-t-il\u{a0}? Convertir 10 000 secondes en heures, minutes, secondes."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "3-5"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Restes et classes d'équivalence."
                        </span>
                        <span class="sub">
                            "Reste de "
                            <span class="mono">
                                "742444"
                            </span>
                            " par 11\u{a0}; classe de "
                            <span class="mono">
                                "7 584 534 525"
                            </span>
                            " modulo 3, 5, 9, 11, 7\u{a0}; "
                            <span class="mono">
                                "2147 + 1"
                            </span>
                            " est-il divisible par 3\u{a0}? Puis "
                            <span class="mono">
                                "f(n) = 10n + 3·4^{n+2} + 5"
                            </span>
                            " divisible par 9 (récurrence via "
                            <span class="mono">
                                "f(n+1) − f(n)"
                            </span>
                            ")."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "6-8"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Structure de ℤ/7Z et ℤ/12ℤ."
                        </span>
                        <span class="sub">
                            "Sommes, opposés, ordre de chaque élément\u{a0}; éléments inversibles de ℤ/12ℤ (ce sont les x avec "
                            <span class="mono">
                                "x ∧ 12 = 1"
                            </span>
                            ")."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "7"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Bijectivité de x ↦ ax + b dans ℤ/nℤ."
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " "
                            <span class="mono">
                                "f : x ↦ 34x + 91"
                            </span>
                            " sur ℤ/101ℤ et "
                            <span class="mono">
                                "g : x ↦ 18x + 15"
                            </span>
                            " sur ℤ/51ℤ. Énoncé identique à l'examen 2024, variante au rattrapage."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "9-16"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Équations et critères de divisibilité."
                        </span>
                        <span class="sub">
                            "Résoudre "
                            <span class="mono">
                                "x⁵ + 3x⁴ + x + 1 ≡ 0 [2]"
                            </span>
                            ", "
                            <span class="mono">
                                "3m ≡ 2 [7]"
                            </span>
                            ", "
                            <span class="mono">
                                "5^{2m} + 5^m ≡ 0 [13]"
                            </span>
                            "\u{a0}; tables d'addition et de multiplication de ℤ/7ℤ\u{a0}; équations dans ℤ/8ℤ\u{a0}; construire un critère de divisibilité par 39 puis par 41\u{a0}; "
                            <span class="mono">
                                "x³ = x"
                            </span>
                            " dans ℤ/11ℤ et ℤ/12ℤ\u{a0}; preuve du critère par 9 et par 11."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "17"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Reste de 4007"
                            <sup>
                                "1235"
                            </sup>
                            " par 13."
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " Exactement l'exercice 4 de l'examen 2024 — corrigé dans le chapitre 07."
                        </span>
                    </span>
                </div>
            </div>
            <h3>
                "Partie 2 — Groupes "
                <span style="font-weight:400;font-size:15px;color:var(--ink-soft)">
                    "(ex. 18 à 30)"
                </span>
            </h3>
            <div class="exos">
                <div class="exo">
                    <span class="n">
                        "18-21"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Sous-groupes et produits."
                        </span>
                        <span class="sub">
                            "Déterminer tous les sous-groupes de (ℤ,+)\u{a0}; inverse d'un produit "
                            <span class="mono">
                                "g₁⋯gₙ"
                            </span>
                            "\u{a0}; montrer que "
                            <span class="mono">
                                "G × H"
                            </span>
                            " muni de la loi composante par composante est un groupe."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "22-23"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Est-ce un groupe\u{a0}?"
                        </span>
                        <span class="sub">
                            "Cinq lois exotiques à tester ("
                            <span class="mono">
                                "x⋆y = x + y − xy"
                            </span>
                            ", etc.)\u{a0}; puis un groupe à quatre éléments donné par sa table — vérifier les axiomes, trouver le neutre, les inverses, la commutativité."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "24-28"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Propriétés générales."
                        </span>
                        <span class="sub">
                            "Si "
                            <span class="mono">
                                "∀x, x² = e"
                            </span>
                            " alors G est commutatif\u{a0}; si "
                            <span class="mono">
                                "(ab)ⁿ = e"
                            </span>
                            " alors "
                            <span class="mono">
                                "(ba)ⁿ = e"
                            </span>
                            "\u{a0}; l'intersection de deux sous-groupes est un sous-groupe\u{a0}; le centre "
                            <span class="mono">
                                "Z(G)"
                            </span>
                            " est un sous-groupe\u{a0}; et la démonstration guidée du "
                            <strong>
                                "théorème de Lagrange"
                            </strong>
                            " par les classes "
                            <span class="mono">
                                "aH"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "29-30"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Deux groupes à construire."
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "x ∗ y = x + y + xy"
                            </span>
                            " sur ℝ\u{a0}: associativité, neutre, inversibles, plus grand sous-ensemble qui en fait un groupe. Puis "
                            <span class="mono">
                                "G = {a + b√2 | a,b ∈ ℚ, a² − 2b² ≠ 0}"
                            </span>
                            " muni de ×."
                        </span>
                    </span>
                </div>
            </div>
            <h3>
                "Partie 3 — Permutations "
                <span style="font-weight:400;font-size:15px;color:var(--ink-soft)">
                    "(ex. 31 à 49)"
                </span>
            </h3>
            <div class="exos">
                <div class="exo">
                    <span class="n">
                        "31-35"
                    </span>
                    <span class="c">
                        <span class="t">
                            "La batterie de calculs."
                        </span>
                        <span class="sub">
                            "Pour chaque permutation donnée\u{a0}: décomposition en cycles disjoints, en transpositions, ordre, signature, "
                            <span class="mono">
                                "σ⁻¹"
                            </span>
                            ", "
                            <span class="mono">
                                "σ^{100}"
                            </span>
                            " ou "
                            <span class="mono">
                                "σ^{9999}"
                            </span>
                            ", diagramme sagittal."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "36-38"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Groupe de Klein et petits résultats."
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " K = {e,u,v,w} est un sous-groupe commutatif de S₄ — table de multiplication, comparaison avec ℤ/2ℤ × ℤ/2ℤ\u{a0}; permutations à n−1 points fixes\u{a0}; signature du «\u{a0}retournement\u{a0}» "
                            <span class="mono">
                                "k ↦ n+1−k"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "39-45"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Théorie."
                        </span>
                        <span class="sub">
                            "Écriture en transpositions et cardinal du support\u{a0}; orbites et relations d'équivalence\u{a0}; "
                            <span class="mono">
                                "σ²"
                            </span>
                            " est toujours paire\u{a0}; Sₙ est engendré par "
                            <span class="mono">
                                "(1,2), …, (1,n)"
                            </span>
                            ", puis par "
                            <span class="mono">
                                "(1,2)"
                            </span>
                            " et le cycle "
                            <span class="mono">
                                "(1,2,…,n)"
                            </span>
                            "\u{a0}; le groupe alterné est engendré par les 3-cycles\u{a0}; théorème de Cayley."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "46-48"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Trois sujets d'examen recopiés dans la fiche."
                        </span>
                        <span class="sub">
                            <span class="tag hot">
                                "annale"
                            </span>
                            " Le groupe de Klein (examen 2024, 4 pts), la permutation de ℕ₁₂ (examen 2024-2025, 8 pts), et le groupe K défini par "
                            <span class="mono">
                                "uv = vu = e = ww"
                            </span>
                            " (examen 2024-2025, 5 pts)."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "49"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Le mélange de cartes."
                        </span>
                        <span class="sub">
                            "Un paquet de 8 cartes mélangé selon σ\u{a0}: décomposer, calculer l'ordre, en déduire combien de mélanges ramènent le paquet dans l'ordre initial — et pourquoi l'ordre d'une permutation intéresse l'algorithmique."
                        </span>
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_sf_recurrent() -> impl IntoView {
    view! {
        <section id="sf-recurrent">
            <div class="sec-head">
                <span class="num">
                    "★"
                </span>
                <h2>
                    "Les questions qui reviennent"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Croisement des cinq sujets disponibles — partiels 2024 et 2025, examens 2024 et 2025, rattrapage juin 2025 — avec les feuilles de TD. Douze familles de questions couvrent la quasi-totalité des points distribués."
                </p>
            </div>
            <div class="tw" style="max-width:none;margin-top:18px">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Question"
                            </th>
                            <th>
                                "Vue en"
                            </th>
                            <th>
                                "Ce qu'il faut savoir faire"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                <strong>
                                    "Limite d'une suite par la définition ε"
                                </strong>
                            </td>
                            <td class="mono">
                                "P24 · E24 · P25 · R25"
                            </td>
                            <td>
                                "Toujours le même moule\u{a0}: "
                                <span class="mono">
                                    "(an²+1)/(n²+2) → a"
                                </span>
                                ". Majorer la différence, poser "
                                <span class="mono">
                                    "n₀ ≥ √(c/ε)"
                                </span>
                                " et enchaîner les majorations. Donner la limite ne rapporte rien\u{a0}: il faut exhiber n₀."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Valeur de vérité d'assertions quantifiées"
                                </strong>
                            </td>
                            <td class="mono">
                                "P24 · P25 · E25"
                            </td>
                            <td>
                                <span class="mono">
                                    "∃x∀y"
                                </span>
                                " contre "
                                <span class="mono">
                                    "∀x∃y"
                                </span>
                                "\u{a0}; "
                                <span class="mono">
                                    "∃x, (P et Q)"
                                </span>
                                " contre "
                                <span class="mono">
                                    "(∃x,P) et (∃x,Q)"
                                </span>
                                ". Vraie ⇒ exhiber un témoin\u{a0}; fausse ⇒ contre-exemple explicite. Sans justification, 0."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "A ∩ B = A ∪ B ⇒ A = B"
                                </strong>
                            </td>
                            <td class="mono">
                                "E24 · R25 · TD2"
                            </td>
                            <td>
                                "Double inclusion, élément par élément. Variante\u{a0}: "
                                <span class="mono">
                                    "A∩B = A∩C"
                                </span>
                                " "
                                <em>
                                    "et"
                                </em>
                                " "
                                <span class="mono">
                                    "A∪B = A∪C"
                                </span>
                                " ⇒ B = C, avec la question piège «\u{a0}une seule condition suffit-elle\u{a0}?\u{a0}» — non, contre-exemples "
                                <span class="mono">
                                    "A = 2ℕ, B = 2ℕ+1, C = 4ℕ+1"
                                </span>
                                "."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "A ∩ B = A ∩ C entraîne-t-il B = C\u{a0}?"
                                </strong>
                            </td>
                            <td class="mono">
                                "P24 · TD2"
                            </td>
                            <td>
                                "Non. Contre-exemple minimal\u{a0}: "
                                <span class="mono">
                                    "A = {1}, B = ∅, C = {2}"
                                </span>
                                "."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Composition\u{a0}: g∘f injective/surjective"
                                </strong>
                            </td>
                            <td class="mono">
                                "P24 · TD2"
                            </td>
                            <td>
                                "g∘f injective ⇒ f injective\u{a0}; g∘f surjective ⇒ g surjective\u{a0}; puis le cas à trois applications f, g, h avec g∘f et h∘g bijectives."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Images directes et réciproques"
                                </strong>
                            </td>
                            <td class="mono">
                                "P24 · P25 · TD2"
                            </td>
                            <td>
                                <span class="mono">
                                    "A ⊂ f⁻¹(f(A))"
                                </span>
                                " et "
                                <span class="mono">
                                    "f(f⁻¹(B)) ⊂ B"
                                </span>
                                ", avec «\u{a0}a-t-on égalité en général\u{a0}?\u{a0}» — non, sauf injectivité (resp. surjectivité). Variante\u{a0}: "
                                <span class="mono">
                                    "f(A) ∩ B = ∅ ⇔ A ∩ f⁻¹(B) = ∅"
                                </span>
                                "."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Bijection de ℝ² et sa réciproque"
                                </strong>
                            </td>
                            <td class="mono">
                                "P25 · TD2"
                            </td>
                            <td>
                                <span class="mono">
                                    "(x,y) ↦ (x+3y, x+y)"
                                </span>
                                " ou "
                                <span class="mono">
                                    "(2x+3y, x−y)"
                                </span>
                                "\u{a0}: injectivité par résolution du système, surjectivité par antécédent explicite, puis expression de f⁻¹."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Fonction indicatrice 1_A"
                                </strong>
                            </td>
                            <td class="mono">
                                "P25 · TD2"
                            </td>
                            <td>
                                <span class="mono">
                                    "1_∅ = 0"
                                </span>
                                ", "
                                <span class="mono">
                                    "1_E = 1"
                                </span>
                                ", "
                                <span class="mono">
                                    "1_{Aᶜ} = 1 − 1_A"
                                </span>
                                ", "
                                <span class="mono">
                                    "1_{A∩B} = 1_A 1_B"
                                </span>
                                ", "
                                <span class="mono">
                                    "1_{A∪B} = 1_A + 1_B − 1_A 1_B"
                                </span>
                                ". Se démontre en distinguant les quatre cas d'appartenance."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "x ↦ ax + b bijective dans ℤ/nℤ\u{a0}?"
                                </strong>
                            </td>
                            <td class="mono">
                                "E24 · R25 · TD4"
                            </td>
                            <td>
                                "Oui si et seulement si "
                                <span class="mono">
                                    "a ∧ n = 1"
                                </span>
                                ". Rédaction attendue\u{a0}: injectivité via «\u{a0}n divise a(x−y) et a est premier avec n donc n | x−y\u{a0}», surjectivité en exhibant l'antécédent à l'aide de l'inverse de a. Sinon, deux antécédents égaux suffisent à conclure."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Reste de a"
                                    <sup>
                                        "N"
                                    </sup>
                                    " modulo m"
                                </strong>
                            </td>
                            <td class="mono">
                                "E24 · R25 · TD4"
                            </td>
                            <td>
                                "Réduire la base, trouver le petit exposant k avec "
                                <span class="mono">
                                    "a₀^k ≡ ±1"
                                </span>
                                ", diviser N par k. Réponse 9 dans les deux annales."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Permutation de ℕ₁₂ — le gros exercice"
                                </strong>
                            </td>
                            <td class="mono">
                                "E24 · R25 · E25 · TD4"
                            </td>
                            <td>
                                "8 points à chaque fois\u{a0}: cycles disjoints, transpositions, ordre (ppcm), signature, σ⁻¹, "
                                <span class="mono">
                                    "σ^{18723}"
                                </span>
                                " via le reste modulo l'ordre, et existence d'une permutation d'un ordre donné (argument du nombre d'éléments disponibles)."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Groupe à quatre éléments"
                                </strong>
                            </td>
                            <td class="mono">
                                "E24 · R25 · TD4"
                            </td>
                            <td>
                                "Compléter la table, en déduire que c'est un groupe, résoudre une équation du type "
                                <span class="mono">
                                    "uvwxxwu = vuw"
                                </span>
                                " en simplifiant à gauche et à droite. Lire quelle table l'énoncé impose\u{a0}: Klein ou cyclique."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <strong>
                                    "Équation du second degré dans ℤ/nℤ"
                                </strong>
                            </td>
                            <td class="mono">
                                "E24 · R25"
                            </td>
                            <td>
                                "Factoriser "
                                <span class="mono">
                                    "x² + bx + c = (x+α)(x+β)"
                                </span>
                                " modulo n = pq, puis énumérer les quatre cas de divisibilité, chacun se ramenant à une équation de Bezout "
                                <span class="mono">
                                    "pu − qv = k"
                                </span>
                                ". Réponses\u{a0}: {4, 26, 46, 68} modulo 77, et {25, 27} modulo 35."
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="body" style="margin-top:14px">
                <p style="font-size:13px;color:var(--ink-soft)">
                    "P24 = partiel 2024-2025 · E24 = examen 2024-2025 · P25 = partiel 2025-2026 · E25 = examen final 2025-2026 · R25 = rattrapage de juin 2025."
                </p>
            </div>
            <div class="note warn" style="margin-top:20px">
                <span class="title">
                    "Le programme du partiel n'est pas celui de l'examen"
                </span>
                <p>
                    "Les "
                    <strong>
                        "partiels"
                    </strong>
                    " portent sur la logique, les ensembles, les applications et les débuts de l'analyse (suites, limites, Cauchy). Les "
                    <strong>
                        "examens finaux"
                    </strong>
                    " ajoutent l'arithmétique modulaire, les groupes et les permutations, qui pèsent alors plus de la moitié des points. Les suites, elles, reviennent dans les deux."
                </p>
            </div>
        </section>
    }
}

fn sec_sf_annales() -> impl IntoView {
    view! {
        <section id="sf-annales">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Annales et corrigés"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Cinq sujets, tous notés sur plus de 20\u{a0}: le total accessible va de 26 à 31,5 points et "
                    <strong>
                        "20 points suffisent pour la note maximale"
                    </strong>
                    ". Le choix des exercices fait donc partie de l'épreuve — lire tout le sujet, repérer les plus abordables, commencer par eux."
                </p>
            </div>
            <div class="annales" style="margin-top:20px">
                <div class="an">
                    <span class="y">
                        "Partiel · 2024-2025 · 26 pts"
                    </span>
                    <h4>
                        "Logique, ensembles, suites"
                    </h4>
                    <ul>
                        <li>
                            "Ex 1 (4 pts) — négations et valeurs de vérité de "
                            <span class="mono">
                                "∃∀ / ∀∀ / ∃∃ / ∀∃"
                            </span>
                        </li>
                        <li>
                            "Ex 2-4 (8 pts) — A∩B = A∩C, produits cartésiens, différence symétrique "
                            <span class="mono">
                                "AΔB = B ⇔ A = ∅"
                            </span>
                        </li>
                        <li>
                            "Ex 5-6 (8 pts) — "
                            <span class="mono">
                                "f(A) ∩ B = ∅ ⇔ A ∩ f⁻¹(B) = ∅"
                            </span>
                            ", composition de trois applications"
                        </li>
                        <li>
                            "Ex 7-10 (6 pts) — cinq contre-exemples de suites, limite par ε, suite non de Cauchy, suite d'entiers convergente"
                        </li>
                    </ul>
                    <span class="foot">
                        "Corrigé complet disponible."
                    </span>
                </div>
                <div class="an">
                    <span class="y">
                        "Examen final · 2024-2025"
                    </span>
                    <h4>
                        "Modulo, ensembles, permutations"
                    </h4>
                    <ul>
                        <li>
                            "Ex 1 (1,5) — limite par ε"
                        </li>
                        <li>
                            "Ex 2 (3) — bijectivité dans ℤ/101ℤ et ℤ/51ℤ"
                        </li>
                        <li>
                            "Ex 3 (4,5) — A∩B = A∪B ⇒ A = B"
                        </li>
                        <li>
                            "Ex 4 (2) — reste de "
                            <span class="mono">
                                "4007¹²³⁵"
                            </span>
                            " par 13"
                        </li>
                        <li>
                            "Ex 5 (8) — permutation de ℕ₁₂"
                        </li>
                        <li>
                            "Ex 6 (4) — groupe de Klein"
                        </li>
                        <li>
                            "Ex 7 (5) — "
                            <span class="mono">
                                "x² + 5x + 41 = 0"
                            </span>
                            " dans ℤ/77ℤ"
                        </li>
                    </ul>
                    <span class="foot">
                        "Le sujet n'a pas été transmis\u{a0}; son corrigé, lui, redonne tous les énoncés."
                    </span>
                </div>
                <div class="an">
                    <span class="y">
                        "Partiel · 2025-2026 · 28 pts"
                    </span>
                    <h4>
                        "Logique, applications, indicatrices"
                    </h4>
                    <ul>
                        <li>
                            "Ex 1-3 (6 pts) — assertions vraies, quantificateurs, contraposée de «\u{a0}a+b irrationnel ⇒ a ou b irrationnel\u{a0}»"
                        </li>
                        <li>
                            "Ex 4 (3,5) — bijection "
                            <span class="mono">
                                "(x,y) ↦ (x+3y, x+y)"
                            </span>
                        </li>
                        <li>
                            "Ex 5-6 (10) — images directes/réciproques, et «\u{a0}"
                            <span class="mono">
                                "A∩B = ∅ ⇒ f(A)∩f(B) = ∅"
                            </span>
                            "\u{a0}» équivaut à f injective"
                        </li>
                        <li>
                            "Ex 7 (3) — construire f, g avec g∘f injective et g non injective, etc."
                        </li>
                        <li>
                            "Ex 8 (4) — fonction indicatrice"
                        </li>
                        <li>
                            "Ex 9 (1,5) — limite par ε"
                        </li>
                    </ul>
                    <span class="foot">
                        "Corrigé complet disponible. Une feuille A4 était autorisée."
                    </span>
                </div>
                <div class="an">
                    <span class="y">
                        "Examen final · 2025-2026 · 31,5 pts"
                    </span>
                    <h4>
                        "Le plus long, le plus varié"
                    </h4>
                    <ul>
                        <li>
                            "Ex 1-3 (6 pts) — "
                            <span class="mono">
                                "(−1)ⁿ"
                            </span>
                            " diverge, six assertions quantifiées, inf et sup de "
                            <span class="mono">
                                "{(−1)ⁿ n/(n+1)}"
                            </span>
                        </li>
                        <li>
                            "Ex 4 (4) — "
                            <span class="mono">
                                "(x,y) ↦ (x+y, x−y)"
                            </span>
                            " sur ℝ² puis sur "
                            <span class="mono">
                                "(ℤ/20ℤ)²"
                            </span>
                        </li>
                        <li>
                            "Ex 5 (9) — théorème de Wilson démontré pas à pas\u{a0}: "
                            <span class="mono">
                                "(p−1)! + 1 ≡ 0 [p]"
                            </span>
                            " et sa réciproque"
                        </li>
                        <li>
                            "Ex 6 (5) — fléchettes à 124 et 47 points\u{a0}: équation de Bezout "
                            <span class="mono">
                                "124x + 47y = 10 000"
                            </span>
                        </li>
                        <li>
                            "Ex 7 (7,5) — la «\u{a0}σ-dance\u{a0}»\u{a0}: permutations mises en scène par des danseurs et des chaises"
                        </li>
                    </ul>
                    <span class="foot">
                        "Pas de corrigé transmis pour ce sujet."
                    </span>
                </div>
                <div class="an">
                    <span class="y">
                        "Rattrapage · juin 2025 · 31 pts"
                    </span>
                    <h4>
                        "Corrigé intégré au sujet"
                    </h4>
                    <ul>
                        <li>
                            "Ex 1-2 (4,5) — limite par ε, bijectivité dans ℤ/67ℤ et ℤ/65ℤ"
                        </li>
                        <li>
                            "Ex 3 (4,5) — A∩B = A∪B ⇒ A = B, et la variante à deux conditions"
                        </li>
                        <li>
                            "Ex 4 (2) — reste de "
                            <span class="mono">
                                "3044³⁰⁴⁴"
                            </span>
                            " par 13"
                        </li>
                        <li>
                            "Ex 5 (8) — permutation de ℕ₁₂, ordres 35 et 33"
                        </li>
                        <li>
                            "Ex 6 (5) — groupe K défini par "
                            <span class="mono">
                                "uv = vu = e = ww"
                            </span>
                        </li>
                        <li>
                            "Ex 7 (7) — "
                            <span class="mono">
                                "x² + 18x + 10 = 0"
                            </span>
                            " dans ℤ/35ℤ"
                        </li>
                    </ul>
                    <span class="foot">
                        "Le document le plus utile du lot\u{a0}: chaque solution est rédigée, avec le barème et les consignes de correction."
                    </span>
                </div>
            </div>
            <div class="note method" style="margin-top:22px">
                <span class="title">
                    "Méthode — la limite par la définition, quatre fois sur cinq"
                </span>
                <p>
                    "Modèle\u{a0}: montrer que "
                    <span class="fi">
                        "uₙ = (4n²+1)/(2n²+2)"
                    </span>
                    " converge vers 2."
                </p>
                <span class="mono" style="display:block;white-space:pre-wrap;font-size:13.5px;margin-top:6px">
                    "Soit ε > 0. Prenons n₀ ∈ ℕ tel que n₀ ≥ √(3/ε), et soit n ≥ n₀. |uₙ − 2| = 3/(2n² + 2) ≤ 3/(2n₀² + 2) ≤ 3/(2·(3/ε) + 2) ≤ 3ε/6 < ε."
                </span>
                <p>
                    "Trois réflexes\u{a0}: calculer "
                    <span class="fi">
                        "|uₙ − ℓ|"
                    </span>
                    " et le simplifier en une seule fraction\u{a0}; majorer en remplaçant n par n₀\u{a0}; choisir n₀ en fonction de ε "
                    <em>
                        "et l'écrire"
                    </em>
                    ". C'est l'exhibition de n₀ qui est notée."
                </p>
            </div>
        </section>
    }
}

fn sec_sf_notes() -> impl IntoView {
    view! {
        <section id="sf-notes">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Le niveau réel au partiel"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Distribution des notes du partiel 2025 sur l'ensemble de la promotion\u{a0}: 353 inscrits, 49 absents, 304 copies notées. Les numéros d'étudiants du relevé d'origine ne sont pas repris ici."
                </p>
            </div>
            <div class="kpis" style="margin-top:18px;margin-bottom:22px">
                <div class="kpi">
                    <span class="v">
                        "4,1"
                    </span>
                    <span class="k">
                        "moyenne / 20"
                    </span>
                </div>
                <div class="kpi">
                    <span class="v">
                        "2,5"
                    </span>
                    <span class="k">
                        "médiane"
                    </span>
                </div>
                <div class="kpi">
                    <span class="v">
                        "32"
                    </span>
                    <span class="k">
                        "copies ≥ 10"
                    </span>
                </div>
                <div class="kpi">
                    <span class="v">
                        "304"
                    </span>
                    <span class="k">
                        "copies notées"
                    </span>
                </div>
            </div>
            <h4 style="margin-top:0">
                "Répartition des 304 copies notées"
            </h4>
            <div class="hist" role="img" aria-label="Histogramme : 111 copies entre 0 et 2, 79 entre 2 et 4, 37 entre 4 et 6, 26 entre 6 et 8, 19 entre 8 et 10, 14 entre 10 et 12, 3 entre 12 et 14, 5 entre 14 et 16, 3 entre 16 et 18, 7 entre 18 et 20">
                <div class="row">
                    <span class="lab">
                        "0 – 2"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:100%"></span>
                    </span>
                    <span class="n">
                        "111"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "2 – 4"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:71.2%"></span>
                    </span>
                    <span class="n">
                        "79"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "4 – 6"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:33.3%"></span>
                    </span>
                    <span class="n">
                        "37"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "6 – 8"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:23.4%"></span>
                    </span>
                    <span class="n">
                        "26"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "8 – 10"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:17.1%"></span>
                    </span>
                    <span class="n">
                        "19"
                    </span>
                </div>
                <div class="cut">
                    <span class="tagline">
                        "10 / 20"
                    </span>
                    <span class="line"></span>
                    <span></span>
                </div>
                <div class="row">
                    <span class="lab">
                        "10 – 12"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:12.6%"></span>
                    </span>
                    <span class="n">
                        "14"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "12 – 14"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:2.7%"></span>
                    </span>
                    <span class="n">
                        "3"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "14 – 16"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:4.5%"></span>
                    </span>
                    <span class="n">
                        "5"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "16 – 18"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:2.7%"></span>
                    </span>
                    <span class="n">
                        "3"
                    </span>
                </div>
                <div class="row">
                    <span class="lab">
                        "18 – 20"
                    </span>
                    <span class="track">
                        <span class="bar" style="width:6.3%"></span>
                    </span>
                    <span class="n">
                        "7"
                    </span>
                </div>
            </div>
            <div class="body" style="margin-top:20px">
                <p>
                    "Deux enseignements. D'abord, "
                    <strong>
                        "une copie sur dix atteint la moyenne"
                    </strong>
                    "\u{a0}: le partiel n'est pas un examen de vérification, et le barème sur 26 à 28 points est là pour compenser. Ensuite, la distribution est "
                    <strong>
                        "bimodale"
                    </strong>
                    "\u{a0}: un gros bloc sous 4, et un petit groupe au-dessus de 18. Ceux qui réussissent ne réussissent pas «\u{a0}un peu\u{a0}» — ils ont travaillé les mêmes exercices de TD que ceux qui tombent au partiel."
                </p>
            </div>
            <div class="note" style="margin-top:18px">
                <span class="title">
                    "Le calcul de la note finale de l'UE"
                </span>
                <span class="mono" style="display:block;white-space:pre-wrap;font-size:13.5px">
                    "si WIMS − E < 10 : note = (WIMS + 4·max(E, (E+P)/2)) / 5 sinon : note = max(E, (E+P)/2)"
                </span>
                <p>
                    "E = examen, P = partiel, WIMS = moyenne de 3 contrôles en ligne. Les trois contrôles WIMS sont "
                    <strong>
                        "obligatoires"
                    </strong>
                    "\u{a0}: en manquer un rend défaillant au module, sans note, avec obligation de passer la seconde session. Volume\u{a0}: 20 h de cours, 28 h de TD, 6 ECTS en licence de maths et 5 en licence d'informatique."
                </p>
            </div>
        </section>
    }
}

fn sec_sf_outils() -> impl IntoView {
    view! {
        <section id="sf-outils">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Boîte à outils"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Les six gestes qui reviennent dans presque tous les exercices, rassemblés."
                </p>
            </div>
            <h3>
                "Coefficients de Bezout par l'algorithme d'Euclide"
            </h3>
            <div class="ex">
                <div class="ex-head">
                    <span>
                        "Exemple — 124u + 47v = 1"
                    </span>
                    <span class="mono">
                        "examen 2025"
                    </span>
                </div>
                <div class="ex-body">
                    <span class="mono">
                        "124 = 2 × 47 + 30        30 = 124 − 2×47\n 47 = 1 × 30 + 17        17 = 47 − 30\n 30 = 1 × 17 + 13        13 = 30 − 17\n 17 = 1 × 13 + 4          4 = 17 − 13\n 13 = 3 ×  4 + 1          1 = 13 − 3×4     ← dernier reste non nul\n\nOn remonte :\n1 = 13 − 3(17 − 13) = 4×13 − 3×17\n  = 4(30 − 17) − 3×17 = 4×30 − 7×17\n  = 4×30 − 7(47 − 30) = 11×30 − 7×47\n  = 11(124 − 2×47) − 7×47 = "
                        <span class="res">
                            "11×124 − 29×47"
                        </span>
                    </span>
                    <p style="margin-top:10px">
                        "Donc "
                        <span class="fi">
                            "(u₀, v₀) = (11, −29)"
                        </span>
                        ". Pour "
                        <span class="fi">
                            "124x + 47y = 10 000"
                        </span>
                        ", on multiplie par 10 000 et on ajoute la solution générale de l'équation homogène\u{a0}: "
                        <span class="fi">
                            "x = 110 000 + 47k"
                        </span>
                        ", "
                        <span class="fi">
                            "y = −290 000 − 124k"
                        </span>
                        "."
                    </p>
                </div>
            </div>
            <h3>
                "Résoudre x² + bx + c = 0 dans ℤ/nℤ avec n = pq"
            </h3>
            <ol class="steps">
                <li>
                    "Factoriser modulo n\u{a0}: trouver α, β avec "
                    <span class="fi">
                        "(x+α)(x+β) ≡ x² + bx + c"
                    </span>
                    " — vérifier que "
                    <span class="fi">
                        "α + β ≡ b"
                    </span>
                    " et "
                    <span class="fi">
                        "αβ ≡ c [n]"
                    </span>
                    "."
                </li>
                <li>
                    "z̄ est solution ⇔ "
                    <span class="fi">
                        "n | (z+α)(z+β)"
                    </span>
                    ". Comme "
                    <span class="fi">
                        "n = pq"
                    </span>
                    ", quatre cas\u{a0}: n | z+α\u{a0}; n | z+β\u{a0}; p | z+α et q | z+β\u{a0}; q | z+α et p | z+β."
                </li>
                <li>
                    "Les deux premiers cas donnent directement "
                    <span class="fi">
                        "z ≡ −α"
                    </span>
                    " et "
                    <span class="fi">
                        "z ≡ −β"
                    </span>
                    "."
                </li>
                <li>
                    "Les deux cas croisés se ramènent à une équation de Bezout "
                    <span class="fi">
                        "pu − qv = β − α"
                    </span>
                    "\u{a0}: on en tire une solution particulière puis la famille complète."
                </li>
                <li>
                    "Ramener chaque famille dans "
                    <span class="fi">
                        "{0, …, n−1}"
                    </span>
                    "."
                </li>
            </ol>
            <div class="body" style="margin-top:12px">
                <p>
                    "Résultats des deux annales\u{a0}: "
                    <span class="fi">
                        "x² + 5x + 41 = 0"
                    </span>
                    " dans ℤ/77ℤ a pour solutions "
                    <span class="fi">
                        "{4, 26, 46, 68}"
                    </span>
                    "\u{a0}; "
                    <span class="fi">
                        "x² + 18x + 10 = 0"
                    </span>
                    " dans ℤ/35ℤ donne les classes "
                    <span class="fi">
                        "z ≡ 25"
                    </span>
                    " et "
                    <span class="fi">
                        "z ≡ 27"
                    </span>
                    ", en plus de "
                    <span class="fi">
                        "−3"
                    </span>
                    " et "
                    <span class="fi">
                        "−15"
                    </span>
                    "."
                </p>
            </div>
            <h3>
                "Les réflexes de rédaction"
            </h3>
            <div class="tw" style="max-width:68ch">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Pour prouver…"
                            </th>
                            <th>
                                "On commence par écrire…"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">
                                "∀x ∈ E, P(x)"
                            </td>
                            <td>
                                "«\u{a0}Soit x ∈ E.\u{a0}» puis on prouve P(x) pour ce x fixé."
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "∃x ∈ E, P(x)"
                            </td>
                            <td>
                                "«\u{a0}Vérifions que x = … convient.\u{a0}» Le brouillon cherche, la copie vérifie."
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "P ⇒ Q"
                            </td>
                            <td>
                                "«\u{a0}Supposons P.\u{a0}»"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "P ⇔ Q"
                            </td>
                            <td>
                                "«\u{a0}Montrons P ⇒ Q.\u{a0}» puis «\u{a0}Réciproquement, supposons Q.\u{a0}»"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "A = B"
                            </td>
                            <td>
                                "«\u{a0}Soit x ∈ A.\u{a0}» → A ⊂ B, puis l'inclusion inverse."
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "f bijective"
                            </td>
                            <td>
                                "«\u{a0}Montrons que f est injective. Soient x, y tels que f(x) = f(y).\u{a0}»"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "une assertion fausse"
                            </td>
                            <td>
                                "«\u{a0}Prenons le contre-exemple suivant…\u{a0}»"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note warn" style="margin-top:18px">
                <span class="title">
                    "Ce qui coûte des points sans erreur de maths"
                </span>
                <p>
                    "Une réponse sans phrase, un résultat sans justification, des quantificateurs utilisés comme abréviations au milieu du français, une copie qui sert de brouillon. Les consignes de correction du rattrapage sont explicites\u{a0}: la même réponse vaut 1,5 point sans justification et 3 points bien rédigée."
                </p>
            </div>
        </section>
    }
}

/// Page complète de la matière.
pub fn page() -> impl IntoView {
    view! {
        <div class="course c-sf">
            {masthead()}
            <div class="shell">
                <Toc course=Course::Sf items=TOC label=TOC_LABEL/>
                <main>
                    <>
                        {sec_sf_logique()}
                        {sec_sf_ensembles()}
                        {sec_sf_applications()}
                        {sec_sf_denombrement()}
                        {sec_sf_nombres()}
                        {sec_sf_complexes()}
                        {sec_sf_algebre()}
                        {sec_sf_td1()}
                        {sec_sf_td2()}
                        {sec_sf_td3()}
                        {sec_sf_td4()}
                        {sec_sf_recurrent()}
                    </>
                    <>
                        {sec_sf_annales()}
                        {sec_sf_notes()}
                        {sec_sf_outils()}
                    </>
                </main>
            </div>
        </div>
    }
}
