//! Entraînement — Algorithmique 1.

use super::answer::minus;
use super::{card, num, qcm, shuffled_choice, txt, vf, Answer, Chapter, Exercise, Fixed, Rng};

pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        id: "notions",
        title: "Algorithme et cycle de développement",
        anchor: "al-cycle",
        fixed: NOTIONS,
        generators: &[],
    },
    Chapter {
        id: "types",
        title: "Types et syntaxe",
        anchor: "al-syntaxe",
        fixed: TYPES,
        generators: &[value_type, expr_type],
    },
    Chapter {
        id: "expressions",
        title: "Évaluer une expression",
        anchor: "al-pieges",
        fixed: EXPRESSIONS,
        generators: &[int_expr, div_mod],
    },
    Chapter {
        id: "traces",
        title: "Traces d'exécution",
        anchor: "al-td1",
        fixed: TRACES,
        generators: &[int_trace, string_trace, swap, duration],
    },
];

const NOTIONS: &[Fixed] = &[
    qcm(
        "Un algorithme correct est un algorithme qui…",
        &[
            "produit la bonne sortie pour chaque instance du problème",
            "produit la bonne sortie sur l'exemple donné",
            "s'exécute le plus vite possible",
            "est écrit dans un langage de programmation",
        ],
        "Correct = pour chaque instance. L'efficacité (vitesse, mémoire…) est une autre qualité.",
    ),
    qcm(
        "Une instance d'un problème, c'est…",
        &[
            "un jeu de données particulier donné en entrée",
            "le programme qui le résout",
            "la sortie attendue",
            "une étape du cycle de développement",
        ],
        "Pour le tri, (31, 41, 59, 26, 41, 58) est une instance.",
    ),
    txt(
        "Trier l'instance (31, 41, 59, 26, 41, 58) : quelle sortie ?",
        &["(26, 31, 41, 41, 58, 59)", "26 31 41 41 58 59"],
        "Une permutation de l'entrée telle que a′₁ ≤ a′₂ ≤ … ≤ a′ₙ. Les doublons restent.",
    ),
    qcm(
        "Dans quel ordre viennent les étapes du cycle de développement ?",
        &[
            "analyse, conception, codage, test",
            "conception, analyse, test, codage",
            "codage, test, analyse, conception",
            "analyse, codage, conception, test",
        ],
        "Problème → analyse → conception (algorithme) → codage (programme) → test → résultat.",
    ),
    qcm(
        "Quelle étape produit l'algorithme ?",
        &["la conception", "l'analyse", "le codage", "le test"],
        "La conception fait ressortir la logique de la résolution, indépendamment du langage.",
    ),
    qcm(
        "Traduire l'algorithme dans un langage de programmation, c'est…",
        &["la programmation (le codage)", "l'analyse", "la spécification", "le test"],
        "La programmation est la traduction du langage algorithmique vers le langage de programmation.",
    ),
    qcm(
        "Que fait gcc cheminMetro.c -o cheminMetro ?",
        &[
            "il traduit le programme en langage machine",
            "il exécute le programme",
            "il affiche le code source",
            "il teste l'algorithme",
        ],
        "C'est la compilation; ./cheminMetro lance ensuite l'exécution.",
    ),
    qcm(
        "Dans le modèle RAM, les instructions…",
        &[
            "s'exécutent l'une après l'autre, chacune en temps constant",
            "s'exécutent toutes en même temps",
            "ont des durées imprévisibles",
            "ne peuvent pas lire la mémoire",
        ],
        "C'est ce qui justifiera de compter les opérations pour mesurer la complexité.",
    ),
    vf(
        "Une solution peut ne jamais s'arrêter, pourvu qu'elle trouve le bon résultat.",
        false,
        "Première contrainte : une solution doit s'arrêter — pas de boucle infinie.",
    ),
    qcm(
        "Note finale d'Algorithmique 1 :",
        &[
            "30 % QCM + 35 % partiel + 35 % examen",
            "50 % partiel + 50 % examen",
            "100 % examen terminal",
            "20 % TP + 40 % partiel + 40 % examen",
        ],
        "Deux QCM de 30 min en TP, un partiel d'1 h, un examen terminal d'1 h.",
    ),
    card(
        "Algorithme — définition informelle",
        "Une procédure de calcul bien définie qui prend en entrée un ensemble de valeurs et produit en sortie un ensemble de valeurs : une suite d'étapes qui transforment l'entrée en sortie et résolvent un problème bien spécifié.",
    ),
    card(
        "L'analyse — que faut-il identifier ?",
        "Le problème (précisément), les données, les résultats, les cas particuliers, le traitement — puis découper le problème en tâches simples et distinctes.",
    ),
];

const TYPES: &[Fixed] = &[
    qcm(
        "Quel symbole sert à l'affectation en langage algorithmique ?",
        &["←", "=", "==", ":="],
        "x ← 3 ; — la flèche, pas le signe =.",
    ),
    vf(
        "Toute variable utilisée doit être déclarée dans la partie « variables ».",
        true,
        "Une variable non déclarée est une erreur du TD 1, exercice 1.",
    ),
    qcm(
        "e est un entier. Quelle instruction est incorrecte ?",
        &["e ← 3.5 ;", "e ← 3 ;", "e ← e + 1 ;", "e ← 7 div 2 ;"],
        "3.5 est un réel : le type de e contraint les valeurs acceptées.",
    ),
    qcm(
        "Quel opérateur donne le reste de la division entière ?",
        &["mod", "div", "/", "%"],
        "a div b : quotient; a mod b : reste. % est l'écriture du C, pas du langage algorithmique.",
    ),
    qcm(
        "Que vaut (entier)(7.5 / 3.0) ?",
        &["2", "3", "2.5", "une erreur"],
        "7.5 / 3.0 = 2.5, et la conversion tronque vers zéro.",
    ),
    qcm(
        "Comment concatène-t-on deux chaînes ?",
        &["avec +", "avec &", "avec .", "on ne peut pas"],
        "salutation ← \"Bonjour \" + nom ;",
    ),
];

const EXPRESSIONS: &[Fixed] = &[
    num(
        "Que vaut 5 * 4 div 3 ?",
        6,
        "Même priorité, donc de gauche à droite : 20 div 3 = 6 — pas 5 × 1.",
    ),
    num(
        "Avec a = 5 et b = 2 : a div b * 2 + 1 ?",
        5,
        "(5 div 2) = 2, puis 2 × 2 + 1 = 5.",
    ),
    num(
        "Avec a = 5 et b = 2 : a mod b + 1 ?",
        2,
        "5 mod 2 = 1, puis + 1.",
    ),
    num(
        "Avec c = 7.5 et d = 3.0 : (entier)(c / d) + 1 ?",
        3,
        "c / d = 2.5, la conversion tronque en 2, puis + 1.",
    ),
    txt(
        "Avec c = 7.5 et d = 3.0 : c / d * 2 + 1 ?",
        &["6.0", "6"],
        "7.5 / 3.0 = 2.5, × 2 = 5.0, + 1 = 6.0 — un réel.",
    ),
    vf(
        "Avec a = 5, b = 2 et e = vrai : (a div b < 0) et (non e ou (a = b)) vaut vrai.",
        false,
        "a div b = 2 et 2 < 0 est faux : le « et » est faux sans regarder la suite.",
    ),
    txt(
        "Évaluer 3*x-8/y*(4*z)+2.5 pour x = 10.0, y = 1.0, z = 2.0.",
        &["-31.5"],
        "3 × 10 = 30; 8 / 1 = 8, puis × (4 × 2) = 64; 30 − 64 + 2.5 = −31.5.",
    ),
    qcm(
        "a / b / c signifie…",
        &[
            "(a / b) / c, soit a / (b × c)",
            "a / (b / c)",
            "(a × c) / b",
            "a / b × c",
        ],
        "Même priorité, de gauche à droite.",
    ),
];

const TRACES: &[Fixed] = &[
    qcm(
        "Que fait cette séquence sur deux nombres x et y ?",
        &["elle échange x et y", "elle double x", "elle remet x et y à zéro", "elle ne change rien"],
        "x = x₀ + y₀, puis y = x₀, puis x = y₀ : un échange sans variable intermédiaire.",
    )
    .code("x ← x + y ;\ny ← x - y ;\nx ← x - y ;"),
    qcm(
        "Pourquoi préférer tmp ← x ; x ← y ; y ← tmp à l'échange par additions ?",
        &[
            "il marche pour tous les types et ne déborde jamais",
            "il est plus court",
            "il n'utilise aucune variable",
            "il est le seul correct pour les entiers",
        ],
        "L'échange par additions exige des nombres et peut dépasser la capacité de représentation.",
    ),
    txt(
        "Que vaut c à la fin ?",
        &["abracadabra"],
        "c = \"a\" + \"br\" + \"a\" + \"c\" + \"a\" + \"d\" + \"a\" + \"br\" + \"a\". Le b ← \"bonjour 007\" d'avant est écrasé.",
    )
    .code("b ← \"a\" ;\na ← \"br\" ;\nc ← b + a + b + \"c\" + b + \"d\" + b + a + b ;"),
    card(
        "Secondes → heures, minutes, secondes",
        "h ← total div 3600 ; r ← total mod 3600 ; min ← r div 60 ; s ← r mod 60.\nDans l'autre sens : total ← h * 3600 + min * 60 + s.",
    ),
];

// ---------------------------------------------------------------- générateurs

const TYPE_NAMES: [&str; 6] = [
    "entier",
    "réel",
    "chaîne",
    "caractère",
    "booléen",
    "pas une valeur valide",
];

fn type_choice(rng: &mut Rng, right: usize, pool: &[usize]) -> Answer {
    let mut wrong: Vec<String> = pool
        .iter()
        .filter(|t| **t != right)
        .map(|t| TYPE_NAMES[*t].to_string())
        .collect();
    rng.shuffle(&mut wrong);
    wrong.truncate(3);
    shuffled_choice(rng, TYPE_NAMES[right].to_string(), wrong)
}

fn value_type(rng: &mut Rng) -> Exercise {
    const VALUES: &[(&str, usize, &str)] = &[
        ("2", 0, "Un nombre sans partie décimale."),
        ("-7", 0, "Un nombre sans partie décimale, signé."),
        ("2.5", 1, "Un nombre avec un point décimal."),
        ("3.14", 1, "Un nombre avec un point décimal."),
        ("0.0", 1, "Le point décimal suffit à en faire un réel."),
        ("\"bonjour\"", 2, "Des guillemets doubles : une chaîne."),
        (
            "\"faux\"",
            2,
            "Entre guillemets, « faux » est une chaîne, pas un booléen.",
        ),
        (
            "\"7\"",
            2,
            "Entre guillemets doubles, même un chiffre est une chaîne.",
        ),
        ("'a'", 3, "Un seul caractère entre apostrophes."),
        (
            "'7'",
            3,
            "Un seul caractère entre apostrophes, même si c'est un chiffre.",
        ),
        (
            "vrai",
            4,
            "vrai et faux, sans guillemets, sont les deux booléens.",
        ),
        (
            "faux",
            4,
            "vrai et faux, sans guillemets, sont les deux booléens.",
        ),
        (
            "'au revoir'",
            5,
            "Plusieurs caractères entre apostrophes : ni caractère, ni chaîne.",
        ),
        ("pi", 5, "pi n'est ni une valeur, ni une variable déclarée."),
        (
            "oui",
            5,
            "oui n'est pas un booléen : seuls vrai et faux le sont.",
        ),
    ];
    let (value, t, why) = *rng.pick(VALUES);
    Exercise::new(
        format!("À quel type appartient la valeur {value} ?"),
        type_choice(rng, t, &[0, 1, 2, 3, 4, 5]),
        why,
    )
}

fn expr_type(rng: &mut Rng) -> Exercise {
    const EXPRS: &[(&str, usize, &str)] = &[
        (
            "a div b * 2 + 1",
            0,
            "div entre entiers donne un entier, et le reste du calcul aussi.",
        ),
        ("a mod b + 1", 0, "mod entre entiers donne un entier."),
        ("a * b", 0, "Produit de deux entiers."),
        (
            "(entier)(c / d) + 1",
            0,
            "La conversion (entier) produit un entier.",
        ),
        (
            "c / d * 2 + 1",
            1,
            "Des réels partout : le résultat est réel.",
        ),
        ("d / b", 1, "Un réel divisé par un entier reste réel."),
        ("a + c", 1, "Entier + réel donne un réel."),
        ("c - d", 1, "Différence de deux réels."),
        (
            "(a div b < 0) et (non e ou (a = b))",
            4,
            "Comparaisons et opérateurs logiques : booléen.",
        ),
        ("a = b", 4, "Une comparaison donne un booléen."),
        ("non e", 4, "non opère sur un booléen et en renvoie un."),
    ];
    let (expr, t, why) = *rng.pick(EXPRS);
    Exercise::new(
        format!("Avec a = 5, b = 2 (entiers), c = 7.5, d = 3.0 (réels) et e = vrai : quel est le type de {expr} ?"),
        type_choice(rng, t, &[0, 1, 2, 4]),
        why,
    )
}

fn int_expr(rng: &mut Rng) -> Exercise {
    let a = rng.range(6, 30);
    let b = rng.range(2, 9);
    let c = rng.range(2, 9);
    let d = rng.range(2, 6);
    let (expr, value, steps) = match rng.below(8) {
        0 => (
            format!("{a} * {b} div {c}"),
            a * b / c,
            format!(
                "{a} × {b} = {}, puis {} div {c} = {}",
                a * b,
                a * b,
                a * b / c
            ),
        ),
        1 => (
            format!("{a} div {b} * {c}"),
            a / b * c,
            format!("{a} div {b} = {}, puis × {c} = {}", a / b, a / b * c),
        ),
        2 => (
            format!("{a} mod {b} + {c}"),
            a % b + c,
            format!("{a} mod {b} = {}, puis + {c} = {}", a % b, a % b + c),
        ),
        3 => (
            format!("{a} + {b} * {c} div {d}"),
            a + b * c / d,
            format!(
                "{b} × {c} = {}, {} div {d} = {}, puis {a} + {} = {}",
                b * c,
                b * c,
                b * c / d,
                b * c / d,
                a + b * c / d
            ),
        ),
        4 => (
            format!("{a} - {b} mod {c}"),
            a - b % c,
            format!(
                "{b} mod {c} = {}, puis {a} − {} = {}",
                b % c,
                b % c,
                a - b % c
            ),
        ),
        5 => (
            format!("{a} div {b} + {a} mod {b}"),
            a / b + a % b,
            format!(
                "{a} div {b} = {}, {a} mod {b} = {}, somme {}",
                a / b,
                a % b,
                a / b + a % b
            ),
        ),
        6 => (
            format!("({a} + {b}) div {c} * {d}"),
            (a + b) / c * d,
            format!(
                "{a} + {b} = {}, div {c} = {}, puis × {d} = {}",
                a + b,
                (a + b) / c,
                (a + b) / c * d
            ),
        ),
        _ => (
            format!("{a} * {b} mod {c}"),
            a * b % c,
            format!(
                "{a} × {b} = {}, puis {} mod {c} = {}",
                a * b,
                a * b,
                a * b % c
            ),
        ),
    };
    Exercise::new(
        format!("Que vaut {expr} ?"),
        Answer::Number { value, radix: 10 },
        format!("*, div et mod passent avant + et −, et à priorité égale on va de gauche à droite : {steps}."),
    )
}

fn div_mod(rng: &mut Rng) -> Exercise {
    let a = rng.range(10, 99);
    let b = rng.range(2, 12);
    let (q, r) = (a / b, a % b);
    let div = rng.coin();
    Exercise::new(
        format!("Que vaut {a} {} {b} ?", if div { "div" } else { "mod" }),
        Answer::Number {
            value: if div { q } else { r },
            radix: 10,
        },
        format!("{a} = {b} × {q} + {r} : quotient {q} (div), reste {r} (mod)."),
    )
}

const VARS: [&str; 3] = ["x", "y", "z"];

fn int_trace(rng: &mut Rng) -> Exercise {
    let mut vals = [rng.range(1, 9), rng.range(1, 9), rng.range(1, 9)];
    let mut code: Vec<String> = VARS
        .iter()
        .zip(vals)
        .map(|(v, n)| format!("{v} ← {n} ;"))
        .collect();
    let mut trail: Vec<String> = VARS
        .iter()
        .zip(vals)
        .map(|(v, n)| format!("{v} = {n}"))
        .collect();
    let steps = rng.range(3, 4);
    for _ in 0..steps {
        let target = rng.below(3);
        let left = rng.below(3);
        let right = rng.below(3);
        let k = rng.range(2, 3);
        let mut op = rng.below(6);
        // div et mod sur un négatif dépendent des conventions : on les évite.
        if matches!(op, 3 | 4) && vals[left] < 0 {
            op = 0;
        }
        let (text, value) = match op {
            0 => (
                format!("{} + {}", VARS[left], VARS[right]),
                vals[left] + vals[right],
            ),
            1 => (
                format!("{} - {}", VARS[left], VARS[right]),
                vals[left] - vals[right],
            ),
            2 => (format!("{} * {k}", VARS[left]), vals[left] * k),
            3 => (format!("{} div {k}", VARS[left]), vals[left].div_euclid(k)),
            4 => (format!("{} mod {k}", VARS[left]), vals[left].rem_euclid(k)),
            _ => (format!("{} + {k}", VARS[left]), vals[left] + k),
        };
        vals[target] = value;
        code.push(format!("{} ← {text} ;", VARS[target]));
        trail.push(format!("{} = {}", VARS[target], minus(value)));
    }
    let asked = rng.below(3);
    Exercise::new(
        format!("Que vaut {} à la fin de l'algorithme ?", VARS[asked]),
        Answer::Number {
            value: vals[asked],
            radix: 10,
        },
        format!(
            "Trace, instruction par instruction : {}. Chaque affectation écrase l'ancienne valeur.",
            trail.join(" → ")
        ),
    )
    .with_code(code.join("\n"))
}

fn string_trace(rng: &mut Rng) -> Exercise {
    const SYLLABLES: &[&str] = &[
        "ab", "ra", "ca", "da", "bo", "po", "ns", "e", "co", "li", "nu", "x",
    ];
    let picks = rng.sample(SYLLABLES.len(), 3);
    let (sa, sb, lit) = (
        SYLLABLES[picks[0]],
        SYLLABLES[picks[1]],
        SYLLABLES[picks[2]],
    );
    let mut a = sa.to_string();
    let mut b = sb.to_string();
    let mut code = vec![format!("a ← \"{a}\" ;"), format!("b ← \"{b}\" ;")];
    let mut trail = vec![format!("a = \"{a}\""), format!("b = \"{b}\"")];
    if rng.coin() {
        b = format!("{b}{a}");
        code.push("b ← b + a ;".to_string());
        trail.push(format!("b = \"{b}\""));
    }
    let len = rng.range(3, 5);
    let mut c = String::new();
    let mut parts = Vec::new();
    for _ in 0..len {
        match rng.below(3) {
            0 => {
                c.push_str(&a);
                parts.push("a".to_string());
            }
            1 => {
                c.push_str(&b);
                parts.push("b".to_string());
            }
            _ => {
                c.push_str(lit);
                parts.push(format!("\"{lit}\""));
            }
        }
    }
    code.push(format!("c ← {} ;", parts.join(" + ")));
    trail.push(format!("c = \"{c}\""));
    if rng.coin() {
        a = format!("{c}!");
        code.push("a ← c + \"!\" ;".to_string());
        trail.push(format!("a = \"{a}\""));
    }
    Exercise::new(
        "Que vaut c à la fin ?",
        Answer::Text { accepted: vec![c] },
        format!(
            "{}. + concatène les chaînes dans l'ordre écrit.",
            trail.join(" → ")
        ),
    )
    .with_code(code.join("\n"))
}

fn swap(rng: &mut Rng) -> Exercise {
    let x0 = rng.range(2, 60);
    let mut y0 = rng.range(2, 60);
    if y0 == x0 {
        y0 += 1;
    }
    let ask_x = rng.coin();
    let (code, x, y, why) = match rng.below(3) {
        0 => (
            "x ← x + y ;\ny ← x - y ;\nx ← x - y ;",
            y0,
            x0,
            format!("x = {}, puis y = {} − {y0} = {x0}, puis x = {} − {x0} = {y0} : échange sans variable intermédiaire.", x0 + y0, x0 + y0, x0 + y0),
        ),
        1 => (
            "x ← y ;\ny ← x ;",
            y0,
            y0,
            format!("x ← y donne x = {y0} et l'ancienne valeur {x0} est perdue; y ← x recopie {y0}. Il faut une variable temporaire."),
        ),
        _ => (
            "tmp ← x ;\nx ← y ;\ny ← tmp ;",
            y0,
            x0,
            format!("tmp garde {x0}, x reçoit {y0}, puis y reçoit tmp = {x0} : l'échange classique."),
        ),
    };
    Exercise::new(
        format!(
            "Au départ x = {x0} et y = {y0}. Que vaut {} à la fin ?",
            if ask_x { "x" } else { "y" }
        ),
        Answer::Number {
            value: if ask_x { x } else { y },
            radix: 10,
        },
        why,
    )
    .with_code(code)
}

fn duration(rng: &mut Rng) -> Exercise {
    let total = rng.range(3600, 86_399);
    let (h, r) = (total / 3600, total % 3600);
    let (min, s) = (r / 60, r % 60);
    let why = format!(
        "h = {total} div 3600 = {h}, r = {total} mod 3600 = {r}, min = r div 60 = {min}, s = r mod 60 = {s}."
    );
    let ex = match rng.below(4) {
        0 => Exercise::new(
            format!("{h} h {min} min {s} s : combien de secondes au total ?"),
            Answer::Number {
                value: total,
                radix: 10,
            },
            format!("total = {h} × 3600 + {min} × 60 + {s} = {total}."),
        ),
        1 => Exercise::new(
            format!("total = {total} secondes. Que vaut h ?"),
            Answer::Number {
                value: h,
                radix: 10,
            },
            why,
        ),
        2 => Exercise::new(
            format!("total = {total} secondes. Que vaut min ?"),
            Answer::Number {
                value: min,
                radix: 10,
            },
            why,
        ),
        _ => Exercise::new(
            format!("total = {total} secondes. Que vaut s ?"),
            Answer::Number {
                value: s,
                radix: 10,
            },
            why,
        ),
    };
    ex.with_code(
        "h   ← total div 3600 ;\nr   ← total mod 3600 ;\nmin ← r div 60 ;\ns   ← r mod 60 ;",
    )
}
