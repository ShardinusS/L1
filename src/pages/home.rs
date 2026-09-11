//! Accueil.
//!
//! Contenu transposé tel quel depuis le classeur HTML d'origine :
//! mêmes textes, mêmes formules, mêmes exemples.

use leptos::prelude::*;
use leptos_router::components::A;

/// Ancres des sections de la page, dans l'ordre d'affichage.
#[rustfmt::skip]
pub const SECTIONS: &[&str] = &[
    "h-matieres",
    "h-revisions",
    "h-docs",
    "h-methode",
];

fn masthead() -> impl IntoView {
    view! {
        <header class="hero">
            <div class="hero-inner">
                <div class="eyebrow">
                    "Licence 1 · Semestre 1 · 2026-2027"
                </div>
                <h1>
                    "Classeur d'informatique"
                </h1>
                <p class="lede">
                    "Cinq matières réunies au même endroit\u{a0}: les fiches de cours, les feuilles de TD, les annales avec leurs corrigés, un espace d'entraînement corrigé automatiquement et un prompt de révision par chapitre. Une recherche unique traverse le tout, en haut à droite."
                </p>
                <div class="meta">
                    <span>
                        "Kenzo Metgy"
                    </span>
                    <span>
                        "Mis à jour le 11 septembre 2026"
                    </span>
                    <span>
                        <span class="mono">
                            "22"
                        </span>
                        " documents dépouillés"
                    </span>
                    <span>
                        <span class="mono">
                            "35"
                        </span>
                        " prompts de révision"
                    </span>
                </div>
            </div>
        </header>
    }
}

fn sec_h_matieres() -> impl IntoView {
    view! {
        <section id="h-matieres">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Les cinq matières"
                </h2>
            </div>
            <div class="mat-grid">
                <A href="/structures-fondamentales" scroll=false attr:class="mat-card c-sf">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Matière 01 · Mathématiques"
                        </span>
                        <h3>
                            "Structures fondamentales"
                        </h3>
                        <p>
                            "Logique, ensembles, applications, dénombrement, nombres complexes, groupes, arithmétique modulaire et permutations."
                        </p>
                        <span class="facts">
                            <span>
                                "7 chapitres"
                            </span>
                            <span>
                                "4 TD"
                            </span>
                            <span>
                                "5 annales"
                            </span>
                        </span>
                    </span>
                </A>
                <A href="/methodes-calcul" scroll=false attr:class="mat-card c-mtc">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Matière 02 · Mathématiques"
                        </span>
                        <h3>
                            "Méthodes et techniques de calcul"
                        </h3>
                        <p>
                            "Suites et limites, récurrence, fonctions usuelles, dérivées, primitives et intégrales\u{a0}: l'outillage de calcul qui sert partout ailleurs."
                        </p>
                        <span class="facts">
                            <span>
                                "9 chapitres"
                            </span>
                            <span>
                                "3 TD"
                            </span>
                            <span>
                                "formulaire"
                            </span>
                        </span>
                    </span>
                </A>
                <A href="/algorithmique" scroll=false attr:class="mat-card c-algo">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Matière 03 · Informatique"
                        </span>
                        <h3>
                            "Algorithmique 1"
                        </h3>
                        <p>
                            "Ce qu'est un algorithme, le cycle de développement, les types, les variables et les premières traces d'exécution."
                        </p>
                        <span class="facts">
                            <span>
                                "CM 1"
                            </span>
                            <span>
                                "TD série 1"
                            </span>
                            <span>
                                "15 exercices"
                            </span>
                        </span>
                    </span>
                </A>
                <A href="/information" scroll=false attr:class="mat-card c-info">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Matière 04 · Informatique"
                        </span>
                        <h3>
                            "Représentation de l'information"
                        </h3>
                        <p>
                            "Du sens à la donnée, représentation binaire, systèmes de numération, conversions et entiers non signés."
                        </p>
                        <span class="facts">
                            <span>
                                "6 chapitres"
                            </span>
                            <span>
                                "4 méthodes"
                            </span>
                            <span>
                                "convertisseur"
                            </span>
                        </span>
                    </span>
                </A>
                <A href="/systemes" scroll=false attr:class="mat-card c-os">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Matière 05 · Informatique"
                        </span>
                        <h3>
                            "Systèmes d'exploitation"
                        </h3>
                        <p>
                            "Linux en ligne de commande\u{a0}: arborescence, déplacement, création, lecture, suppression et composition de commandes."
                        </p>
                        <span class="facts">
                            <span>
                                "8 chapitres"
                            </span>
                            <span>
                                "30+ commandes"
                            </span>
                            <span>
                                "aide-mémoire"
                            </span>
                        </span>
                    </span>
                </A>
            </div>
        </section>
    }
}

fn sec_h_revisions() -> impl IntoView {
    view! {
        <section id="h-revisions">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Pour réviser vite"
                </h2>
            </div>
            <div class="body" style="margin-bottom:20px">
                <p>
                    "Le dépouillement des cinq sujets de Structures fondamentales depuis la réforme — les deux partiels, les deux examens finals, le rattrapage — fait ressortir un noyau de questions qui reviennent presque à l'identique d'une année sur l'autre. Et deux chapitres entiers qui ne tombent jamais. C'est ce dépouillement qui calibre les prompts de révision."
                </p>
            </div>
            <div class="kpis" style="margin-bottom:20px">
                <div class="kpi">
                    <span class="v">
                        "12"
                    </span>
                    <span class="k">
                        "questions récurrentes"
                    </span>
                </div>
                <div class="kpi">
                    <span class="v">
                        "4×"
                    </span>
                    <span class="k">
                        "la limite par la définition ε"
                    </span>
                </div>
                <div class="kpi">
                    <span class="v">
                        "4,1"
                    </span>
                    <span class="k">
                        "moyenne du partiel 2025"
                    </span>
                </div>
                <div class="kpi">
                    <span class="v">
                        "11 %"
                    </span>
                    <span class="k">
                        "de copies ≥ 10 au partiel"
                    </span>
                </div>
                <div class="kpi">
                    <span class="v">
                        "0×"
                    </span>
                    <span class="k">
                        "dénombrement et complexes"
                    </span>
                </div>
            </div>
            <div class="mat-grid">
                <A href="/structures-fondamentales#sf-recurrent" scroll=false attr:class="mat-card c-sf">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Structures fondamentales"
                        </span>
                        <h3>
                            "Les questions qui reviennent"
                        </h3>
                        <p>
                            "Le tableau croisé des cinq sujets 2024-2025\u{a0}: ce qui tombe chaque fois, où le réviser, et la réponse-type."
                        </p>
                        <span class="facts">
                            <span>
                                "12 familles"
                            </span>
                            <span>
                                "+ méthodes"
                            </span>
                        </span>
                    </span>
                </A>
                <A href="/prompts" scroll=false attr:class="mat-card">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Les cinq matières"
                        </span>
                        <h3>
                            "Un prompt par chapitre"
                        </h3>
                        <p>
                            "Trente-cinq prompts sur mesure à coller dans un assistant. Chacun borne le programme et dit ce qui tombe vraiment\u{a0}: deux chapitres y sont explicitement signalés comme n'étant jamais tombés."
                        </p>
                        <span class="facts">
                            <span>
                                "35 prompts"
                            </span>
                            <span>
                                "calibrés sur 5 sujets"
                            </span>
                        </span>
                    </span>
                </A>
                <A href="/structures-fondamentales#sf-annales" scroll=false attr:class="mat-card c-sf">
                    <span class="bar"></span>
                    <span class="in">
                        <span class="k">
                            "Structures fondamentales"
                        </span>
                        <h3>
                            "Annales & corrigés"
                        </h3>
                        <p>
                            "Partiels et examens 2024-2025, rattrapage, avec les corrigés officiels et la distribution réelle des notes."
                        </p>
                        <span class="facts">
                            <span>
                                "5 sujets"
                            </span>
                            <span>
                                "4 corrigés"
                            </span>
                        </span>
                    </span>
                </A>
            </div>
        </section>
    }
}

fn sec_h_docs() -> impl IntoView {
    view! {
        <section id="h-docs">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Les documents dépouillés"
                </h2>
            </div>
            <div class="body" style="margin-bottom:16px">
                <p>
                    "Ce que contient chaque fichier d'origine et où il se retrouve dans le classeur."
                </p>
            </div>
            <div class="tw" style="max-width:none">
                <table class="compact">
                    <thead>
                        <tr>
                            <th>
                                "Document"
                            </th>
                            <th>
                                "Nature"
                            </th>
                            <th>
                                "Où dans le classeur"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">
                                "CoursStructFond2627.pdf"
                            </td>
                            <td>
                                "Cours complet, 54 p. — F. Durand, UPJV"
                            </td>
                            <td>
                                "Matière 01 · chapitres 01 à 07"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "TD1SF2627logique.pdf"
                            </td>
                            <td>
                                "Feuille n°1 — logique"
                            </td>
                            <td>
                                "Matière 01 · TD 1"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "TD2SF2627ensembles.pdf"
                            </td>
                            <td>
                                "Feuille n°2 — ensembles, applications, dénombrement"
                            </td>
                            <td>
                                "Matière 01 · TD 2"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "TD3SFcomplexes.pdf"
                            </td>
                            <td>
                                "Feuille n°3 — nombres complexes"
                            </td>
                            <td>
                                "Matière 01 · TD 3"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "TD4SF2627GroupesPermutations.pdf"
                            </td>
                            <td>
                                "Feuille n°4 — Z/nZ, groupes, permutations"
                            </td>
                            <td>
                                "Matière 01 · TD 4"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "Partiel SF2024.pdf"
                            </td>
                            <td>
                                "Sujet du partiel 2024-2025"
                            </td>
                            <td>
                                "Matière 01 · Annales"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "CorrectionpartielSF2024.pdf"
                            </td>
                            <td>
                                "Corrigé du partiel 2024-2025"
                            </td>
                            <td>
                                "Matière 01 · Annales"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "CorrectionexamenSF2024.pdf"
                            </td>
                            <td>
                                "Corrigé de l'examen 2024-2025"
                            </td>
                            <td>
                                "Matière 01 · Annales"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "RattrapgeSF2024.pdf"
                            </td>
                            <td>
                                "Rattrapage juin 2025, corrigé inclus"
                            </td>
                            <td>
                                "Matière 01 · Annales"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "partielSF2025.pdf"
                            </td>
                            <td>
                                "Sujet du partiel 2025-2026"
                            </td>
                            <td>
                                "Matière 01 · Annales"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "CorrectionpartielSF2025.pdf"
                            </td>
                            <td>
                                "Corrigé du partiel 2025-2026"
                            </td>
                            <td>
                                "Matière 01 · Annales"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "ExamenSF2025.pdf"
                            </td>
                            <td>
                                "Sujet de l'examen final 2025-2026"
                            </td>
                            <td>
                                "Matière 01 · Annales"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "ExamenSF2025 1.pdf"
                            </td>
                            <td>
                                "Doublon strict du précédent"
                            </td>
                            <td>
                                "— non repris"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "NotesPartiel2025.pdf"
                            </td>
                            <td>
                                "353 numéros d'étudiants et leurs notes"
                            </td>
                            <td>
                                "Statistiques agrégées seulement "
                                <span class="tag fix">
                                    "anonymisé"
                                </span>
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "Cours.pdf (MTC)"
                            </td>
                            <td>
                                "Cours complet — F. Durand, UPJV"
                            </td>
                            <td>
                                "Matière 02 · chapitres 01 à 09"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "TD1.pdf (MTC)"
                            </td>
                            <td>
                                "Fiche n°1 — suites, limites, récurrence"
                            </td>
                            <td>
                                "Matière 02 · TD 1"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "TD2.pdf (MTC)"
                            </td>
                            <td>
                                "Fiche n°2 — représentations graphiques"
                            </td>
                            <td>
                                "Matière 02 · TD 2"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "TD3.pdf (MTC)"
                            </td>
                            <td>
                                "Fiche n°3 — fonctions, dérivées, primitives"
                            </td>
                            <td>
                                "Matière 02 · TD 3"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "CM1.pdf"
                            </td>
                            <td>
                                "Cours magistral 1, 45 diapositives"
                            </td>
                            <td>
                                "Matière 03 · chapitre 01"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "Algo1_TD_Serie1.pdf"
                            </td>
                            <td>
                                "TD série 1 — premières notions"
                            </td>
                            <td>
                                "Matière 03 · TD 1"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "Representation de l'information.one"
                            </td>
                            <td>
                                "Notes OneNote personnelles"
                            </td>
                            <td>
                                "Matière 04"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "Systèmes d'exploitation.pdf"
                            </td>
                            <td>
                                "Notes personnelles"
                            </td>
                            <td>
                                "Matière 05"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note warn" style="margin-top:18px">
                <span class="title">
                    "Deux fichiers manquants et un fichier écarté"
                </span>
                <p>
                    <span class="mono">
                        "ExamenSF2024.pdf"
                    </span>
                    " et "
                    <span class="mono">
                        "notesPARTIELS.pdf"
                    </span>
                    " apparaissaient dans ta liste mais ne sont pas arrivés\u{a0}: pour l'examen 2024, le classeur se rabat sur son corrigé, qui redonne l'intégralité des énoncés."
                </p>
                <p>
                    "Le relevé "
                    <span class="mono">
                        "NotesPartiel2025.pdf"
                    </span>
                    " associe des numéros d'étudiants à leurs notes\u{a0}: il n'a pas sa place sur une page que tu peux partager. Seules les statistiques d'ensemble, qui n'identifient personne, ont été reprises."
                </p>
            </div>
        </section>
    }
}

fn sec_h_methode() -> impl IntoView {
    view! {
        <section id="h-methode">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Comment lire ce classeur"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Les matières 01 à 03 sont des "
                    <strong>
                        "synthèses"
                    </strong>
                    " des documents du cours\u{a0}: définitions, théorèmes, méthodes et énoncés y sont réorganisés pour la révision, ils ne remplacent pas les PDF d'origine, notamment pour les démonstrations faites en amphi. Les matières 04 et 05 reprennent tes propres notes, mises en forme et relues."
                </p>
                <p>
                    "Trois repères visuels reviennent partout\u{a0}: "
                    <span class="tag">
                        "méthode"
                    </span>
                    " pour un enchaînement à appliquer tel quel, "
                    <span class="tag hot">
                        "annale"
                    </span>
                    " pour une question déjà tombée en examen, et "
                    <span class="tag fix">
                        "corrigé"
                    </span>
                    " pour un point rectifié par rapport aux notes d'origine."
                </p>
            </div>
            <div class="note" style="margin-top:18px">
                <span class="title">
                    "Sources"
                </span>
                <p>
                    <strong>
                        "Structures fondamentales"
                    </strong>
                    " — cours, TD, sujets et corrigés de Fabien Durand, Université de Picardie Jules Verne, licence 1 de mathématiques et d'informatique."
                </p>
                <p>
                    <strong>
                        "Méthodes et techniques de calcul"
                    </strong>
                    " — cours et trois feuilles de TD de Fabien Durand, UPJV, licence 1 d'informatique 2026-2027."
                </p>
                <p>
                    <strong>
                        "Algorithmique 1"
                    </strong>
                    " — cours de Jordan Caracotte et Léo Robert, et feuille de TD de la licence STS, semestre 1."
                </p>
                <p>
                    <strong>
                        "Représentation de l'information"
                    </strong>
                    " et "
                    <strong>
                        "Systèmes d'exploitation"
                    </strong>
                    " — notes de cours personnelles."
                </p>
            </div>
        </section>
    }
}

/// Page complète de la matière.
pub fn page() -> impl IntoView {
    view! {
        <div class="course">
            {masthead()}
            <div class="shell wide">
                <main>
                    {sec_h_matieres()}
                    {sec_h_revisions()}
                    {sec_h_docs()}
                    {sec_h_methode()}
                </main>
            </div>
        </div>
    }
}
