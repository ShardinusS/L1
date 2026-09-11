//! Entraînement — Structures fondamentales : chapitres et questions fixes.
//! Les générateurs sont dans `sf_gen`.

use super::sf_gen as g;
use super::{card, num, qcm, set, txt, vf, Chapter, Fixed};

pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        id: "logique",
        title: "Logique et raisonnements",
        anchor: "sf-logique",
        fixed: LOGIQUE,
        generators: &[g::truth_value],
    },
    Chapter {
        id: "ensembles",
        title: "Ensembles",
        anchor: "sf-ensembles",
        fixed: ENSEMBLES,
        generators: &[g::set_ops, g::parts_count],
    },
    Chapter {
        id: "applications",
        title: "Relations et applications",
        anchor: "sf-applications",
        fixed: APPLICATIONS,
        generators: &[g::finite_map_kind, g::image_sets],
    },
    Chapter {
        id: "denombrement",
        title: "Cardinaux et dénombrement",
        anchor: "sf-denombrement",
        fixed: DENOMBREMENT,
        generators: &[g::counting],
    },
    Chapter {
        id: "nombres",
        title: "Ensembles de nombres",
        anchor: "sf-nombres",
        fixed: NOMBRES,
        generators: &[g::floor_part],
    },
    Chapter {
        id: "complexes",
        title: "Nombres complexes",
        anchor: "sf-complexes",
        fixed: COMPLEXES,
        generators: &[
            g::complex_product,
            g::complex_modulus,
            g::i_power,
            g::roots_of_unity,
        ],
    },
    Chapter {
        id: "arithmetique",
        title: "Arithmétique et ℤ/nℤ",
        anchor: "sf-algebre",
        fixed: ARITHMETIQUE,
        generators: &[
            g::euclid_division,
            g::gcd_lcm,
            g::mod_power,
            g::inverse_in_zn,
            g::affine_bijection,
            g::bezout_pair,
        ],
    },
    Chapter {
        id: "groupes",
        title: "Groupes, anneaux, corps",
        anchor: "sf-algebre",
        fixed: GROUPES,
        generators: &[g::order_in_zn],
    },
    Chapter {
        id: "permutations",
        title: "Permutations",
        anchor: "sf-algebre",
        fixed: PERMUTATIONS,
        generators: &[
            g::perm_cycles,
            g::perm_order_sig,
            g::perm_inverse,
            g::perm_power,
            g::order_exists,
        ],
    },
    Chapter {
        id: "redaction",
        title: "Méthodes et rédaction",
        anchor: "sf-outils",
        fixed: REDACTION,
        generators: &[],
    },
];

const LOGIQUE: &[Fixed] = &[
    vf(
        "« Les poules ont des dents » est une assertion.",
        true,
        "On peut lui attribuer une valeur de vérité (ici, faux) : c'est une assertion.",
    ),
    vf(
        "« Soit n un entier pair » est une assertion.",
        false,
        "Aucune valeur de vérité : c'est l'introduction d'une variable.",
    ),
    qcm(
        "P ⇒ Q est définie comme…",
        &["Q ∨ ¬P", "P ∧ Q", "¬P ∧ Q", "(P ∨ Q) ∧ ¬P"],
        "D'où : P ⇒ Q est vraie dès que P est fausse.",
    ),
    vf(
        "(1 = 2) ⇒ (6 > 7) est vraie.",
        true,
        "Prémisse fausse, donc implication vraie. ⇒ n'est pas « donc » : rien n'est affirmé sur 1 = 2 ni sur 6 > 7.",
    ),
    qcm(
        "La négation de P ⇒ Q est…",
        &["P ∧ ¬Q", "¬P ⇒ ¬Q", "¬Q ⇒ ¬P", "¬P ∧ Q"],
        "¬(P ⇒ Q) ⇔ ¬(Q ∨ ¬P) ⇔ P ∧ ¬Q.",
    ),
    qcm(
        "La contraposée de P ⇒ Q est…",
        &["¬Q ⇒ ¬P", "Q ⇒ P", "¬P ⇒ ¬Q", "P ⇒ ¬Q"],
        "Elle lui est équivalente. Q ⇒ P est la réciproque, qui ne l'est pas.",
    ),
    qcm(
        "¬(P ∧ Q) équivaut à…",
        &["¬P ∨ ¬Q", "¬P ∧ ¬Q", "P ∨ Q", "¬P ⇒ Q"],
        "Loi de De Morgan; de même ¬(P ∨ Q) ⇔ ¬P ∧ ¬Q.",
    ),
    qcm(
        "¬(∀x ∈ E, P(x)) équivaut à…",
        &["∃x ∈ E, ¬P(x)", "∀x ∈ E, ¬P(x)", "∃x ∈ E, P(x)", "∀x ∉ E, P(x)"],
        "On échange ∀ et ∃ et on nie la propriété.",
    ),
    vf(
        "Dans ℝ, ∃x, ∀y, x + y ≥ 0 est vraie.",
        false,
        "Pour tout x, y = −x − 1 donne x + y < 0. En revanche ∀x, ∃y, x + y ≥ 0 est vraie : l'ordre des quantificateurs change tout.",
    ),
    vf("Dans ℝ, ∀x, ∃y, x + y ≥ 0 est vraie.", true, "Pour x donné, y = −x convient."),
    vf(
        "Si E est vide, ∀x ∈ E, P(x) est vraie quelle que soit P.",
        true,
        "Il n'y a aucun x pour la contredire.",
    ),
    qcm(
        "Quel raisonnement pour montrer « n² pair ⇒ n pair » ?",
        &["par contraposée", "par récurrence", "par contre-exemple", "par double inclusion"],
        "On montre n impair ⇒ n² impair : n = 2k + 1 donne n² = 2(2k² + 2k) + 1.",
    ),
    qcm(
        "Pour réfuter ∀x, P(x), il suffit…",
        &[
            "d'exhiber un x₀ tel que P(x₀) est fausse",
            "de montrer ∀x, ¬P(x)",
            "d'une récurrence",
            "de montrer ∃x, P(x)",
        ],
        "Un seul contre-exemple suffit.",
    ),
    qcm(
        "∃x, (P(x) ∧ Q(x)) et (∃x, P(x)) ∧ (∃x, Q(x)) :",
        &[
            "la première implique la seconde, sans réciproque",
            "elles sont équivalentes",
            "la seconde implique la première, sans réciproque",
            "aucune n'implique l'autre",
        ],
        "Dans ℕ, il existe un pair et il existe un impair, mais aucun entier n'est les deux.",
    ),
    qcm(
        "Sur quelle propriété de ℕ repose la récurrence ?",
        &[
            "toute partie non vide de ℕ admet un plus petit élément",
            "ℕ est infini",
            "toute partie de ℕ admet un plus grand élément",
            "ℕ est dénombrable",
        ],
        "ℕ est bien ordonné. Rien de tel dans ℝ : ]0,1[ n'a pas de plus petit élément.",
    ),
    vf(
        "« 10ⁿ + 7 multiple de 9 ⇒ 10ⁿ⁺¹ + 7 multiple de 9 » est vraie, donc 10ⁿ + 7 est toujours multiple de 9.",
        false,
        "L'hérédité est vraie, mais sans initialisation elle ne prouve rien : 10ⁿ + 7 ≡ 8 [9] pour tout n.",
    ),
    card(
        "Rédaction type d'une récurrence",
        "Poser P(n). Initialisation : P(n₀) est vraie. Hérédité : soit n ≥ n₀, supposons P(n); montrons P(n+1). Conclusion : P(n) est vraie pour tout n ≥ n₀.",
    ),
];

const ENSEMBLES: &[Fixed] = &[
    vf(
        "{∅} = ∅",
        false,
        "{∅} a un élément (l'ensemble vide), ∅ n'en a aucun.",
    ),
    num(
        "Cardinal de {1, 3, 1, 2} ?",
        3,
        "Les répétitions ne comptent pas : {1, 3, 1, 2} = {1, 2, 3}.",
    ),
    qcm(
        "Laquelle de ces écritures est correcte ?",
        &["{1} ⊂ [0,2]", "1 ⊂ [0,2]", "{1} ∈ [0,2]", "[0,2] ∈ {1}"],
        "∈ relie un élément à un ensemble, ⊂ relie deux ensembles : 1 ∈ [0,2] et {1} ⊂ [0,2].",
    ),
    qcm(
        "E = {a, b, c}. Laquelle n'a pas de sens ?",
        &["a ⊂ E", "a ∈ E", "{a} ⊂ E", "∅ ⊂ E"],
        "a est un élément, pas un ensemble : on écrit a ∈ E ou {a} ⊂ E.",
    ),
    num(
        "Combien d'éléments dans P({a, b}) ?",
        4,
        "∅, {a}, {b}, {a, b} : 2² parties.",
    ),
    num(
        "Combien d'éléments dans P(∅) ?",
        1,
        "P(∅) = {∅} : une seule partie, l'ensemble vide.",
    ),
    vf(
        "A ∩ B = A ∩ C entraîne B = C.",
        false,
        "Contre-exemple : A = {1}, B = ∅, C = {2}. Il faut en plus A ∪ B = A ∪ C.",
    ),
    vf(
        "Si A ∩ B = A ∪ B, alors A = B.",
        true,
        "Soit x ∈ A : x ∈ A ∪ B = A ∩ B, donc x ∈ B. D'où A ⊂ B, et B ⊂ A de même.",
    ),
    qcm(
        "Pour prouver A = B, on montre en pratique…",
        &[
            "A ⊂ B et B ⊂ A",
            "A ∈ B et B ∈ A",
            "card A = card B",
            "A ∩ B ≠ ∅",
        ],
        "La double inclusion, élément par élément.",
    ),
    vf(
        "L'inclusion est un ordre total sur P(E).",
        false,
        "Elle n'est pas totale : {0,1} et {1,2} ne sont pas comparables.",
    ),
    qcm(
        "A \\ B est défini par…",
        &[
            "{ x ∈ A | x ∉ B }",
            "{ x ∈ B | x ∉ A }",
            "A ∩ B",
            "(A ∪ B) \\ (A ∩ B)",
        ],
        "B n'a pas besoin d'être inclus dans A.",
    ),
    set(
        "A = {1, 2}, B = {2, 3}. Que vaut A ∩ (A ∪ B) ?",
        &[1, 2],
        "A ∩ (A ∪ B) = A, toujours : tout élément de A est dans A ∪ B.",
    ),
];

const APPLICATIONS: &[Fixed] = &[
    qcm(
        "f est injective si tout y de F admet…",
        &["au plus un antécédent", "au moins un antécédent", "exactement un antécédent", "aucun antécédent"],
        "Caractérisation utile : f(x) = f(x′) ⇒ x = x′.",
    ),
    qcm(
        "f est surjective si et seulement si…",
        &["Im f = F", "f(x) = f(x′) ⇒ x = x′", "Dom f = E", "f⁻¹(F) = E"],
        "Tout y de F a au moins un antécédent.",
    ),
    qcm(
        "Si g ∘ f est injective, alors…",
        &["f est injective", "g est injective", "f et g sont injectives", "g est surjective"],
        "L'injectivité remonte vers la première appliquée.",
    ),
    qcm(
        "Si g ∘ f est surjective, alors…",
        &["g est surjective", "f est surjective", "f et g sont surjectives", "f est injective"],
        "La surjectivité remonte vers la dernière appliquée.",
    ),
    qcm(
        "En général, f(A₁ ∩ A₂) et f(A₁) ∩ f(A₂) vérifient…",
        &[
            "f(A₁ ∩ A₂) ⊂ f(A₁) ∩ f(A₂)",
            "f(A₁ ∩ A₂) = f(A₁) ∩ f(A₂)",
            "f(A₁) ∩ f(A₂) ⊂ f(A₁ ∩ A₂)",
            "aucune inclusion générale",
        ],
        "Égalité si f est injective. L'image directe ne respecte que la réunion.",
    ),
    qcm(
        "A = f⁻¹(f(A)) pour toute partie A si et seulement si…",
        &["f est injective", "f est surjective", "f est constante", "toujours"],
        "En général seulement A ⊂ f⁻¹(f(A)).",
    ),
    qcm(
        "f(f⁻¹(B)) = B pour toute partie B si et seulement si…",
        &["f est surjective", "f est injective", "toujours", "jamais"],
        "En général seulement f(f⁻¹(B)) ⊂ B.",
    ),
    qcm(
        "x ↦ x² de ℝ dans ℝ est…",
        &["ni injective ni surjective", "injective", "surjective", "bijective"],
        "f(−1) = f(1), et −1 n'a pas d'antécédent.",
    ),
    qcm(
        "x ↦ x² de ℝ⁺ dans ℝ⁺ est…",
        &["bijective", "injective seulement", "surjective seulement", "ni injective ni surjective"],
        "Réciproque y ↦ √y. Même formule, autres ensembles : autre application.",
    ),
    qcm(
        "x ↦ eˣ de ℝ dans ℝ est…",
        &["injective non surjective", "bijective", "surjective non injective", "ni injective ni surjective"],
        "Strictement croissante, mais −1 n'a pas d'antécédent.",
    ),
    qcm(
        "Si f et g sont bijectives, (g ∘ f)⁻¹ =",
        &["f⁻¹ ∘ g⁻¹", "g⁻¹ ∘ f⁻¹", "g ∘ f", "f ∘ g"],
        "On défait d'abord la dernière appliquée.",
    ),
    qcm(
        "Une relation réflexive, symétrique et transitive est…",
        &["une relation d'équivalence", "une relation d'ordre", "un ordre total", "un ordre strict"],
        "Exemple : la congruence modulo n.",
    ),
    qcm(
        "Une relation réflexive, antisymétrique et transitive est…",
        &["une relation d'ordre", "une relation d'équivalence", "une fonction", "une bijection"],
        "Exemples : ≤ sur ℝ (total), ⊂ sur P(E) (non total).",
    ),
    txt(
        "f(x, y) = (x + 3y, x + y) est bijective de ℝ² dans ℝ². Que vaut f⁻¹(4, 2) ?",
        &["(1, 1)"],
        "f⁻¹(a, b) = ((3b − a)/2, (a − b)/2) = ((6 − 4)/2, (4 − 2)/2) = (1, 1). Vérification : f(1, 1) = (4, 2).",
    ),
    card(
        "Montrer qu'une application est bijective — deux voies",
        "Voie 1 : injectivité (f(x) = f(x′) ⇒ x = x′) puis surjectivité (on exhibe un antécédent). Voie 2 : on devine g et on vérifie g ∘ f = Id et f ∘ g = Id. Cas fini avec card E = card F : une seule des deux propriétés suffit.",
    ),
];

const DENOMBREMENT: &[Fixed] = &[
    qcm(
        "Théorème de Cantor : pour tout ensemble E…",
        &[
            "il n'existe pas de surjection de E sur P(E)",
            "il n'existe pas d'injection de E dans P(E)",
            "E et P(E) sont équipotents",
            "P(E) est fini",
        ],
        "x ↦ {x} est une injection. Pour une surjection f, P = { x | x ∉ f(x) } = f(a) donnerait a ∈ P ⇔ a ∉ P.",
    ),
    vf("ℚ est dénombrable.", true, "ℤ, ℕ² et ℚ sont dénombrables."),
    vf("ℝ est dénombrable.", false, "Argument de la diagonale : card ℕ < card ℝ."),
    qcm(
        "E fini et f : E → F injective. Alors…",
        &["card E ≤ card F", "card E ≥ card F", "card E = card F", "rien de général"],
        "Surjective donnerait card E ≥ card F.",
    ),
    qcm(
        "C(n, k) =",
        &["C(n, n − k)", "C(n − 1, k)", "C(k, n)", "n! / k!"],
        "Choisir les k éléments gardés, c'est choisir les n − k laissés.",
    ),
    qcm(
        "Formule de Pascal :",
        &[
            "C(n,k) = C(n−1,k−1) + C(n−1,k)",
            "C(n,k) = C(n−1,k) × C(n−1,k−1)",
            "C(n,k) = C(n,k−1) + 1",
            "C(n,k) = n · C(n−1,k)",
        ],
        "On sépare les parties qui contiennent un élément fixé de celles qui ne le contiennent pas.",
    ),
    num("Que vaut 0! ?", 1, "Convention : 0! = 1! = 1."),
    qcm(
        "card(E ∪ F) =",
        &[
            "card E + card F − card(E ∩ F)",
            "card E + card F",
            "card E × card F",
            "card E + card F + card(E ∩ F)",
        ],
        "Sans retrancher l'intersection, on la compterait deux fois.",
    ),
];

const NOMBRES: &[Fixed] = &[
    num(
        "E(−π) = ⌊−π⌋ vaut :",
        -4,
        "L'unique n ∈ ℤ avec n ≤ x < n + 1 : −4 ≤ −π < −3.",
    ),
    qcm(
        "Propriété d'Archimède :",
        &[
            "∀ε > 0, ∀y ∈ ℝ, ∃n ∈ ℕ, y ≤ nε",
            "∀x ∈ ℝ, ∃n ∈ ℕ, x = n",
            "toute partie non vide de ℝ a un plus petit élément",
            "ℝ est dénombrable",
        ],
        "Aussi petit soit ε, un multiple de ε dépasse y.",
    ),
    vf(
        "Tout intervalle ouvert non vide de ℝ contient un rationnel et un irrationnel.",
        true,
        "Densité, avec rₙ = E(nx)/n qui vérifie 0 ≤ x − rₙ < 1/n.",
    ),
    qcm(
        "√a ∈ ℚ (a > 0 rationnel) si et seulement si…",
        &[
            "toutes les valuations val_p(a) sont paires",
            "a est premier",
            "a est pair",
            "a est entier",
        ],
        "D'où √2 ∉ ℚ : val₂(2) = 1 est impair.",
    ),
    vf(
        "]0, 1[ admet un plus petit élément.",
        false,
        "ℝ n'est pas bien ordonné : ]0,1[ est non vide et borné, sans plus petit élément.",
    ),
];

const COMPLEXES: &[Fixed] = &[
    txt(
        "Forme algébrique de (2 + i)/(1 − i) ?",
        &["(1 + 3i)/2", "1/2 + 3i/2", "1/2 + (3/2)i", "0,5 + 1,5i"],
        "On multiplie par le conjugué du dénominateur : (2 + i)(1 + i) / ((1 − i)(1 + i)) = (1 + 3i)/2.",
    ),
    qcm(
        "z z̄ =",
        &["x² + y², un réel positif", "x² − y²", "2x", "2iy"],
        "z z̄ = |z|². C'est ce qui permet d'écrire z⁻¹ = z̄ / |z|².",
    ),
    qcm(
        "arg(z z′) =",
        &["arg z + arg z′ [2π]", "arg z × arg z′", "arg z − arg z′", "|z| · arg z′"],
        "Et arg(z/z′) = arg z − arg z′ [2π].",
    ),
    qcm(
        "Formule de De Moivre :",
        &[
            "(cos θ + i sin θ)ⁿ = cos nθ + i sin nθ",
            "(cos θ + i sin θ)ⁿ = cosⁿθ + i sinⁿθ",
            "e^{iθ} = cos θ − i sin θ",
            "cos nθ = n cos θ",
        ],
        "Conséquence de (e^{iθ})ⁿ = e^{inθ}.",
    ),
    qcm("cos(π/3) =", &["1/2", "√3/2", "√2/2", "0"], "cos(π/3) = 1/2 et sin(π/3) = √3/2."),
    qcm("sin(π/6) =", &["1/2", "√3/2", "√2/2", "1"], "sin(π/6) = 1/2 et cos(π/6) = √3/2."),
    qcm("cos(π/4) =", &["√2/2", "1/2", "√3/2", "1"], "cos(π/4) = sin(π/4) = √2/2."),
    qcm(
        "Forme exponentielle de 1 + i√3 :",
        &["2e^{iπ/3}", "2e^{iπ/6}", "√3 e^{iπ/3}", "e^{iπ/3}"],
        "|z| = √(1 + 3) = 2, cos θ = 1/2 et sin θ = √3/2 : θ = π/3.",
    ),
    vf(
        "Un polynôme de degré n ≥ 1 à coefficients complexes a exactement n racines dans ℂ, comptées avec multiplicité.",
        true,
        "Corollaire du théorème de d'Alembert-Gauss.",
    ),
    qcm(
        "Pour résoudre az² + bz + c = 0 dans ℂ avec Δ complexe, on cherche…",
        &["δ tel que δ² = Δ", "√Δ au sens réel", "|Δ|", "le conjugué de Δ"],
        "Si Δ = re^{iθ}, δ = √r e^{iθ/2} convient; z = (−b ± δ)/2a.",
    ),
    qcm(
        "Les racines 4-ièmes de l'unité sont…",
        &["1, i, −1, −i", "1, −1", "1, i", "1, e^{iπ/4}, i, e^{3iπ/4}"],
        "ω_k = e^{2ikπ/4} = i^k : les sommets d'un carré inscrit dans le cercle unité.",
    ),
    card(
        "Linéariser cosᵖx · sinᵍx",
        "Remplacer cos x = (e^{ix} + e^{−ix})/2 et sin x = (e^{ix} − e^{−ix})/2i; développer (binôme); regrouper avec e^{inx} + e^{−inx} = 2 cos nx et e^{inx} − e^{−inx} = 2i sin nx. Exemple : cos²x = cos(2x)/2 + 1/2.",
    ),
];

const ARITHMETIQUE: &[Fixed] = &[
    txt(
        "Division euclidienne de −17 par 5 : le couple (q, r) ?",
        &["(-4, 3)", "q = -4, r = 3"],
        "−17 = 5 × (−4) + 3 avec 0 ≤ 3 < 5. (−3, −2) ne convient pas : le reste doit être positif.",
    ),
    set(
        "Éléments inversibles de ℤ/12ℤ (représentants de 0 à 11) ?",
        &[1, 5, 7, 11],
        "ā est inversible si et seulement si a ∧ 12 = 1.",
    ),
    qcm(
        "ℤ/nℤ est un corps si et seulement si…",
        &["n est premier", "n est pair", "n > 1", "n est un carré"],
        "Tout ā non nul est alors premier avec n, donc inversible.",
    ),
    qcm(
        "ā est inversible dans ℤ/nℤ si et seulement si…",
        &["a ∧ n = 1", "a divise n", "n divise a", "a < n"],
        "Bezout : ua + vn = 1 donne ū inverse de ā.",
    ),
    qcm(
        "x ↦ ax + b est une bijection de ℤ/nℤ si et seulement si…",
        &["a ∧ n = 1", "b ∧ n = 1", "a ≠ 0", "n est premier"],
        "Tombé à l'examen 2024 et au rattrapage : f(x) = 34x + 91 sur ℤ/101ℤ est bijective, 34 ∧ 101 = 1.",
    ),
    qcm(
        "Identité de Bezout :",
        &[
            "∃u, v ∈ ℤ, ua + vb = a ∧ b",
            "∃u, v ∈ ℕ, ua + vb = a ∨ b",
            "a ∧ b = a + b",
            "ua + vb = 1 pour tous a, b",
        ],
        "Les coefficients se trouvent en remontant l'algorithme d'Euclide; ils ne sont pas uniques.",
    ),
    num(
        "Reste de 4007¹²³⁵ par 13 ?",
        9,
        "4007 ≡ 3 [13], 3³ = 27 ≡ 1, 1235 = 3 × 411 + 2, donc 4007¹²³⁵ ≡ 3² = 9. (Examen 2024.)",
    ),
    qcm(
        "Pour a, b > 0 : ab =",
        &["(a ∧ b)(a ∨ b)", "(a ∧ b) + (a ∨ b)", "(a ∨ b)²", "a ∧ b"],
        "PGCD × PPCM = produit. En valuations : min + max = somme.",
    ),
    qcm(
        "Restes chinois : un système de congruences à modules deux à deux premiers entre eux…",
        &[
            "a une solution, unique modulo le produit des modules",
            "n'a jamais de solution",
            "a une solution seulement si les modules sont premiers",
            "a exactement une solution entière",
        ],
        "n ↦ (n mod a₁, …, n mod a_N) est une bijection de {0, …, a − 1} sur le produit des ℤ/aᵢℤ.",
    ),
    vf(
        "L'ensemble des nombres premiers est fini.",
        false,
        "Tout facteur premier de p₁p₂⋯pₙ + 1 échappe à la liste des n premiers.",
    ),
];

const GROUPES: &[Fixed] = &[
    qcm(
        "Un groupe (G, ⋆) exige…",
        &[
            "une loi associative, un neutre, un symétrique pour chaque élément",
            "une loi commutative et un neutre",
            "une loi associative et commutative",
            "un neutre et un élément absorbant",
        ],
        "La commutativité n'est pas requise : elle fait du groupe un groupe abélien.",
    ),
    vf("(ℝ, ·) est un groupe.", false, "0 n'a pas d'inverse. (ℝ*, ·) en est un."),
    vf("(ℤ, +, ·) est un corps.", false, "Inv ℤ = {−1, 1} : 2 n'est pas inversible."),
    qcm("Dans un groupe, (xy)⁻¹ =", &["y⁻¹x⁻¹", "x⁻¹y⁻¹", "xy", "yx"], "On défait dans l'ordre inverse."),
    qcm(
        "Théorème de Lagrange : si H est un sous-groupe du groupe fini G…",
        &["card H divise card G", "card H = card G / 2", "H est commutatif", "card G divise card H"],
        "Démonstration guidée au TD 4 par les classes aH.",
    ),
    qcm(
        "Pour montrer que A ⊂ G est un sous-groupe, on vérifie…",
        &[
            "stabilité, présence du neutre, présence des symétriques",
            "associativité, neutre, symétriques",
            "commutativité",
            "que card A divise card G",
        ],
        "L'associativité est héritée de G : inutile de la revérifier.",
    ),
    qcm(
        "Si φ : G → G′ est un morphisme de groupes, φ(e) =",
        &["e′, le neutre de G′", "e", "φ⁻¹(e′)", "ker φ"],
        "Et φ(g⁻¹) = φ(g)⁻¹.",
    ),
    qcm(
        "Le groupe de Klein K = {e, u, v, w} est isomorphe à…",
        &["ℤ/2ℤ × ℤ/2ℤ", "ℤ/4ℤ", "S₃", "ℤ/3ℤ"],
        "Chaque élément est son propre inverse; ℤ/4ℤ, lui, a un élément d'ordre 4. Lire la table imposée par l'énoncé !",
    ),
    vf("Une loi admet au plus un élément neutre.", true, "Si e et e′ sont neutres, e = e ⋆ e′ = e′."),
    vf(
        "(Perm E, ∘) est commutatif dès que card E ≥ 3.",
        false,
        "C'est l'inverse : il est non commutatif dès n ≥ 3, par exemple (1,2) ∘ (2,3) ≠ (2,3) ∘ (1,2).",
    ),
];

const PERMUTATIONS: &[Fixed] = &[
    num("card S₅ ?", 120, "card Sₙ = n! = 5! = 120."),
    qcm(
        "Un cycle de longueur P se décompose en…",
        &["P − 1 transpositions", "P transpositions", "P/2 transpositions", "P + 1 transpositions"],
        "(a₀, …, a_{P−1}) = (a₀, a₁) ∘ … ∘ (a_{P−2}, a_{P−1}).",
    ),
    qcm(
        "L'ordre d'une permutation est…",
        &[
            "le PPCM des longueurs de ses cycles disjoints",
            "la somme des longueurs de ses cycles",
            "le produit des longueurs de ses cycles",
            "le nombre de ses cycles",
        ],
        "Chaque cycle revient au départ après sa longueur; tous ensemble au PPCM.",
    ),
    vf("σ² est toujours une permutation paire.", true, "ε(σ²) = ε(σ)² = 1."),
    qcm("ε(transposition) =", &["−1", "1", "0", "2"], "Et ε(σ ∘ τ) = ε(σ) ε(τ)."),
    vf("Deux cycles à supports disjoints commutent.", true, "Ils agissent sur des éléments différents."),
    vf(
        "Il existe une permutation d'ordre 26 sur 13 éléments.",
        false,
        "Il faudrait un 2-cycle et un 13-cycle disjoints, soit 15 éléments.",
    ),
    vf(
        "Il existe une permutation d'ordre 30 sur 13 éléments.",
        true,
        "(0,1,2,3,4)(5,6,7,8,9,10) : 5 + 6 = 11 éléments, ppcm(5, 6) = 30.",
    ),
    card(
        "La question type sur une permutation",
        "Cycles : partir de 0, suivre σ jusqu'au retour, reprendre au plus petit élément non traité. Transpositions : P − 1 par cycle. Ordre : PPCM des longueurs. Signature : (−1)^(nombre de transpositions). σ⁻¹ : retourner chaque cycle. σᴺ : réduire N modulo l'ordre.",
    ),
];

const REDACTION: &[Fixed] = &[
    card(
        "Limite par la définition : uₙ = (4n² + 1)/(2n² + 2) → 2",
        "Soit ε > 0. Prenons n₀ ∈ ℕ avec n₀ ≥ √(3/ε), et soit n ≥ n₀. |uₙ − 2| = 3/(2n² + 2) ≤ 3/(2n₀² + 2) ≤ 3/(2·3/ε + 2) < ε. C'est l'exhibition de n₀ qui est notée.",
    ),
    card("Pour prouver ∀x ∈ E, P(x), on commence par…", "« Soit x ∈ E. » puis on prouve P(x) pour ce x fixé."),
    card("Pour prouver ∃x ∈ E, P(x)…", "« Vérifions que x = … convient. » Le brouillon cherche, la copie vérifie."),
    card("Pour prouver A = B (ensembles)…", "« Soit x ∈ A. » … donc x ∈ B : A ⊂ B. Puis l'inclusion inverse."),
    card(
        "Fonction indicatrice : les formules",
        "1_∅ = 0, 1_E = 1, 1_{Aᶜ} = 1 − 1_A, 1_{A∩B} = 1_A · 1_B, 1_{A∪B} = 1_A + 1_B − 1_A · 1_B. Preuve : distinguer les quatre cas d'appartenance.",
    ),
    card(
        "Reste de aᴺ modulo m — méthode",
        "Réduire la base : a ≡ a₀ [m]. Chercher le plus petit k avec a₀ᵏ ≡ ±1 [m]. Écrire N = kq + r et conclure aᴺ ≡ (±1)^q · a₀ʳ.",
    ),
    card(
        "x ↦ ax + b bijective dans ℤ/nℤ — rédaction",
        "Si a ∧ n = 1 : injective car n | a(x − y) et a premier avec n donnent n | x − y (Gauss); surjective en exhibant x = a⁻¹(y − b). Sinon, deux éléments distincts de même image suffisent.",
    ),
    card(
        "Ce qui coûte des points sans erreur de maths",
        "Une réponse sans phrase, un résultat sans justification, des quantificateurs comme abréviations, une copie brouillon. Au rattrapage : 1,5 point sans justification, 3 points bien rédigé.",
    ),
];
