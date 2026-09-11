//! Entraînement — Méthodes et techniques de calcul.

use super::{card, num, qcm, sup, txt, vf, Answer, Chapter, Exercise, Fixed, Rng};

pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        id: "suites",
        title: "Suites et limites",
        anchor: "m-suites",
        fixed: SUITES,
        generators: &[limite_quotient, forme_indeterminee],
    },
    Chapter {
        id: "arithgeo",
        title: "Arithmétiques et géométriques",
        anchor: "m-arithgeo",
        fixed: ARITHGEO,
        generators: &[
            terme_arithmetique,
            somme_arithmetique,
            terme_geometrique,
            somme_geometrique,
            point_fixe,
            rang_depassement,
        ],
    },
    Chapter {
        id: "recurrence",
        title: "Raisonnement par récurrence",
        anchor: "m-recurrence",
        fixed: RECURRENCE,
        generators: &[somme_par_recurrence],
    },
    Chapter {
        id: "fonctions",
        title: "Fonctions et domaines",
        anchor: "m-fonctions",
        fixed: FONCTIONS,
        generators: &[domaine_quotient],
    },
    Chapter {
        id: "limites-f",
        title: "Limites et asymptotes",
        anchor: "m-limites",
        fixed: LIMITES,
        generators: &[],
    },
    Chapter {
        id: "continuite",
        title: "Continuité",
        anchor: "m-continuite",
        fixed: CONTINUITE,
        generators: &[],
    },
    Chapter {
        id: "usuelles",
        title: "Fonctions usuelles",
        anchor: "m-usuelles",
        fixed: USUELLES,
        generators: &[equation_exponentielle, equation_logarithme, valeur_trigo],
    },
    Chapter {
        id: "derivees",
        title: "Dérivées et variations",
        anchor: "m-derivees",
        fixed: DERIVEES,
        generators: &[derivee_polynome, coefficient_tangente],
    },
    Chapter {
        id: "primitives",
        title: "Primitives et intégrales",
        anchor: "m-primitives",
        fixed: PRIMITIVES,
        generators: &[integrale_affine, primitive_reconnue],
    },
];

// ————————————————————————————————— questions rédigées

const SUITES: &[Fixed] = &[
    qcm(
        "Une suite numérique, c'est…",
        &[
            "une fonction de ℕ dans ℝ",
            "une fonction de ℝ dans ℝ",
            "une partie finie de ℝ",
            "une fonction de ℝ dans ℕ",
        ],
        "u : ℕ → ℝ. On pose uₙ = u(n) : ce qui compte est le comportement quand n devient grand.",
    ),
    txt(
        "Complète la définition : (uₙ) tend vers ℓ si pour tout ε > 0 il existe … tel que n ≥ … entraîne |uₙ − ℓ| < ε.",
        &["n0", "n₀", "un rang n0", "un entier n0"],
        "(∀ε > 0)(∃n₀)(∀n ≥ n₀, |uₙ − ℓ| < ε). C'est l'exhibition de n₀ qui est notée à l'examen.",
    ),
    vf(
        "Donner la valeur de la limite suffit à répondre à une question « démontrer en utilisant la définition ».",
        false,
        "Non : il faut exhiber n₀ en fonction de ε. Sans n₀, la question ne rapporte rien.",
    ),
    qcm(
        "Le théorème des gendarmes s'applique quand…",
        &[
            "uₙ ≤ vₙ ≤ wₙ et (uₙ), (wₙ) convergent vers la même limite",
            "uₙ ≤ vₙ ≤ wₙ et (uₙ), (wₙ) convergent",
            "uₙ ≤ vₙ et (uₙ) tend vers +∞",
            "(vₙ) est bornée",
        ],
        "L'encadrement ne suffit pas : les deux gendarmes doivent converger vers la MÊME limite.",
    ),
    qcm(
        "Si uₙ ≤ vₙ à partir d'un certain rang et lim uₙ = +∞, alors…",
        &[
            "lim vₙ = +∞",
            "lim vₙ = −∞",
            "(vₙ) converge",
            "on ne peut rien dire",
        ],
        "Théorème de comparaison : ce qui est au-dessus d'une suite qui explose explose aussi.",
    ),
    vf(
        "Si uₙ < vₙ pour tout n et si les deux suites convergent, alors lim uₙ < lim vₙ.",
        false,
        "Faux : l'inégalité stricte devient large au passage à la limite. uₙ = 0 et vₙ = 1/n donnent deux limites égales.",
    ),
    qcm(
        "Une suite croissante et majorée…",
        &[
            "converge",
            "tend vers +∞",
            "est de Cauchy mais ne converge pas",
            "peut diverger",
        ],
        "Théorème de la limite monotone. Si aucun majorant n'existe, alors lim uₙ = +∞.",
    ),
    qcm(
        "(uₙ) est de Cauchy signifie…",
        &[
            "pour tout ε > 0 il existe n₀ tel que p, q ≥ n₀ entraîne |u_p − u_q| < ε",
            "pour tout ε > 0 il existe n₀ tel que |u_{n+1} − uₙ| < ε",
            "(uₙ) est bornée",
            "(uₙ) est monotone",
        ],
        "Deux termes quelconques de rang assez grand, pas seulement deux termes consécutifs.",
    ),
    vf(
        "Une suite converge si, et seulement si, elle est de Cauchy.",
        true,
        "C'est le théorème du cours. Il permet de prouver qu'une suite diverge sans connaître de limite.",
    ),
    qcm(
        "La suite uₙ = 1 + 1/2 + 1/3 + ⋯ + 1/n…",
        &[
            "n'est pas de Cauchy, donc diverge",
            "converge vers 2",
            "est de Cauchy",
            "est décroissante",
        ],
        "|u_{2n} − uₙ| ≥ 1/2 quel que soit n : le critère de Cauchy est violé, la série harmonique diverge.",
    ),
    qcm(
        "Laquelle de ces écritures n'est PAS une forme indéterminée ?",
        &["0 × ℓ avec ℓ fini", "∞ − ∞", "0/0", "0 × ∞"],
        "Les quatre formes indéterminées sont ∞ − ∞, 0 × ∞, 0/0 et ∞/∞. Un produit 0 × ℓ vaut 0.",
    ),
    card(
        "Théorème des gendarmes",
        "Si uₙ ≤ vₙ ≤ wₙ à partir d'un certain rang et si lim uₙ = lim wₙ = ℓ, alors lim vₙ = ℓ.",
    ),
    card(
        "Théorème de la limite monotone",
        "Une suite croissante majorée converge. Une suite croissante non majorée tend vers +∞.",
    ),
];

const ARITHGEO: &[Fixed] = &[
    qcm(
        "Une suite est arithmétique lorsque…",
        &[
            "la différence uₙ₊₁ − uₙ est constante",
            "le quotient uₙ₊₁ / uₙ est constant",
            "elle est croissante",
            "elle converge",
        ],
        "Différence constante = arithmétique, quotient constant = géométrique.",
    ),
    txt(
        "Terme général d'une suite arithmétique de premier terme u₀ et de raison r :",
        &["u0 + nr", "uₙ = u0 + nr", "u0+n*r", "u0 + n×r", "u0+nr"],
        "uₙ = u₀ + n·r. À ne pas confondre avec la formule géométrique uₙ = u₀·qⁿ.",
    ),
    txt(
        "Terme général d'une suite géométrique de premier terme u₀ et de raison q :",
        &["u0 q^n", "uₙ = u0 q^n", "u0*q^n", "u0 × q^n", "u0qn"],
        "uₙ = u₀·qⁿ.",
    ),
    qcm(
        "La somme u₀ + u₁ + ⋯ + uₙ d'une suite arithmétique vaut…",
        &[
            "(n + 1)(u₀ + uₙ)/2",
            "n(u₀ + uₙ)/2",
            "(n + 1)(u₀ + uₙ)",
            "u₀(1 − qⁿ⁺¹)/(1 − q)",
        ],
        "Il y a n + 1 termes, pas n : c'est l'erreur la plus fréquente sur cette formule.",
    ),
    qcm(
        "La somme u₀ + u₁ + ⋯ + uₙ d'une suite géométrique de raison q ≠ 1 vaut…",
        &[
            "u₀(1 − qⁿ⁺¹)/(1 − q)",
            "u₀(1 − qⁿ)/(1 − q)",
            "(n + 1)u₀",
            "u₀qⁿ/(1 − q)",
        ],
        "Exposant n + 1 au numérateur. Si q = 1, la somme vaut simplement (n + 1)u₀.",
    ),
    num(
        "Combien vaut 1 + 2 + ⋯ + 100 ?",
        5050,
        "n(n+1)/2 = 100 × 101/2 = 5050.",
    ),
    num(
        "Combien vaut 1² + 2² + ⋯ + 10² ?",
        385,
        "n(n+1)(2n+1)/6 = 10 × 11 × 21/6 = 385.",
    ),
    num(
        "Combien vaut 1³ + 2³ + ⋯ + 5³ ?",
        225,
        "(n(n+1)/2)² = (5 × 6/2)² = 15² = 225.",
    ),
    qcm(
        "Pour uₙ₊₁ = 2uₙ + 3, quelle suite auxiliaire rend le calcul géométrique ?",
        &["vₙ = uₙ + 3", "vₙ = uₙ − 3", "vₙ = 2uₙ", "vₙ = uₙ/2"],
        "Le point fixe ℓ vérifie ℓ = 2ℓ + 3, donc ℓ = −3 et vₙ = uₙ − ℓ = uₙ + 3.",
    ),
    qcm(
        "Une suite géométrique de raison q converge vers 0 lorsque…",
        &["|q| < 1", "q > 1", "q < 0", "q = 1"],
        "Si |q| < 1 alors qⁿ → 0. Si q > 1 elle tend vers ±∞, si q ≤ −1 elle n'a pas de limite.",
    ),
    card(
        "Somme géométrique",
        "1 + q + q² + ⋯ + qⁿ = (1 − qⁿ⁺¹)/(1 − q) pour q ≠ 1, et n + 1 si q = 1.",
    ),
];

const RECURRENCE: &[Fixed] = &[
    qcm(
        "Les deux étapes d'une récurrence sont…",
        &[
            "l'initialisation et l'hérédité",
            "l'hypothèse et la conclusion",
            "la conjecture et la vérification",
            "le cas de base et la contraposée",
        ],
        "Initialisation au rang n₀, puis hérédité : P(n) ⇒ P(n+1).",
    ),
    qcm(
        "Dans l'hérédité, on suppose…",
        &[
            "P(n) vraie pour un entier n ≥ n₀ fixé",
            "P(n) vraie pour tout n",
            "P(n+1) vraie",
            "P(n₀) fausse",
        ],
        "On fixe n et on suppose P(n) : c'est l'hypothèse de récurrence. Supposer P(n) pour tout n reviendrait à supposer le résultat.",
    ),
    vf(
        "On peut se passer de l'initialisation si l'hérédité est démontrée proprement.",
        false,
        "Non : sans rang initial vrai, l'hérédité ne démarre jamais. La démonstration est nulle.",
    ),
    vf(
        "Utiliser la formule à démontrer au rang n + 1 pendant l'hérédité est une erreur.",
        true,
        "C'est l'erreur listée par le cours : on ne peut utiliser que l'hypothèse au rang n.",
    ),
    qcm(
        "La conclusion d'une récurrence doit…",
        &[
            "invoquer explicitement le principe de récurrence",
            "se contenter de « donc c'est vrai pour tout n »",
            "rappeler l'énoncé",
            "être omise si l'hérédité est claire",
        ],
        "Le cours l'écrit noir sur blanc : écrire « la propriété est vraie pour tout n » sans citer le principe de récurrence est une erreur fréquente.",
    ),
    qcm(
        "Pour montrer que 4 divise 5ⁿ − 1, le bon geste dans l'hérédité est…",
        &[
            "écrire 5ⁿ⁺¹ − 1 = 5(5ⁿ − 1) + 4",
            "factoriser par 5ⁿ",
            "diviser par 4",
            "poser n = 4k",
        ],
        "5ⁿ⁺¹ − 1 = 5·5ⁿ − 1 = 5(5ⁿ − 1) + 4 : les deux morceaux sont divisibles par 4.",
    ),
    num(
        "À quel rang faut-il initialiser une récurrence sur « pour tout n ≥ 1, n! ≥ 2ⁿ⁻¹ » ?",
        1,
        "L'énoncé fixe lui-même le rang initial : n₀ = 1.",
    ),
    card(
        "Les cinq temps de la rédaction",
        "1. Énoncer P(n). 2. Initialisation. 3. Supposer P(n) pour n ≥ n₀ fixé. 4. Démontrer P(n+1). 5. Conclure par le principe de récurrence.",
    ),
];

const FONCTIONS: &[Fixed] = &[
    qcm(
        "Combien d'expressions posent problème pour le domaine de définition ?",
        &[
            "trois : division, racine, logarithme",
            "deux : division et racine",
            "quatre",
            "une seule : la division",
        ],
        "Division par 0, racines et puissances non entières (≥ 0), logarithmes (> 0). Rien d'autre.",
    ),
    qcm(
        "Le domaine de définition de f(x) = ln(x − 3) est…",
        &["]3, +∞[", "[3, +∞[", "ℝ \\ {3}", "]−∞, 3["],
        "Le logarithme exige un argument strictement positif : x − 3 > 0, donc x > 3. La borne est exclue.",
    ),
    qcm(
        "Le domaine de définition de f(x) = √(x − 1) est…",
        &["[1, +∞[", "]1, +∞[", "ℝ", "]−∞, 1]"],
        "La racine exige un argument positif OU NUL : x − 1 ≥ 0. La borne 1 est incluse, contrairement au cas du logarithme.",
    ),
    qcm(
        "f : x ↦ x² définie de ℝ dans ℝ est…",
        &[
            "ni injective ni surjective",
            "injective et surjective",
            "injective seulement",
            "surjective seulement",
        ],
        "f(−x) = f(x) : pas injective. Aucun y < 0 n'a d'antécédent : pas surjective. Elle devient bijective de ℝ₊ dans ℝ₊.",
    ),
    vf(
        "Une fonction strictement monotone sur I est une bijection de I sur f(I).",
        true,
        "C'est le théorème du cours : la stricte monotonie donne l'injectivité, et f est surjective sur f(I) par définition.",
    ),
    vf(
        "L'image d'un réel par une fonction est unique, son antécédent aussi.",
        false,
        "L'image est unique, l'antécédent ne l'est pas : 16 a deux antécédents par x ↦ x², à savoir 4 et −4.",
    ),
    qcm(
        "f est paire lorsque…",
        &[
            "D_f est symétrique par rapport à 0 et f(−x) = f(x)",
            "f(−x) = −f(x)",
            "f est croissante",
            "le graphe est symétrique par rapport à l'origine",
        ],
        "Paire : symétrie du graphe par rapport à l'axe Oy. Impaire : symétrie par rapport à l'origine.",
    ),
    qcm(
        "Si f : x ↦ x³ et g : x ↦ cos x, alors g ∘ f est…",
        &["x ↦ cos(x³)", "x ↦ (cos x)³", "x ↦ cos(x)·x³", "x ↦ 3x²·cos x"],
        "(g ∘ f)(x) = g(f(x)) = cos(x³). L'ordre compte : f ∘ g donnerait (cos x)³.",
    ),
    card(
        "Domaine de définition — la check-list",
        "Dénominateur ≠ 0 ; argument d'une racine ou d'une puissance non entière ≥ 0 ; argument d'un logarithme > 0. Traiter chaque contrainte, puis intersecter.",
    ),
];

const LIMITES: &[Fixed] = &[
    qcm(
        "f admet une limite ℓ en x₀ si, et seulement si…",
        &[
            "les limites à gauche et à droite existent et valent toutes deux ℓ",
            "la limite à droite vaut ℓ",
            "f est définie en x₀",
            "f est continue en x₀",
        ],
        "Les deux limites latérales doivent exister ET coïncider.",
    ),
    qcm(
        "lim_{x→2} (x² − 3x + 2)/(x − 2) vaut…",
        &["1", "0", "+∞", "n'existe pas"],
        "Forme 0/0 : on factorise x² − 3x + 2 = (x − 1)(x − 2), il reste x − 1, qui vaut 1 en 2.",
    ),
    qcm(
        "Pour f(x) = (x² + 3)/(x − 2), la limite en 2⁺ vaut…",
        &["+∞", "−∞", "7", "0"],
        "Le numérateur tend vers 7 > 0, le dénominateur vers 0⁺ : le quotient tend vers +∞. À gauche, le dénominateur tend vers 0⁻ et la limite est −∞.",
    ),
    qcm(
        "La droite y = ax + b est asymptote oblique à (C_f) en +∞ lorsque…",
        &[
            "lim_{x→+∞} f(x) − (ax + b) = 0",
            "lim_{x→+∞} f(x) = ax + b",
            "lim_{x→+∞} f(x)/x = a",
            "f(x) = ax + b pour x grand",
        ],
        "C'est la différence qui doit tendre vers 0. Pour la trouver, on fait la division euclidienne.",
    ),
    qcm(
        "x = x₀ est asymptote verticale lorsque…",
        &[
            "lim_{x→x₀} f(x) = ±∞",
            "f(x₀) = 0",
            "lim_{x→+∞} f(x) = x₀",
            "f n'est pas définie en x₀",
        ],
        "Une valeur interdite ne suffit pas : il faut que la limite soit infinie. (x²−3x+2)/(x−2) n'a pas d'asymptote en 2.",
    ),
    qcm(
        "f(x) = (2x² − 3x)/(x − 2) s'écrit aussi…",
        &[
            "2x + 1 + 2/(x − 2)",
            "2x − 1 + 2/(x − 2)",
            "2x + 1 − 2/(x − 2)",
            "2x + 3/(x − 2)",
        ],
        "Division euclidienne : l'asymptote oblique est y = 2x + 1, et x = 2 est asymptote verticale. C'est l'exercice 30 du TD 3.",
    ),
    vf(
        "lim_{x→0} (x + √(x²))/x existe.",
        false,
        "√(x²) = |x| : la fonction vaut x + 1 à droite et x − 1 à gauche. Les limites valent 1 et −1, donc pas de limite en 0.",
    ),
    card(
        "Les quatre asymptotes",
        "Verticale x = x₀ si lim en x₀ est infinie. Horizontale y = ℓ si lim en ±∞ est finie. Oblique y = ax + b si f(x) − (ax+b) → 0. Quelconque y = g(x) si f(x) − g(x) → 0.",
    ),
];

const CONTINUITE: &[Fixed] = &[
    qcm(
        "La caractérisation séquentielle de la continuité dit que f est continue en x₀ si…",
        &[
            "pour toute suite uₙ → x₀, on a f(uₙ) → f(x₀)",
            "pour une suite uₙ → x₀, on a f(uₙ) → f(x₀)",
            "f est dérivable en x₀",
            "f est monotone au voisinage de x₀",
        ],
        "« Pour toute suite » : une seule suite ne suffit pas à prouver la continuité, mais une seule suite suffit à la réfuter.",
    ),
    qcm(
        "Quelles opérations conservent la continuité ?",
        &[
            "somme, produit, composée, et 1/f si f ne s'annule pas",
            "somme et produit seulement",
            "toutes, sans condition",
            "composée seulement",
        ],
        "Le quotient demande que le dénominateur ne s'annule pas sur l'intervalle considéré.",
    ),
    vf(
        "La réciproque d'une bijection continue strictement monotone est continue.",
        true,
        "C'est ainsi que le cours établit la continuité de √ sur ℝ₊ : c'est la réciproque de x ↦ x² sur ℝ₊.",
    ),
    qcm(
        "x ↦ 1/x² est continue sur…",
        &["]−∞, 0[ et ]0, +∞[", "ℝ", "ℝ₊", "[0, +∞["],
        "Sur chacun des deux intervalles, pas sur leur réunion prise comme un seul intervalle : 0 est exclu.",
    ),
    vf(
        "La valeur absolue est continue sur ℝ.",
        true,
        "|x| = √(x²) : c'est une composée de deux fonctions continues.",
    ),
    card(
        "Continuité — à quoi ça sert",
        "À justifier en une ligne que la fonction étudiée se comporte bien : polynômes, exp, ln, sin, cos et leurs composées sont continues sur leur domaine.",
    ),
];

const USUELLES: &[Fixed] = &[
    qcm(
        "ln est définie et continue sur…",
        &["]0, +∞[", "[0, +∞[", "ℝ", "ℝ*"],
        "Strictement positif : la borne 0 est exclue. ln est strictement croissante et ln(1) = 0.",
    ),
    txt(
        "ln(a/b) = ?",
        &["ln(a) - ln(b)", "ln a - ln b", "lna-lnb", "ln(a)-ln(b)"],
        "ln(a/b) = ln a − ln b, et ln(1/a) = −ln a.",
    ),
    qcm(
        "ln(aⁿ) vaut…",
        &["n·ln(a)", "(ln a)ⁿ", "ln(a)/n", "aⁿ·ln a"],
        "L'exposant descend en facteur. Même règle pour log_b(xⁿ) = n·log_b(x).",
    ),
    qcm(
        "log_b(x) se définit par…",
        &["ln(x)/ln(b)", "ln(b)/ln(x)", "ln(x)·ln(b)", "b·ln(x)"],
        "D'où log_b(b) = 1. En informatique on utilise surtout log₂.",
    ),
    vf(
        "eˣ peut être nul pour certaines valeurs de x.",
        false,
        "Jamais : eˣ > 0 pour tout réel x. C'est pourquoi (x² + 1)e^{x−1} = −1 n'a pas de solution (exercice 14 du TD 3).",
    ),
    qcm(
        "e^{a+b} vaut…",
        &["eᵃ·eᵇ", "eᵃ + eᵇ", "e^{ab}", "(eᵃ)ᵇ"],
        "Une somme d'exposants devient un produit. Et eᵃ/eᵇ = e^{a−b}.",
    ),
    qcm(
        "lim_{x→+∞} eˣ/xᵃ pour a > 0 vaut…",
        &["+∞", "0", "1", "a"],
        "Croissance comparée : l'exponentielle l'emporte sur toute puissance.",
    ),
    qcm(
        "lim_{x→+∞} (ln x)/xᵃ pour a > 0 vaut…",
        &["0", "+∞", "1", "a"],
        "Croissance comparée : toute puissance l'emporte sur le logarithme.",
    ),
    qcm(
        "lim_{x→0⁺} x·ln x vaut…",
        &["0", "−∞", "+∞", "1"],
        "Forme 0 × (−∞), levée par croissance comparée : lim_{x→0⁺} xᵃ·ln x = 0 pour tout a > 0.",
    ),
    qcm(
        "αˣ pour α > 0 se réécrit…",
        &["e^{x·ln α}", "e^{α·ln x}", "x·eᵅ", "ln(αˣ)"],
        "C'est la définition. Elle sert aussi pour xᵝ = e^{β ln x} et pour xˣ = e^{x ln x} (exercice 31 du TD 3).",
    ),
    qcm(
        "La fonction tangente est périodique de période…",
        &["π", "2π", "π/2", "4π"],
        "π, et non 2π comme sin et cos. Son domaine est ℝ privé de π/2 + kπ.",
    ),
    qcm(
        "sin est…",
        &["impaire", "paire", "ni paire ni impaire", "périodique de période π"],
        "sin(−x) = −sin(x) : impaire. cos est paire.",
    ),
    txt(
        "sin²x + cos²x = ?",
        &["1"],
        "L'identité fondamentale. Elle donne aussi 1 + tan²x = 1/cos²x en divisant par cos²x.",
    ),
    card(
        "Croissances comparées",
        "L'exponentielle l'emporte sur toute puissance, qui l'emporte sur le logarithme. C'est ce qui lève les formes ∞/∞ et 0 × ∞.",
    ),
];

const DERIVEES: &[Fixed] = &[
    qcm(
        "f est dérivable en a lorsque…",
        &[
            "lim_{x→a} (f(x) − f(a))/(x − a) existe et est finie",
            "lim_{x→a} f(x) = f(a)",
            "f est continue en a",
            "f est monotone au voisinage de a",
        ],
        "« Et est finie » : pour √ en 0, la limite existe mais vaut +∞, donc √ n'est pas dérivable en 0.",
    ),
    qcm(
        "L'équation de la tangente à (C_f) au point d'abscisse a est…",
        &[
            "y = f′(a)(x − a) + f(a)",
            "y = f′(a)·x + f(a)",
            "y = f(a)(x − a) + f′(a)",
            "y = f′(a)(x + a) − f(a)",
        ],
        "Le coefficient directeur est f′(a) et la droite passe par (a, f(a)).",
    ),
    txt(
        "Dérivée de ln|x| :",
        &["1/x", "1 / x", "x^-1"],
        "Sur ℝ*, la dérivée de ln|x| est 1/x.",
    ),
    txt(
        "Dérivée de cos x :",
        &["-sin x", "-sin(x)", "−sin x", "-sinx"],
        "Avec le signe moins. La dérivée de sin x est cos x, sans signe.",
    ),
    qcm(
        "(f/g)′ vaut…",
        &[
            "(f′g − g′f)/g²",
            "(f′g + g′f)/g²",
            "f′/g′",
            "(g′f − f′g)/g²",
        ],
        "Numérateur puis dénominateur, et le signe moins au milieu. L'ordre compte.",
    ),
    qcm(
        "(g ∘ f)′(x₀) vaut…",
        &[
            "f′(x₀)·g′(f(x₀))",
            "g′(x₀)·f′(g(x₀))",
            "f′(x₀)·g′(x₀)",
            "(g′ ∘ f′)(x₀)",
        ],
        "Dérivée de l'intérieure, multipliée par la dérivée de l'extérieure évaluée en f(x₀).",
    ),
    txt(
        "Dérivée de e^{u(x)} :",
        &["u'(x)e^{u(x)}", "u' e^u", "u'(x)*e^(u(x))", "u'e^u"],
        "u′·e^u. De même (ln|u|)′ = u′/u et (√u)′ = u′/(2√u).",
    ),
    vf(
        "f strictement croissante sur I équivaut à f′(x) > 0 pour tout x de I.",
        false,
        "Faux : x ↦ x³ est strictement croissante sur ℝ, pourtant sa dérivée s'annule en 0. L'équivalence ne vaut qu'au sens large.",
    ),
    qcm(
        "f est croissante sur I si, et seulement si…",
        &[
            "f′(x) ≥ 0 pour tout x de I",
            "f′(x) > 0 pour tout x de I",
            "f′(x) ≠ 0 pour tout x de I",
            "f′ est croissante",
        ],
        "Au sens large, c'est une équivalence. Au sens strict, seulement une implication depuis f′ > 0 sur l'ouvert.",
    ),
    qcm(
        "tan′x vaut…",
        &["1/cos²x", "1/sin²x", "−1/cos²x", "cos²x"],
        "Ou, ce qui revient au même, 1 + tan²x. La première forme sert à intégrer, la seconde à étudier le signe.",
    ),
    qcm(
        "Dans une étude de fonction, le domaine de dérivabilité…",
        &[
            "peut être plus petit que le domaine de définition",
            "est toujours égal au domaine de définition",
            "est toujours ℝ",
            "n'a pas à être précisé",
        ],
        "√ est définie sur [0, +∞[ mais dérivable seulement sur ]0, +∞[. Le TD 3 demande les deux à chaque exercice.",
    ),
    card(
        "Le plan d'une étude de fonction",
        "1. D_f et D_f′. 2. Limites aux bornes. 3. Asymptotes. 4. f′ factorisée. 5. Tableau de variations. 6. Allure de la courbe.",
    ),
];

const PRIMITIVES: &[Fixed] = &[
    qcm(
        "F est une primitive de f lorsque…",
        &["F′ = f", "f′ = F", "F = f′", "F·f = 1"],
        "On note F = ∫f(x)dx. Deux primitives d'une même fonction diffèrent d'une constante.",
    ),
    txt(
        "∫ dx/x = ?",
        &["ln|x|", "ln |x|", "ln(|x|)", "lnx", "ln x"],
        "Avec la valeur absolue : la primitive vaut ln|x| sur ℝ*.",
    ),
    qcm(
        "∫ xᵅ dx pour α ≠ −1 vaut…",
        &[
            "x^{α+1}/(α+1)",
            "α·x^{α−1}",
            "x^{α−1}/(α−1)",
            "x^{α+1}",
        ],
        "On monte l'exposant et on divise par le nouvel exposant. Le cas α = −1 donne ln|x|.",
    ),
    qcm(
        "∫ tan x dx vaut…",
        &["− ln|cos x|", "ln|cos x|", "1/cos²x", "− cos x"],
        "Car tan = sin/cos est de la forme −u′/u avec u = cos.",
    ),
    qcm(
        "∫ dx/(1 + x²) vaut…",
        &["arctan x", "ln(1 + x²)", "1/(2x)", "tan x"],
        "C'est la brique qui sort des trinômes sans racine réelle au dénominateur.",
    ),
    qcm(
        "La formule d'intégration par parties est…",
        &[
            "∫u′v = uv − ∫v′u",
            "∫u′v = uv + ∫v′u",
            "∫uv = u′v − ∫uv′",
            "∫u′v′ = uv",
        ],
        "On choisit comme v le facteur qui se simplifie en dérivant : un polynôme, ou ln x.",
    ),
    qcm(
        "Pour ∫ x·eˣ dx, on pose…",
        &[
            "v = x et u′ = eˣ",
            "v = eˣ et u′ = x",
            "u = x et v = eˣ",
            "un changement de variable U = eˣ",
        ],
        "On dérive le polynôme, on intègre l'exponentielle : ∫x·eˣdx = x·eˣ − eˣ.",
    ),
    qcm(
        "Pour ∫ 2x·sin(x²) dx, la bonne technique est…",
        &[
            "le changement de variable U = x²",
            "l'intégration par parties",
            "la décomposition en éléments simples",
            "une primitive directe",
        ],
        "On reconnaît u′·v′(u) avec u = x² : dU = 2x dx, l'intégrale devient ∫sin U dU = −cos(x²).",
    ),
    qcm(
        "∫ c/(x − λ) dx vaut…",
        &[
            "c·ln|x − λ|",
            "c/(x − λ)²",
            "ln|c(x − λ)|",
            "−c/(x − λ)",
        ],
        "Première brique de la décomposition en éléments simples.",
    ),
    qcm(
        "Pour ∫ P(cos x, sin x)/Q(cos x, sin x) dx, on pose…",
        &["T = tan(x/2)", "T = tan x", "T = sin x", "U = cos x"],
        "Alors sin x = 2T/(1+T²), cos x = (1−T²)/(1+T²) et dx = 2dT/(1+T²) : tout devient rationnel.",
    ),
    card(
        "Choisir la technique en trois secondes",
        "Je vois u′ à côté de u → changement de variable. Je vois polynôme × (eˣ, sin, cos, ln) → parties. Je vois un quotient de polynômes → éléments simples.",
    ),
];

// ————————————————————————————————— générateurs

fn terme_arithmetique(rng: &mut Rng) -> Exercise {
    let u0 = rng.range(1, 40);
    let r = rng.range(2, 15);
    let n = rng.range(5, 20);
    let un = u0 + n * r;
    Exercise::new(
        format!(
            "Suite arithmétique de premier terme u₀ = {u0} et de raison r = {r}. Combien vaut u{} ?",
            sub_n(n)
        ),
        Answer::Number {
            value: un,
            radix: 10,
        },
        format!("uₙ = u₀ + n·r = {u0} + {n} × {r} = {un}."),
    )
}

fn somme_arithmetique(rng: &mut Rng) -> Exercise {
    let u0 = rng.range(1, 12);
    let r = rng.range(2, 9);
    let n = rng.range(4, 15);
    let un = u0 + n * r;
    let s = (n + 1) * (u0 + un) / 2;
    Exercise::new(
        format!(
            "Suite arithmétique u₀ = {u0}, r = {r}. Combien vaut la somme u₀ + u₁ + ⋯ + u{} ?",
            sub_n(n)
        ),
        Answer::Number {
            value: s,
            radix: 10,
        },
        format!(
            "u{} = {u0} + {n} × {r} = {un}, puis (n+1)(u₀ + uₙ)/2 = {} × {}/2 = {s}. Attention : n + 1 termes.",
            sub_n(n),
            n + 1,
            u0 + un
        ),
    )
}

fn terme_geometrique(rng: &mut Rng) -> Exercise {
    let u0 = rng.range(1, 6);
    let q = rng.range(2, 4);
    let n = rng.range(3, 8);
    let un = u0 * q.pow(n as u32);
    Exercise::new(
        format!(
            "Suite géométrique de premier terme u₀ = {u0} et de raison q = {q}. Combien vaut u{} ?",
            sub_n(n)
        ),
        Answer::Number {
            value: un,
            radix: 10,
        },
        format!("uₙ = u₀·qⁿ = {u0} × {q}{} = {un}.", sup(n as u32)),
    )
}

fn somme_geometrique(rng: &mut Rng) -> Exercise {
    let u0 = rng.range(1, 5);
    let q = rng.range(2, 4);
    let n = rng.range(3, 8);
    let s = u0 * (q.pow(n as u32 + 1) - 1) / (q - 1);
    Exercise::new(
        format!(
            "Suite géométrique u₀ = {u0}, q = {q}. Combien vaut u₀ + u₁ + ⋯ + u{} ?",
            sub_n(n)
        ),
        Answer::Number {
            value: s,
            radix: 10,
        },
        format!(
            "u₀(1 − qⁿ⁺¹)/(1 − q) = {u0} × ({}{} − 1)/({} − 1) = {s}. L'exposant est n + 1, pas n.",
            q,
            sup(n as u32 + 1),
            q
        ),
    )
}

fn point_fixe(rng: &mut Rng) -> Exercise {
    // uₙ₊₁ = a·uₙ + b, avec (1 − a) qui divise b pour un point fixe entier.
    let a = *rng.pick(&[2i64, 3, 4, 5]);
    let l = *rng.pick(&[-6i64, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6]);
    let b = l * (1 - a);
    Exercise::new(
        format!(
            "Soit la suite définie par uₙ₊₁ = {a}·uₙ {}. Quel est son point fixe ℓ (la solution de ℓ = {a}ℓ {}) ?",
            signed(b),
            signed(b)
        ),
        Answer::Number {
            value: l,
            radix: 10,
        },
        format!(
            "ℓ = {a}ℓ {} donne ℓ(1 − {a}) = {b}, donc ℓ = {l}. La suite vₙ = uₙ − ({l}) est alors géométrique de raison {a}.",
            signed(b)
        ),
    )
}

fn rang_depassement(rng: &mut Rng) -> Exercise {
    let u0 = rng.range(2, 9);
    let q = *rng.pick(&[2i64, 3]);
    let steps = rng.range(4, 11);
    // Seuil placé strictement entre u_{steps−1} et u_steps : la réponse est `steps`.
    let before = u0 * q.pow(steps as u32 - 1);
    let at = u0 * q.pow(steps as u32);
    let seuil = (before + at) / 2;
    Exercise::new(
        format!(
            "Une quantité vaut {u0} au départ et est multipliée par {q} à chaque étape. À partir de quelle étape n dépasse-t-elle {seuil} ?"
        ),
        Answer::Number {
            value: steps,
            radix: 10,
        },
        format!(
            "uₙ = {u0} × {q}ⁿ. On a u{} = {before} ≤ {seuil} et u{} = {at} > {seuil}, donc n = {steps}.",
            sub_n(steps - 1),
            sub_n(steps)
        ),
    )
}

fn somme_par_recurrence(rng: &mut Rng) -> Exercise {
    let n = rng.range(4, 14);
    let kind = rng.below(3);
    let (prompt, value, explain) = match kind {
        0 => (
            format!("Combien vaut 1 + 2 + ⋯ + {n} ?"),
            n * (n + 1) / 2,
            format!("n(n+1)/2 = {n} × {}/2 = {}.", n + 1, n * (n + 1) / 2),
        ),
        1 => (
            format!("Combien vaut 1² + 2² + ⋯ + {n}² ?"),
            n * (n + 1) * (2 * n + 1) / 6,
            format!(
                "n(n+1)(2n+1)/6 = {n} × {} × {}/6 = {}.",
                n + 1,
                2 * n + 1,
                n * (n + 1) * (2 * n + 1) / 6
            ),
        ),
        _ => (
            format!(
                "Combien vaut 1 + 3 + 5 + ⋯ + {} (les {n} premiers impairs) ?",
                2 * n - 1
            ),
            n * n,
            format!("∑(2i − 1) pour i de 1 à n vaut n², soit {n}² = {}.", n * n),
        ),
    };
    Exercise::new(prompt, Answer::Number { value, radix: 10 }, explain)
}

fn limite_quotient(rng: &mut Rng) -> Exercise {
    let d = rng.range(1, 6);
    let k = rng.range(1, 6);
    let a = k * d;
    let b = rng.range(1, 9);
    let c = rng.range(1, 9);
    Exercise::new(
        format!("Combien vaut lim_{{n→+∞}} ({a}n² + {b})/({d}n² + {c}) ?"),
        Answer::Number {
            value: k,
            radix: 10,
        },
        format!(
            "On garde les termes dominants : ({a}n²)/({d}n²) = {a}/{d} = {k}. Les constantes {b} et {c} deviennent négligeables."
        ),
    )
}

fn forme_indeterminee(rng: &mut Rng) -> Exercise {
    // √(n² + 2an) − n → a. On prend a entier pour une réponse entière.
    let a = rng.range(1, 9);
    Exercise::new(
        format!("Combien vaut lim_{{n→+∞}} √(n² + {}n) − n ?", 2 * a),
        Answer::Number {
            value: a,
            radix: 10,
        },
        format!(
            "Quantité conjuguée : √(n² + {}n) − n = {}n/(√(n² + {}n) + n) → {}n/(2n) = {a}.",
            2 * a,
            2 * a,
            2 * a,
            2 * a
        ),
    )
}

fn domaine_quotient(rng: &mut Rng) -> Exercise {
    let a = rng.range(1, 9);
    let kind = rng.below(3);
    let (prompt, right, wrong) = match kind {
        0 => (
            format!("Quel est le domaine de définition de f(x) = ln(x − {a}) ?"),
            format!("]{a}, +∞["),
            vec![
                format!("[{a}, +∞["),
                format!("]−∞, {a}["),
                format!("ℝ \\ {{{a}}}"),
            ],
        ),
        1 => (
            format!("Quel est le domaine de définition de f(x) = √(x − {a}) ?"),
            format!("[{a}, +∞["),
            vec![
                format!("]{a}, +∞["),
                format!("]−∞, {a}]"),
                format!("ℝ \\ {{{a}}}"),
            ],
        ),
        _ => (
            format!("Quel est le domaine de définition de f(x) = 1/(x − {a}) ?"),
            format!("ℝ \\ {{{a}}}"),
            vec![
                format!("]{a}, +∞["),
                format!("[{a}, +∞["),
                format!("]−∞, {a}["),
            ],
        ),
    };
    let explain = match kind {
        0 => {
            "Le logarithme exige un argument strictement positif : la borne est exclue.".to_string()
        }
        1 => "La racine exige un argument positif ou nul : la borne est incluse.".to_string(),
        _ => "Seule la division pose problème : on retire la valeur qui annule le dénominateur."
            .to_string(),
    };
    Exercise::new(prompt, super::shuffled_choice(rng, right, wrong), explain)
}

fn equation_exponentielle(rng: &mut Rng) -> Exercise {
    let a = *rng.pick(&[1i64, 2, 3, 4, 5]);
    let x = *rng.pick(&[-5i64, -4, -3, -2, -1, 1, 2, 3, 4, 5]);
    let b = -a * x;
    Exercise::new(
        format!("Résous e^({a}x {}) = 1. Que vaut x ?", signed(b)),
        Answer::Number {
            value: x,
            radix: 10,
        },
        format!(
            "eᵘ = 1 équivaut à u = 0, donc {a}x {} = 0 et x = {x}.",
            signed(b)
        ),
    )
}

fn equation_logarithme(rng: &mut Rng) -> Exercise {
    let a = *rng.pick(&[1i64, 2, 3, 4, 5]);
    let x = rng.range(1, 8);
    let b = 1 - a * x;
    Exercise::new(
        format!("Résous ln({a}x {}) = 0. Que vaut x ?", signed(b)),
        Answer::Number {
            value: x,
            radix: 10,
        },
        format!(
            "ln(u) = 0 équivaut à u = 1, donc {a}x {} = 1 et x = {x}. Il reste à vérifier que l'argument est bien > 0.",
            signed(b)
        ),
    )
}

fn valeur_trigo(rng: &mut Rng) -> Exercise {
    const TABLE: [(&str, &str, &str); 6] = [
        ("0", "0", "1"),
        ("π/6", "1/2", "√3/2"),
        ("π/4", "√2/2", "√2/2"),
        ("π/3", "√3/2", "1/2"),
        ("π/2", "1", "0"),
        ("π", "0", "−1"),
    ];
    let (angle, sin, cos) = *rng.pick(&TABLE);
    let want_sin = rng.coin();
    let right = if want_sin { sin } else { cos };
    let wrong: Vec<String> = ["0", "1", "−1", "1/2", "√2/2", "√3/2"]
        .iter()
        .filter(|v| **v != right)
        .map(|v| v.to_string())
        .collect();
    Exercise::new(
        format!(
            "Combien vaut {}({angle}) ?",
            if want_sin { "sin" } else { "cos" }
        ),
        super::shuffled_choice(rng, right.to_string(), wrong),
        format!("Sur le cercle : sin({angle}) = {sin} et cos({angle}) = {cos}."),
    )
}

fn derivee_polynome(rng: &mut Rng) -> Exercise {
    let a = rng.range(1, 5);
    let b = *rng.pick(&[-5i64, -4, -3, -2, -1, 1, 2, 3, 4, 5]);
    let c = *rng.pick(&[-6i64, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6]);
    let x0 = rng.range(-3, 3);
    let value = 3 * a * x0 * x0 + 2 * b * x0 + c;
    Exercise::new(
        format!(
            "Soit f(x) = {a}x³ {}x² {}x + 1. Combien vaut f′({x0}) ?",
            signed_coef(b),
            signed_coef(c)
        ),
        Answer::Number { value, radix: 10 },
        format!(
            "f′(x) = {}x² {}x {}, donc f′({x0}) = {} {} {} = {value}.",
            3 * a,
            signed_coef(2 * b),
            signed(c),
            3 * a * x0 * x0,
            signed(2 * b * x0),
            signed(c)
        ),
    )
}

fn coefficient_tangente(rng: &mut Rng) -> Exercise {
    let a = rng.range(1, 6);
    let b = *rng.pick(&[-6i64, -5, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6]);
    let x0 = rng.range(-4, 4);
    let value = 2 * a * x0 + b;
    Exercise::new(
        format!(
            "Soit f(x) = {a}x² {}x + 3. Quel est le coefficient directeur de la tangente à (C_f) au point d'abscisse {x0} ?",
            signed_coef(b)
        ),
        Answer::Number {
            value,
            radix: 10,
        },
        format!(
            "Le coefficient directeur vaut f′({x0}). Or f′(x) = {}x {}, donc f′({x0}) = {value}. La tangente a pour équation y = f′(a)(x − a) + f(a).",
            2 * a,
            signed(b)
        ),
    )
}

fn integrale_affine(rng: &mut Rng) -> Exercise {
    // ∫₀^m (2a·x + b) dx = a·m² + b·m : toujours entier.
    let a = rng.range(1, 6);
    let b = *rng.pick(&[-5i64, -4, -3, -2, -1, 1, 2, 3, 4, 5, 6]);
    let m = rng.range(1, 6);
    let value = a * m * m + b * m;
    Exercise::new(
        format!(
            "Combien vaut ∫₀{} ({}x {}) dx ?",
            sup(m as u32),
            2 * a,
            signed(b)
        ),
        Answer::Number { value, radix: 10 },
        format!(
            "Une primitive est {a}x² {}x. Évaluée entre 0 et {m} : {a}×{m}² {} = {value}.",
            signed_coef(b),
            signed(b * m)
        ),
    )
}

fn primitive_reconnue(rng: &mut Rng) -> Exercise {
    const CASES: [(&str, &str, &str); 6] = [
        (
            "1/x",
            "ln|x|",
            "Le cas α = −1, qui échappe à la formule x^{α+1}/(α+1).",
        ),
        (
            "eˣ",
            "eˣ",
            "L'exponentielle est sa propre primitive, comme sa propre dérivée.",
        ),
        (
            "sin x",
            "− cos x",
            "Avec le signe moins. Inversement, la primitive de cos x est sin x, sans signe.",
        ),
        (
            "cos x",
            "sin x",
            "Sans signe. C'est la primitive de sin x qui porte le moins.",
        ),
        (
            "tan x",
            "− ln|cos x|",
            "tan = sin/cos est de la forme −u′/u avec u = cos.",
        ),
        (
            "1/(1 + x²)",
            "arctan x",
            "La brique qui sort des trinômes sans racine réelle.",
        ),
    ];
    let (f, right, why) = *rng.pick(&CASES);
    let wrong: Vec<String> = CASES
        .iter()
        .map(|(_, p, _)| p.to_string())
        .filter(|p| p != right)
        .collect();
    Exercise::new(
        format!("Quelle est une primitive de {f} ?"),
        super::shuffled_choice(rng, right.to_string(), wrong),
        why.to_string(),
    )
}

// ————————————————————————————————— mise en forme

/// Indice typographique d'un entier positif : `sub_n(12)` = `₁₂`.
fn sub_n(n: i64) -> String {
    super::sub(n.max(0) as u32)
}

/// « + 3 » ou « − 3 », avec le signe moins typographique.
fn signed(n: i64) -> String {
    if n < 0 {
        format!("− {}", -n)
    } else {
        format!("+ {n}")
    }
}

/// Coefficient signé collé à sa variable : « + 3 » ou « − 3 ».
fn signed_coef(n: i64) -> String {
    signed(n)
}
