//! Matiere 02 - Algorithmique 1.
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
    "al-orga",
    "al-def",
    "al-cycle",
    "al-langages",
    "al-machine",
    "al-syntaxe",
    "al-td1",
    "al-pieges",
];

/// Sommaire de la matière : (ancre, numéro, titre).
#[rustfmt::skip]
pub const TOC: &[TocItem] = &[
    TocItem { anchor: "al-orga", num: "01", title: "Organisation et évaluation" },
    TocItem { anchor: "al-def", num: "02", title: "Qu'est-ce qu'un algorithme\u{a0}?" },
    TocItem { anchor: "al-cycle", num: "03", title: "Le cycle de développement" },
    TocItem { anchor: "al-langages", num: "04", title: "Un exemple, quatre langages" },
    TocItem { anchor: "al-machine", num: "05", title: "Algorithmes et machine" },
    TocItem { anchor: "al-syntaxe", num: "—", title: "Fiche de syntaxe" },
    TocItem { anchor: "al-td1", num: "TD1", title: "Premières notions" },
    TocItem { anchor: "al-pieges", num: "—", title: "Les pièges du TD 1" },
];

/// Libellé accessible du sommaire.
pub const TOC_LABEL: &str = "Sommaire d'Algorithmique 1";

/// Entrées de recherche de la matière, dans l'ordre du cours.
#[rustfmt::skip]
pub const INDEX: &[IndexEntry] = &[
    IndexEntry::new(Course::Algo, "al-orga", "Organisation et évaluation", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-orga", "Note finale", "Organisation et évaluation"),
    IndexEntry::new(Course::Algo, "al-def", "Qu'est-ce qu'un algorithme\u{a0}?", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-def", "Les entrées\u{a0}: quelles informations, sous quelle forme\u{a0}?", "Qu'est-ce qu'un algorithme\u{a0}?"),
    IndexEntry::new(Course::Algo, "al-def", "Algorithme définition informelle", "Qu'est-ce qu'un algorithme\u{a0}?"),
    IndexEntry::new(Course::Algo, "al-def", "Instance", "Qu'est-ce qu'un algorithme\u{a0}?"),
    IndexEntry::new(Course::Algo, "al-def", "Algorithme correct", "Qu'est-ce qu'un algorithme\u{a0}?"),
    IndexEntry::new(Course::Algo, "al-cycle", "Le cycle de développement", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-cycle", "1. Analyse — la phase de réflexion", "Le cycle de développement"),
    IndexEntry::new(Course::Algo, "al-cycle", "2. Conception — l'algorithme", "Le cycle de développement"),
    IndexEntry::new(Course::Algo, "al-cycle", "3. Codage — le programme", "Le cycle de développement"),
    IndexEntry::new(Course::Algo, "al-cycle", "4. Compilation et exécution", "Le cycle de développement"),
    IndexEntry::mono(Course::Algo, "al-cycle", "le problème", "L'identifier précisément. «\u{a0}Plus court chemin\u{a0}»\u{a0}: en temps ou en distance\u{a0}?"),
    IndexEntry::mono(Course::Algo, "al-cycle", "les données", "Plan du réseau, temps entre deux stations, stations concernées."),
    IndexEntry::mono(Course::Algo, "al-cycle", "les résultats", "Lignes à prendre, stations où changer."),
    IndexEntry::mono(Course::Algo, "al-cycle", "les cas particuliers", "Stations non reliées par le réseau\u{a0}?"),
    IndexEntry::mono(Course::Algo, "al-cycle", "le traitement", "Calculer le plus court chemin — puis découper le problème en tâches simples et distinctes."),
    IndexEntry::new(Course::Algo, "al-langages", "Un exemple, quatre langages", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-machine", "Algorithmes et machine", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-machine", "Le modèle RAM", "Algorithmes et machine"),
    IndexEntry::new(Course::Algo, "al-syntaxe", "Fiche de syntaxe algorithmique", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-syntaxe", "La règle qui décide de la moitié des exercices", "Fiche de syntaxe algorithmique"),
    IndexEntry::new(Course::Algo, "al-td1", "Premières notions", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-td1", "Chaque instruction est-elle correcte\u{a0}?", "Premières notions · ex. 1"),
    IndexEntry::new(Course::Algo, "al-td1", "À quel type appartient chaque valeur\u{a0}?", "Premières notions · ex. 2"),
    IndexEntry::new(Course::Algo, "al-td1", "Traces d'exécution.", "Premières notions · ex. 3-4"),
    IndexEntry::new(Course::Algo, "al-td1", "Expressions\u{a0}: de l'algorithme aux maths et retour.", "Premières notions · ex. 5-6"),
    IndexEntry::new(Course::Algo, "al-td1", "Corriger, compléter, tracer.", "Premières notions · ex. 7-9"),
    IndexEntry::new(Course::Algo, "al-td1", "Typage d'expressions mixtes.", "Premières notions · ex. 10"),
    IndexEntry::new(Course::Algo, "al-td1", "Échanger le contenu de variables.", "Premières notions · ex. 11-13"),
    IndexEntry::new(Course::Algo, "al-td1", "Premiers algorithmes complets.", "Premières notions · ex. 14-15"),
    IndexEntry::new(Course::Algo, "al-pieges", "Les pièges du TD 1", "Algorithmique 1"),
    IndexEntry::new(Course::Algo, "al-pieges", "Exercice 10 — typage et évaluation", "Les pièges du TD 1"),
    IndexEntry::new(Course::Algo, "al-pieges", "Exercices 3 et 4 — les traces cachent un mot", "Les pièges du TD 1"),
    IndexEntry::new(Course::Algo, "al-pieges", "Exercice 13 — l'échange sans variable temporaire", "Les pièges du TD 1"),
    IndexEntry::new(Course::Algo, "al-pieges", "Exercice 15 — durée et division euclidienne", "Les pièges du TD 1"),
];

fn masthead() -> impl IntoView {
    view! {
        <header class="masthead">
            <div class="masthead-inner">
                <div class="eyebrow">
                    "Matière 02 · Informatique"
                </div>
                <h1>
                    "Algorithmique 1"
                </h1>
                <p class="lede">
                    "Réfléchir en termes algorithmiques avant d'écrire du code\u{a0}: ce qu'est un algorithme, comment on passe d'un problème à un programme qui tourne, et les premières briques — types, variables, affectations, traces d'exécution."
                </p>
                <div class="meta">
                    <span>
                        "Cours de Jordan Caracotte et Léo Robert"
                    </span>
                    <span>
                        "Licence STS · S1 · 2026-2027"
                    </span>
                    <span>
                        <span class="mono">
                            "CM 1"
                        </span>
                    </span>
                    <span>
                        <span class="mono">
                            "15"
                        </span>
                        " exercices"
                    </span>
                </div>
            </div>
        </header>
    }
}

fn sec_al_orga() -> impl IntoView {
    view! {
        <section id="al-orga">
            <div class="sec-head">
                <span class="num">
                    "01"
                </span>
                <h2>
                    "Organisation et évaluation"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Module "
                    <strong>
                        "indispensable"
                    </strong>
                    " pour la licence d'informatique\u{a0}: les notions s'accumulent au fil du semestre, un retard ne se rattrape pas seul."
                </p>
            </div>
            <div class="tw" style="max-width:68ch;margin-top:16px">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Format"
                            </th>
                            <th>
                                "Volume"
                            </th>
                            <th>
                                "Détail"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                "CM"
                            </td>
                            <td class="mono">
                                "6 × 2 h"
                            </td>
                            <td>
                                "répartis entre deux enseignants"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "TD"
                            </td>
                            <td class="mono">
                                "12 × 2 h"
                            </td>
                            <td>
                                "sujets imprimables à l'avance\u{a0}; ni téléphone, ni PC, ni tablette"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "TP"
                            </td>
                            <td class="mono">
                                "10 × 2 h"
                            </td>
                            <td>
                                "langage C, dont 4 séances sur Arduino\u{a0}; garder ses notes"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note" style="margin-top:18px">
                <span class="title">
                    "Note finale"
                </span>
                <span class="mono" style="display:block;font-size:14px">
                    "30 % QCM + 35 % partiel + 35 % examen terminal"
                </span>
                <p>
                    "Deux QCM de 30 minutes sur papier pendant les TP, un partiel d'1 h à mi-parcours, un examen terminal d'1 h. Des outils accompagnent le module sur MesCoursJV\u{a0}: un traducteur algorithme/code et un interpréteur."
                </p>
            </div>
            <div class="body" style="margin-top:16px">
                <p>
                    <strong>
                        "Objectif affiché\u{a0}:"
                    </strong>
                    " acquérir les techniques de base de l'algorithmique et de la programmation, c'est-à-dire réfléchir en termes algorithmiques et savoir transmettre des instructions à une machine. Le travail personnel hors TD fait partie des attentes."
                </p>
            </div>
        </section>
    }
}

fn sec_al_def() -> impl IntoView {
    view! {
        <section id="al-def">
            <div class="sec-head">
                <span class="num">
                    "02"
                </span>
                <h2>
                    "Qu'est-ce qu'un algorithme\u{a0}?"
                </h2>
            </div>
            <dl class="deflist">
                <div class="def">
                    <dt>
                        "Algorithme "
                        <span style="font-weight:400;font-size:14px;color:var(--ink-soft)">
                            "définition informelle"
                        </span>
                    </dt>
                    <dd>
                        "Une procédure de calcul bien définie qui prend en entrée un ensemble de valeurs et produit en sortie un ensemble de valeurs. C'est donc une suite d'étapes de calcul qui transforment l'entrée en sortie, et qui résout un problème de calcul "
                        <em>
                            "bien spécifié"
                        </em>
                        "."
                    </dd>
                </div>
                <div class="def">
                    <dt>
                        "Instance"
                    </dt>
                    <dd>
                        "Un jeu de données particulier donné en entrée du problème."
                    </dd>
                </div>
                <div class="def">
                    <dt>
                        "Algorithme correct"
                    </dt>
                    <dd>
                        "Celui qui, pour "
                        <strong>
                            "chaque"
                        </strong>
                        " instance du problème, produit la bonne sortie. La plupart de ceux vus en licence sont en plus "
                        <em>
                            "efficaces"
                        </em>
                        "\u{a0}: la mesure habituelle est la vitesse de résolution — mais la mémoire, l'énergie ou la simplicité comptent aussi dans un contexte concret."
                    </dd>
                </div>
            </dl>
            <div class="ex" style="margin-top:18px">
                <div class="ex-head">
                    <span>
                        "Le problème du tri"
                    </span>
                    <span class="mono">
                        "spécification"
                    </span>
                </div>
                <div class="ex-body">
                    <span class="mono">
                        "Entrée : une suite de n nombres (a₁, …, aₙ)\nSortie : une permutation (a′₁, …, a′ₙ) de cette suite\n         telle que a′₁ ≤ a′₂ ≤ … ≤ a′ₙ\n\nInstance : (31, 41, 59, 26, 41, 58)\nSortie   : "
                        <span class="res">
                            "(26, 31, 41, 41, 58, 59)"
                        </span>
                    </span>
                </div>
            </div>
            <h3>
                "Les entrées\u{a0}: quelles informations, sous quelle forme\u{a0}?"
            </h3>
            <div class="body">
                <p>
                    "Des nombres (entiers, réels, signés ou non), du texte (avec ses encodages ASCII, UTF-8…), des images (jpeg, png, gif, bmp, svg), du son (wav, mp3, ogg, aac), de la vidéo (MPEG-2, H.264)… et toutes ces informations sont, dans la machine, "
                    <strong>
                        "du code binaire"
                    </strong>
                    " — des 0 et des 1."
                </p>
                <p style="color:var(--ink-soft);font-size:14px">
                    "C'est exactement le sujet de la matière 03 de ce classeur, "
                    <em>
                        "Représentation de l'information"
                    </em>
                    "\u{a0}: les deux cours se rejoignent ici."
                </p>
            </div>
        </section>
    }
}

fn sec_al_cycle() -> impl IntoView {
    view! {
        <section id="al-cycle">
            <div class="sec-head">
                <span class="num">
                    "03"
                </span>
                <h2>
                    "Le cycle de développement"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Du problème au résultat, quatre étapes et trois objets intermédiaires\u{a0}:"
                </p>
            </div>
            <div class="tree" style="margin-top:16px">
                <pre>
                    "  "
                    <span class="cmt">
                        "analyse"
                    </span>
                    "        "
                    <span class="cmt">
                        "conception"
                    </span>
                    "       "
                    <span class="cmt">
                        "codage"
                    </span>
                    "          "
                    <span class="cmt">
                        "test"
                    </span>
                    "\nproblème  →   problème    →   "
                    <span class="root">
                        "algorithme"
                    </span>
                    "  →   programme   →   résultat\nà résoudre    spécifique"
                </pre>
            </div>
            <div class="body" style="margin-top:16px">
                <p>
                    "Fil rouge du cours\u{a0}: "
                    <em>
                        "donner le plus court chemin en métro entre deux stations"
                    </em>
                    "."
                </p>
            </div>
            <h3>
                "1. Analyse — la phase de réflexion"
            </h3>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "le problème"
                    </span>
                    <span class="d">
                        "L'identifier précisément. «\u{a0}Plus court chemin\u{a0}»\u{a0}: en temps ou en distance\u{a0}?"
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "les données"
                    </span>
                    <span class="d">
                        "Plan du réseau, temps entre deux stations, stations concernées."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "les résultats"
                    </span>
                    <span class="d">
                        "Lignes à prendre, stations où changer."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "les cas particuliers"
                    </span>
                    <span class="d">
                        "Stations non reliées par le réseau\u{a0}?"
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "le traitement"
                    </span>
                    <span class="d">
                        "Calculer le plus court chemin — puis découper le problème en tâches simples et distinctes."
                    </span>
                </div>
            </div>
            <h3>
                "2. Conception — l'algorithme"
            </h3>
            <div class="body">
                <p>
                    "Une suite d'opérations qui fait "
                    <strong>
                        "ressortir la logique"
                    </strong>
                    " de la résolution, proche des langages de programmation mais indépendante d'eux, et correcte quelles que soient les valeurs des données."
                </p>
            </div>
            <h3>
                "3. Codage — le programme"
            </h3>
            <div class="body">
                <p>
                    "Un ou plusieurs fichiers texte d'instructions écrites dans un langage de programmation (C, Java, Fortran, Cobol, Basic…). Certains sous-programmes peuvent vivre dans des fichiers annexes\u{a0}: une "
                    <strong>
                        "bibliothèque de fonctions"
                    </strong>
                    ". La "
                    <em>
                        "programmation"
                    </em>
                    " est la traduction du langage algorithmique vers le langage de programmation."
                </p>
            </div>
            <h3>
                "4. Compilation et exécution"
            </h3>
            <div class="term" style="margin-top:14px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "compiler puis exécuter"
                    </span>
                </div>
                <pre>
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "gcc cheminMetro.c -o cheminMetro"
                    </span>
                    "   "
                    <span class="hash">
                        "# traduction en langage machine"
                    </span>
                    "\n"
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "./cheminMetro"
                    </span>
                    "                      "
                    <span class="hash">
                        "# l'ordinateur exécute les instructions"
                    </span>
                </pre>
            </div>
        </section>
    }
}

fn sec_al_langages() -> impl IntoView {
    view! {
        <section id="al-langages">
            <div class="sec-head">
                <span class="num">
                    "04"
                </span>
                <h2>
                    "Un exemple, quatre langages"
                </h2>
            </div>
            <div class="body">
                <p>
                    <strong>
                        "Problème\u{a0}:"
                    </strong>
                    " saluer une personne qui donne son nom. "
                    <strong>
                        "Spécification\u{a0}:"
                    </strong>
                    " donnée = le nom\u{a0}; résultat = afficher «\u{a0}bonjour\u{a0}» suivi du nom."
                </p>
            </div>
            <div class="ex" style="margin-top:16px">
                <div class="ex-head">
                    <span>
                        "L'algorithme"
                    </span>
                    <span class="mono">
                        "langage algorithmique"
                    </span>
                </div>
                <div class="ex-body">
                    <span class="mono">
                        "variables\n    chaîne nom, salutation ;\ndébut\n    écrire \"Quel est votre nom ?\" ;\n    lire nom ;\n    salutation ← \"Bonjour \" + nom ;\n    écrire salutation ;\nfin"
                    </span>
                </div>
            </div>
            <div class="term" style="margin-top:16px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "hello.c — gcc hello.c -o hello; ./hello"
                    </span>
                </div>
                <pre>
                    <span class="cm">
                        "#include"
                    </span>
                    " <stdio.h>\n"
                    <span class="cm">
                        "#include"
                    </span>
                    " <string.h>\ntypedef char string[1024];\nint main() {\n    string nom, salutation;\n    printf(\"Quel est votre nom ? \");\n    scanf(\"%s\", nom);\n    sprintf(salutation, \"Bonjour %s\", nom);\n    printf(\"%s\\n\", salutation);\n}"
                </pre>
            </div>
            <div class="term" style="margin-top:14px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "Hello.java — javac Hello.java; java Hello"
                    </span>
                </div>
                <pre>
                    "import java.util.Scanner;\npublic class Hello {\n    public static void main(String[] args) {\n        String nom, salutation;\n        System.out.println(\"Quel est votre nom ?\");\n        nom = (new Scanner(System.in)).next();\n        salutation = \"Bonjour \" + nom;\n        System.out.println(salutation);\n    }\n}"
                </pre>
            </div>
            <div class="term" style="margin-top:14px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "hello.py — python3 hello.py"
                    </span>
                </div>
                <pre>
                    "if __name__ == '__main__':\n    nom = input(\"Quel est votre nom ? \")\n    salutation = \"Bonjour \" + nom\n    print(salutation)"
                </pre>
            </div>
            <div class="body" style="margin-top:16px">
                <p>
                    "Le cours montre aussi la version "
                    <strong>
                        "Scratch"
                    </strong>
                    "\u{a0}: même algorithme, quatre syntaxes. C'est précisément l'intérêt de raisonner d'abord en pseudo-code."
                </p>
            </div>
        </section>
    }
}

fn sec_al_machine() -> impl IntoView {
    view! {
        <section id="al-machine">
            <div class="sec-head">
                <span class="num">
                    "05"
                </span>
                <h2>
                    "Algorithmes et machine"
                </h2>
            </div>
            <div class="body">
                <p>
                    <em>
                        "Pourquoi existe-t-il autant de langages de programmation\u{a0}?"
                    </em>
                    " Parce que les performances globales d'un système dépendent autant des algorithmes que du matériel. Quatre contraintes reviennent\u{a0}:"
                </p>
            </div>
            <ol class="steps" style="margin-top:14px">
                <li>
                    "Une solution doit "
                    <strong>
                        "s'arrêter"
                    </strong>
                    " à un moment — pas de boucle infinie."
                </li>
                <li>
                    "La solution la plus "
                    <strong>
                        "simple"
                    </strong>
                    " doit être privilégiée."
                </li>
                <li>
                    "La "
                    <strong>
                        "ressource est toujours limitée"
                    </strong>
                    "\u{a0}: temps, mémoire."
                </li>
                <li>
                    "L'"
                    <strong>
                        "inter-opérabilité"
                    </strong>
                    " compte."
                </li>
            </ol>
            <div class="note" style="margin-top:18px">
                <span class="title">
                    "Le modèle RAM"
                </span>
                <p>
                    "Traditionnellement, on considère que la technologie est modélisée par une "
                    <strong>
                        "machine à accès aléatoire"
                    </strong>
                    "\u{a0}: les instructions s'exécutent l'une après l'autre, sans opérations simultanées\u{a0}; les instructions sont arithmétiques, de transfert de données (lecture, stockage) ou de contrôle (appel de sous-routine)\u{a0}; et chacune a un "
                    <strong>
                        "temps d'exécution constant"
                    </strong>
                    "."
                </p>
                <p>
                    "C'est ce modèle qui rendra plus tard légitime de compter les opérations pour mesurer la complexité."
                </p>
            </div>
        </section>
    }
}

fn sec_al_syntaxe() -> impl IntoView {
    view! {
        <section id="al-syntaxe">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Fiche de syntaxe algorithmique"
                </h2>
            </div>
            <div class="body">
                <p>
                    "La notation utilisée en TD, telle qu'elle apparaît dans les énoncés."
                </p>
            </div>
            <div class="tw" style="max-width:68ch;margin-top:16px">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Élément"
                            </th>
                            <th>
                                "Écriture"
                            </th>
                            <th>
                                "Remarque"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                "Structure"
                            </td>
                            <td class="mono">
                                "variables … début … fin"
                            </td>
                            <td>
                                "toute variable utilisée doit être déclarée"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Affectation"
                            </td>
                            <td class="mono">
                                "x ← 3 ;"
                            </td>
                            <td>
                                "la flèche, pas le signe ="
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Types"
                            </td>
                            <td class="mono">
                                "entier, réel, chaîne, caractère, booléen"
                            </td>
                            <td>
                                "le type contraint les valeurs acceptées"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Chaîne"
                            </td>
                            <td class="mono">
                                "\"bonjour\""
                            </td>
                            <td>
                                "guillemets doubles\u{a0}; concaténation avec "
                                <span class="mono">
                                    "+"
                                </span>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Caractère"
                            </td>
                            <td class="mono">
                                "'a'"
                            </td>
                            <td>
                                "apostrophes, "
                                <strong>
                                    "un seul"
                                </strong>
                                " caractère"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Booléen"
                            </td>
                            <td class="mono">
                                "vrai, faux"
                            </td>
                            <td>
                                <span class="mono">
                                    "\"faux\""
                                </span>
                                " entre guillemets est une chaîne, pas un booléen"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Division entière"
                            </td>
                            <td class="mono">
                                "a div b"
                            </td>
                            <td>
                                "quotient\u{a0}; "
                                <span class="mono">
                                    "a mod b"
                                </span>
                                " pour le reste"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Conversion"
                            </td>
                            <td class="mono">
                                "(entier)(c / d)"
                            </td>
                            <td>
                                "tronque vers zéro"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Logique"
                            </td>
                            <td class="mono">
                                "et, ou, non"
                            </td>
                            <td>
                                "opèrent sur des booléens"
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "Entrées / sorties"
                            </td>
                            <td class="mono">
                                "lire x ; écrire x ;"
                            </td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="note warn" style="margin-top:18px">
                <span class="title">
                    "La règle qui décide de la moitié des exercices"
                </span>
                <p>
                    "Les opérateurs de même priorité s'évaluent "
                    <strong>
                        "de gauche à droite"
                    </strong>
                    ". Ainsi "
                    <span class="fi">
                        "a*b/c"
                    </span>
                    " vaut "
                    <span class="fi">
                        "(a×b)/c"
                    </span>
                    ", "
                    <span class="fi">
                        "a/b*c"
                    </span>
                    " vaut "
                    <span class="fi">
                        "(a/b)×c"
                    </span>
                    ", et "
                    <span class="fi">
                        "5 * 4 div 3"
                    </span>
                    " vaut "
                    <span class="fi">
                        "20 div 3 = 6"
                    </span>
                    " — pas "
                    <span class="fi">
                        "5 × 1"
                    </span>
                    "."
                </p>
            </div>
        </section>
    }
}

fn sec_al_td1() -> impl IntoView {
    view! {
        <section id="al-td1">
            <div class="sec-head">
                <span class="num">
                    "TD 1"
                </span>
                <h2>
                    "Premières notions"
                </h2>
            </div>
            <div class="docmeta">
                <span class="pill on">
                    "Série 1"
                </span>
                <span class="pill">
                    "15 exercices"
                </span>
                <span class="pill">
                    "types"
                </span>
                <span class="pill">
                    "affectations"
                </span>
                <span class="pill">
                    "traces d'exécution"
                </span>
                <span class="pill">
                    "expressions"
                </span>
            </div>
            <div class="exos">
                <div class="exo">
                    <span class="n">
                        "1"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Chaque instruction est-elle correcte\u{a0}?"
                        </span>
                        <span class="sub">
                            "Un algorithme de 14 lignes avec "
                            <span class="mono">
                                "entier e, f, g"
                            </span>
                            " et "
                            <span class="mono">
                                "chaîne c, c1, c2"
                            </span>
                            ". À traquer\u{a0}: affecter 3.5 à un entier, une chaîne sans guillemets, un nom de type utilisé comme variable, une variable non déclarée, une variable lue avant d'avoir été initialisée."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "2"
                    </span>
                    <span class="c">
                        <span class="t">
                            "À quel type appartient chaque valeur\u{a0}?"
                        </span>
                        <span class="sub">
                            <span class="mono">
                                "2 · 2.5 · \"bonjour\" · \"faux\" · 'a' · 'au revoir' · 3.14 · pi · oui · vrai"
                            </span>
                            " — trois des dix ne sont pas des valeurs valides."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "3-4"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Traces d'exécution."
                        </span>
                        <span class="sub">
                            "Trois algorithmes à dérouler ligne à ligne, en notant la valeur de chaque variable après chaque instruction. Deux d'entre eux cachent un mot — voir ci-dessous."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "5-6"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Expressions\u{a0}: de l'algorithme aux maths et retour."
                        </span>
                        <span class="sub">
                            "Traduire "
                            <span class="mono">
                                "a*b/c"
                            </span>
                            ", "
                            <span class="mono">
                                "a/b*c"
                            </span>
                            ", "
                            <span class="mono">
                                "a/b/c"
                            </span>
                            "\u{a0}; écrire "
                            <span class="mono">
                                "(3x + 5y(z−6)) / 4x"
                            </span>
                            " en langage algorithmique\u{a0}; évaluer sans calculatrice "
                            <span class="mono">
                                "3*x-8/y*(4*z)+2.5"
                            </span>
                            " pour (10.0, 1.0, 2.0), et "
                            <span class="mono">
                                "5 * 4 div 3"
                            </span>
                            "."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "7-9"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Corriger, compléter, tracer."
                        </span>
                        <span class="sub">
                            "Un algorithme utilise "
                            <span class="mono">
                                "c"
                            </span>
                            " sans l'avoir déclarée ni lue\u{a0}: corriger, puis le modifier pour faire saisir a et c et afficher b, puis donner la trace pour les valeurs 12 et 4."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "10"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Typage d'expressions mixtes."
                        </span>
                        <span class="sub">
                            "Six affectations à typer et à évaluer, avec "
                            <span class="mono">
                                "a = 5, b = 2, c = 7.5, d = 3.0, e = vrai"
                            </span>
                            ". Le cœur du TD."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "11-13"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Échanger le contenu de variables."
                        </span>
                        <span class="sub">
                            "Échange de deux variables (change-t-il selon le type\u{a0}? et si x est entier et y réel\u{a0}?), permutation circulaire de trois variables, puis la séquence "
                            <span class="mono">
                                "x←x+y ; y←x−y ; x←x−y"
                            </span>
                            " à identifier."
                        </span>
                    </span>
                </div>
                <div class="exo">
                    <span class="n">
                        "14-15"
                    </span>
                    <span class="c">
                        <span class="t">
                            "Premiers algorithmes complets."
                        </span>
                        <span class="sub">
                            "Moyenne de deux réels saisis\u{a0}; conversion d'une durée h/min/s en secondes, puis l'opération inverse (celle-ci se fait avec "
                            <span class="mono">
                                "div"
                            </span>
                            " et "
                            <span class="mono">
                                "mod"
                            </span>
                            ")."
                        </span>
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_al_pieges() -> impl IntoView {
    view! {
        <section id="al-pieges">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Les pièges du TD 1"
                </h2>
            </div>
            <h3>
                "Exercice 10 — typage et évaluation"
            </h3>
            <div class="tw" style="max-width:68ch">
                <table class="compact">
                    <thead>
                        <tr>
                            <th>
                                "Instruction"
                            </th>
                            <th>
                                "Type"
                            </th>
                            <th>
                                "Valeur"
                            </th>
                            <th>
                                "Pourquoi"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">
                                "x ← a div b * 2 + 1"
                            </td>
                            <td>
                                "entier"
                            </td>
                            <td class="mono">
                                "5"
                            </td>
                            <td>
                                "(5 div 2) = 2, puis 2×2+1"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "y ← c / d * 2 + 1"
                            </td>
                            <td>
                                "réel"
                            </td>
                            <td class="mono">
                                "6.0"
                            </td>
                            <td>
                                "7.5/3.0 = 2.5, puis ×2 +1"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "z ← a mod b + 1"
                            </td>
                            <td>
                                "entier"
                            </td>
                            <td class="mono">
                                "2"
                            </td>
                            <td>
                                "5 mod 2 = 1"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "v ← (entier)(c / d) + 1"
                            </td>
                            <td>
                                "entier"
                            </td>
                            <td class="mono">
                                "3"
                            </td>
                            <td>
                                "la conversion tronque 2.5 en 2"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "w ← (a div b < 0) et (non e ou (a = b))"
                            </td>
                            <td>
                                "booléen"
                            </td>
                            <td class="mono">
                                "faux"
                            </td>
                            <td>
                                "2 < 0 est faux, le «\u{a0}et\u{a0}» suffit à conclure"
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "t ← d / b"
                            </td>
                            <td>
                                "réel"
                            </td>
                            <td class="mono">
                                "1.5"
                            </td>
                            <td>
                                "un réel divisé par un entier reste réel"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <h3>
                "Exercices 3 et 4 — les traces cachent un mot"
            </h3>
            <div class="ex" style="margin-top:14px">
                <div class="ex-head">
                    <span>
                        "Exercice 3"
                    </span>
                    <span class="mono">
                        "chaînes"
                    </span>
                </div>
                <div class="ex-body">
                    <span class="mono">
                        "a ← \"0\"                          a = \"0\"\ne ← 2                            e = 2\nd ← 3 * e + 4                    d = 10\nb ← \"bonjour \" + a + a + \"7\"     b = \"bonjour 007\"\nb ← \"a\"                          b = \"a\"      ← la valeur précédente est perdue\na ← \"br\"                         a = \"br\"\nc ← \"toto\"                       c = \"toto\"\nc ← b+a+b+\"c\"+b+\"d\"+b+a+b        c = "
                        <span class="res">
                            "\"abracadabra\""
                        </span>
                    </span>
                </div>
            </div>
            <div class="ex" style="margin-top:14px">
                <div class="ex-head">
                    <span>
                        "Exercice 4 — second algorithme"
                    </span>
                    <span class="mono">
                        "chaînes"
                    </span>
                </div>
                <div class="ex-body">
                    <span class="mono">
                        "b ← \"r\"                b = \"r\"\na ← \"e\"                a = \"e\"\nc ← b + a + \"po\"       c = \"repo\"\nb ← b + b + a          b = \"rre\"\nc ← c + \"ns\" + a + \" \" c = \"reponse \"\na ← b + \"ct\" + a       a = \"rrecte\"\na ← c + \"co\" + a       a = "
                        <span class="res">
                            "\"reponse correcte\""
                        </span>
                    </span>
                    <p style="margin-top:10px">
                        "Premier algorithme, celui avec des entiers\u{a0}: on termine sur "
                        <span class="fi">
                            "a = 7, x = 11, y = 2, z = 4"
                        </span>
                        " — le "
                        <span class="fi">
                            "y ← 2*x + z"
                        </span>
                        " est écrasé par le "
                        <span class="fi">
                            "y ← z div 2"
                        </span>
                        " qui suit."
                    </p>
                </div>
            </div>
            <h3>
                "Exercice 13 — l'échange sans variable temporaire"
            </h3>
            <div class="ex">
                <div class="ex-head">
                    <span>
                        "Que fait cette séquence\u{a0}?"
                    </span>
                    <span class="mono">
                        "x₀, y₀ au départ"
                    </span>
                </div>
                <div class="ex-body">
                    <span class="mono">
                        "x ← x + y      x = x₀ + y₀        y = y₀\ny ← x - y      x = x₀ + y₀        y = x₀\nx ← x - y      x = "
                        <span class="res">
                            "y₀"
                        </span>
                        "              y = "
                        <span class="res">
                            "x₀"
                        </span>
                    </span>
                    <p style="margin-top:10px">
                        "C'est un "
                        <strong>
                            "échange"
                        </strong>
                        " de x et y sans variable intermédiaire. Astucieux, mais l'échange classique reste préférable\u{a0}: "
                        <span class="fi">
                            "tmp ← x ; x ← y ; y ← tmp"
                        </span>
                        ", qui fonctionne pour "
                        <em>
                            "tous"
                        </em>
                        " les types alors que celui-ci exige des nombres — et déborde si les valeurs sont proches du maximum représentable, exactement le dépassement de capacité de la matière 03."
                    </p>
                </div>
            </div>
            <h3>
                "Exercice 15 — durée et division euclidienne"
            </h3>
            <div class="ex">
                <div class="ex-head">
                    <span>
                        "Secondes vers h / min / s"
                    </span>
                    <span class="mono">
                        "div et mod"
                    </span>
                </div>
                <div class="ex-body">
                    <span class="mono">
                        "lire total ;\nh   ← total div 3600 ;\nr   ← total mod 3600 ;\nmin ← r div 60 ;\ns   ← r mod 60 ;\nécrire h, min, s ;\n\n"
                        <span class="hash">
                            "// dans l'autre sens :  total ← h * 3600 + min * 60 + s"
                        </span>
                    </span>
                    <p style="margin-top:10px">
                        "C'est la même division euclidienne que dans le TD 4 de Structures fondamentales\u{a0}: "
                        <span class="fi">
                            "a = bq + r"
                        </span>
                        " avec "
                        <span class="fi">
                            "0 ≤ r < b"
                        </span>
                        "."
                    </p>
                </div>
            </div>
        </section>
    }
}

/// Page complète de la matière.
pub fn page() -> impl IntoView {
    view! {
        <div class="course c-algo">
            {masthead()}
            <div class="shell">
                <Toc course=Course::Algo items=TOC label=TOC_LABEL/>
                <main>
                    {sec_al_orga()}
                    {sec_al_def()}
                    {sec_al_cycle()}
                    {sec_al_langages()}
                    {sec_al_machine()}
                    {sec_al_syntaxe()}
                    {sec_al_td1()}
                    {sec_al_pieges()}
                </main>
            </div>
        </div>
    }
}
