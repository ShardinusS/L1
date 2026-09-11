//! Matiere 02 - Methodes et techniques de calcul.
//!
//! Ecrit a partir du cours de Fabien Durand (UPJV, L1 informatique,
//! 2026-2027) et des trois feuilles de TD : suites, recurrence, fonctions,
//! limites, continuite, derivabilite, primitives et integrales.

use leptos::prelude::*;

use crate::components::toc::{Toc, TocItem};
use crate::course::Course;
use crate::search::IndexEntry;

/// Ancres des sections de la page, dans l'ordre d'affichage.
#[rustfmt::skip]
pub const SECTIONS: &[&str] = &[
    "m-suites",
    "m-arithgeo",
    "m-recurrence",
    "m-fonctions",
    "m-limites",
    "m-continuite",
    "m-usuelles",
    "m-derivees",
    "m-primitives",
    "m-td1",
    "m-td2",
    "m-td3",
    "m-eval",
    "m-outils",
];

/// Sommaire de la matière : (ancre, numéro, titre).
#[rustfmt::skip]
pub const TOC: &[TocItem] = &[
    TocItem { anchor: "m-suites", num: "01", title: "Suites et limites" },
    TocItem { anchor: "m-arithgeo", num: "02", title: "Suites arithmétiques et géométriques" },
    TocItem { anchor: "m-recurrence", num: "03", title: "Raisonnement par récurrence" },
    TocItem { anchor: "m-fonctions", num: "04", title: "Fonctions et domaine de définition" },
    TocItem { anchor: "m-limites", num: "05", title: "Limites de fonctions et asymptotes" },
    TocItem { anchor: "m-continuite", num: "06", title: "Continuité" },
    TocItem { anchor: "m-usuelles", num: "07", title: "Les fonctions usuelles" },
    TocItem { anchor: "m-derivees", num: "08", title: "Dérivabilité et variations" },
    TocItem { anchor: "m-primitives", num: "09", title: "Primitives et intégrales" },
    TocItem { anchor: "m-td1", num: "TD1", title: "Suites et récurrence" },
    TocItem { anchor: "m-td2", num: "TD2", title: "Représentations graphiques" },
    TocItem { anchor: "m-td3", num: "TD3", title: "Fonctions, dérivées, primitives" },
    TocItem { anchor: "m-eval", num: "—", title: "Évaluation et note finale" },
    TocItem { anchor: "m-outils", num: "—", title: "Formulaire" },
];

/// Libellé accessible du sommaire.
pub const TOC_LABEL: &str = "Sommaire de Méthodes et techniques de calcul";

/// Entrées de recherche de la matière, dans l'ordre du cours.
#[rustfmt::skip]
pub const INDEX: &[IndexEntry] = &[
    IndexEntry::new(Course::Mtc, "m-suites", "Suites et limites", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-suites", "Suite numérique", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Limite d'une suite", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Limite infinie", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Opérations sur les limites", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Formes indéterminées", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Théorème des gendarmes", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Théorème de comparaison", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Passage à la limite dans une inégalité", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Suite croissante, suite décroissante", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Théorème de la limite monotone", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Suite de Cauchy", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-suites", "Méthode — prouver une limite avec ε", "Suites et limites"),
    IndexEntry::new(Course::Mtc, "m-arithgeo", "Suites arithmétiques et géométriques", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-arithgeo", "Suite arithmétique", "Suites arithmétiques et géométriques"),
    IndexEntry::new(Course::Mtc, "m-arithgeo", "Suite géométrique", "Suites arithmétiques et géométriques"),
    IndexEntry::new(Course::Mtc, "m-arithgeo", "Somme des termes d'une suite arithmétique", "Suites arithmétiques et géométriques"),
    IndexEntry::new(Course::Mtc, "m-arithgeo", "Somme des termes d'une suite géométrique", "Suites arithmétiques et géométriques"),
    IndexEntry::new(Course::Mtc, "m-arithgeo", "Sommes classiques", "Suites arithmétiques et géométriques"),
    IndexEntry::new(Course::Mtc, "m-arithgeo", "Suites auxiliaires vₙ = uₙ + b", "Suites arithmétiques et géométriques"),
    IndexEntry::new(Course::Mtc, "m-recurrence", "Raisonnement par récurrence", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-recurrence", "Principe de récurrence", "Raisonnement par récurrence"),
    IndexEntry::new(Course::Mtc, "m-recurrence", "Initialisation et hérédité", "Raisonnement par récurrence"),
    IndexEntry::new(Course::Mtc, "m-recurrence", "Méthode de rédaction en cinq temps", "Raisonnement par récurrence"),
    IndexEntry::new(Course::Mtc, "m-recurrence", "Erreurs fréquentes", "Raisonnement par récurrence"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Fonctions et domaine de définition", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Les quatre données d'une fonction", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Domaine de définition", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Les trois expressions qui posent problème", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Fonctions composées", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Images et antécédents", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Injectivité, surjectivité, bijectivité", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Graphe d'une fonction", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-fonctions", "Parité", "Fonctions et domaine de définition"),
    IndexEntry::new(Course::Mtc, "m-limites", "Limites de fonctions et asymptotes", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-limites", "Limite en un point", "Limites de fonctions et asymptotes"),
    IndexEntry::new(Course::Mtc, "m-limites", "Limite à gauche, limite à droite", "Limites de fonctions et asymptotes"),
    IndexEntry::new(Course::Mtc, "m-limites", "Limites en l'infini", "Limites de fonctions et asymptotes"),
    IndexEntry::new(Course::Mtc, "m-limites", "Opérations sur les limites de fonctions", "Limites de fonctions et asymptotes"),
    IndexEntry::new(Course::Mtc, "m-limites", "Asymptote verticale", "Limites de fonctions et asymptotes"),
    IndexEntry::new(Course::Mtc, "m-limites", "Asymptote horizontale", "Limites de fonctions et asymptotes"),
    IndexEntry::new(Course::Mtc, "m-limites", "Asymptote oblique", "Limites de fonctions et asymptotes"),
    IndexEntry::new(Course::Mtc, "m-continuite", "Continuité", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-continuite", "Fonction continue en un point", "Continuité"),
    IndexEntry::new(Course::Mtc, "m-continuite", "Caractérisation séquentielle", "Continuité"),
    IndexEntry::new(Course::Mtc, "m-continuite", "Opérations sur les fonctions continues", "Continuité"),
    IndexEntry::new(Course::Mtc, "m-continuite", "Continuité de la bijection réciproque", "Continuité"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Les fonctions usuelles", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Fonctions affines", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Logarithme népérien", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Logarithme en base b", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Fonction exponentielle", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Exponentielle de base α", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Fonctions puissances", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Croissances comparées", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Fonctions trigonométriques", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-usuelles", "Valeurs remarquables du cercle", "Les fonctions usuelles"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Dérivabilité et variations", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Taux d'accroissement", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Nombre dérivé", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Équation de la tangente", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Dérivées des fonctions usuelles", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Règles de dérivation", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Dérivée d'une composée", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Dérivée et sens de variation", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-derivees", "Méthode — l'étude de fonction complète", "Dérivabilité et variations"),
    IndexEntry::new(Course::Mtc, "m-primitives", "Primitives et intégrales", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-primitives", "Primitive d'une fonction", "Primitives et intégrales"),
    IndexEntry::new(Course::Mtc, "m-primitives", "Primitives usuelles", "Primitives et intégrales"),
    IndexEntry::new(Course::Mtc, "m-primitives", "Intégration par parties", "Primitives et intégrales"),
    IndexEntry::new(Course::Mtc, "m-primitives", "Changement de variable", "Primitives et intégrales"),
    IndexEntry::new(Course::Mtc, "m-primitives", "Primitives de fractions rationnelles", "Primitives et intégrales"),
    IndexEntry::new(Course::Mtc, "m-primitives", "Fractions rationnelles trigonométriques", "Primitives et intégrales"),
    IndexEntry::new(Course::Mtc, "m-td1", "Suites et récurrence", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-td1", "Modéliser une situation par une suite.", "Suites et récurrence · ex. 1-4"),
    IndexEntry::new(Course::Mtc, "m-td1", "La batterie de limites.", "Suites et récurrence · ex. 5-6"),
    IndexEntry::new(Course::Mtc, "m-td1", "Les formes indéterminées en √(n²+an) − n.", "Suites et récurrence · ex. 7-8"),
    IndexEntry::new(Course::Mtc, "m-td1", "Sommes géométriques et nombres rationnels.", "Suites et récurrence · ex. 9-10"),
    IndexEntry::new(Course::Mtc, "m-td1", "Suites auxiliaires.", "Suites et récurrence · ex. 11-13"),
    IndexEntry::new(Course::Mtc, "m-td1", "Séries de référence.", "Suites et récurrence · ex. 14-16"),
    IndexEntry::new(Course::Mtc, "m-td1", "Les six récurrences de base.", "Suites et récurrence · ex. 17-19"),
    IndexEntry::new(Course::Mtc, "m-td1", "Conjecturer puis démontrer.", "Suites et récurrence · ex. 20-24"),
    IndexEntry::new(Course::Mtc, "m-td1", "Croissante et majorée donc convergente.", "Suites et récurrence · ex. 25-26"),
    IndexEntry::new(Course::Mtc, "m-td2", "Représentations graphiques", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-td2", "Lire une équation, une inéquation, un tableau de signes.", "Représentations graphiques · ex. 1-7"),
    IndexEntry::new(Course::Mtc, "m-td2", "Parité et antécédents sur une courbe.", "Représentations graphiques · ex. 8"),
    IndexEntry::new(Course::Mtc, "m-td2", "Lire un graphique de situation.", "Représentations graphiques · ex. 9-13"),
    IndexEntry::new(Course::Mtc, "m-td3", "Fonctions, dérivées, primitives", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-td3", "Douze domaines de définition.", "Fonctions, dérivées, primitives · ex. 1"),
    IndexEntry::new(Course::Mtc, "m-td3", "Mettre un problème en fonction.", "Fonctions, dérivées, primitives · ex. 2-7"),
    IndexEntry::new(Course::Mtc, "m-td3", "Identités trigonométriques et équations.", "Fonctions, dérivées, primitives · ex. 8-12"),
    IndexEntry::new(Course::Mtc, "m-td3", "Simplifier, résoudre avec ln et exp.", "Fonctions, dérivées, primitives · ex. 13-14"),
    IndexEntry::new(Course::Mtc, "m-td3", "Les modèles exponentiels appliqués.", "Fonctions, dérivées, primitives · ex. 15-19"),
    IndexEntry::new(Course::Mtc, "m-td3", "Quinze dérivées à calculer.", "Fonctions, dérivées, primitives · ex. 20"),
    IndexEntry::new(Course::Mtc, "m-td3", "Primitives et intégrales.", "Fonctions, dérivées, primitives · ex. 21-22"),
    IndexEntry::new(Course::Mtc, "m-td3", "Onze études de fonctions complètes.", "Fonctions, dérivées, primitives · ex. 23-33"),
    IndexEntry::new(Course::Mtc, "m-eval", "Évaluation et note finale", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-eval", "Le calcul de la note finale", "Évaluation et note finale"),
    IndexEntry::new(Course::Mtc, "m-eval", "Ce qui est attendu à l'examen", "Évaluation et note finale"),
    IndexEntry::new(Course::Mtc, "m-outils", "Formulaire", "Méthodes et techniques de calcul"),
    IndexEntry::new(Course::Mtc, "m-outils", "Les limites de référence", "Formulaire"),
    IndexEntry::new(Course::Mtc, "m-outils", "Dérivées et primitives en vis-à-vis", "Formulaire"),
    IndexEntry::new(Course::Mtc, "m-outils", "Les réflexes qui font gagner du temps", "Formulaire"),
];

fn masthead() -> impl IntoView {
    view! {
        <header class="masthead">
            <div class="masthead-inner">
                <div class="eyebrow">"Matière 02 · Mathématiques"</div>
                <h1>"Méthodes et techniques de calcul"</h1>
                <p class="lede">
                    "L'outillage de calcul qui sert dans toutes les autres matières\u{a0}: suites et limites, récurrence, fonctions usuelles, dérivées, primitives et intégrales. Peu de théorie, beaucoup de gestes à automatiser."
                </p>
                <div class="meta">
                    <span>"Cours de Fabien Durand · UPJV"</span>
                    <span>"L1 informatique · 2026-2027"</span>
                    <span><span class="mono">"9"</span>" chapitres"</span>
                    <span><span class="mono">"3"</span>" TD"</span>
                    <span><span class="mono">"8 h"</span>" de CM, "<span class="mono">"16 h"</span>" de TD"</span>
                </div>
            </div>
        </header>
    }
}

fn sec_m_suites() -> impl IntoView {
    view! {
        <section id="m-suites">
            <div class="sec-head">
                <span class="num">"01"</span>
                <h2>"Suites et limites"</h2>
            </div>
            <div class="body">
                <p>
                    "Une suite n'est rien d'autre qu'une fonction de ℕ dans ℝ. Ce qui intéresse le cours, ce ne sont jamais les premiers termes\u{a0}: c'est le comportement quand n devient grand."
                </p>
            </div>
            <dl class="deflist def" style="margin-top:16px">
                <div>
                    <dt>"Suite numérique"</dt>
                    <dd>
                        "Une fonction "<span class="fi">"u : ℕ → ℝ"</span>". On pose "
                        <span class="fi">"uₙ = u(n)"</span>" et on note la suite "
                        <span class="fi">"(uₙ)ₙ∈ℕ"</span>", ou "<span class="fi">"(uₙ)ₙ≥ₙ₀"</span>
                        " quand elle ne démarre pas à 0."
                    </dd>
                </div>
                <div>
                    <dt>"Limite finie"</dt>
                    <dd>
                        "(uₙ) tend vers ℓ si\u{a0}: pour tout ε > 0, il existe n₀ tel que n ≥ n₀ entraîne |uₙ − ℓ| < ε. Avec les quantificateurs\u{a0}: "
                        <span class="fi">"(∀ε > 0)(∃n₀)(∀n ≥ n₀, |uₙ − ℓ| < ε)"</span>
                        ". On dit alors que (uₙ) "<em>"converge"</em>
                        " vers ℓ. Une suite qui ne converge pas est dite "<em>"divergente"</em>"."
                    </dd>
                </div>
                <div>
                    <dt>"Limite infinie"</dt>
                    <dd>
                        <span class="fi">"lim uₙ = +∞"</span>
                        " si pour tout K il existe n₀ tel que n ≥ n₀ entraîne uₙ ≥ K. Idem pour −∞ avec uₙ ≤ K. Autrement dit\u{a0}: (uₙ) est aussi grand que l'on veut à partir d'un certain rang."
                    </dd>
                </div>
                <div>
                    <dt>"Suite de Cauchy"</dt>
                    <dd>
                        "Pour tout ε > 0 il existe n₀ tel que p, q ≥ n₀ entraîne |u_p − u_q| < ε. "
                        <strong>"Théorème\u{a0}: une suite converge si, et seulement si, elle est de Cauchy."</strong>
                        " C'est l'outil pour montrer qu'une suite "<em>"ne"</em>
                        " converge pas sans en connaître la limite — la série harmonique 1 + 1/2 + … + 1/n en est l'exemple du cours."
                    </dd>
                </div>
            </dl>
            <div class="note method" style="margin-top:20px">
                <span class="title">"Méthode — prouver une limite avec ε"</span>
                <p>
                    "La question tombe sous cette forme quasiment à chaque épreuve de maths de L1. Le barème ne porte pas sur la valeur de la limite mais sur l'exhibition de n₀."
                </p>
                <ol class="steps">
                    <li>"Calculer |uₙ − ℓ| et le réduire à "<em>"une seule"</em>" fraction."</li>
                    <li>"Majorer cette fraction en remplaçant n par n₀ (licite car n ≥ n₀)."</li>
                    <li>"Choisir n₀ en fonction de ε — et l'écrire noir sur blanc."</li>
                    <li>"Conclure\u{a0}: « donc pour tout n ≥ n₀, |uₙ − ℓ| < ε »."</li>
                </ol>
                <p>
                    "La même méthode sert en "
                    <a href="/structures-fondamentales#sf-recurrent">"Structures fondamentales"</a>
                    ", où elle est tombée quatre fois sur cinq."
                </p>
            </div>
            <h3 style="margin-top:24px">"Opérations sur les limites"</h3>
            <div class="tw" style="max-width:none;margin-top:10px">
                <table>
                    <thead>
                        <tr>
                            <th>"lim uₙ"</th>
                            <th>"lim vₙ"</th>
                            <th>"uₙ + vₙ"</th>
                            <th>"uₙ · vₙ"</th>
                            <th>"uₙ / vₙ"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">"ℓ"</td>
                            <td class="mono">"m ≠ 0"</td>
                            <td class="mono">"ℓ + m"</td>
                            <td class="mono">"ℓ·m"</td>
                            <td class="mono">"ℓ/m"</td>
                        </tr>
                        <tr>
                            <td class="mono">"ℓ ≠ 0"</td>
                            <td class="mono">"0"</td>
                            <td class="mono">"ℓ"</td>
                            <td class="mono">"0"</td>
                            <td class="mono">"±∞ selon le signe"</td>
                        </tr>
                        <tr>
                            <td class="mono">"0"</td>
                            <td class="mono">"0"</td>
                            <td class="mono">"0"</td>
                            <td class="mono">"0"</td>
                            <td><span class="tag hot">"F.I."</span></td>
                        </tr>
                        <tr>
                            <td class="mono">"+∞"</td>
                            <td class="mono">"+∞"</td>
                            <td class="mono">"+∞"</td>
                            <td class="mono">"+∞"</td>
                            <td><span class="tag hot">"F.I."</span></td>
                        </tr>
                        <tr>
                            <td class="mono">"+∞"</td>
                            <td class="mono">"−∞"</td>
                            <td><span class="tag hot">"F.I."</span></td>
                            <td class="mono">"−∞"</td>
                            <td><span class="tag hot">"F.I."</span></td>
                        </tr>
                        <tr>
                            <td class="mono">"±∞"</td>
                            <td class="mono">"0"</td>
                            <td class="mono">"±∞"</td>
                            <td><span class="tag hot">"F.I."</span></td>
                            <td class="mono">"±∞"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note warn" style="margin-top:16px">
                <span class="title">"Les quatre formes indéterminées"</span>
                <p>
                    <span class="fi">"∞ − ∞"</span>", "<span class="fi">"0 × ∞"</span>", "
                    <span class="fi">"0/0"</span>" et "<span class="fi">"∞/∞"</span>
                    ". « Indéterminée » ne veut pas dire « pas de limite »\u{a0}: cela veut dire que les limites de uₙ et vₙ ne suffisent pas à conclure. Il faut factoriser, multiplier par la quantité conjuguée, ou comparer les croissances."
                </p>
            </div>
            <h3 style="margin-top:24px">"Les trois théorèmes de comparaison"</h3>
            <dl class="deflist def" style="margin-top:12px">
                <div>
                    <dt>"Théorème des gendarmes"</dt>
                    <dd>
                        "Si "<span class="fi">"uₙ ≤ vₙ ≤ wₙ"</span>
                        " à partir d'un certain rang et si (uₙ) et (wₙ) convergent vers la "
                        <em>"même"</em>" limite ℓ, alors (vₙ) converge vers ℓ."
                    </dd>
                </div>
                <div>
                    <dt>"Théorème de comparaison"</dt>
                    <dd>
                        "Si "<span class="fi">"uₙ ≤ vₙ"</span>
                        " à partir d'un certain rang\u{a0}: lim uₙ = +∞ entraîne lim vₙ = +∞\u{a0}; lim vₙ = −∞ entraîne lim uₙ = −∞."
                    </dd>
                </div>
                <div>
                    <dt>"Passage à la limite"</dt>
                    <dd>
                        "Si (uₙ) et (vₙ) convergent et si uₙ ≤ vₙ à partir d'un certain rang, alors lim uₙ ≤ lim vₙ. "
                        <strong>"Attention"</strong>
                        "\u{a0}: une inégalité stricte devient large au passage à la limite."
                    </dd>
                </div>
                <div>
                    <dt>"Théorème de la limite monotone"</dt>
                    <dd>
                        "Soit (uₙ) croissante. S'il existe M avec uₙ ≤ M pour tout n, alors (uₙ) converge. Sinon lim uₙ = +∞. C'est l'argument des exercices 25 et 26 du TD 1\u{a0}: on montre que la suite est majorée, qu'elle est croissante, on conclut qu'elle converge — et seulement "
                        <em>"après"</em>
                        " on cherche la limite en résolvant ℓ = f(ℓ)."
                    </dd>
                </div>
            </dl>
        </section>
    }
}

fn sec_m_arithgeo() -> impl IntoView {
    view! {
        <section id="m-arithgeo">
            <div class="sec-head">
                <span class="num">"02"</span>
                <h2>"Suites arithmétiques et géométriques"</h2>
            </div>
            <div class="body">
                <p>
                    "Deux familles, quatre formules. Elles reviennent dans chaque situation modélisée du TD 1\u{a0}: une sauvegarde qui grossit de 18 Mo par jour est arithmétique, un réseau social qui gagne 15 % par semaine est géométrique."
                </p>
            </div>
            <div class="tw" style="max-width:none;margin-top:16px">
                <table>
                    <thead>
                        <tr>
                            <th>""</th>
                            <th>"Arithmétique"</th>
                            <th>"Géométrique"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Relation de récurrence"</td>
                            <td class="mono">"uₙ₊₁ = uₙ + r"</td>
                            <td class="mono">"uₙ₊₁ = q·uₙ"</td>
                        </tr>
                        <tr>
                            <td>"Terme général"</td>
                            <td class="mono">"uₙ = u₀ + n·r"</td>
                            <td class="mono">"uₙ = u₀·qⁿ"</td>
                        </tr>
                        <tr>
                            <td>"Somme u₀ + … + uₙ"</td>
                            <td class="mono">"(n+1)(u₀ + uₙ)/2"</td>
                            <td class="mono">"u₀·(1 − qⁿ⁺¹)/(1 − q)"</td>
                        </tr>
                        <tr>
                            <td>"Cas limite"</td>
                            <td>"r = 0\u{a0}: suite constante"</td>
                            <td class="mono">"q = 1 : somme = (n+1)u₀"</td>
                        </tr>
                        <tr>
                            <td>"Reconnaître"</td>
                            <td>"la différence uₙ₊₁ − uₙ est constante"</td>
                            <td>"le quotient uₙ₊₁ / uₙ est constant"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note method" style="margin-top:18px">
                <span class="title">"Comment le cours démontre la somme arithmétique"</span>
                <p>
                    "On écrit Sₙ à l'endroit puis à l'envers et on additionne terme à terme\u{a0}: chaque paire u_k + u_{n−k} vaut u₀ + uₙ, il y en a n + 1, donc 2Sₙ = (n+1)(u₀ + uₙ). Pour la géométrique, on calcule Sₙ − q·Sₙ, tout se télescope et il reste u₀(1 − qⁿ⁺¹)."
                </p>
            </div>
            <h3 style="margin-top:24px">"Les sommes à connaître par cœur"</h3>
            <span class="f center">
"1 + 2 + ⋯ + n  =  n(n + 1)/2
1² + 2² + ⋯ + n²  =  n(n + 1)(2n + 1)/6
1³ + 2³ + ⋯ + n³  =  ( n(n + 1)/2 )²
1 + q + q² + ⋯ + qⁿ  =  (1 − qⁿ⁺¹)/(1 − q)   (q ≠ 1)"
            </span>
            <div class="note plan" style="margin-top:18px">
                <span class="title">"Suites auxiliaires — le réflexe des exercices 11 à 13"</span>
                <p>
                    "Une suite du type "<span class="fi">"uₙ₊₁ = a·uₙ + b"</span>
                    " n'est ni arithmétique ni géométrique. On la ramène à une géométrique en posant "
                    <span class="fi">"vₙ = uₙ − ℓ"</span>", où ℓ est le "<em>"point fixe"</em>
                    " solution de ℓ = a·ℓ + b, c'est-à-dire ℓ = b/(1 − a)."
                </p>
                <ol>
                    <li>"Résoudre ℓ = a·ℓ + b pour trouver le décalage."</li>
                    <li>"Poser vₙ = uₙ − ℓ et vérifier que vₙ₊₁ = a·vₙ."</li>
                    <li>"Écrire vₙ = v₀·aⁿ, puis uₙ = v₀·aⁿ + ℓ."</li>
                </ol>
                <p>
                    "Exemple du TD (ex. 11)\u{a0}: u₀ = 1, uₙ₊₁ = 2uₙ + 3. Le point fixe est −3, donc vₙ = uₙ + 3 vérifie vₙ₊₁ = 2vₙ, d'où vₙ = 4·2ⁿ et "
                    <span class="fi">"uₙ = 2ⁿ⁺² − 3"</span>"."
                </p>
            </div>
        </section>
    }
}

fn sec_m_recurrence() -> impl IntoView {
    view! {
        <section id="m-recurrence">
            <div class="sec-head">
                <span class="num">"03"</span>
                <h2>"Raisonnement par récurrence"</h2>
            </div>
            <div class="body">
                <p>
                    "Une méthode de démonstration, pas un calcul. Elle établit qu'une propriété est vraie pour tous les entiers à partir d'un certain rang. C'est le chapitre où les points se perdent sur la rédaction, jamais sur l'idée."
                </p>
            </div>
            <div class="note" style="margin-top:16px">
                <span class="title">"Principe de récurrence"</span>
                <p>
                    "Soit P(n) une propriété dépendant d'un entier naturel n. Si\u{a0}:"
                </p>
                <ul>
                    <li><strong>"Initialisation"</strong>"\u{a0}: P(n₀) est vraie pour un certain entier n₀\u{a0};"</li>
                    <li><strong>"Hérédité"</strong>"\u{a0}: pour tout n ≥ n₀, P(n) vraie entraîne P(n + 1) vraie\u{a0};"</li>
                </ul>
                <p>"alors P(n) est vraie pour tout n ≥ n₀."</p>
            </div>
            <div class="note plan" style="margin-top:18px">
                <span class="title">"La rédaction en cinq temps"</span>
                <ol>
                    <li>"Définir clairement la propriété P(n) — l'écrire, pas seulement la penser."</li>
                    <li>"Vérifier l'initialisation au rang n₀."</li>
                    <li>"Supposer P(n) vraie pour un entier n ≥ n₀ fixé. C'est l'"<em>"hypothèse de récurrence"</em>"."</li>
                    <li>"Démontrer que cette hypothèse entraîne P(n + 1). C'est l'"<em>"hérédité"</em>"."</li>
                    <li>"Conclure "<em>"en invoquant le principe de récurrence"</em>"."</li>
                </ol>
            </div>
            <div class="exos" style="margin-top:22px">
                <div class="exo">
                    <span class="n">"Ex. 1"</span>
                    <span class="c">
                        <span class="t">"1 + 2 + ⋯ + n = n(n+1)/2"</span>
                        <span class="sub">
                            <strong>"Initialisation."</strong>
                            " Pour n = 1\u{a0}: 1 = 1×2/2 = 1. "
                            <strong>"Hérédité."</strong>
                            " Si la formule vaut au rang n, alors 1 + ⋯ + n + (n+1) = n(n+1)/2 + (n+1) = (n+1)(n/2 + 1) = (n+1)(n+2)/2, qui est exactement la formule au rang n + 1."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"Ex. 2"</span>
                    <span class="c">
                        <span class="t">"2ⁿ ≥ n + 1 pour tout n ≥ 0"</span>
                        <span class="sub">
                            <strong>"Initialisation."</strong>
                            " 2⁰ = 1 = 0 + 1. "
                            <strong>"Hérédité."</strong>
                            " Si 2ⁿ ≥ n + 1 alors 2ⁿ⁺¹ = 2·2ⁿ ≥ 2(n+1) = n + 2 + n ≥ n + 2 = (n+1) + 1."
                        </span>
                    </span>
                </div>
            </div>
            <div class="note warn" style="margin-top:20px">
                <span class="title">"Les quatre erreurs qui coûtent des points"</span>
                <ul>
                    <li>"Commencer par l'hérédité sans vérifier le rang initial."</li>
                    <li>
                        "Utiliser au rang n + 1 la formule que l'on cherche justement à démontrer — c'est l'erreur la plus fréquente et elle annule tout le raisonnement."
                    </li>
                    <li>"Oublier de conclure."</li>
                    <li>"Écrire « la propriété est vraie pour tout n » sans avoir cité le principe de récurrence."</li>
                </ul>
            </div>
            <div class="note method" style="margin-top:18px">
                <span class="title">"Le schéma « conjecturer puis démontrer »"</span>
                <p>
                    "Plusieurs exercices du TD (20, 21, 22, 24) suivent le même moule\u{a0}: on donne une suite par récurrence, on demande de calculer les premiers termes, de "
                    <em>"conjecturer"</em>" une formule explicite, puis de la "
                    <em>"démontrer"</em>
                    " par récurrence. Calculer quatre ou cinq termes suffit presque toujours à voir la forme\u{a0}: puissance, factorielle, ou fraction 1/(n+1)."
                </p>
            </div>
        </section>
    }
}

fn sec_m_fonctions() -> impl IntoView {
    view! {
        <section id="m-fonctions">
            <div class="sec-head">
                <span class="num">"04"</span>
                <h2>"Fonctions et domaine de définition"</h2>
            </div>
            <div class="body">
                <p>
                    "Le chapitre reprend les notions du lycée en les nommant proprement. Le seul vrai geste nouveau est le "
                    <strong>"domaine de définition"</strong>
                    ", demandé en première question de presque tous les exercices d'étude de fonction."
                </p>
            </div>
            <div class="note" style="margin-top:16px">
                <span class="title">"Les quatre données d'une fonction"</span>
                <ol class="steps">
                    <li>"l'ensemble de départ\u{a0};"</li>
                    <li>"la variable\u{a0};"</li>
                    <li>"l'ensemble d'arrivée\u{a0};"</li>
                    <li>"la règle de calcul qui donne f(x) à partir de x."</li>
                </ol>
                <p>
                    "Lorsqu'elle existe, f(x) est l'"<em>"image"</em>" de x par f."
                </p>
            </div>
            <h3 style="margin-top:24px">"Domaine de définition"</h3>
            <div class="body">
                <p>
                    "Les valeurs pour lesquelles le calcul peut être mené forment le domaine de définition, noté "
                    <span class="fi">"D_f"</span>". Trois types d'expressions posent problème, et seulement trois."
                </p>
            </div>
            <div class="tw" style="max-width:none;margin-top:12px">
                <table>
                    <thead>
                        <tr>
                            <th>"Expression"</th>
                            <th>"Condition"</th>
                            <th>"Ce qu'on écrit"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">"A / B"</td>
                            <td>"le dénominateur ne s'annule pas"</td>
                            <td class="mono">"B ≠ 0"</td>
                        </tr>
                        <tr>
                            <td class="mono">"√A  ou  A^α, α ∉ ℤ"</td>
                            <td>"seuls les réels positifs ou nuls ont une racine"</td>
                            <td class="mono">"A ≥ 0"</td>
                        </tr>
                        <tr>
                            <td class="mono">"ln(A)"</td>
                            <td>"seuls les réels strictement positifs ont un logarithme"</td>
                            <td class="mono">"A > 0"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="ex" style="margin-top:18px">
                <div class="ex-head">
                    <span>"Les deux exemples du cours"</span>
                    <span class="cat">"Domaine"</span>
                </div>
                <div class="ex-body">
                    <p>
                        <span class="fi">"f(x) = (x − 1)/(x − 3/2)"</span>
                        " — seule la division pose problème, donc "
                        <span class="res">"D_f = ℝ \\ {3/2} = ]−∞, 3/2[ ∪ ]3/2, +∞["</span>"."
                    </p>
                    <p>
                        <span class="fi">"f(x) = √(x − 1)/(x − 3/2)"</span>
                        " — il faut "<em>"et"</em>" x − 1 ≥ 0 "<em>"et"</em>" x ≠ 3/2, donc "
                        <span class="res">"D_f = [1, 3/2[ ∪ ]3/2, +∞["</span>"."
                    </p>
                </div>
            </div>
            <dl class="deflist def" style="margin-top:22px">
                <div>
                    <dt>"Composée"</dt>
                    <dd>
                        <span class="fi">"(g ∘ f)(x) = g(f(x))"</span>
                        ". Si f : x ↦ x³ et g : x ↦ cos(x), alors g ∘ f : x ↦ cos(x³). Si f : x ↦ |x| et g : x ↦ √x, alors g ∘ f : x ↦ |x|."
                    </dd>
                </div>
                <div>
                    <dt>"Image et antécédent"</dt>
                    <dd>
                        "f(x) est l'image de x. Tout x tel que f(x) = y est un antécédent de y. "
                        <strong>"L'image est unique, l'antécédent ne l'est pas\u{a0}:"</strong>
                        " pour f : x ↦ x², l'image de −3 est 9, mais 16 a deux antécédents, 4 et −4."
                    </dd>
                </div>
                <div>
                    <dt>"Injective, surjective, bijective"</dt>
                    <dd>
                        "f est injective sur A si x₁ ≠ x₂ entraîne f(x₁) ≠ f(x₂)\u{a0}; surjective sur B si tout y de B a un antécédent\u{a0}; bijective de A dans B si elle est injective sur A et si tout y de B a un antécédent "
                        <em>"dans A"</em>
                        " — cet antécédent est alors unique. Par définition, f est toujours surjective sur f(A)."
                    </dd>
                </div>
                <div>
                    <dt>"Graphe"</dt>
                    <dd>
                        <span class="fi">"Gr(f) = { (a, f(a)) | a ∈ D_f }"</span>
                        ". Une courbe qui repasse deux fois au-dessus d'une même abscisse n'est pas un graphe de fonction."
                    </dd>
                </div>
                <div>
                    <dt>"Parité"</dt>
                    <dd>
                        "f est "<strong>"paire"</strong>
                        " si D_f est symétrique par rapport à 0 et f(−x) = f(x) — le graphe est symétrique par rapport à l'axe Oy. Elle est "
                        <strong>"impaire"</strong>
                        " si f(−x) = −f(x) — le graphe est symétrique par rapport à l'origine."
                    </dd>
                </div>
            </dl>
            <div class="note method" style="margin-top:18px">
                <span class="title">"Le théorème qui sert le plus"</span>
                <p>
                    "Si f : I → ℝ est "<strong>"strictement monotone"</strong>
                    ", alors f est une bijection de I sur f(I). C'est ce qui permet de justifier qu'une fonction admet une réciproque sans la calculer."
                </p>
                <p>
                    "Exemple du cours\u{a0}: x ↦ x² n'est ni injective ni surjective sur ℝ, mais elle est bijective de ℝ₊ dans ℝ₊, et aussi de ℝ₋ dans ℝ₊."
                </p>
            </div>
            <div class="note plan" style="margin-top:16px">
                <span class="title">"Le même vocabulaire qu'en Structures fondamentales"</span>
                <p>
                    "Injection, surjection, bijection, composition\u{a0}: ce sont exactement les notions du chapitre 3 de "
                    <a href="/structures-fondamentales#sf-applications">"Structures fondamentales"</a>
                    ". Ici on les applique à des fonctions numériques concrètes, là-bas on les démontre sur des ensembles abstraits. Réviser les deux ensemble fait gagner du temps."
                </p>
            </div>
        </section>
    }
}

fn sec_m_limites() -> impl IntoView {
    view! {
        <section id="m-limites">
            <div class="sec-head">
                <span class="num">"05"</span>
                <h2>"Limites de fonctions et asymptotes"</h2>
            </div>
            <div class="body">
                <p>
                    "Même idée que pour les suites, mais x peut tendre vers un réel, et par la gauche ou par la droite. Le cours donne la définition formelle avec ε et δ mais précise qu'"
                    <em>"on ne la manipulera pas"</em>" : ce qui est évalué, c'est le calcul."
                </p>
            </div>
            <dl class="deflist def" style="margin-top:16px">
                <div>
                    <dt>"Limite en un point"</dt>
                    <dd>
                        <span class="fi">"lim_{x→x₀} f(x) = ℓ"</span>
                        "\u{a0}: pour tout ε > 0 il existe δ > 0 tel que |x − x₀| < δ entraîne |f(x) − ℓ| < ε."
                    </dd>
                </div>
                <div>
                    <dt>"Limite à droite, limite à gauche"</dt>
                    <dd>
                        <span class="fi">"lim_{x→x₀⁺}"</span>" par valeurs supérieures, "
                        <span class="fi">"lim_{x→x₀⁻}"</span>" par valeurs inférieures. "
                        <strong>"Critère\u{a0}:"</strong>
                        " f admet une limite ℓ en x₀ si, et seulement si, les limites à droite et à gauche existent et valent toutes deux ℓ."
                    </dd>
                </div>
                <div>
                    <dt>"Limite en l'infini"</dt>
                    <dd>
                        "f(x) tend vers ℓ quand x → +∞ si pour tout ε > 0 il existe r tel que x > r entraîne |f(x) − ℓ| < ε. Analogue en −∞ avec x < r."
                    </dd>
                </div>
            </dl>
            <div class="ex" style="margin-top:20px">
                <div class="ex-head">
                    <span>"Les trois exemples du cours"</span>
                    <span class="cat">"Limites"</span>
                </div>
                <div class="ex-body">
                    <p>
                        <strong>"1. "</strong><span class="fi">"f(x) = (x² + 3)/(x − 2)"</span>
                        ", D_f = ℝ \\ {2}. En ±∞ on garde les termes dominants\u{a0}: f(x) ~ x²/x = x, donc "
                        <span class="res">"+∞ en +∞"</span>" et "<span class="res">"−∞ en −∞"</span>
                        ". En 2\u{a0}: le numérateur tend vers 7, le dénominateur vers 0⁺ à droite et 0⁻ à gauche, donc "
                        <span class="res">"+∞ à droite, −∞ à gauche"</span>
                        " — les deux diffèrent, f n'a pas de limite en 2."
                    </p>
                    <p>
                        <strong>"2. "</strong><span class="fi">"f(x) = (x + √(x²))/x"</span>
                        ", D_f = ℝ*. Comme √(x²) = |x|, on a f(x) = x + 1 si x > 0 et f(x) = x − 1 si x < 0. Les limites en 0 valent "
                        <span class="res">"1 à droite et −1 à gauche"</span>"\u{a0}: pas de limite en 0."
                    </p>
                    <p>
                        <strong>"3. "</strong><span class="fi">"f(x) = (x² − 3x + 2)/(x − 2)"</span>
                        ". On factorise\u{a0}: x² − 3x + 2 = (x − 1)(x − 2), donc f(x) = x − 1 sur D_f. Les deux limites en 2 valent 1, donc "
                        <span class="res">"lim_{x→2} f(x) = 1"</span>
                        ". C'est la levée d'indétermination 0/0 par factorisation."
                    </p>
                </div>
            </div>
            <h3 style="margin-top:24px">"Les quatre asymptotes"</h3>
            <div class="tw" style="max-width:none;margin-top:10px">
                <table>
                    <thead>
                        <tr>
                            <th>"Nom"</th>
                            <th>"Condition"</th>
                            <th>"Droite ou courbe"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"Verticale"</td>
                            <td class="mono">"lim_{x→x₀} f(x) = ±∞"</td>
                            <td class="mono">"x = x₀"</td>
                        </tr>
                        <tr>
                            <td>"Horizontale"</td>
                            <td class="mono">"lim_{x→+∞} f(x) = ℓ"</td>
                            <td class="mono">"y = ℓ"</td>
                        </tr>
                        <tr>
                            <td>"Oblique"</td>
                            <td class="mono">"lim_{x→+∞} f(x) − (ax + b) = 0"</td>
                            <td class="mono">"y = ax + b"</td>
                        </tr>
                        <tr>
                            <td>"Quelconque"</td>
                            <td class="mono">"lim_{x→+∞} f(x) − g(x) = 0"</td>
                            <td class="mono">"y = g(x)"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note method" style="margin-top:18px">
                <span class="title">"Trouver une asymptote oblique sans la deviner"</span>
                <p>
                    "Faire la division euclidienne du numérateur par le dénominateur. L'exercice 30 du TD 3 le demande explicitement\u{a0}: "
                    <span class="fi">"f(x) = (2x² − 3x)/(x − 2) = 2x + 1 + 2/(x − 2)"</span>
                    ". Le reste 2/(x − 2) tend vers 0, donc "<span class="res">"y = 2x + 1"</span>
                    " est asymptote oblique, et x = 2 est asymptote verticale."
                </p>
            </div>
        </section>
    }
}

fn sec_m_continuite() -> impl IntoView {
    view! {
        <section id="m-continuite">
            <div class="sec-head">
                <span class="num">"06"</span>
                <h2>"Continuité"</h2>
            </div>
            <div class="body">
                <p>
                    "Le chapitre le plus court du cours, et le plus utilitaire\u{a0}: il sert à justifier en une ligne que la fonction qu'on étudie est continue, donc dérivable là où il faut, donc que le tableau de variations a un sens."
                </p>
            </div>
            <dl class="deflist def" style="margin-top:16px">
                <div>
                    <dt>"Continuité en un point"</dt>
                    <dd>
                        "f est continue en x₀ si pour tout ε > 0 il existe δ > 0 tel que |x − x₀| < δ entraîne |f(x) − f(x₀)| < ε. Si f est continue en tout point de A, on dit que f est continue "
                        <em>"sur"</em>" A."
                    </dd>
                </div>
                <div>
                    <dt>"Caractérisation séquentielle"</dt>
                    <dd>
                        "f est continue en x₀ si, et seulement si, pour "<em>"toute"</em>
                        " suite (uₙ) d'éléments de A qui converge vers x₀, la suite (f(uₙ)) converge vers f(x₀). C'est le pont entre le chapitre 1 et celui-ci, et le bon outil pour prouver qu'une fonction n'est "
                        <em>"pas"</em>" continue\u{a0}: il suffit d'exhiber une suite qui le contredit."
                    </dd>
                </div>
            </dl>
            <div class="note" style="margin-top:18px">
                <span class="title">"Opérations — ce qui conserve la continuité"</span>
                <ul>
                    <li>"λf, f + g et f·g sont continues si f et g le sont\u{a0};"</li>
                    <li>"1/f est continue si f ne s'annule pas\u{a0};"</li>
                    <li>"g ∘ f est continue si f et g le sont\u{a0};"</li>
                    <li>
                        "la bijection réciproque d'une fonction continue "
                        <strong>"strictement monotone"</strong>
                        " est continue — c'est ainsi que le cours établit la continuité de √ sur ℝ₊, réciproque de x ↦ x²."
                    </li>
                </ul>
            </div>
            <div class="note plan" style="margin-top:16px">
                <span class="title">"Le catalogue des fonctions continues"</span>
                <p>
                    "Constantes, identité, linéaires et affines, puissances, polynômes\u{a0}: continues sur ℝ. Les fonctions x ↦ 1/xⁿ\u{a0}: continues sur ]−∞, 0[ et sur ]0, +∞[. La valeur absolue x ↦ |x| = √(x²)\u{a0}: continue sur ℝ, comme composée. Et donc x ↦ cos(x³ − 2x + 1) aussi."
                </p>
            </div>
        </section>
    }
}

fn sec_m_usuelles() -> impl IntoView {
    view! {
        <section id="m-usuelles">
            <div class="sec-head">
                <span class="num">"07"</span>
                <h2>"Les fonctions usuelles"</h2>
            </div>
            <div class="body">
                <p>
                    "Logarithme, exponentielle, puissances, trigonométrie. Rien à démontrer ici\u{a0}: tout est à savoir de mémoire, parce que chaque étude de fonction du TD 3 en utilise au moins une."
                </p>
            </div>
            <h3 style="margin-top:22px">"Logarithme népérien"</h3>
            <div class="body">
                <p>
                    "Définie et continue sur "<span class="fi">"]0, +∞["</span>
                    ", strictement croissante. "<span class="fi">"ln(x) = 0 ⇔ x = 1"</span>
                    ", et il existe un unique réel e tel que ln(e) = 1\u{a0}: e = 2,718281828459045…"
                </p>
            </div>
            <span class="f">
"ln(ab) = ln(a) + ln(b)          ln(1/a) = −ln(a)
ln(a/b) = ln(a) − ln(b)         ln(aⁿ) = n·ln(a)

log_b(x) = ln(x)/ln(b)          log_b(b) = 1     (b ≥ 2 entier)
log_b(xy) = log_b(x) + log_b(y) log_b(xⁿ) = n·log_b(x)"
            </span>
            <h3 style="margin-top:22px">"Exponentielle"</h3>
            <div class="body">
                <p>
                    "Définie et continue sur ℝ, "<span class="fi">"eˣ > 0"</span>
                    " pour tout x. C'est la réciproque du logarithme\u{a0}: "
                    <span class="fi">"e^{ln x} = x"</span>" pour x > 0 et "
                    <span class="fi">"ln(eˣ) = x"</span>
                    " pour tout x. Son graphe est le symétrique de celui de ln par rapport à la droite y = x."
                </p>
            </div>
            <span class="f">
"e^{a+b} = eᵃ·eᵇ         e^{−a} = 1/eᵃ
eᵃ/eᵇ = e^{a−b}         (eᵃ)ⁿ = e^{na}

αˣ = e^{x·ln α}  (α > 0)      a^{x+y} = aˣ·aʸ      (ab)ˣ = aˣ·bˣ"
            </span>
            <div class="note method" style="margin-top:20px">
                <span class="title">"Croissances comparées — les limites à connaître"</span>
                <span class="f center">
"lim_{x→+∞} ln x = +∞          lim_{x→0⁺} ln x = −∞
lim_{x→0⁺} xᵃ·ln x = 0        lim_{x→+∞} (ln x)/xᵃ = 0      (a > 0)

lim_{x→+∞} eˣ = +∞            lim_{x→−∞} eˣ = 0
lim_{x→+∞} eˣ/xᵃ = +∞         lim_{x→−∞} xᵃ·eˣ = 0          (a > 0)"
                </span>
                <p>
                    "À retenir en une phrase\u{a0}: "
                    <strong>"l'exponentielle l'emporte sur toute puissance, qui l'emporte sur le logarithme."</strong>
                    " C'est ce qui lève les formes indéterminées ∞/∞ et 0 × ∞ dans toutes les études de fonction du TD 3."
                </p>
            </div>
            <h3 style="margin-top:24px">"Fonctions puissances"</h3>
            <div class="tw" style="max-width:none;margin-top:10px">
                <table>
                    <thead>
                        <tr>
                            <th>"Exposant"</th>
                            <th>"Domaine"</th>
                            <th>"Parité"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">"n ∈ ℕ*"</td>
                            <td class="mono">"ℝ"</td>
                            <td>"paire si n pair, impaire si n impair"</td>
                        </tr>
                        <tr>
                            <td class="mono">"n entier négatif non nul"</td>
                            <td class="mono">"ℝ*"</td>
                            <td>"paire si n pair, impaire si n impair"</td>
                        </tr>
                        <tr>
                            <td class="mono">"β > 0 non entier"</td>
                            <td class="mono">"]0, +∞["</td>
                            <td>"à valeurs positives, "<span class="fi">"xᵝ = e^{β ln x}"</span></td>
                        </tr>
                        <tr>
                            <td class="mono">"β < 0 non entier"</td>
                            <td class="mono">"]0, +∞["</td>
                            <td class="mono">"xᵝ = 1/x^{−β}"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <h3 style="margin-top:24px">"Fonctions trigonométriques"</h3>
            <div class="body">
                <p>
                    "Sur le cercle de centre O et de rayon 1, le point E(t) tel que l'arc (A, E(t)) mesure t a pour coordonnées "
                    <span class="fi">"(cos t, sin t)"</span>
                    ". La tangente de t est l'ordonnée de l'intersection de la droite (OE(t)) avec la verticale passant par A = (1, 0)."
                </p>
            </div>
            <div class="tw" style="max-width:none;margin-top:12px">
                <table>
                    <thead>
                        <tr>
                            <th>"Radians"</th>
                            <th class="mono">"0"</th>
                            <th class="mono">"π/6"</th>
                            <th class="mono">"π/4"</th>
                            <th class="mono">"π/3"</th>
                            <th class="mono">"π/2"</th>
                            <th class="mono">"π"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">"sin"</td>
                            <td class="mono">"0"</td>
                            <td class="mono">"1/2"</td>
                            <td class="mono">"√2/2"</td>
                            <td class="mono">"√3/2"</td>
                            <td class="mono">"1"</td>
                            <td class="mono">"0"</td>
                        </tr>
                        <tr>
                            <td class="mono">"cos"</td>
                            <td class="mono">"1"</td>
                            <td class="mono">"√3/2"</td>
                            <td class="mono">"√2/2"</td>
                            <td class="mono">"1/2"</td>
                            <td class="mono">"0"</td>
                            <td class="mono">"−1"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note plan" style="margin-top:18px">
                <span class="title">"Les propriétés à ne pas confondre"</span>
                <ul>
                    <li>
                        <strong>"cos"</strong>
                        "\u{a0}: définie sur ℝ, 2π-périodique, "<em>"paire"</em>
                        ", et −1 ≤ cos x ≤ 1."
                    </li>
                    <li>
                        <strong>"sin"</strong>
                        "\u{a0}: définie sur ℝ, 2π-périodique, "<em>"impaire"</em>
                        ", et −1 ≤ sin x ≤ 1."
                    </li>
                    <li>
                        <strong>"tan"</strong>" = sin/cos\u{a0}: définie sur "
                        <span class="fi">"ℝ \\ {π/2 + kπ, k ∈ ℤ}"</span>", "
                        <em>"π"</em>"-périodique (pas 2π\u{a0}!), impaire."
                    </li>
                </ul>
            </div>
            <span class="f center" style="margin-top:14px">
"sin²x + cos²x = 1
sin(x + π/2) = cos(x)        cos(x − π/2) = sin(x)
1 + tan²x = 1/cos²x"
            </span>
        </section>
    }
}

fn sec_m_derivees() -> impl IntoView {
    view! {
        <section id="m-derivees">
            <div class="sec-head">
                <span class="num">"08"</span>
                <h2>"Dérivabilité et variations"</h2>
            </div>
            <div class="body">
                <p>
                    "Le cœur opératoire du cours. La dérivée naît d'une idée géométrique\u{a0}: la corde qui pivote autour d'un point jusqu'à devenir la tangente."
                </p>
            </div>
            <dl class="deflist def" style="margin-top:16px">
                <div>
                    <dt>"Taux d'accroissement"</dt>
                    <dd>
                        "Le coefficient directeur de la corde entre les abscisses a et b\u{a0}: "
                        <span class="fi">"(f(b) − f(a))/(b − a)"</span>
                        ". La corde a pour équation y = ((f(b) − f(a))/(b − a))·(x − a) + f(a)."
                    </dd>
                </div>
                <div>
                    <dt>"Nombre dérivé"</dt>
                    <dd>
                        "f est dérivable en a lorsque "
                        <span class="fi">"lim_{x→a} (f(x) − f(a))/(x − a)"</span>
                        " existe "<strong>"et est finie"</strong>
                        ". Cette limite est le nombre dérivé f′(a)."
                    </dd>
                </div>
                <div>
                    <dt>"Tangente"</dt>
                    <dd>
                        <span class="fi">"y = f′(a)(x − a) + f(a)"</span>
                        ". En physique, la dérivée est la vitesse, la dérivée seconde l'accélération. On note aussi f′(x) = df/dx."
                    </dd>
                </div>
            </dl>
            <div class="ex" style="margin-top:18px">
                <div class="ex-head">
                    <span>"L'exemple du cours — la dérivée de √x"</span>
                    <span class="cat">"Taux d'accroissement"</span>
                </div>
                <div class="ex-body">
                    <p>
                        "Pour x₀ ∈ ℝ₊, "
                        <span class="fi">"(√x − √x₀)/(x − x₀) = 1/(√x + √x₀)"</span>
                        " après multiplication par la quantité conjuguée. En passant à la limite, si x₀ ≠ 0\u{a0}: "
                        <span class="res">"f′(x₀) = 1/(2√x₀)"</span>
                        ". En 0 la limite est infinie\u{a0}: √ n'est pas dérivable en 0, mais elle y est continue."
                    </p>
                </div>
            </div>
            <h3 style="margin-top:24px">"Dérivées usuelles et règles"</h3>
            <div class="tw" style="max-width:none;margin-top:10px">
                <table>
                    <thead>
                        <tr>
                            <th>"f(x)"</th>
                            <th>"f′(x)"</th>
                            <th>"D_f"</th>
                            <th>"D_f′"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">"eˣ"</td>
                            <td class="mono">"eˣ"</td>
                            <td class="mono">"ℝ"</td>
                            <td class="mono">"ℝ"</td>
                        </tr>
                        <tr>
                            <td class="mono">"ln |x|"</td>
                            <td class="mono">"1/x"</td>
                            <td class="mono">"ℝ*"</td>
                            <td class="mono">"ℝ*"</td>
                        </tr>
                        <tr>
                            <td class="mono">"sin x"</td>
                            <td class="mono">"cos x"</td>
                            <td class="mono">"ℝ"</td>
                            <td class="mono">"ℝ"</td>
                        </tr>
                        <tr>
                            <td class="mono">"cos x"</td>
                            <td class="mono">"− sin x"</td>
                            <td class="mono">"ℝ"</td>
                            <td class="mono">"ℝ"</td>
                        </tr>
                        <tr>
                            <td class="mono">"xᵅ, α ∈ ℝ"</td>
                            <td class="mono">"α·x^{α−1}"</td>
                            <td class="mono">"]0, +∞["</td>
                            <td class="mono">"]0, +∞["</td>
                        </tr>
                        <tr>
                            <td class="mono">"√x"</td>
                            <td class="mono">"1/(2√x)"</td>
                            <td class="mono">"ℝ₊"</td>
                            <td class="mono">"]0, +∞["</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <span class="f" style="margin-top:14px">
"(f + g)′ = f′ + g′              (fg)′ = f′g + g′f
(f/g)′ = (f′g − g′f)/g²         (1/f)′ = −f′/f²

(g ∘ f)′(x₀) = f′(x₀)·g′(f(x₀))

(e^{u})′ = u′·e^{u}             (ln|u|)′ = u′/u
(sin u)′ = u′·cos u             (cos u)′ = −u′·sin u
(uᵅ)′ = α·u′·u^{α−1}            (√u)′ = u′/(2√u)"
            </span>
            <div class="ex" style="margin-top:18px">
                <div class="ex-head">
                    <span>"Application — la dérivée de tan"</span>
                    <span class="cat">"Quotient"</span>
                </div>
                <div class="ex-body">
                    <p>
                        <span class="fi">
                            "tan′x = (cos x·cos x + sin x·sin x)/cos²x = 1/cos²x = 1 + tan²x"
                        </span>
                        ". Les deux écritures sont utiles\u{a0}: la première pour intégrer, la seconde pour étudier le signe."
                    </p>
                </div>
            </div>
            <h3 style="margin-top:24px">"Dérivée et sens de variation"</h3>
            <div class="note" style="margin-top:10px">
                <span class="title">"Théorème"</span>
                <p>"Soit f dérivable sur un intervalle I. Alors\u{a0}:"</p>
                <ul>
                    <li>"f est croissante sur I "<strong>"si et seulement si"</strong>" f′(x) ≥ 0 sur I\u{a0};"</li>
                    <li>"f est décroissante sur I si et seulement si f′(x) ≤ 0 sur I\u{a0};"</li>
                    <li>"f est constante sur I si et seulement si f′(x) = 0 sur I."</li>
                </ul>
            </div>
            <div class="note warn" style="margin-top:16px">
                <span class="title">"Le piège du « strictement »"</span>
                <p>
                    "L'équivalence ne vaut "<strong>"que"</strong>
                    " pour la croissance au sens large. "
                    <span class="fi">"f strictement croissante ⇎ f′(x) > 0 pour tout x"</span>
                    ". Contre-exemple du cours\u{a0}: f(x) = x³ est strictement croissante sur ℝ, pourtant f′(0) = 0."
                </p>
                <p>
                    "Ce qui reste vrai\u{a0}: si f est dérivable sur [a, b] et f′(x) > 0 sur l'"
                    <em>"ouvert"</em>
                    " ]a, b[, alors f est strictement croissante sur [a, b]. La dérivée peut s'annuler aux bornes."
                </p>
            </div>
            <div class="note method" style="margin-top:18px">
                <span class="title">"Méthode — l'étude de fonction complète"</span>
                <p>
                    "Onze exercices du TD 3 suivent ce plan à la lettre. Le respecter, c'est la note."
                </p>
                <ol class="steps">
                    <li>"Domaine de définition D_f, puis domaine de dérivabilité D_f′ (ils diffèrent\u{a0}: √ en est l'exemple)."</li>
                    <li>"Limites aux bornes de D_f, y compris à gauche et à droite des valeurs interdites."</li>
                    <li>"Asymptotes déduites de ces limites."</li>
                    <li>"Calcul de f′(x), puis factorisation pour en lire le signe."</li>
                    <li>"Tableau de variations\u{a0}: signe de f′, sens de f, valeurs aux bornes et aux extremums."</li>
                    <li>"Allure de la courbe, cohérente avec le tableau."</li>
                </ol>
            </div>
        </section>
    }
}

fn sec_m_primitives() -> impl IntoView {
    view! {
        <section id="m-primitives">
            <div class="sec-head">
                <span class="num">"09"</span>
                <h2>"Primitives et intégrales"</h2>
            </div>
            <div class="body">
                <p>
                    "Une primitive de f est une fonction F telle que "
                    <span class="fi">"F′(x) = f(x)"</span>". On note "
                    <span class="fi">"F = ∫ f(x) dx"</span>
                    ". Trois techniques suffisent au programme\u{a0}: intégration par parties, changement de variable, décomposition des fractions rationnelles."
                </p>
            </div>
            <h3 style="margin-top:22px">"Primitives usuelles"</h3>
            <span class="f">
"∫ xᵅ dx = x^{α+1}/(α+1)   (α ≠ −1)      ∫ dx/x = ln|x|
∫ eˣ dx = eˣ                             ∫ sin x dx = − cos x
∫ cos x dx = sin x                       ∫ tan x dx = − ln|cos x|
∫ dx/(1 + x²) = arctan x                 ∫ dx/cos²x = tan x
∫ dx/sin²x = −1/tan x"
            </span>
            <h3 style="margin-top:22px">"Intégration par parties"</h3>
            <span class="f center">"∫ u′(x)·v(x) dx  =  u(x)·v(x)  −  ∫ v′(x)·u(x) dx"</span>
            <div class="ex" style="margin-top:14px">
                <div class="ex-head">
                    <span>"∫ x·eˣ dx"</span>
                    <span class="cat">"IPP"</span>
                </div>
                <div class="ex-body">
                    <p>
                        "On pose v(x) = x et u′(x) = eˣ, donc u(x) = eˣ et v′(x) = 1\u{a0}: "
                        <span class="fi">"∫ x·eˣ dx = x·eˣ − ∫ eˣ dx = x·eˣ − eˣ"</span>"."
                    </p>
                    <p>
                        "Même geste pour "<span class="fi">"∫ ln(x) dx"</span>
                        "\u{a0}: on pose v = ln x et u′ = 1, ce qui donne x·ln x − ∫ 1 dx = x·ln x − x."
                    </p>
                </div>
            </div>
            <h3 style="margin-top:22px">"Changement de variable"</h3>
            <span class="f center">"∫ u′(x)·v′(u(x)) dx  =  v(u(x))"</span>
            <div class="ex" style="margin-top:14px">
                <div class="ex-head">
                    <span>"Deux applications du cours"</span>
                    <span class="cat">"Substitution"</span>
                </div>
                <div class="ex-body">
                    <p>
                        <span class="fi">"∫ 2x·sin(x²) dx"</span>
                        "\u{a0}: on pose U = x², donc dU = 2x dx, et l'intégrale devient ∫ sin U dU = − cos U, soit "
                        <span class="res">"− cos(x²)"</span>"."
                    </p>
                    <p>
                        <span class="fi">"∫ 2x·cos(x²)·sin(x²) dx"</span>
                        "\u{a0}: on pose U = sin(x²), donc dU = 2x·cos(x²) dx, et l'intégrale devient ∫ U dU = U²/2, soit "
                        <span class="res">"½·sin²(x²)"</span>"."
                    </p>
                </div>
            </div>
            <h3 style="margin-top:22px">"Fractions rationnelles"</h3>
            <div class="body">
                <p>
                    "Pour "<span class="fi">"∫ P(x)/Q(x) dx"</span>
                    ", on décompose en éléments simples. Trois briques, et seulement trois, sont au programme."
                </p>
            </div>
            <span class="f">
"∫ c/(x − λ) dx = c·ln|x − λ|

∫ c/(x − λ)ⁿ dx = −c/((n − 1)(x − λ)^{n−1})

Si δ = α² − β < 0  (le trinôme x² + 2αx + β n'a pas de racine réelle) :
∫ dx/(x² + 2αx + β) = (1/√|δ|)·arctan( (x + α)/√|δ| )"
            </span>
            <div class="ex" style="margin-top:16px">
                <div class="ex-head">
                    <span>"Les deux exemples du cours"</span>
                    <span class="cat">"Décomposition"</span>
                </div>
                <div class="ex-body">
                    <p>
                        <span class="fi">"∫ (2x² + x + 1)/(x³ − x) dx"</span>
                        ". On factorise x³ − x = x(x − 1)(x + 1) et on décompose en 2/(x − 1) + 1/(x + 1) − 1/x, d'où "
                        <span class="res">"ln( (x − 1)²(x + 1)/x )"</span>"."
                    </p>
                    <p>
                        <span class="fi">"∫ (x + 2)/(x² − x + 1) dx"</span>
                        ". On fait apparaître la dérivée du dénominateur\u{a0}: (x + 2) = ½(2x − 1) + 5/2. Le premier morceau donne ½·ln(x² − x + 1), le second se met sous forme canonique (x − ½)² + ¾ et donne "
                        <span class="res">"(5/√3)·arctan((2x − 1)/√3)"</span>"."
                    </p>
                </div>
            </div>
            <div class="note method" style="margin-top:18px">
                <span class="title">"Fractions rationnelles en cos et sin"</span>
                <p>
                    "Pour "<span class="fi">"∫ P(cos x, sin x)/Q(cos x, sin x) dx"</span>
                    ", on pose "<span class="fi">"T = tan(x/2)"</span>", ce qui rend tout rationnel\u{a0}:"
                </p>
                <span class="f">
"x = 2·arctan T        dx = 2/(1 + T²) dT
sin x = 2T/(1 + T²)   cos x = (1 − T²)/(1 + T²)"
                </span>
                <p>
                    "Exemple\u{a0}: "<span class="fi">"∫ dx/(1 − cos x)"</span>
                    " devient ∫ dT/T² = −1/T = "<span class="res">"−1/tan(x/2)"</span>"."
                </p>
            </div>
        </section>
    }
}

fn sec_m_td1() -> impl IntoView {
    view! {
        <section id="m-td1">
            <div class="sec-head">
                <span class="num">"TD1"</span>
                <h2>"Suites et récurrence"</h2>
            </div>
            <div class="body">
                <p>
                    "Vingt-six exercices, trois ou quatre séances. La feuille alterne des situations informatiques modélisées par une suite, une longue batterie de limites à calculer, et une série de récurrences."
                </p>
            </div>
            <div class="exos" style="margin-top:18px">
                <div class="exo">
                    <span class="n">"1-4"</span>
                    <span class="c">
                        <span class="t">"Modéliser une situation par une suite."</span>
                        <span class="sub">
                            "Sauvegarde qui grossit de 18 Mo par jour (arithmétique), réseau social à +15 % par semaine, batterie qui perd 12 % de la charge restante par heure, virus à +30 % par heure (géométriques). Même schéma à chaque fois\u{a0}: relation de récurrence, nature, terme général, valeur à un rang donné, puis « à partir de quand dépasse-t-on X\u{a0}? » — qui se résout avec un logarithme."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"5-6"</span>
                    <span class="c">
                        <span class="t">"La batterie de limites."</span>
                        <span class="sub">
                            "Une vingtaine de limites de suites\u{a0}: géométriques de raison négative, quotients n²/2ⁿ, (−1)ⁿ, quotients de polynômes, quotients d'exponentielles 10ⁿ/(3ⁿ + 9ⁿ). L'exercice 6 isole les comparaisons 3ⁿ − 2ⁿ, 3ⁿ − (2n)², 3ⁿ − 2n², 11n − 2n²."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"7-8"</span>
                    <span class="c">
                        <span class="t">"Les formes indéterminées en √(n² + an) − n."</span>
                        <span class="sub">
                            "Deux exercices, une seule technique\u{a0}: multiplier par la quantité conjuguée. √(n² + 3n) − n = 3n/(√(n² + 3n) + n) → 3/2, et √(n² + n + 1) − n → 1/2."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"9-10"</span>
                    <span class="c">
                        <span class="t">"Sommes géométriques et nombres rationnels."</span>
                        <span class="sub">
                            "Démontrer (1 − r)(a + ar + ⋯ + arⁿ) = a − ar^{n+1} pour en déduire la formule, puis l'appliquer\u{a0}: ∑1/2ᵏ → 2, et surtout montrer que 5,121212… et 0,590590… sont rationnels en les écrivant comme des séries géométriques."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"11-13"</span>
                    <span class="c">
                        <span class="t">"Suites auxiliaires."</span>
                        <span class="sub">
                            "uₙ₊₁ = 2uₙ + 3, uₙ₊₁ = 3uₙ − 4, uₙ₊₁ = ½uₙ + 3. On cherche b tel que vₙ = uₙ + b soit géométrique, on résout vₙ, on revient à uₙ. Le troisième converge (raison ½ < 1), les deux premiers divergent."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"14-16"</span>
                    <span class="c">
                        <span class="t">"Séries de référence."</span>
                        <span class="sub">
                            "La série harmonique ∑1/n diverge, ∑1/n² converge (par comparaison avec ∑1/(n(n+1)) qui se télescope). L'exercice 16 est le plus dur de la feuille\u{a0}: le critère de condensation de Cauchy, qui donne la nature de ∑1/nᵅ."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"17-19"</span>
                    <span class="c">
                        <span class="t">"Les six récurrences de base."</span>
                        <span class="sub">
                            "∑(2i − 1) = n², 2ⁿ ≥ n + 1, 3ⁿ ≥ 2n + 1, n! ≥ 2^{n−1}, 4 divise 5ⁿ − 1, 5 divise 8ⁿ − 3ⁿ. Puis ∑k² = n(n+1)(2n+1)/6 et ∑k³ = (n(n+1)/2)². Ce sont celles à savoir refaire sans réfléchir."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"20-24"</span>
                    <span class="c">
                        <span class="t">"Conjecturer puis démontrer."</span>
                        <span class="sub">
                            "u₀ = 2, uₙ₊₁ = 3uₙ + 2 donne uₙ = 3ⁿ⁺¹ − 1\u{a0}; un compteur de comparaisons uₙ = uₙ₋₁ + 2n − 1 donne uₙ = n²\u{a0}; vₙ₊₁ = vₙ/(vₙ + 1) donne vₙ = 1/(n + 1). L'exercice 23 demande un encadrement 2,5 ≤ uₙ₊₁ ≤ uₙ ≤ 10 établi par récurrence."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"25-26"</span>
                    <span class="c">
                        <span class="t">"Croissante et majorée donc convergente."</span>
                        <span class="sub">
                            "uₙ₊₁ = (uₙ + 2)/2 et uₙ₊₁ = √(uₙ + 2). Le plan est imposé\u{a0}: majorer par récurrence, montrer la croissance, invoquer le théorème de la limite monotone, puis résoudre ℓ = f(ℓ) — les limites valent 2 dans les deux cas."
                        </span>
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_m_td2() -> impl IntoView {
    view! {
        <section id="m-td2">
            <div class="sec-head">
                <span class="num">"TD2"</span>
                <h2>"Représentations graphiques"</h2>
            </div>
            <div class="body">
                <p>
                    "Une seule séance. La feuille annonce elle-même sa nature\u{a0}: "
                    <em>
                        "« Il s'agit d'une feuille de révisions faite pour celles et ceux qui ne sont pas à l'aise avec les notions indiquées ci-dessus. »"
                    </em>
                    " Rien de nouveau, mais c'est le prérequis de tout le chapitre 8."
                </p>
            </div>
            <div class="exos" style="margin-top:18px">
                <div class="exo">
                    <span class="n">"1-7"</span>
                    <span class="c">
                        <span class="t">"Lire une équation, une inéquation, un tableau de signes."</span>
                        <span class="sub">
                            "Résoudre graphiquement f(x) = 1, f(x) ≥ 1, f(x) < 2, f(x) = g(x), f(x) > 0. Trouver les antécédents d'une valeur. Dresser un tableau de signes à partir de la courbe. L'exercice 7 fait résoudre f(x) = 0 sur quatre paraboles dont on donne le graphe et l'expression."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"8"</span>
                    <span class="c">
                        <span class="t">"Parité et antécédents sur une courbe."</span>
                        <span class="sub">
                            "Onze fonctions données par leur graphe et leur formule\u{a0}: x² + 1, (x−1)² − 2, −x² + 1, x³/2 − 7x/2, √(x²+1)/x, (x³+1)/(x²+1). Reconnaître paires et impaires, puis donner les antécédents de 1. La symétrie du graphe donne la réponse plus vite que le calcul."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"9-13"</span>
                    <span class="c">
                        <span class="t">"Lire un graphique de situation."</span>
                        <span class="sub">
                            "Richesse en espèces de fourmis selon la latitude, refroidissement d'un verre d'eau, course de 100 m à trois coureurs, alcoolémie relevée toutes les quinze minutes, satisfaction thermique S(x) = 100/(1 + ((x−22)/5)²). Décrire avec des mots, esquisser, et pour le dernier résoudre S(x) = 50 par le calcul."
                        </span>
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_m_td3() -> impl IntoView {
    view! {
        <section id="m-td3">
            <div class="sec-head">
                <span class="num">"TD3"</span>
                <h2>"Fonctions, dérivées, primitives"</h2>
            </div>
            <div class="body">
                <p>
                    "Trente-trois exercices, la feuille la plus longue et la plus décisive\u{a0}: elle couvre à elle seule les chapitres 4 à 9, c'est-à-dire l'essentiel de ce qui sera évalué."
                </p>
            </div>
            <div class="exos" style="margin-top:18px">
                <div class="exo">
                    <span class="n">"1"</span>
                    <span class="c">
                        <span class="t">"Douze domaines de définition."</span>
                        <span class="sub">
                            "Polynômes (domaine ℝ, réponse immédiate), racines carrées, quotients, et surtout les combinaisons\u{a0}: √(3x+2) − 1/(3−x), 1/√(−x²+2x−1), √(3x−1)/(x²−x−2). Traiter chaque contrainte séparément puis intersecter."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"2-7"</span>
                    <span class="c">
                        <span class="t">"Mettre un problème en fonction."</span>
                        <span class="sub">
                            "Air à ajouter pour gonfler un ballon de r à r+1, aire d'un rectangle de périmètre 20, périmètre d'un rectangle d'aire 100 (et la comparaison avec le disque de même aire), aire d'un triangle équilatéral, surface d'un cube en fonction de son volume, boîte ouverte de volume 2 m³ à surface minimale. Ces derniers préparent les optimisations de la section 7."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"8-12"</span>
                    <span class="c">
                        <span class="t">"Identités trigonométriques et équations."</span>
                        <span class="sub">
                            "Démontrer cos² + sin² = 1 par Pythagore, cos² − sin² = 1 − 2sin², 1 + tan² = 1/cos². Puis les équations sin x = √3/2, 2cos x + √2 = 0, 2tan x − 1 = 0 à résoudre sur ]−3π, 2π] — attention à donner "
                            <em>"toutes"</em>
                            " les solutions de l'intervalle, pas seulement celle du cercle. L'exercice 12 mélange limites et trigonométrie."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"13-14"</span>
                    <span class="c">
                        <span class="t">"Simplifier, résoudre avec ln et exp."</span>
                        <span class="sub">
                            "Six expressions à simplifier (e^{3+ln 8}, 3ln(2x)/ln(e^{−x}), ln(e^{3x−2}/4x²)…), puis une douzaine d'équations et d'inéquations. Les deux réflexes\u{a0}: une exponentielle ne s'annule jamais (donc (x²+1)e^{x−1} = −1 n'a pas de solution), et toute inéquation avec ln exige de vérifier le domaine avant de conclure."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"15-19"</span>
                    <span class="c">
                        <span class="t">"Les modèles exponentiels appliqués."</span>
                        <span class="sub">
                            "Régulation génétique m(t) = ½e^{−t}(sin t − cos t) + ½, pH et concentration en H₃O⁺, intensité sonore en décibels I = 10·log₁₀(J/J₀), culture bactérienne N(t) = N₀e^{βt}, élimination d'un médicament A(t) = 250e^{−0,3t}. Toujours le même geste\u{a0}: isoler l'exponentielle, passer au logarithme."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"20"</span>
                    <span class="c">
                        <span class="t">"Quinze dérivées à calculer."</span>
                        <span class="sub">
                            "Avec le domaine de dérivabilité à chaque fois\u{a0}: (x²−x+2)⁴, (2x−3)²(−x+2), √(x²−3x+2), x³/(x²+x−2), √((x−3)/(x−2)), ln(x²−2x−3), ln((x−1)/(x+1)), e^{−x³+2x}, (x+1)e^{−x}, x·ln x. C'est l'exercice à refaire jusqu'à ce qu'il ne coûte plus rien."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"21-22"</span>
                    <span class="c">
                        <span class="t">"Primitives et intégrales."</span>
                        <span class="sub">
                            "Douze primitives puis quinze intégrales définies. On y trouve les trois techniques\u{a0}: directe, changement de variable (∫x·e^{x²}), intégration par parties (∫x·ln x, ∫x·cos 4x, ∫eˣ·sin x qui demande deux IPP successives), et fractions rationnelles (∫(2x+1)/(x²+x))."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">"23-33"</span>
                    <span class="c">
                        <span class="t">"Onze études de fonctions complètes."</span>
                        <span class="sub">
                            "x² − 4x + 3, (x+1)/(x−2), xe^{−x}, ln x − x, un bénéfice B(x) = R(x) − C(x) à maximiser, un temps d'exécution T(x) = x²/1000 + 250000/x à minimiser, un débit D(x) = x/(1+x²), 3x⁴ − 4x³ − 12x² + 5, (2x²−3x)/(x−2), xˣ = e^{x ln x}, √(x²+2x−3), (x+1)e^{−x}. Toutes suivent le plan en six points de la section 08."
                        </span>
                    </span>
                </div>
            </div>
            <div class="note method" style="margin-top:20px">
                <span class="title">"Les trois exercices à traiter en priorité"</span>
                <p>
                    <strong>"27"</strong>" (temps d'exécution T(x) = x²/1000 + 250000/x), "
                    <strong>"28"</strong>" (débit D(x) = x/(1+x²)) et "<strong>"26"</strong>
                    " (bénéfice) sont les seuls exercices du TD à porter sur des situations informatiques ou économiques. Ils combinent tout\u{a0}: domaine, dérivée d'un quotient, tableau de variations, extremum, interprétation. Un sujet d'examen se construit naturellement autour d'un exercice de ce type."
                </p>
            </div>
        </section>
    }
}

fn sec_m_eval() -> impl IntoView {
    view! {
        <section id="m-eval">
            <div class="sec-head">
                <span class="num">"—"</span>
                <h2>"Évaluation et note finale"</h2>
            </div>
            <div class="body">
                <p>
                    "8 h de cours magistral, 16 h de TD. L'évaluation repose sur deux notes seulement\u{a0}: un examen terminal et la plateforme d'exercices WIMS."
                </p>
            </div>
            <div class="note" style="margin-top:16px">
                <span class="title">"Le calcul de la note finale"</span>
                <span class="f center">
"Si W − E > 10   alors   NF = E
Sinon                    NF = (8·E + 2·W)/10"
                </span>
                <p>
                    "E est la note d'examen terminal, W la note WIMS. La règle du dessus est une clause anti-triche\u{a0}: une note WIMS très supérieure à la note d'examen ne compte pas. Autrement dit, "
                    <strong>"WIMS ne peut rapporter que 2 points sur 20, et seulement si l'examen suit."</strong>
                </p>
            </div>
            <div class="note warn" style="margin-top:16px">
                <span class="title">"Pas d'annales publiées pour cette matière"</span>
                <p>
                    "Contrairement à "
                    <a href="/structures-fondamentales#sf-annales">"Structures fondamentales"</a>
                    ", aucun sujet des années précédentes n'est diffusé — la matière n'existe sous cette forme que depuis la réforme. Le programme officiel et les trois feuilles de TD sont donc la seule source fiable sur ce qui sera demandé."
                </p>
            </div>
            <h3 style="margin-top:24px">"Ce qui est attendu à l'examen"</h3>
            <div class="body">
                <p>
                    "Déduit du programme annoncé et du volume consacré à chaque notion dans les TD. Les gestes cités ci-dessous couvrent la quasi-totalité des exercices des trois feuilles."
                </p>
            </div>
            <div class="tw" style="max-width:none;margin-top:14px">
                <table>
                    <thead>
                        <tr>
                            <th>"Geste"</th>
                            <th>"Poids dans les TD"</th>
                            <th>"Ce qu'il faut savoir faire"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><strong>"Étude de fonction complète"</strong></td>
                            <td class="mono">"11 exercices"</td>
                            <td>
                                "Domaine, limites aux bornes, asymptotes, dérivée factorisée, tableau de variations, allure. Le plan en six points, appliqué sans en sauter un."
                            </td>
                        </tr>
                        <tr>
                            <td><strong>"Calculer une dérivée"</strong></td>
                            <td class="mono">"15 fonctions"</td>
                            <td>
                                "Quotient, composée, ln et exp d'une expression. Avec le domaine de dérivabilité, qui n'est pas toujours le domaine de définition."
                            </td>
                        </tr>
                        <tr>
                            <td><strong>"Primitive ou intégrale"</strong></td>
                            <td class="mono">"27 calculs"</td>
                            <td>
                                "Reconnaître laquelle des trois techniques s'applique\u{a0}: directe, changement de variable si on voit u′·v′(u), IPP si un facteur se simplifie en dérivant."
                            </td>
                        </tr>
                        <tr>
                            <td><strong>"Limite de suite"</strong></td>
                            <td class="mono">"~25 limites"</td>
                            <td>
                                "Termes dominants, quantité conjuguée pour √(n²+an) − n, croissances comparées pour les quotients d'exponentielles."
                            </td>
                        </tr>
                        <tr>
                            <td><strong>"Récurrence rédigée"</strong></td>
                            <td class="mono">"10 exercices"</td>
                            <td>
                                "Les cinq temps de la rédaction. C'est là que se perdent les points de forme."
                            </td>
                        </tr>
                        <tr>
                            <td><strong>"Domaine de définition"</strong></td>
                            <td class="mono">"1re question partout"</td>
                            <td>
                                "Trois contraintes seulement, à traiter puis à intersecter. Question courte, points faciles."
                            </td>
                        </tr>
                        <tr>
                            <td><strong>"Équation avec ln ou exp"</strong></td>
                            <td class="mono">"12 + 5 appliqués"</td>
                            <td>
                                "Isoler, passer au logarithme, vérifier le domaine. Ne jamais oublier qu'une exponentielle ne s'annule pas."
                            </td>
                        </tr>
                        <tr>
                            <td><strong>"Suite arithmétique ou géométrique"</strong></td>
                            <td class="mono">"4 situations"</td>
                            <td>
                                "Reconnaître la nature, donner le terme général, sommer, résoudre « à partir de quel rang »."
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note method" style="margin-top:20px">
                <span class="title">"Où placer l'effort"</span>
                <p>
                    "Les études de fonction et les calculs de primitives représentent à eux seuls près de la moitié du travail demandé dans les TD. Ce sont des gestes mécaniques\u{a0}: ils se travaillent par répétition, pas par compréhension. À l'inverse, le critère de condensation (ex. 16 du TD 1) et les fractions rationnelles trigonométriques sont des exercices isolés, très au-dessus du reste\u{a0}: les garder pour la fin, pas pour la veille."
                </p>
            </div>
        </section>
    }
}

fn sec_m_outils() -> impl IntoView {
    view! {
        <section id="m-outils">
            <div class="sec-head">
                <span class="num">"—"</span>
                <h2>"Formulaire"</h2>
            </div>
            <div class="body">
                <p>
                    "Ce qui tient sur une feuille A4, et qu'il vaut mieux savoir sans la regarder."
                </p>
            </div>
            <h3 style="margin-top:20px">"Les limites de référence"</h3>
            <span class="f">
"lim_{n→+∞} qⁿ = 0 si |q| < 1,  +∞ si q > 1,  pas de limite si q ≤ −1

lim_{x→+∞} (ln x)/xᵃ = 0        lim_{x→0⁺} xᵃ·ln x = 0
lim_{x→+∞} eˣ/xᵃ = +∞           lim_{x→−∞} xᵃ·eˣ = 0

√(n² + an) − n  →  a/2          (quantité conjuguée)"
            </span>
            <h3 style="margin-top:22px">"Dérivées et primitives en vis-à-vis"</h3>
            <div class="tw" style="max-width:none;margin-top:10px">
                <table>
                    <thead>
                        <tr>
                            <th>"∫ f = F"</th>
                            <th>"f"</th>
                            <th>"f′"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">"x^{α+1}/(α+1)"</td>
                            <td class="mono">"xᵅ"</td>
                            <td class="mono">"α·x^{α−1}"</td>
                        </tr>
                        <tr>
                            <td class="mono">"eˣ"</td>
                            <td class="mono">"eˣ"</td>
                            <td class="mono">"eˣ"</td>
                        </tr>
                        <tr>
                            <td class="mono">"x·ln x − x"</td>
                            <td class="mono">"ln x"</td>
                            <td class="mono">"1/x"</td>
                        </tr>
                        <tr>
                            <td class="mono">"− cos x"</td>
                            <td class="mono">"sin x"</td>
                            <td class="mono">"cos x"</td>
                        </tr>
                        <tr>
                            <td class="mono">"sin x"</td>
                            <td class="mono">"cos x"</td>
                            <td class="mono">"− sin x"</td>
                        </tr>
                        <tr>
                            <td class="mono">"− ln|cos x|"</td>
                            <td class="mono">"tan x"</td>
                            <td class="mono">"1 + tan²x"</td>
                        </tr>
                        <tr>
                            <td class="mono">"arctan x"</td>
                            <td class="mono">"1/(1 + x²)"</td>
                            <td class="mono">"−2x/(1 + x²)²"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note plan" style="margin-top:20px">
                <span class="title">"Les réflexes qui font gagner du temps"</span>
                <ol>
                    <li>
                        "Une forme indéterminée ∞/∞ sur des polynômes\u{a0}: garder les termes dominants."
                    </li>
                    <li>
                        "Une différence de racines\u{a0}: multiplier par la quantité conjuguée."
                    </li>
                    <li>
                        "Une exponentielle contre une puissance\u{a0}: croissances comparées, l'exponentielle gagne."
                    </li>
                    <li>
                        "Une intégrale où l'on voit u′ à côté de u\u{a0}: changement de variable."
                    </li>
                    <li>
                        "Une intégrale avec un polynôme × (eˣ, sin, cos, ln)\u{a0}: intégration par parties, en dérivant le polynôme."
                    </li>
                    <li>
                        "Une fraction avec un trinôme sans racine au dénominateur\u{a0}: forme canonique, puis arctan."
                    </li>
                    <li>
                        "Une suite uₙ₊₁ = a·uₙ + b\u{a0}: point fixe ℓ = b/(1 − a), puis vₙ = uₙ − ℓ."
                    </li>
                    <li>
                        "« À partir de quel rang\u{a0}? » sur une suite géométrique\u{a0}: passer au logarithme, et attention au sens de l'inégalité si ln q < 0."
                    </li>
                </ol>
            </div>
        </section>
    }
}

/// Page complète de la matière.
pub fn page() -> impl IntoView {
    view! {
        <div class="course c-mtc">
            {masthead()}
            <div class="shell">
                <Toc course=Course::Mtc items=TOC label=TOC_LABEL/>
                <main>
                    <>
                        {sec_m_suites()}
                        {sec_m_arithgeo()}
                        {sec_m_recurrence()}
                        {sec_m_fonctions()}
                        {sec_m_limites()}
                        {sec_m_continuite()}
                        {sec_m_usuelles()}
                        {sec_m_derivees()}
                        {sec_m_primitives()}
                    </>
                    <>
                        {sec_m_td1()}
                        {sec_m_td2()}
                        {sec_m_td3()}
                        {sec_m_eval()}
                        {sec_m_outils()}
                    </>
                </main>
            </div>
        </div>
    }
}
