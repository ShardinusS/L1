//! Entraînement — Systèmes d'exploitation.

use super::{qcm, shuffled_choice, txt, vf, Answer, Chapter, Exercise, Fixed, Rng};
use crate::components::cmd_table::COMMANDS;

pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        id: "arborescence",
        title: "Arborescence et déplacement",
        anchor: "o-deplacer",
        fixed: ARBO,
        generators: &[cd_walk, ls_l_line],
    },
    Chapter {
        id: "fichiers",
        title: "Créer, lire, supprimer",
        anchor: "o-creer",
        fixed: FICHIERS,
        generators: &[redirections, head_tail],
    },
    Chapter {
        id: "aide",
        title: "Obtenir de l'aide",
        anchor: "o-aide",
        fixed: AIDE,
        generators: &[],
    },
    Chapter {
        id: "composer",
        title: "Composer des commandes",
        anchor: "o-composer",
        fixed: COMPOSER,
        generators: &[sort_uniq, cut_field],
    },
    Chapter {
        id: "commandes",
        title: "Les 32 commandes",
        anchor: "o-recap",
        fixed: &[],
        generators: &[cmd_for_task, role_of_cmd],
    },
];

const ARBO: &[Fixed] = &[
    qcm(
        "Que désigne « / » ?",
        &[
            "la racine de l'arborescence",
            "le répertoire courant",
            "le répertoire père",
            "le répertoire personnel",
        ],
        "L'arborescence part d'un point unique, la racine, notée /.",
    ),
    qcm(
        "Que désigne « .. » ?",
        &[
            "le répertoire père",
            "le répertoire courant",
            "la racine",
            "les fichiers cachés",
        ],
        "D'où cd .. pour remonter d'un cran.",
    ),
    qcm(
        "Que désigne « . » ?",
        &[
            "le répertoire courant",
            "le répertoire père",
            "la racine",
            "un fichier caché",
        ],
        "Le répertoire courant — « moi ».",
    ),
    txt(
        "Quelle commande affiche l'emplacement courant dans l'arborescence ?",
        &["pwd"],
        "pwd : print working directory.",
    ),
    txt(
        "Quelle commande liste aussi les fichiers cachés ?",
        &["ls -a"],
        "ls -a : les fichiers dont le nom commence par un point ne s'affichent pas par défaut.",
    ),
    vf(
        "Un fichier nommé .bashrc apparaît avec un simple ls.",
        false,
        "Son nom commence par un point : il est caché, il faut ls -a.",
    ),
    qcm(
        "Quelle commande détermine le type d'un fichier ?",
        &["file", "type", "ls -l", "cat"],
        "file notes.txt répond par exemple « ASCII text ».",
    ),
    qcm(
        "Lequel n'est pas un système d'exploitation ?",
        &["bash", "Linux", "Android", "macOS"],
        "bash est l'interpréteur de la ligne de commande, pas un système d'exploitation.",
    ),
];

const FICHIERS: &[Fixed] = &[
    qcm(
        "Quelle commande crée un répertoire ?",
        &["mkdir", "touch", "rmdir", "cd"],
        "mkdir : make directory.",
    ),
    qcm(
        "Quelle commande crée un fichier vide ?",
        &["touch", "mkdir", "nano", "cat"],
        "touch crée le fichier s'il n'existe pas.",
    ),
    qcm(
        "notes.txt existe déjà. Que fait echo bonjour > notes.txt ?",
        &[
            "le contenu précédent est écrasé",
            "bonjour est ajouté à la fin",
            "une erreur s'affiche",
            "un second fichier est créé",
        ],
        "> écrase le contenu existant; c'est >> qui ajoute à la suite.",
    ),
    qcm(
        "Les trois façons de créer un fichier vues en cours :",
        &[
            "touch, une redirection, un éditeur de texte",
            "mkdir, cp, mv",
            "cat, less, head",
            "touch, rm, rmdir",
        ],
        "touch, > ou >>, et un éditeur comme nano.",
    ),
    qcm(
        "Quelle commande affiche un fichier page par page ?",
        &["less", "cat", "head", "wc"],
        "cat affiche tout d'un bloc; less pagine.",
    ),
    qcm(
        "wc donne…",
        &[
            "le nombre de lignes, de mots et de caractères",
            "la liste des fichiers",
            "le type du fichier",
            "les premières lignes du fichier",
        ],
        "wc : word count. wc -l ne garde que le nombre de lignes.",
    ),
    vf(
        "Un fichier supprimé avec rm se récupère dans la corbeille.",
        false,
        "Pas de corbeille : rm supprime définitivement.",
    ),
    qcm(
        "rmdir supprime…",
        &[
            "un répertoire vide",
            "un répertoire et tout son contenu",
            "un fichier",
            "le répertoire courant",
        ],
        "rmdir refuse un répertoire qui contient encore des fichiers.",
    ),
    qcm(
        "Pour renommer notes.txt en cours.txt :",
        &[
            "mv notes.txt cours.txt",
            "cp notes.txt cours.txt",
            "mv cours.txt notes.txt",
            "rm notes.txt cours.txt",
        ],
        "mv source destination renomme ou déplace. cp laisserait l'original en place.",
    ),
    txt(
        "Commande pour copier a.txt vers b.txt ?",
        &["cp a.txt b.txt"],
        "cp source destination.",
    ),
    txt(
        "Commande pour afficher les 3 dernières lignes de notes.txt ?",
        &["tail -n 3 notes.txt", "tail -3 notes.txt"],
        "tail pour la fin, head pour le début; -n fixe le nombre de lignes.",
    ),
];

const AIDE: &[Fixed] = &[
    qcm(
        "Je connais la commande, je cherche ses options :",
        &["man commande", "apropos", "file", "echo"],
        "man affiche le manuel de la commande.",
    ),
    qcm(
        "Je connais le besoin, je cherche la commande :",
        &["apropos", "man", "ls", "file"],
        "apropos cherche la commande à utiliser pour faire quelque chose.",
    ),
];

const COMPOSER: &[Fixed] = &[
    qcm(
        "Le pipe | sert à…",
        &[
            "envoyer la sortie d'une commande en entrée de la suivante",
            "écrire la sortie dans un fichier",
            "lancer deux commandes indépendantes",
            "séparer les colonnes d'un fichier",
        ],
        "La sortie de la première commande devient l'entrée de la seconde.",
    ),
    qcm(
        "Pourquoi trie-t-on avec sort juste avant uniq ?",
        &[
            "uniq ne supprime que les doublons consécutifs",
            "uniq ne fonctionne que sur des nombres",
            "sort supprime déjà les doublons",
            "pour afficher le résultat à l'envers",
        ],
        "Trier regroupe les lignes identiques, que uniq peut alors fusionner.",
    ),
    qcm(
        "Dans cut, l'option -d indique…",
        &["le délimiteur entre les colonnes", "la colonne à garder", "les caractères à garder", "la colonne de tri"],
        "-d: pour des champs séparés par « : », comme dans /etc/passwd.",
    ),
    qcm(
        "L'option -k (colonne de tri) appartient à…",
        &["sort", "cut", "uniq", "paste"],
        "Pour cut, le champ se choisit avec -f et le délimiteur avec -d.",
    ),
    qcm(
        "sort -n trie…",
        &["de manière numérique", "dans le sens inverse", "en supprimant les doublons", "par date"],
        "Sans -n, « 10 » passe avant « 9 » : l'ordre est celui des caractères. sort -r inverse l'ordre.",
    ),
    txt(
        "Commande qui garde le 1er champ de /etc/passwd (champs séparés par « : ») ?",
        &["cut -d: -f1 /etc/passwd", "cut -f1 -d: /etc/passwd", "cut -d : -f 1 /etc/passwd"],
        "-d: fixe le délimiteur, -f1 le champ gardé.",
    ),
    qcm(
        "cut -c1-5 garde…",
        &[
            "les 5 premiers caractères de chaque ligne",
            "les 5 premières lignes",
            "les colonnes 1 et 5",
            "les champs 1 à 5",
        ],
        "-c sélectionne des caractères; head -n 5 donnerait les 5 premières lignes.",
    ),
];

// ---------------------------------------------------------------- générateurs

/// Répertoires de l'arborescence d'exemple du cours, complétée.
const DIRS: &[&str] = &[
    "/",
    "/home",
    "/home/kenzo",
    "/home/kenzo/cours",
    "/home/kenzo/cours/algo",
    "/home/kenzo/cours/maths",
    "/home/invite",
    "/etc",
    "/var",
    "/var/log",
];

fn parent(dir: &str) -> String {
    match dir.rfind('/') {
        Some(0) | None => "/".to_string(),
        Some(i) => dir[..i].to_string(),
    }
}

fn children(dir: &str) -> Vec<&'static str> {
    DIRS.iter()
        .filter(|d| **d != "/" && parent(d) == dir)
        .map(|d| d.rsplit('/').next().unwrap_or(d))
        .collect()
}

fn join(dir: &str, name: &str) -> String {
    if dir == "/" {
        format!("/{name}")
    } else {
        format!("{dir}/{name}")
    }
}

fn cd_walk(rng: &mut Rng) -> Exercise {
    let mut dir = rng.pick(&DIRS[1..]).to_string();
    let start = dir.clone();
    let steps = rng.range(2, 3);
    let mut code = format!("$ pwd\n{start}");
    let mut trail = Vec::new();
    for _ in 0..steps {
        let kids = children(&dir);
        let siblings: Vec<&str> = children(&parent(&dir))
            .into_iter()
            .filter(|s| join(&parent(&dir), s) != dir)
            .collect();
        let choice = rng.below(10);
        let cmd = if choice < 4 && !kids.is_empty() {
            let k = rng.pick(&kids);
            dir = join(&dir, k);
            format!("cd {k}")
        } else if choice < 6 && !siblings.is_empty() && dir != "/" {
            let s = rng.pick(&siblings);
            dir = join(&parent(&dir), s);
            format!("cd ../{s}")
        } else if choice < 7 {
            let target = *rng.pick(&["/etc", "/var/log", "/home"]);
            dir = target.to_string();
            format!("cd {target}")
        } else {
            dir = parent(&dir);
            "cd ..".to_string()
        };
        code.push_str(&format!("\n$ {cmd}"));
        trail.push(format!("{cmd} → {dir}"));
    }
    code.push_str("\n$ pwd");
    let accepted = if dir == "/" {
        vec![dir.clone()]
    } else {
        vec![dir.clone(), format!("{dir}/")]
    };
    Exercise::new(
        "Qu'affiche le dernier pwd ?",
        Answer::Text { accepted },
        format!(
            "Départ : {start}. {}. Rappel : .. est le répertoire père, un chemin qui commence par / part de la racine.",
            trail.join(" ; ")
        ),
    )
    .with_code(code)
}

fn ls_l_line(rng: &mut Rng) -> Exercise {
    let is_dir = rng.coin();
    let name = *rng.pick(&[
        "cours",
        "notes.txt",
        "photos",
        "rapport.pdf",
        "algo",
        "td1.c",
        "archive.zip",
    ]);
    let line = if is_dir {
        format!("drwxr-xr-x  2 kenzo kenzo  4096  7 sept. 15:54 {name}")
    } else {
        format!("-rw-r--r--  1 kenzo kenzo   128  7 sept. 15:56 {name}")
    };
    Exercise::new(
        format!("D'après ls -l, qu'est-ce que « {name} » ?"),
        Answer::Choice {
            options: vec!["Un répertoire".into(), "Un fichier".into()],
            correct: if is_dir { 0 } else { 1 },
        },
        format!(
            "Le premier caractère décide : d → répertoire, - → fichier. Ici « {} ». Le nom, lui, ne prouve rien.",
            if is_dir { 'd' } else { '-' }
        ),
    )
    .with_code(format!("$ ls -l\n{line}"))
}

const WORDS: &[&str] = &[
    "bonjour",
    "au revoir",
    "salut",
    "merci",
    "linux",
    "bash",
    "coucou",
];

fn redirections(rng: &mut Rng) -> Exercise {
    let n = rng.range(2, 4) as usize;
    let words: Vec<&str> = rng
        .sample(WORDS.len(), n)
        .into_iter()
        .map(|i| WORDS[i])
        .collect();
    let mut ops: Vec<&str> = (0..n)
        .map(|_| if rng.coin() { ">" } else { ">>" })
        .collect();
    // Au moins un « > » après le début, sans quoi la question est triviale.
    if !ops[1..].contains(&">") {
        let i = rng.range(1, n as i64 - 1) as usize;
        ops[i] = ">";
    }
    let mut content: Vec<&str> = Vec::new();
    let mut code = String::new();
    for (w, op) in words.iter().zip(&ops) {
        if *op == ">" {
            content.clear();
        }
        content.push(w);
        code.push_str(&format!("$ echo {w} {op} notes.txt\n"));
    }
    code.push_str("$ cat notes.txt");
    let right = content.join("\n");
    let wrong = [
        words.join("\n"),
        words.last().map(|w| w.to_string()).unwrap_or_default(),
        content.iter().rev().copied().collect::<Vec<_>>().join("\n"),
    ];
    Exercise::new(
        "On part d'un dossier sans notes.txt. Qu'affiche cat ?",
        shuffled_choice(rng, right, wrong),
        "> vide le fichier puis écrit; >> ajoute à la fin sans rien effacer.",
    )
    .with_code(code)
}

const FRUITS: &[&str] = &[
    "pomme", "poire", "kiwi", "banane", "cerise", "fraise", "mangue", "prune",
];

fn head_tail(rng: &mut Rng) -> Exercise {
    let len = rng.range(5, 7) as usize;
    let lines: Vec<&str> = rng
        .sample(FRUITS.len(), len)
        .into_iter()
        .map(|i| FRUITS[i])
        .collect();
    let k = rng.range(2, 3) as usize;
    let head = rng.coin();
    let cmd = if head { "head" } else { "tail" };
    let take = |from_start: bool, k: usize| -> String {
        if from_start {
            lines[..k].join("\n")
        } else {
            lines[len - k..].join("\n")
        }
    };
    let right = take(head, k);
    let wrong = [take(!head, k), take(head, k + 1), take(head, k - 1)];
    Exercise::new(
        format!("Qu'affiche {cmd} -n {k} fruits.txt ?"),
        shuffled_choice(rng, right, wrong),
        format!(
            "{cmd} affiche {} du fichier; -n {k} en garde {k} lignes.",
            if head { "le début" } else { "la fin" }
        ),
    )
    .with_code(format!("$ cat fruits.txt\n{}", lines.join("\n")))
}

fn sort_uniq(rng: &mut Rng) -> Exercise {
    const SHELLS: &[&str] = &["/bin/bash", "/bin/false", "/usr/sbin/nologin", "/bin/sh"];
    let len = rng.range(6, 8) as usize;
    let kinds = rng.range(2, 4) as usize;
    let lines: Vec<&str> = (0..len).map(|_| SHELLS[rng.below(kinds)]).collect();
    let mut distinct = lines.clone();
    distinct.sort_unstable();
    distinct.dedup();
    let runs = 1 + lines.windows(2).filter(|w| w[0] != w[1]).count();
    let sorted = rng.coin();
    let (cmd, value, explain) = if sorted {
        (
            "sort shells.txt | uniq | wc -l",
            distinct.len(),
            format!(
                "sort regroupe les lignes identiques, uniq n'en garde qu'une par groupe : il reste {} lignes distinctes.",
                distinct.len()
            ),
        )
    } else {
        (
            "uniq shells.txt | wc -l",
            runs,
            format!(
                "Sans tri, uniq ne fusionne que les doublons consécutifs : {runs} blocs de lignes identiques qui se suivent."
            ),
        )
    };
    Exercise::new(
        "Quel nombre affiche la dernière commande ?",
        Answer::Number {
            value: value as i64,
            radix: 10,
        },
        explain,
    )
    .with_code(format!("$ cat shells.txt\n{}\n$ {cmd}", lines.join("\n")))
}

fn cut_field(rng: &mut Rng) -> Exercise {
    const USERS: &[[&str; 7]] = &[
        [
            "kenzo",
            "x",
            "1000",
            "1000",
            "Kenzo",
            "/home/kenzo",
            "/bin/bash",
        ],
        ["root", "x", "0", "0", "root", "/root", "/bin/bash"],
        [
            "invite",
            "x",
            "1001",
            "1001",
            "Invité",
            "/home/invite",
            "/bin/sh",
        ],
        [
            "daemon",
            "x",
            "1",
            "1",
            "daemon",
            "/usr/sbin",
            "/usr/sbin/nologin",
        ],
    ];
    let user = rng.pick(USERS);
    let line = user.join(":");
    if rng.below(3) == 0 {
        let n = rng.range(2, 5) as usize;
        let expected: String = line.chars().take(n).collect();
        return Exercise::new(
            format!("Qu'affiche cut -c1-{n} sur cette ligne ?"),
            Answer::Text {
                accepted: vec![expected.clone()],
            },
            format!("-c garde des caractères : les {n} premiers, « {expected} »."),
        )
        .with_code(line);
    }
    let f = rng.range(1, 7) as usize;
    let numbered: Vec<String> = user
        .iter()
        .enumerate()
        .map(|(i, v)| format!("{}={v}", i + 1))
        .collect();
    Exercise::new(
        format!("Qu'affiche cut -d: -f{f} sur cette ligne de /etc/passwd ?"),
        Answer::Text {
            accepted: vec![user[f - 1].to_string()],
        },
        format!(
            "Champs séparés par « : », numérotés à partir de 1 : {}.",
            numbered.join("  ")
        ),
    )
    .with_code(line)
}

/// Trois autres commandes, pour servir de leurres.
fn other_commands(rng: &mut Rng, avoid: usize) -> Vec<usize> {
    rng.sample(COMMANDS.len(), COMMANDS.len())
        .into_iter()
        .filter(|i| *i != avoid)
        .take(3)
        .collect()
}

fn cmd_for_task(rng: &mut Rng) -> Exercise {
    let i = rng.below(COMMANDS.len());
    let cmd = &COMMANDS[i];
    let wrong: Vec<String> = other_commands(rng, i)
        .into_iter()
        .map(|j| COMMANDS[j].name.to_string())
        .collect();
    Exercise::new(
        format!("Quelle commande pour : « {} » ?", cmd.role),
        shuffled_choice(rng, cmd.name.to_string(), wrong),
        format!(
            "{} — {} (chapitre « {} »).",
            cmd.name, cmd.role, cmd.chapter
        ),
    )
}

fn role_of_cmd(rng: &mut Rng) -> Exercise {
    let i = rng.below(COMMANDS.len());
    let cmd = &COMMANDS[i];
    let wrong: Vec<String> = other_commands(rng, i)
        .into_iter()
        .map(|j| COMMANDS[j].role.to_string())
        .collect();
    Exercise::new(
        format!("Que fait « {} » ?", cmd.name),
        shuffled_choice(rng, cmd.role.to_string(), wrong),
        format!(
            "{} — {} (chapitre « {} »).",
            cmd.name, cmd.role, cmd.chapter
        ),
    )
}
