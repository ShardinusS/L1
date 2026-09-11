//! Matiere 04 - Systemes d'exploitation.
//!
//! Contenu transposé tel quel depuis le classeur HTML d'origine :
//! mêmes textes, mêmes formules, mêmes exemples.

use leptos::prelude::*;

use crate::components::cmd_table::CmdTable;
use crate::components::toc::{Toc, TocItem};
use crate::course::Course;
use crate::search::IndexEntry;

/// Ancres des sections de la page, dans l'ordre d'affichage.
#[rustfmt::skip]
pub const SECTIONS: &[&str] = &[
    "o-os",
    "o-arbo",
    "o-deplacer",
    "o-creer",
    "o-lire",
    "o-gerer",
    "o-aide",
    "o-composer",
    "o-recap",
    "o-relecture",
];

/// Sommaire de la matière : (ancre, numéro, titre).
#[rustfmt::skip]
pub const TOC: &[TocItem] = &[
    TocItem { anchor: "o-os", num: "01", title: "Le système d'exploitation" },
    TocItem { anchor: "o-arbo", num: "02", title: "L'arborescence" },
    TocItem { anchor: "o-deplacer", num: "03", title: "Se repérer et se déplacer" },
    TocItem { anchor: "o-creer", num: "04", title: "Créer fichiers et répertoires" },
    TocItem { anchor: "o-lire", num: "05", title: "Lire un fichier" },
    TocItem { anchor: "o-gerer", num: "06", title: "Copier, déplacer, supprimer" },
    TocItem { anchor: "o-aide", num: "07", title: "Obtenir de l'aide" },
    TocItem { anchor: "o-composer", num: "08", title: "Composer des commandes" },
    TocItem { anchor: "o-recap", num: "—", title: "Aide-mémoire" },
    TocItem { anchor: "o-relecture", num: "—", title: "Notes de relecture" },
];

/// Libellé accessible du sommaire.
pub const TOC_LABEL: &str = "Sommaire du cours Systèmes d'exploitation";

/// Entrées de recherche de la matière, dans l'ordre du cours.
#[rustfmt::skip]
pub const INDEX: &[IndexEntry] = &[
    IndexEntry::new(Course::Os, "o-os", "Le système d'exploitation", "Systèmes d'exploitation"),
    IndexEntry::new(Course::Os, "o-arbo", "L'arborescence de fichiers", "Systèmes d'exploitation"),
    IndexEntry::mono(Course::Os, "o-arbo", "/", "La racine de l'arborescence."),
    IndexEntry::mono(Course::Os, "o-arbo", ".", "Le répertoire courant — «\u{a0}moi\u{a0}»."),
    IndexEntry::mono(Course::Os, "o-arbo", "..", "Le répertoire père. D'où cd .. pour remonter d'un cran."),
    IndexEntry::new(Course::Os, "o-deplacer", "Se repérer et se déplacer", "Systèmes d'exploitation"),
    IndexEntry::mono(Course::Os, "o-deplacer", "pwd", "Affiche l'emplacement courant dans l'arborescence."),
    IndexEntry::mono(Course::Os, "o-deplacer", "ls", "Liste tous les fichiers du répertoire."),
    IndexEntry::mono(Course::Os, "o-deplacer", "ls -l", "Liste les fichiers avec plus d'informations\u{a0}: les lignes qui commencent par d sont des dossiers, celles qui commencent par - sont des fichiers."),
    IndexEntry::mono(Course::Os, "o-deplacer", "ls -a", "Affiche aussi les fichiers cachés\u{a0}: tous les fichiers dont le nom commence par un . ne s'affichent pas par défaut."),
    IndexEntry::mono(Course::Os, "o-deplacer", "cd", "Change de répertoire."),
    IndexEntry::mono(Course::Os, "o-deplacer", "file", "Détermine le type d'un fichier."),
    IndexEntry::new(Course::Os, "o-creer", "Créer des fichiers et des répertoires", "Systèmes d'exploitation"),
    IndexEntry::new(Course::Os, "o-creer", "Les redirections", "Créer des fichiers et des répertoires"),
    IndexEntry::mono(Course::Os, "o-creer", "mkdir", "Crée un répertoire."),
    IndexEntry::mono(Course::Os, "o-creer", "touch", "Crée un fichier (vide)."),
    IndexEntry::mono(Course::Os, "o-creer", "nano", "Édite un fichier dans un éditeur de texte."),
    IndexEntry::mono(Course::Os, "o-creer", "commande > fichier.txt", "Écrit dans le fichier le texte affiché par la commande — le contenu précédent est écrasé. ✱"),
    IndexEntry::mono(Course::Os, "o-creer", "commande >> fichier.txt", "Ajoute le texte affiché à la fin du fichier, sans rien effacer."),
    IndexEntry::new(Course::Os, "o-lire", "Lire le contenu d'un fichier", "Systèmes d'exploitation"),
    IndexEntry::mono(Course::Os, "o-lire", "cat", "Voir le contenu du fichier, d'un seul bloc."),
    IndexEntry::mono(Course::Os, "o-lire", "less", "Affiche le contenu d'un fichier page par page."),
    IndexEntry::mono(Course::Os, "o-lire", "head", "Affiche juste le début d'un fichier."),
    IndexEntry::mono(Course::Os, "o-lire", "tail", "Affiche la fin d'un fichier."),
    IndexEntry::mono(Course::Os, "o-lire", "-n 3", "Avec head ou tail\u{a0}: les 3 premières ou 3 dernières lignes."),
    IndexEntry::mono(Course::Os, "o-lire", "wc", "Donne le nombre de lignes, de mots et de caractères."),
    IndexEntry::new(Course::Os, "o-gerer", "Copier, déplacer, supprimer", "Systèmes d'exploitation"),
    IndexEntry::mono(Course::Os, "o-gerer", "cp source destination", "Copie un fichier."),
    IndexEntry::mono(Course::Os, "o-gerer", "mv", "Renomme ou déplace un fichier."),
    IndexEntry::mono(Course::Os, "o-gerer", "rm", "Supprime un fichier."),
    IndexEntry::mono(Course::Os, "o-gerer", "rmdir", "Supprime un répertoire sans fichiers à l'intérieur."),
    IndexEntry::new(Course::Os, "o-gerer", "Pas de corbeille", "Copier, déplacer, supprimer"),
    IndexEntry::new(Course::Os, "o-aide", "Obtenir de l'aide", "Systèmes d'exploitation"),
    IndexEntry::mono(Course::Os, "o-aide", "man commande", "Affiche le manuel de la commande. — je connais la commande, je cherche ses options."),
    IndexEntry::mono(Course::Os, "o-aide", "apropos", "Cherche la commande à utiliser pour faire quelque chose. — je connais le besoin, je cherche la commande."),
    IndexEntry::new(Course::Os, "o-composer", "Composer des commandes", "Systèmes d'exploitation"),
    IndexEntry::new(Course::Os, "o-composer", "Les options de sélection ✱", "Composer des commandes"),
    IndexEntry::mono(Course::Os, "o-composer", "cut", "Sélectionner."),
    IndexEntry::mono(Course::Os, "o-composer", "paste", "Assembler."),
    IndexEntry::mono(Course::Os, "o-composer", "sort", "Trier."),
    IndexEntry::mono(Course::Os, "o-composer", "sort -r", "Trier dans le sens inverse."),
    IndexEntry::mono(Course::Os, "o-composer", "sort -n", "Trier de manière numérique."),
    IndexEntry::mono(Course::Os, "o-composer", "uniq", "Si quelque chose est présent en plusieurs exemplaires, n'en laisse qu'un seul."),
    IndexEntry::mono(Course::Os, "o-composer", "|", "Le pipe (tube)\u{a0}: sert à combiner deux commandes, la sortie de la première devient l'entrée de la seconde."),
    IndexEntry::new(Course::Os, "o-recap", "Aide-mémoire", "Systèmes d'exploitation"),
    IndexEntry::new(Course::Os, "o-relecture", "Notes de relecture", "Systèmes d'exploitation"),
];

fn masthead() -> impl IntoView {
    view! {
        <header class="masthead">
            <div class="masthead-inner">
                <div class="eyebrow">
                    "Matière 04 · Notes personnelles"
                </div>
                <h1>
                    "Systèmes d'exploitation"
                </h1>
                <p class="lede">
                    "Prise en main de Linux en ligne de commande\u{a0}: l'arborescence de fichiers, les commandes de base pour se déplacer, créer, lire et supprimer, et la composition de commandes avec les tubes."
                </p>
                <div class="meta">
                    <span>
                        "Lundi 7 septembre 2026"
                    </span>
                    <span>
                        <span class="mono">
                            "8"
                        </span>
                        " chapitres"
                    </span>
                    <span>
                        <span class="mono">
                            "30+"
                        </span>
                        " commandes"
                    </span>
                </div>
            </div>
        </header>
    }
}

fn sec_o_os() -> impl IntoView {
    view! {
        <section id="o-os">
            <div class="sec-head">
                <span class="num">
                    "01"
                </span>
                <h2>
                    "Le système d'exploitation"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Le "
                    <strong>
                        "système d'exploitation"
                    </strong>
                    " sert à faire fonctionner notre PC\u{a0}: Windows, Linux, macOS, iOS, Android."
                </p>
                <p>
                    "Objectif du cours\u{a0}: la "
                    <strong>
                        "ligne de commande"
                    </strong>
                    ", pour manipuler Linux."
                </p>
            </div>
            <div class="term" style="margin-top:18px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "bash"
                    </span>
                </div>
                <pre>
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "echo bonjour"
                    </span>
                    "\nbonjour        "
                    <span class="hash">
                        "# echo répète ce que j'écris"
                    </span>
                </pre>
            </div>
        </section>
    }
}

fn sec_o_arbo() -> impl IntoView {
    view! {
        <section id="o-arbo">
            <div class="sec-head">
                <span class="num">
                    "02"
                </span>
                <h2>
                    "L'arborescence de fichiers"
                </h2>
            </div>
            <div class="body">
                <p>
                    "Les fichiers sont rangés dans des répertoires, eux-mêmes rangés dans d'autres répertoires\u{a0}: c'est l'"
                    <strong>
                        "arborescence"
                    </strong>
                    ". Elle part d'un point unique, la racine."
                </p>
            </div>
            <div class="tree" style="margin-top:16px">
                <pre>
                    <span class="root">
                        "/"
                    </span>
                    "                     "
                    <span class="cmt">
                        "← la racine"
                    </span>
                    "\n├── home\n│   └── kenzo         "
                    <span class="cmt">
                        "← répertoire courant «\u{a0}.\u{a0}»"
                    </span>
                    "\n│       ├── cours\n│       └── notes.txt\n├── etc\n└── var"
                </pre>
            </div>
            <div class="cmds" style="margin-top:22px">
                <div class="cmd">
                    <span class="c">
                        "/"
                    </span>
                    <span class="d">
                        "La racine de l'arborescence."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "."
                    </span>
                    <span class="d">
                        "Le répertoire courant — «\u{a0}moi\u{a0}»."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        ".."
                    </span>
                    <span class="d">
                        "Le répertoire père. D'où "
                        <span class="mono">
                            "cd .."
                        </span>
                        " pour remonter d'un cran."
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_o_deplacer() -> impl IntoView {
    view! {
        <section id="o-deplacer">
            <div class="sec-head">
                <span class="num">
                    "03"
                </span>
                <h2>
                    "Se repérer et se déplacer"
                </h2>
            </div>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "pwd"
                    </span>
                    <span class="d">
                        "Affiche l'emplacement courant dans l'arborescence."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "ls"
                    </span>
                    <span class="d">
                        "Liste tous les fichiers du répertoire."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "ls -l"
                    </span>
                    <span class="d">
                        "Liste les fichiers avec plus d'informations\u{a0}: les lignes qui commencent par "
                        <span class="mono">
                            "d"
                        </span>
                        " sont des dossiers, celles qui commencent par "
                        <span class="mono">
                            "-"
                        </span>
                        " sont des fichiers."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "ls -a"
                    </span>
                    <span class="d">
                        "Affiche aussi les fichiers cachés\u{a0}: tous les fichiers dont le nom commence par un "
                        <span class="mono">
                            "."
                        </span>
                        " ne s'affichent pas par défaut."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "cd"
                    </span>
                    <span class="d">
                        "Change de répertoire."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "file"
                    </span>
                    <span class="d">
                        "Détermine le type d'un fichier."
                    </span>
                </div>
            </div>
            <div class="term" style="margin-top:20px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "ls -l — lecture de la première colonne "
                        <span class="tag add" style="vertical-align:0">
                            "complément"
                        </span>
                    </span>
                </div>
                <pre>
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "pwd"
                    </span>
                    "\n/home/kenzo\n"
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "ls -l"
                    </span>
                    "\n"
                    <span class="w">
                        "d"
                    </span>
                    "rwxr-xr-x  2 kenzo kenzo  4096  7 sept. 15:54 cours     "
                    <span class="hash">
                        "# d → répertoire"
                    </span>
                    "\n"
                    <span class="w">
                        "-"
                    </span>
                    "rw-r--r--  1 kenzo kenzo   128  7 sept. 15:56 notes.txt "
                    <span class="hash">
                        "# - → fichier"
                    </span>
                </pre>
            </div>
        </section>
    }
}

fn sec_o_creer() -> impl IntoView {
    view! {
        <section id="o-creer">
            <div class="sec-head">
                <span class="num">
                    "04"
                </span>
                <h2>
                    "Créer des fichiers et des répertoires"
                </h2>
            </div>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "mkdir"
                    </span>
                    <span class="d">
                        "Crée un répertoire."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "touch"
                    </span>
                    <span class="d">
                        "Crée un fichier (vide)."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "nano"
                    </span>
                    <span class="d">
                        "Édite un fichier dans un éditeur de texte."
                    </span>
                </div>
            </div>
            <p style="margin-top:18px;max-width:68ch">
                "Trois façons de créer un fichier\u{a0}: "
                <strong>
                    "touch"
                </strong>
                ", une "
                <strong>
                    "redirection"
                </strong>
                ", ou un "
                <strong>
                    "éditeur de texte"
                </strong>
                "."
            </p>
            <h3>
                "Les redirections"
            </h3>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "commande > fichier.txt"
                    </span>
                    <span class="d">
                        "Écrit dans le fichier le texte affiché par la commande — le contenu précédent est "
                        <strong>
                            "écrasé"
                        </strong>
                        ". "
                        <sup class="fix">
                            "✱"
                        </sup>
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "commande >> fichier.txt"
                    </span>
                    <span class="d">
                        "Ajoute le texte affiché à la fin du fichier, sans rien effacer."
                    </span>
                </div>
            </div>
            <div class="term" style="margin-top:20px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "redirections"
                    </span>
                </div>
                <pre>
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "echo bonjour > notes.txt"
                    </span>
                    "   "
                    <span class="hash">
                        "# crée (ou écrase) notes.txt"
                    </span>
                    "\n"
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "echo au revoir >> notes.txt"
                    </span>
                    " "
                    <span class="hash">
                        "# ajoute une ligne à la fin"
                    </span>
                    "\n"
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "cat notes.txt"
                    </span>
                    "\nbonjour\nau revoir"
                </pre>
            </div>
        </section>
    }
}

fn sec_o_lire() -> impl IntoView {
    view! {
        <section id="o-lire">
            <div class="sec-head">
                <span class="num">
                    "05"
                </span>
                <h2>
                    "Lire le contenu d'un fichier"
                </h2>
            </div>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "cat"
                    </span>
                    <span class="d">
                        "Voir le contenu du fichier, d'un seul bloc."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "less"
                    </span>
                    <span class="d">
                        "Affiche le contenu d'un fichier page par page."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "head"
                    </span>
                    <span class="d">
                        "Affiche juste le début d'un fichier."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "tail"
                    </span>
                    <span class="d">
                        "Affiche la fin d'un fichier."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "-n 3"
                    </span>
                    <span class="d">
                        "Avec "
                        <span class="mono">
                            "head"
                        </span>
                        " ou "
                        <span class="mono">
                            "tail"
                        </span>
                        "\u{a0}: les 3 premières ou 3 dernières lignes."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "wc"
                    </span>
                    <span class="d">
                        "Donne le nombre de lignes, de mots et de caractères."
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_o_gerer() -> impl IntoView {
    view! {
        <section id="o-gerer">
            <div class="sec-head">
                <span class="num">
                    "06"
                </span>
                <h2>
                    "Copier, déplacer, supprimer"
                </h2>
            </div>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "cp source destination"
                    </span>
                    <span class="d">
                        "Copie un fichier."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "mv"
                    </span>
                    <span class="d">
                        "Renomme ou déplace un fichier."
                    </span>
                </div>
                <div class="cmd danger">
                    <span class="c">
                        "rm"
                    </span>
                    <span class="d">
                        "Supprime un fichier."
                    </span>
                </div>
                <div class="cmd danger">
                    <span class="c">
                        "rmdir"
                    </span>
                    <span class="d">
                        "Supprime un répertoire sans fichiers à l'intérieur."
                    </span>
                </div>
            </div>
            <div class="note warn" style="margin-top:20px">
                <span class="title">
                    "Pas de corbeille"
                </span>
                <p>
                    <span class="mono">
                        "rm"
                    </span>
                    " supprime définitivement\u{a0}: rien n'est mis de côté, rien ne se récupère."
                </p>
            </div>
        </section>
    }
}

fn sec_o_aide() -> impl IntoView {
    view! {
        <section id="o-aide">
            <div class="sec-head">
                <span class="num">
                    "07"
                </span>
                <h2>
                    "Obtenir de l'aide"
                </h2>
            </div>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "man commande"
                    </span>
                    <span class="d">
                        "Affiche le manuel de la commande. "
                        <em>
                            "— je connais la commande, je cherche ses options."
                        </em>
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "apropos"
                    </span>
                    <span class="d">
                        "Cherche la commande à utiliser pour faire quelque chose. "
                        <em>
                            "— je connais le besoin, je cherche la commande."
                        </em>
                    </span>
                </div>
            </div>
        </section>
    }
}

fn sec_o_composer() -> impl IntoView {
    view! {
        <section id="o-composer">
            <div class="sec-head">
                <span class="num">
                    "08"
                </span>
                <h2>
                    "Composer des commandes"
                </h2>
            </div>
            <div class="cmds">
                <div class="cmd">
                    <span class="c">
                        "cut"
                    </span>
                    <span class="d">
                        "Sélectionner."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "paste"
                    </span>
                    <span class="d">
                        "Assembler."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "sort"
                    </span>
                    <span class="d">
                        "Trier."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "sort -r"
                    </span>
                    <span class="d">
                        "Trier dans le sens inverse."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "sort -n"
                    </span>
                    <span class="d">
                        "Trier de manière numérique."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "uniq"
                    </span>
                    <span class="d">
                        "Si quelque chose est présent en plusieurs exemplaires, n'en laisse qu'un seul."
                    </span>
                </div>
                <div class="cmd">
                    <span class="c">
                        "|"
                    </span>
                    <span class="d">
                        "Le "
                        <strong>
                            "pipe"
                        </strong>
                        " (tube)\u{a0}: sert à combiner deux commandes, la sortie de la première devient l'entrée de la seconde."
                    </span>
                </div>
            </div>
            <h3>
                "Les options de sélection "
                <sup class="fix">
                    "✱"
                </sup>
            </h3>
            <div class="tw">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Option"
                            </th>
                            <th>
                                "Commande"
                            </th>
                            <th>
                                "Rôle"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="mono">
                                ":"
                            </td>
                            <td class="mono">
                                "—"
                            </td>
                            <td>
                                "Le séparateur utilisé dans le fichier (exemple\u{a0}: "
                                <span class="mono">
                                    "/etc/passwd"
                                </span>
                                ")."
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "-d"
                            </td>
                            <td class="mono">
                                "cut"
                            </td>
                            <td>
                                <strong>
                                    "Délimiteur"
                                </strong>
                                "\u{a0}: indique quel caractère sépare les colonnes — "
                                <span class="mono">
                                    "-d:"
                                </span>
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "-f"
                            </td>
                            <td class="mono">
                                "cut"
                            </td>
                            <td>
                                <strong>
                                    "Champ"
                                </strong>
                                "\u{a0}: la colonne que je veux garder — "
                                <span class="mono">
                                    "-f1"
                                </span>
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "-c"
                            </td>
                            <td class="mono">
                                "cut"
                            </td>
                            <td>
                                <strong>
                                    "Caractère"
                                </strong>
                                "\u{a0}: les caractères que je veux garder — "
                                <span class="mono">
                                    "-c1-5"
                                </span>
                            </td>
                        </tr>
                        <tr>
                            <td class="mono">
                                "-k"
                            </td>
                            <td class="mono">
                                "sort"
                            </td>
                            <td>
                                <strong>
                                    "Champ de tri"
                                </strong>
                                "\u{a0}: la colonne sur laquelle trier — "
                                <span class="mono">
                                    "-k2"
                                </span>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="term" style="margin-top:20px">
                <div class="term-bar">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="t">
                        "cut | sort | uniq "
                        <span class="tag add" style="vertical-align:0">
                            "complément"
                        </span>
                    </span>
                </div>
                <pre>
                    <span class="p">
                        "$"
                    </span>
                    " "
                    <span class="cm">
                        "cut -d: -f7 /etc/passwd | sort | uniq"
                    </span>
                    "\n/bin/bash\n/bin/false\n/usr/sbin/nologin\n"
                    <span class="hash">
                        "# cut : 7e champ, séparé par « : » → sort : trie → uniq : un seul exemplaire"
                    </span>
                </pre>
            </div>
            <div class="note" style="margin-top:16px">
                <p>
                    <span class="mono">
                        "uniq"
                    </span>
                    " ne supprime que les doublons "
                    <strong>
                        "consécutifs"
                    </strong>
                    "\u{a0}: c'est pour cela qu'on trie avec "
                    <span class="mono">
                        "sort"
                    </span>
                    " juste avant."
                </p>
            </div>
        </section>
    }
}

fn sec_o_recap() -> impl IntoView {
    view! {
        <section id="o-recap">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Aide-mémoire"
                </h2>
            </div>
            <CmdTable/>
        </section>
    }
}

fn sec_o_relecture() -> impl IntoView {
    view! {
        <section id="o-relecture">
            <div class="sec-head">
                <span class="num">
                    "—"
                </span>
                <h2>
                    "Notes de relecture"
                </h2>
            </div>
            <div class="body">
                <p style="color:var(--ink-soft);font-size:14px">
                    "Contenu repris intégralement des notes d'origine, orthographe et accents rétablis. Deux points précisés et trois exemples ajoutés, tous signalés dans la page."
                </p>
            </div>
            <div class="tw" style="margin-top:14px">
                <table>
                    <thead>
                        <tr>
                            <th>
                                "Chapitre"
                            </th>
                            <th>
                                "Note d'origine"
                            </th>
                            <th>
                                "Précision"
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>
                                "04 · Créer"
                            </td>
                            <td class="mono">
                                "« > remplit le fichier »"
                            </td>
                            <td>
                                <span class="mono">
                                    ">"
                                </span>
                                " "
                                <strong>
                                    "écrase"
                                </strong>
                                " le contenu existant du fichier\u{a0}; c'est "
                                <span class="mono">
                                    ">>"
                                </span>
                                " qui ajoute à la suite."
                            </td>
                        </tr>
                        <tr>
                            <td>
                                "08 · Composer"
                            </td>
                            <td class="mono">
                                "« -k je veux ce champ »"
                            </td>
                            <td>
                                <span class="mono">
                                    "-k"
                                </span>
                                " est l'option de "
                                <strong>
                                    "sort"
                                </strong>
                                " (colonne de tri). Pour "
                                <span class="mono">
                                    "cut"
                                </span>
                                ", le champ se choisit avec "
                                <strong>
                                    <span class="mono">
                                        "-f"
                                    </span>
                                </strong>
                                ", et "
                                <span class="mono">
                                    "-d"
                                </span>
                                " définit le délimiteur."
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <p style="font-size:13px;color:var(--ink-soft);margin-top:14px;max-width:68ch">
                "Ajouts signalés "
                <span class="tag add">
                    "complément"
                </span>
                "\u{a0}: le schéma d'arborescence, la lecture d'un "
                <span class="mono">
                    "ls -l"
                </span>
                ", l'exemple "
                <span class="mono">
                    "cut | sort | uniq"
                </span>
                " et l'aide-mémoire final, qui ne fait que regrouper les commandes du cours."
            </p>
        </section>
    }
}

/// Page complète de la matière.
pub fn page() -> impl IntoView {
    view! {
        <div class="course c-os">
            {masthead()}
            <div class="shell">
                <Toc course=Course::Os items=TOC label=TOC_LABEL/>
                <main>
                    {sec_o_os()}
                    {sec_o_arbo()}
                    {sec_o_deplacer()}
                    {sec_o_creer()}
                    {sec_o_lire()}
                    {sec_o_gerer()}
                    {sec_o_aide()}
                    {sec_o_composer()}
                    {sec_o_recap()}
                    {sec_o_relecture()}
                </main>
            </div>
        </div>
    }
}
