//! Générateurs de Structures fondamentales : chaque appel tire de nouvelles
//! valeurs, avec le corrigé rédigé comme dans le cours.

use super::answer::{format_set, minus};
use super::math;
use super::{shuffled_choice, sup, true_false, yes_no, Answer, Exercise, Rng};

// ---------------------------------------------------------------- logique

pub fn truth_value(rng: &mut Rng) -> Exercise {
    type Rule = fn(bool, bool) -> bool;
    let forms: [(&str, Rule, &str); 11] = [
        ("¬P", |p, _| !p, "¬P a la valeur contraire de P."),
        (
            "P ∧ Q",
            |p, q| p && q,
            "P ∧ Q n'est vraie que si P et Q le sont.",
        ),
        (
            "P ∨ Q",
            |p, q| p || q,
            "Le « ou » est inclusif : P ∨ Q n'est fausse que si P et Q le sont.",
        ),
        (
            "P ⇒ Q",
            |p, q| !p || q,
            "P ⇒ Q ≡ Q ∨ ¬P : fausse seulement si P est vraie et Q fausse.",
        ),
        (
            "Q ⇒ P",
            |p, q| p || !q,
            "Q ⇒ P ≡ P ∨ ¬Q : fausse seulement si Q est vraie et P fausse.",
        ),
        (
            "P ⇔ Q",
            |p, q| p == q,
            "P ⇔ Q est vraie quand P et Q ont la même valeur.",
        ),
        ("¬P ⇒ Q", |p, q| p || q, "¬P ⇒ Q ≡ Q ∨ P."),
        (
            "¬(P ∧ Q)",
            |p, q| !(p && q),
            "De Morgan : ¬(P ∧ Q) ≡ ¬P ∨ ¬Q.",
        ),
        (
            "¬(P ∨ Q)",
            |p, q| !(p || q),
            "De Morgan : ¬(P ∨ Q) ≡ ¬P ∧ ¬Q.",
        ),
        (
            "¬Q ⇒ ¬P",
            |p, q| !p || q,
            "C'est la contraposée de P ⇒ Q : même valeur.",
        ),
        ("P ∧ ¬Q", |p, q| p && !q, "C'est la négation de P ⇒ Q."),
    ];
    let (p, q) = (rng.coin(), rng.coin());
    let (formula, rule, why) = *rng.pick(&forms);
    let value = rule(p, q);
    let word = |b: bool| if b { "vraie" } else { "fausse" };
    Exercise::new(
        format!(
            "P est {} et Q est {}. Que vaut {formula} ?",
            word(p),
            word(q)
        ),
        true_false(value),
        format!("{why} Ici : {}.", if value { "vraie" } else { "fausse" }),
    )
}

// ---------------------------------------------------------------- ensembles

fn subset(rng: &mut Rng, universe: usize, lo: usize, hi: usize) -> Vec<i64> {
    let k = rng.range(lo as i64, hi as i64) as usize;
    let mut s: Vec<i64> = rng
        .sample(universe, k)
        .into_iter()
        .map(|x| x as i64)
        .collect();
    s.sort_unstable();
    s
}

pub fn set_ops(rng: &mut Rng) -> Exercise {
    let a = subset(rng, 10, 3, 5);
    let b = subset(rng, 10, 3, 5);
    let inter: Vec<i64> = a.iter().copied().filter(|x| b.contains(x)).collect();
    let a_minus_b: Vec<i64> = a.iter().copied().filter(|x| !b.contains(x)).collect();
    let b_minus_a: Vec<i64> = b.iter().copied().filter(|x| !a.contains(x)).collect();
    let (op, result, def) = match rng.below(5) {
        0 => ("A ∩ B", inter, "A ∩ B = { x | x ∈ A et x ∈ B }"),
        1 => (
            "A ∪ B",
            [a.clone(), b.clone()].concat(),
            "A ∪ B = { x | x ∈ A ou x ∈ B }",
        ),
        2 => ("A \\ B", a_minus_b, "A \\ B = { x ∈ A | x ∉ B }"),
        3 => ("B \\ A", b_minus_a, "B \\ A = { x ∈ B | x ∉ A }"),
        _ => (
            "A Δ B",
            [a_minus_b, b_minus_a].concat(),
            "A Δ B = (A \\ B) ∪ (B \\ A) : les éléments d'un seul des deux",
        ),
    };
    Exercise::new(
        format!(
            "A = {}, B = {}. Que vaut {op} ?",
            format_set(&a),
            format_set(&b)
        ),
        Answer::Set {
            elems: result.clone(),
        },
        format!("{def}. Résultat : {}.", format_set(&result)),
    )
}

pub fn parts_count(rng: &mut Rng) -> Exercise {
    let n = rng.range(2, 6);
    if rng.coin() {
        Exercise::new(
            format!("E a {n} éléments. Combien d'éléments dans P(E) ?"),
            Answer::Number {
                value: 1 << n,
                radix: 10,
            },
            format!("card P(E) = 2ⁿ = 2{} = {}.", sup(n as u32), 1 << n),
        )
    } else {
        let m = rng.range(2, 6);
        Exercise::new(
            format!("card E = {n} et card F = {m}. card(E × F) ?"),
            Answer::Number {
                value: n * m,
                radix: 10,
            },
            format!("card(E × F) = card E × card F = {n} × {m} = {}.", n * m),
        )
    }
}

// ---------------------------------------------------------------- applications

fn map_table(values: &[usize]) -> String {
    let xs: Vec<String> = (1..=values.len()).map(|x| format!("{x:>2}")).collect();
    let ys: Vec<String> = values.iter().map(|y| format!("{y:>2}")).collect();
    format!("x    │ {}\nf(x) │ {}", xs.join(" "), ys.join(" "))
}

fn set_of(values: impl IntoIterator<Item = usize>) -> Vec<i64> {
    let mut v: Vec<i64> = values.into_iter().map(|x| x as i64).collect();
    v.sort_unstable();
    v.dedup();
    v
}

pub fn finite_map_kind(rng: &mut Rng) -> Exercise {
    const KINDS: [&str; 4] = [
        "bijective",
        "injective, non surjective",
        "surjective, non injective",
        "ni injective ni surjective",
    ];
    let n = rng.range(3, 5) as usize;
    let kind = rng.below(4);
    let (m, values): (usize, Vec<usize>) = match kind {
        0 => (n, rng.sample(n, n).into_iter().map(|x| x + 1).collect()),
        1 => (
            n + 1,
            rng.sample(n + 1, n).into_iter().map(|x| x + 1).collect(),
        ),
        2 => {
            let m = n - 1;
            let mut v: Vec<usize> = (1..=m).collect();
            v.push(rng.range(1, m as i64) as usize);
            rng.shuffle(&mut v);
            (m, v)
        }
        _ => {
            let mut v: Vec<usize> = rng.sample(n, n).into_iter().map(|x| x + 1).collect();
            let (i, j) = (rng.below(n), rng.below(n - 1));
            let j = if j >= i { j + 1 } else { j };
            v[j] = v[i];
            (n, v)
        }
    };
    let injective = set_of(values.iter().copied()).len() == values.len();
    let missing: Vec<i64> = (1..=m as i64)
        .filter(|y| !values.contains(&(*y as usize)))
        .collect();
    let why_inj = if injective {
        "les images sont deux à deux distinctes : injective".to_string()
    } else {
        let (i, j) = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .find(|(i, j)| values[*i] == values[*j])
            .unwrap_or((0, 0));
        format!(
            "f({}) = f({}) = {} : pas injective",
            i + 1,
            j + 1,
            values[i]
        )
    };
    let why_surj = if missing.is_empty() {
        "tout élément d'arrivée est atteint : surjective".to_string()
    } else {
        format!("{} n'a pas d'antécédent : pas surjective", missing[0])
    };
    let wrong: Vec<String> = KINDS
        .iter()
        .enumerate()
        .filter(|(k, _)| *k != kind)
        .map(|(_, s)| s.to_string())
        .collect();
    Exercise::new(
        format!("f : {{1, …, {n}}} → {{1, …, {m}}} est donnée par son tableau. Elle est…"),
        shuffled_choice(rng, KINDS[kind].to_string(), wrong),
        format!("{why_inj}; {why_surj}."),
    )
    .with_code(map_table(&values))
}

pub fn image_sets(rng: &mut Rng) -> Exercise {
    let values: Vec<usize> = (0..6).map(|_| rng.range(1, 5) as usize).collect();
    let code = map_table(&values);
    if rng.coin() {
        let k = rng.range(2, 4) as usize;
        let a = set_of(rng.sample(6, k).into_iter().map(|x| x + 1));
        let image = set_of(a.iter().map(|x| values[*x as usize - 1]));
        Exercise::new(
            format!(
                "f : {{1, …, 6}} → {{1, …, 5}}. Que vaut f(A) pour A = {} ?",
                format_set(&a)
            ),
            Answer::Set {
                elems: image.clone(),
            },
            format!("f(A) = {{ f(a) | a ∈ A }} = {}.", format_set(&image)),
        )
        .with_code(code)
    } else {
        let k = rng.range(1, 3) as usize;
        let b = set_of(rng.sample(5, k).into_iter().map(|x| x + 1));
        let pre = set_of((1..=6).filter(|x| b.contains(&(values[x - 1] as i64))));
        Exercise::new(
            format!("f : {{1, …, 6}} → {{1, …, 5}}. Que vaut f⁻¹(B) pour B = {} ?", format_set(&b)),
            Answer::Set { elems: pre.clone() },
            format!(
                "f⁻¹(B) = {{ x | f(x) ∈ B }} = {}. L'image réciproque existe même si f n'est pas bijective, et peut être vide.",
                format_set(&pre)
            ),
        )
        .with_code(code)
    }
}

// ---------------------------------------------------------------- dénombrement

pub fn counting(rng: &mut Rng) -> Exercise {
    let (prompt, value, why): (String, u64, String) = match rng.below(7) {
        0 => {
            let (n, p) = (rng.range(5, 30) as u64, rng.range(2, 4) as u64);
            (
                format!("De combien de façons choisir {p} délégués parmi {n} étudiants ?"),
                math::binom(n, p),
                format!(
                    "L'ordre ne compte pas : C({n}, {p}) = {n}! / ({p}! {}!) = {}.",
                    n - p,
                    math::binom(n, p)
                ),
            )
        }
        1 => {
            let (n, p) = (rng.range(5, 15) as u64, rng.range(2, 3) as u64);
            (
                format!("Un podium de {p} places parmi {n} coureurs : combien de classements possibles ?"),
                math::arrangements(n, p),
                format!("L'ordre compte, sans répétition : A({n}, {p}) = {n}! / {}! = {}.", n - p, math::arrangements(n, p)),
            )
        }
        2 => {
            let n = rng.range(3, 8) as u64;
            (
                format!("De combien de façons ranger {n} livres différents sur une étagère ?"),
                math::factorial(n),
                format!(
                    "Permutations de {n} éléments : {n}! = {}.",
                    math::factorial(n)
                ),
            )
        }
        3 => {
            let (k, p) = (*rng.pick(&[2u64, 3, 10, 26]), rng.range(2, 4) as u32);
            (
                format!("Combien de mots de {p} caractères sur un alphabet de {k} symboles (répétitions permises) ?"),
                k.pow(p),
                format!("p-uplets : {k}{} = {}.", sup(p), k.pow(p)),
            )
        }
        4 => {
            let (n, m) = (rng.range(2, 4) as u32, rng.range(2, 5) as u64);
            (
                format!("Combien d'applications d'un ensemble à {n} éléments dans un ensemble à {m} éléments ?"),
                m.pow(n),
                format!("Chacun des {n} éléments de départ choisit son image parmi {m} : {m}{} = {}.", sup(n), m.pow(n)),
            )
        }
        5 => {
            let (a, b) = (rng.range(5, 30) as u64, rng.range(5, 30) as u64);
            let c = rng.range(0, a.min(b) as i64) as u64;
            (
                format!("card E = {a}, card F = {b}, card(E ∩ F) = {c}. card(E ∪ F) ?"),
                a + b - c,
                format!(
                    "card(E ∪ F) = card E + card F − card(E ∩ F) = {a} + {b} − {c} = {}.",
                    a + b - c
                ),
            )
        }
        _ => {
            let (n, p) = (rng.range(4, 12) as u64, rng.range(2, 4) as u64);
            (
                format!("Combien de parties à {p} éléments dans un ensemble à {n} éléments ?"),
                math::binom(n, p),
                format!("C({n}, {p}) = {}.", math::binom(n, p)),
            )
        }
    };
    Exercise::new(
        prompt,
        Answer::Number {
            value: value as i64,
            radix: 10,
        },
        why,
    )
}

// ---------------------------------------------------------------- nombres

pub fn floor_part(rng: &mut Rng) -> Exercise {
    let mut tenths = rng.range(1, 250);
    if tenths % 10 == 0 {
        tenths += 3;
    }
    if rng.coin() {
        tenths = -tenths;
    }
    let n = tenths.div_euclid(10);
    let sign = if tenths < 0 { "−" } else { "" };
    let shown = format!("{sign}{},{}", tenths.abs() / 10, tenths.abs() % 10);
    Exercise::new(
        format!("Partie entière E({shown}) ?"),
        Answer::Number {
            value: n,
            radix: 10,
        },
        format!(
            "E(x) est l'unique entier n avec n ≤ x < n + 1 : {} ≤ {shown} < {}.{}",
            minus(n),
            minus(n + 1),
            if tenths < 0 {
                " Pour un négatif, on descend : ce n'est pas « enlever la virgule »."
            } else {
                ""
            }
        ),
    )
}

// ---------------------------------------------------------------- complexes

/// Écriture algébrique `a + bi` avec les signes typographiques.
fn complex(a: i64, b: i64) -> String {
    let im = match b.abs() {
        0 => return minus(a),
        1 => "i".to_string(),
        n => format!("{n}i"),
    };
    if a == 0 {
        if b < 0 {
            format!("−{im}")
        } else {
            im
        }
    } else {
        format!("{} {} {im}", minus(a), if b < 0 { "−" } else { "+" })
    }
}

fn nonzero(rng: &mut Rng, lo: i64, hi: i64) -> i64 {
    let x = rng.range(lo, hi);
    if x == 0 {
        1
    } else {
        x
    }
}

pub fn complex_product(rng: &mut Rng) -> Exercise {
    let (a, b, c, d) = (
        nonzero(rng, -5, 5),
        nonzero(rng, -5, 5),
        nonzero(rng, -5, 5),
        nonzero(rng, -5, 5),
    );
    let (re, im) = (a * c - b * d, a * d + b * c);
    let real = rng.coin();
    Exercise::new(
        format!(
            "Partie {} de ({})({}) ?",
            if real { "réelle" } else { "imaginaire" },
            complex(a, b),
            complex(c, d)
        ),
        Answer::Number {
            value: if real { re } else { im },
            radix: 10,
        },
        format!(
            "(a + ib)(c + id) = (ac − bd) + i(ad + bc), car i² = −1. Ici : {}.",
            complex(re, im)
        ),
    )
}

pub fn complex_modulus(rng: &mut Rng) -> Exercise {
    const TRIPLES: [(i64, i64, i64); 6] = [
        (3, 4, 5),
        (5, 12, 13),
        (8, 15, 17),
        (6, 8, 10),
        (7, 24, 25),
        (9, 12, 15),
    ];
    let pick = |rng: &mut Rng| {
        let (x, y, r) = *rng.pick(&TRIPLES);
        let (x, y) = if rng.coin() { (x, y) } else { (y, x) };
        let sx = if rng.coin() { 1 } else { -1 };
        let sy = if rng.coin() { 1 } else { -1 };
        (sx * x, sy * y, r)
    };
    let (x, y, r) = pick(rng);
    if rng.below(3) == 0 {
        let (x2, y2, r2) = pick(rng);
        return Exercise::new(
            format!("Module de ({})({}) ?", complex(x, y), complex(x2, y2)),
            Answer::Number {
                value: r * r2,
                radix: 10,
            },
            format!(
                "|zz′| = |z| |z′| = {r} × {r2} = {} — inutile de développer.",
                r * r2
            ),
        );
    }
    Exercise::new(
        format!("Module de {} ?", complex(x, y)),
        Answer::Number {
            value: r,
            radix: 10,
        },
        format!("|z| = √(x² + y²) = √({} + {}) = {r}.", x * x, y * y),
    )
}

pub fn i_power(rng: &mut Rng) -> Exercise {
    const VALUES: [&str; 4] = ["1", "i", "−1", "−i"];
    let k = rng.range(5, 250);
    let r = (k % 4) as usize;
    let wrong: Vec<String> = VALUES
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != r)
        .map(|(_, v)| v.to_string())
        .collect();
    Exercise::new(
        format!("Que vaut i{} ?", sup(k as u32)),
        shuffled_choice(rng, VALUES[r].to_string(), wrong),
        format!(
            "i⁴ = 1, donc on réduit l'exposant modulo 4 : {k} = 4 × {} + {r}, et i{} = i{} = {}.",
            k / 4,
            sup(k as u32),
            sup(r as u32),
            VALUES[r]
        ),
    )
}

pub fn roots_of_unity(rng: &mut Rng) -> Exercise {
    let n = rng.range(3, 12);
    match rng.below(4) {
        0 => Exercise::new(
            format!("Combien de racines {n}-ièmes de l'unité dans ℂ ?"),
            Answer::Number {
                value: n,
                radix: 10,
            },
            format!(
                "ω_k = e^{{2ikπ/{n}}}, k = 0, …, {} : {n} racines, sommets d'un polygone régulier.",
                n - 1
            ),
        ),
        1 => Exercise::new(
            format!("Somme des racines {n}-ièmes de l'unité ?"),
            Answer::Number {
                value: 0,
                radix: 10,
            },
            "1 + ω + … + ω^{n−1} = (1 − ωⁿ)/(1 − ω) = 0 pour ω = e^{2iπ/n} ≠ 1.",
        ),
        2 => {
            let p = if n % 2 == 1 { 1 } else { -1 };
            Exercise::new(
                format!("Produit des racines {n}-ièmes de l'unité ?"),
                Answer::Number {
                    value: p,
                    radix: 10,
                },
                format!("Π ω^k = (−1)^(n+1) = (−1)^{} = {}.", n + 1, minus(p)),
            )
        }
        _ => {
            let k = rng.range(n + 1, 60);
            Exercise::new(
                format!("ω = e^{{2iπ/{n}}}. ω^{k} = ω^r avec 0 ≤ r < {n} : que vaut r ?"),
                Answer::Number {
                    value: k % n,
                    radix: 10,
                },
                format!(
                    "ωⁿ = 1, donc on réduit {k} modulo {n} : {k} = {n} × {} + {}.",
                    k / n,
                    k % n
                ),
            )
        }
    }
}

// ---------------------------------------------------------------- arithmétique

pub fn euclid_division(rng: &mut Rng) -> Exercise {
    let b = rng.range(2, 13);
    let mut a = rng.range(-90, 200);
    if a % b == 0 {
        a += 1;
    }
    let (q, r) = (a.div_euclid(b), a.rem_euclid(b));
    let ask_q = rng.coin();
    let trap = if a < 0 {
        " Pour a < 0, le reste reste positif : le quotient descend d'un cran."
    } else {
        ""
    };
    Exercise::new(
        format!(
            "Division euclidienne de {} par {b} : que vaut {} ?",
            minus(a),
            if ask_q { "le quotient q" } else { "le reste r" }
        ),
        Answer::Number {
            value: if ask_q { q } else { r },
            radix: 10,
        },
        format!(
            "{} = {b} × ({}) + {r}, avec 0 ≤ {r} < {b}.{trap}",
            minus(a),
            minus(q)
        ),
    )
}

fn euclid_code(a: i64, b: i64) -> String {
    math::euclid_steps(a, b)
        .iter()
        .map(|s| format!("{:>4} = {:>3} × {:>3} + {}", s.a, s.b, s.q, s.r))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn gcd_lcm(rng: &mut Rng) -> Exercise {
    let g = rng.range(2, 12);
    let x = rng.range(3, 15);
    let y = rng.range(2, x - 1);
    let (a, b) = (g * x, g * y);
    let d = math::gcd(a, b);
    let code = euclid_code(a, b);
    if rng.coin() {
        Exercise::new(
            format!("PGCD de {a} et {b} ?"),
            Answer::Number {
                value: d,
                radix: 10,
            },
            format!("Algorithme d'Euclide : le PGCD est le dernier reste non nul, {d}."),
        )
        .with_code(code)
    } else {
        let l = math::lcm(a, b);
        Exercise::new(
            format!("PPCM de {a} et {b} ?"),
            Answer::Number {
                value: l,
                radix: 10,
            },
            format!(
                "ab = (a ∧ b)(a ∨ b), et a ∧ b = {d} (Euclide) : a ∨ b = {a} × {b} / {d} = {l}."
            ),
        )
        .with_code(code)
    }
}

pub fn mod_power(rng: &mut Rng) -> Exercise {
    let m = *rng.pick(&[5i64, 7, 9, 11, 13]);
    let mut a = rng.range(20, 5000);
    while math::gcd(a, m) != 1 {
        a += 1;
    }
    let n = rng.range(100, 5000) as u64;
    let a0 = a.rem_euclid(m);
    let (k, minus_one) = (1..m as u64)
        .find_map(|k| match math::pow_mod(a0, k, m) {
            1 => Some((k, false)),
            r if r == m - 1 => Some((k, true)),
            _ => None,
        })
        .unwrap_or((1, false));
    let (q, r) = (n / k, n % k);
    let result = math::pow_mod(a, n, m);
    let sign = if minus_one { "−1" } else { "1" };
    let code = format!(
        "{a} ≡ {a0} [{m}]          donc {a}{} ≡ {a0}{}\n{a0}{} ≡ {sign} [{m}]\n{n} = {k} × {q} + {r}\n{a0}{} ≡ ({a0}{}){} · {a0}{} ≡ {result} [{m}]",
        sup(n as u32),
        sup(n as u32),
        sup(k as u32),
        sup(n as u32),
        sup(k as u32),
        sup(q as u32),
        sup(r as u32),
    );
    Exercise::new(
        format!("Reste de {a}{} dans la division par {m} ?", sup(n as u32)),
        Answer::Number { value: result, radix: 10 },
        format!(
            "Méthode : réduire la base, trouver le plus petit exposant k avec {a0}ᵏ ≡ ±1 [{m}] (ici k = {k}), puis diviser l'exposant par k."
        ),
    )
    .with_code(code)
}

pub fn inverse_in_zn(rng: &mut Rng) -> Exercise {
    let n = rng.range(5, 30);
    let a = rng.range(2, n - 1);
    let d = math::gcd(a, n);
    if rng.coin() {
        return Exercise::new(
            format!("{a}̄ est-il inversible dans ℤ/{n}ℤ ?"),
            yes_no(d == 1),
            format!("ā est inversible si et seulement si a ∧ n = 1. Ici {a} ∧ {n} = {d}."),
        );
    }
    let mut a = a;
    while math::gcd(a, n) != 1 {
        a += 1;
    }
    let inv = math::inverse_mod(a, n).unwrap_or(1);
    Exercise::new(
        format!("Inverse de {a}̄ dans ℤ/{n}ℤ (représentant entre 0 et {}) ?", n - 1),
        Answer::Number { value: inv, radix: 10 },
        format!("{a} × {inv} = {} = {} × {n} + 1, donc {a}̄ · {inv}̄ = 1̄. Bezout donne l'inverse en général.", a * inv, a * inv / n),
    )
}

pub fn affine_bijection(rng: &mut Rng) -> Exercise {
    let n = rng.range(10, 60);
    let a = rng.range(2, n - 1);
    let b = rng.range(0, n - 1);
    let d = math::gcd(a, n);
    let why = if d == 1 {
        format!("{a} ∧ {n} = 1 : {a}̄ est inversible, donc f est bijective, de réciproque y ↦ {a}̄⁻¹(y − {b}).")
    } else {
        format!(
            "{a} ∧ {n} = {d} ≠ 1 : f(0) = f({}) puisque {a} × {} = {} est multiple de {n}. Pas injective, donc pas bijective.",
            n / d,
            n / d,
            a * (n / d)
        )
    };
    Exercise::new(
        format!("f : x ↦ {a}x + {b} est-elle une bijection de ℤ/{n}ℤ ?"),
        yes_no(d == 1),
        why,
    )
}

pub fn bezout_pair(rng: &mut Rng) -> Exercise {
    let (a, b) = loop {
        let (a, b) = (rng.range(20, 150), rng.range(10, 90));
        if a != b && math::gcd(a, b) <= 6 {
            break (a.max(b), a.min(b));
        }
    };
    let (g, u, v) = math::bezout(a, b);
    Exercise::new(
        format!("Trouver u, v ∈ ℤ tels que {a}u + {b}v = {g}."),
        Answer::Bezout { a, b, g },
        format!(
            "{a} ∧ {b} = {g}. En remontant l'algorithme d'Euclide : {a} × ({}) + {b} × ({}) = {g}. Tout couple (u + {}k, v − {}k) convient aussi.",
            minus(u),
            minus(v),
            b / g,
            a / g
        ),
    )
    .with_code(euclid_code(a, b))
}

// ---------------------------------------------------------------- groupes

pub fn order_in_zn(rng: &mut Rng) -> Exercise {
    let n = rng.range(4, 30);
    let k = rng.range(1, n - 1);
    let d = math::gcd(k, n);
    Exercise::new(
        format!("Ordre de {k}̄ dans le groupe (ℤ/{n}ℤ, +) ?"),
        Answer::Number {
            value: n / d,
            radix: 10,
        },
        format!(
            "Plus petit p ≥ 1 avec p × {k} ≡ 0 [{n}] : p = {n} / ({k} ∧ {n}) = {n} / {d} = {}.",
            n / d
        ),
    )
}

// ---------------------------------------------------------------- permutations

fn random_perm(rng: &mut Rng) -> Vec<usize> {
    let n = rng.range(6, 10) as usize;
    loop {
        let p = rng.sample(n, n);
        if math::cycles(&p).len() >= 2 {
            return p;
        }
    }
}

fn perm_table(perm: &[usize]) -> String {
    let xs: Vec<String> = (0..perm.len()).map(|x| format!("{x:>2}")).collect();
    let ys: Vec<String> = perm.iter().map(|y| format!("{y:>2}")).collect();
    format!("x    │ {}\nσ(x) │ {}", xs.join(" "), ys.join(" "))
}

fn lengths(perm: &[usize]) -> String {
    math::cycles(perm)
        .iter()
        .map(|c| c.len().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn perm_cycles(rng: &mut Rng) -> Exercise {
    let p = random_perm(rng);
    Exercise::new(
        format!("Décomposer σ en cycles à supports disjoints (σ agit sur {{0, …, {}}}).", p.len() - 1),
        Answer::Cycles { perm: p.clone() },
        "Partir de 0 et suivre σ jusqu'au retour, fermer le cycle, reprendre au plus petit élément non traité. Les points fixes s'omettent.",
    )
    .with_code(perm_table(&p))
}

pub fn perm_order_sig(rng: &mut Rng) -> Exercise {
    let p = random_perm(rng);
    let decomposition = math::cycles_to_string(&p);
    let t = math::transpositions(&p);
    let ex = match rng.below(3) {
        0 => Exercise::new(
            "Ordre de σ ?",
            Answer::Number { value: math::perm_order(&p), radix: 10 },
            format!("σ = {decomposition}. Ordre = PPCM des longueurs ({}) = {}.", lengths(&p), math::perm_order(&p)),
        ),
        1 => Exercise::new(
            "Signature de σ ?",
            Answer::Choice {
                options: vec!["1".into(), "−1".into()],
                correct: if t.is_multiple_of(2) { 0 } else { 1 },
            },
            format!(
                "σ = {decomposition} : chaque cycle de longueur P donne P − 1 transpositions, soit {t} en tout. ε(σ) = (−1)^{t} = {}.",
                minus(math::signature(&p))
            ),
        ),
        _ => Exercise::new(
            "En combien de transpositions σ se décompose-t-elle, en enchaînant chaque cycle ?",
            Answer::Number { value: t as i64, radix: 10 },
            format!("σ = {decomposition} : P − 1 transpositions par cycle de longueur P, soit {t}."),
        ),
    };
    ex.with_code(perm_table(&p))
}

pub fn perm_inverse(rng: &mut Rng) -> Exercise {
    let p = random_perm(rng);
    Exercise::new(
        "Écrire σ⁻¹ en cycles disjoints.",
        Answer::Cycles {
            perm: math::inverse_perm(&p),
        },
        format!(
            "σ = {} : on retourne chaque cycle, σ⁻¹ = {}.",
            math::cycles_to_string(&p),
            math::cycles_to_string(&math::inverse_perm(&p))
        ),
    )
    .with_code(perm_table(&p))
}

pub fn perm_power(rng: &mut Rng) -> Exercise {
    let p = random_perm(rng);
    let order = math::perm_order(&p) as u64;
    let n = rng.range(50, 20_000) as u64;
    let result = math::perm_pow(&p, n);
    Exercise::new(
        format!("Écrire σ{} en cycles disjoints.", sup(n as u32)),
        Answer::Cycles {
            perm: result.clone(),
        },
        format!(
            "σ = {} est d'ordre {order}. {n} = {order} × {} + {}, donc σ{} = σ{} = {}.",
            math::cycles_to_string(&p),
            n / order,
            n % order,
            sup(n as u32),
            sup((n % order) as u32),
            math::cycles_to_string(&result)
        ),
    )
    .with_code(perm_table(&p))
}

pub fn order_exists(rng: &mut Rng) -> Exercise {
    let n = rng.range(7, 13) as u64;
    let k = rng.range(6, 45) as u64;
    let parts: Vec<u64> = math::factorize(k).iter().map(|(p, e)| p.pow(*e)).collect();
    let need = math::min_elements_for_order(k);
    let list = parts
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" + ");
    let why = if need <= n {
        format!("Des cycles disjoints de longueurs {list} ont pour PPCM {k} et occupent {need} ≤ {n} éléments.")
    } else {
        format!(
            "Pour atteindre l'ordre {k}, il faut au mieux des cycles de longueurs {list}, soit {need} éléments : on n'en a que {n}."
        )
    };
    Exercise::new(
        format!("Existe-t-il une permutation d'ordre {k} sur un ensemble à {n} éléments ?"),
        yes_no(need <= n),
        why,
    )
}
