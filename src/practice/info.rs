//! Entraînement — Représentation de l'information.

use super::answer::format_int;
use super::{num, qcm, sub, sup, txt, vf, yes_no, Answer, Chapter, Exercise, Fixed, Rng};
use crate::convert::alphabet;

pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        id: "notions",
        title: "Du sens au bit",
        anchor: "i-binaire",
        fixed: NOTIONS,
        generators: &[],
    },
    Chapter {
        id: "numeration",
        title: "Systèmes de numération",
        anchor: "i-numeration",
        fixed: NUMERATION,
        generators: &[digit_value, valid_in_base, polynomial],
    },
    Chapter {
        id: "conversion",
        title: "Conversions entre bases",
        anchor: "i-conversion",
        fixed: CONVERSION,
        generators: &[bin_to_dec, dec_to_bin, hex_bin, oct_bin, dec_to_hex],
    },
    Chapter {
        id: "fractionnaire",
        title: "Nombres fractionnaires",
        anchor: "i-fractionnaire",
        fixed: FRACTION,
        generators: &[frac_to_bin, bin_to_frac],
    },
    Chapter {
        id: "entiers",
        title: "Entiers non signés",
        anchor: "i-entiers",
        fixed: ENTIERS,
        generators: &[max_on_bits, bits_needed, byte_addition, overflow],
    },
];

const NOTIONS: &[Fixed] = &[
    qcm(
        "Dans un ordinateur, une donnée est toujours…",
        &["une succession de 0 et de 1", "un nombre écrit en base 10", "une information qui a du sens", "un texte en ASCII"],
        "« Donnée » abrège « représentation d'information » : dans la machine, c'est toujours une suite de 0 et de 1.",
    ),
    vf(
        "Un ordinateur travaille sur le sens de l'information.",
        false,
        "Le sens n'est pas pertinent pour une machine purement matérielle : elle transforme uniquement des représentations.",
    ),
    qcm(
        "« Bit » est la contraction de…",
        &["binary digit", "byte information", "binary item", "basic integer"],
        "Bit = binary digit, chiffre binaire : l'unité minimale d'information, 0 ou 1.",
    ),
    num("Combien de bits compte un octet ?", 8, "Un octet est un mot de 8 bits, comme 10110011."),
    num("Combien de valeurs différentes un bit peut-il prendre ?", 2, "Deux : 0 ou 1. C'est la représentation binaire."),
    qcm(
        "Le principe « tout ou rien » des machines relève de l'électronique…",
        &["numérique (digitale)", "analogique", "continue", "quantique"],
        "Tout ou rien = électronique numérique ou digitale, par opposition à l'électronique analogique.",
    ),
    vf(
        "Un bit seul suffit à représenter une information complexe.",
        false,
        "Il faut combiner une multitude de bits, regroupés en ensembles (octets, mots).",
    ),
    super::card(
        "Information — définition",
        "Connaissance qui fait du sens pour les personnes concernées et qui peut être représentée par des symboles.",
    ),
    super::card(
        "Donnée — définition",
        "Représentation d'une information. Dans un ordinateur, c'est toujours une succession de 0 et de 1.",
    ),
];

const NUMERATION: &[Fixed] = &[
    qcm(
        "Un système de numération est défini par…",
        &[
            "une base B et un ensemble ordonné de B chiffres",
            "une base B et une virgule",
            "un nombre de bits fixé",
            "une table des puissances de 2",
        ],
        "La base B et l'alphabet C, ensemble ordonné de cardinal B.",
    ),
    qcm(
        "MSB désigne…",
        &[
            "le chiffre de poids fort, le plus à gauche",
            "le chiffre de poids faible, le plus à droite",
            "le bit de signe",
            "le nombre de bits du mot",
        ],
        "Most Significant Bit : poids fort, à gauche. LSB (Least Significant Bit) : poids faible, à droite.",
    ),
    qcm("Quel chiffre n'existe pas en base 8 ?", &["8", "0", "7", "5"], "C = {0, 1, 2, 3, 4, 5, 6, 7} : la base 8 a huit chiffres, de 0 à 7."),
    num("Combien de chiffres compte l'alphabet hexadécimal ?", 16, "C = {0…9, A, B, C, D, E, F} : seize chiffres."),
    num("(1011)₂ vaut, en base 10 :", 11, "1·2³ + 0·2² + 1·2¹ + 1·2⁰ = 8 + 2 + 1 = 11."),
    num(
        "(A7C5)₁₆ vaut, en base 10 :",
        42949,
        "10·16³ + 7·16² + 12·16¹ + 5·16⁰ = 40960 + 1792 + 192 + 5 = 42949.",
    ),
];

const CONVERSION: &[Fixed] = &[
    qcm(
        "Méthode des divisions successives : on lit les restes…",
        &[
            "de bas en haut, du dernier au premier",
            "de haut en bas, du premier au dernier",
            "dans n'importe quel ordre",
            "en ne gardant que les restes non nuls",
        ],
        "Le premier reste est le chiffre de poids faible, le dernier celui de poids fort.",
    ),
    qcm("Un chiffre hexadécimal correspond à…", &["4 bits", "3 bits", "8 bits", "16 bits"], "16 = 2⁴ : un chiffre hexadécimal ↔ 4 bits."),
    qcm("Un chiffre octal correspond à…", &["3 bits", "4 bits", "2 bits", "8 bits"], "8 = 2³ : un chiffre octal ↔ 3 bits."),
    txt(
        "(1110 1011)₂ en hexadécimal ?",
        &["EB"],
        "On code chaque groupe de 4 bits : 1110 = E, 1011 = B.",
    ),
    qcm(
        "Méthode des soustractions successives : à chaque étape, on retire…",
        &[
            "la plus grande puissance de 2 inférieure ou égale au nombre",
            "la plus petite puissance de 2",
            "le reste de la division par 2",
            "la moitié du nombre",
        ],
        "On place un 1 à la position de cette puissance, et on recommence tant que le résultat n'est pas nul.",
    ),
    qcm(
        "Méthode du polynôme : les calculs se font dans…",
        &["la base d'arrivée B′", "la base de départ B", "la base 2", "la base 16"],
        "On calcule Σ chiffre × Bᵖᵒˢⁱᵗⁱᵒⁿ dans la base de destination — en pratique la base 10.",
    ),
];

const FRACTION: &[Fixed] = &[
    qcm(
        "Un chiffre C placé n rangs après la virgule, en base B, vaut…",
        &["C × B⁻ⁿ", "C × Bⁿ", "C / n", "C × n⁻ᴮ"],
        "Par exemple (0,1)₂ = 1 × 2⁻¹ = 0,5.",
    ),
    qcm(
        "Pour convertir une partie fractionnaire vers la base B′, on procède par…",
        &[
            "multiplications successives par B′",
            "divisions successives par B′",
            "soustractions de puissances de B′",
            "une table de codes",
        ],
        "À la i-ème multiplication, la partie entière du résultat donne le i-ème chiffre après la virgule.",
    ),
    txt("0,5 en binaire ?", &["0,1", ",1"], "0,5 × 2 = 1 → chiffre 1, il ne reste rien : (0,1)₂."),
];

const ENTIERS: &[Fixed] = &[
    qcm(
        "Sur n bits, un entier non signé prend ses valeurs dans…",
        &["[0 ; 2ⁿ − 1]", "[0 ; 2ⁿ]", "[1 ; 2ⁿ]", "[−2ⁿ⁻¹ ; 2ⁿ⁻¹ − 1]"],
        "2ⁿ valeurs possibles, de 0 à 2ⁿ − 1.",
    ),
    num(
        "Plus grande valeur d'un entier non signé sur un octet ?",
        255,
        "2⁸ − 1 = 255, soit 11111111.",
    ),
    vf(
        "Calculé sur un octet, 147 + 200 donne 347.",
        false,
        "347 > 255 : dépassement de capacité. Le 9ᵉ bit est perdu et on lit 01011011 = 91.",
    )
    .code("  10010011   (147)\n+ 11001000   (200)\n──────────\n1 01011011"),
    qcm(
        "Sur 32 bits, la valeur maximale est de l'ordre de…",
        &["4·10⁹", "4·10⁶", "32", "10³²"],
        "2³² − 1 ≈ 4,3 milliards.",
    ),
    qcm(
        "« Définir un format d'écriture » pour les entiers non signés, c'est…",
        &[
            "compléter l'écriture binaire à gauche par des 0",
            "ajouter un bit de signe",
            "compléter l'écriture à droite par des 0",
            "écrire le nombre en hexadécimal",
        ],
        "Sur 8 bits, 5 s'écrit 00000101.",
    ),
];

// ---------------------------------------------------------------- générateurs

const HEX: &[u8] = b"0123456789ABCDEF";

/// Chiffres de `n` en base `radix`, poids fort en tête.
fn digits(n: u64, radix: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut d = Vec::new();
    let mut m = n;
    while m > 0 {
        d.push(m % radix);
        m /= radix;
    }
    d.reverse();
    d
}

fn digit_value(rng: &mut Rng) -> Exercise {
    let v = rng.range(10, 15);
    let c = HEX[v as usize] as char;
    let explain = "A = 10, B = 11, C = 12, D = 13, E = 14, F = 15.";
    if rng.coin() {
        Exercise::new(
            format!("Que vaut le chiffre hexadécimal {c} en base 10 ?"),
            Answer::Number {
                value: v,
                radix: 10,
            },
            explain,
        )
    } else {
        Exercise::new(
            format!("Quel chiffre hexadécimal vaut {v} ?"),
            Answer::Text {
                accepted: vec![c.to_string()],
            },
            explain,
        )
    }
}

fn valid_in_base(rng: &mut Rng) -> Exercise {
    let base = *rng.pick(&[2u32, 8, 10]);
    let len = rng.range(3, 6) as usize;
    let mut chars: Vec<char> = (0..len)
        .map(|_| HEX[rng.below(base as usize)] as char)
        .collect();
    if chars[0] == '0' {
        chars[0] = '1';
    }
    let invalid = rng.coin();
    let mut bad = None;
    if invalid {
        let pos = rng.below(len);
        let c = HEX[rng.range(base as i64, base as i64 + 1).min(15) as usize] as char;
        chars[pos] = c;
        bad = Some(c);
    }
    let s: String = chars.iter().collect();
    let mut explain = format!("En base {base}, C = {{{}}}.", alphabet(base));
    if let Some(c) = bad {
        explain.push_str(&format!(" Le chiffre « {c} » n'en fait pas partie."));
    } else {
        explain.push_str(" Tous les chiffres en font partie.");
    }
    Exercise::new(
        format!("L'écriture {s} est-elle valide en base {base} ?"),
        yes_no(!invalid),
        explain,
    )
}

/// Développement polynomial `d·Bᵏ + …` et somme des termes.
fn expansion(ds: &[u64], base: u64) -> (String, String) {
    let n = ds.len();
    let terms: Vec<String> = ds
        .iter()
        .enumerate()
        .map(|(i, d)| format!("{d}·{base}{}", sup((n - 1 - i) as u32)))
        .collect();
    let values: Vec<String> = ds
        .iter()
        .enumerate()
        .map(|(i, d)| (d * base.pow((n - 1 - i) as u32)).to_string())
        .collect();
    (terms.join(" + "), values.join(" + "))
}

fn polynomial(rng: &mut Rng) -> Exercise {
    let (base, lo, hi) = *rng.pick(&[(2u64, 32, 255), (8, 64, 4095), (16, 256, 65535)]);
    let n = rng.range(lo, hi) as u64;
    let ds = digits(n, base);
    let written = format_int(n as i64, base as u32);
    let (terms, values) = expansion(&ds, base);
    Exercise::new(
        format!("Convertir ({written}){} en base 10.", sub(base as u32)),
        Answer::Number {
            value: n as i64,
            radix: 10,
        },
        format!("Méthode du polynôme : {terms} = {values} = {n}."),
    )
}

fn bin_to_dec(rng: &mut Rng) -> Exercise {
    let n = rng.range(5, 255) as u64;
    let ds = digits(n, 2);
    let k = ds.len();
    let powers: Vec<String> = ds
        .iter()
        .enumerate()
        .filter(|(_, d)| **d == 1)
        .map(|(i, _)| (1u64 << (k - 1 - i)).to_string())
        .collect();
    Exercise::new(
        format!("Convertir ({n:b})₂ en base 10."),
        Answer::Number {
            value: n as i64,
            radix: 10,
        },
        format!(
            "On additionne les puissances de 2 des bits à 1 : {} = {n}.",
            powers.join(" + ")
        ),
    )
}

/// Tableau des divisions successives, comme dans le cours.
fn divisions(n: u64, base: u64) -> String {
    let mut lines = Vec::new();
    let mut m = n;
    while m > 0 {
        let (q, r) = (m / base, m % base);
        let shown = if r >= 10 {
            format!("{r} → {}", HEX[r as usize] as char)
        } else {
            r.to_string()
        };
        lines.push(format!("{m:>5} / {base} = {q:>4}   reste {shown}"));
        m = q;
    }
    lines.join("\n")
}

fn dec_to_bin(rng: &mut Rng) -> Exercise {
    let n = rng.range(5, 255) as u64;
    Exercise::new(
        format!("Convertir {n} en binaire."),
        Answer::Number {
            value: n as i64,
            radix: 2,
        },
        format!("Divisions successives par 2, restes lus de bas en haut : ({n:b})₂."),
    )
    .with_code(divisions(n, 2))
}

fn dec_to_hex(rng: &mut Rng) -> Exercise {
    let n = rng.range(20, 1500) as u64;
    Exercise::new(
        format!("Convertir {n} en hexadécimal."),
        Answer::Number {
            value: n as i64,
            radix: 16,
        },
        format!("Divisions successives par 16, restes lus de bas en haut : ({n:X})₁₆."),
    )
    .with_code(divisions(n, 16))
}

/// Correspondance chiffre par chiffre entre la base `2^bits` et le binaire.
fn table_codes(n: u64, bits: u32) -> String {
    let base = 1u64 << bits;
    digits(n, base)
        .iter()
        .map(|d| {
            format!(
                "{} → {:0width$b}",
                HEX[*d as usize] as char,
                d,
                width = bits as usize
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn grouped_binary(n: u64, bits: u32) -> String {
    let base = 1u64 << bits;
    digits(n, base)
        .iter()
        .map(|d| format!("{:0width$b}", d, width = bits as usize))
        .collect::<Vec<_>>()
        .join(" ")
}

fn base_power_of_two(rng: &mut Rng, bits: u32) -> Exercise {
    let base = 1u32 << bits;
    let n = rng.range(base as i64 * 2, if bits == 4 { 4095 } else { 511 }) as u64;
    let written = format_int(n as i64, base);
    let why = format!(
        "{base} = 2{} : un chiffre ↔ {bits} bits. {}.",
        sup(bits),
        table_codes(n, bits)
    );
    if rng.coin() {
        Exercise::new(
            format!("Convertir ({written}){} en binaire.", sub(base)),
            Answer::Number {
                value: n as i64,
                radix: 2,
            },
            format!("{why} Résultat : ({})₂.", grouped_binary(n, bits)),
        )
    } else {
        Exercise::new(
            format!("Convertir ({})₂ en base {base}.", grouped_binary(n, bits)),
            Answer::Number {
                value: n as i64,
                radix: base,
            },
            format!("{why} Résultat : ({written}){}.", sub(base)),
        )
    }
}

fn hex_bin(rng: &mut Rng) -> Exercise {
    base_power_of_two(rng, 4)
}

fn oct_bin(rng: &mut Rng) -> Exercise {
    base_power_of_two(rng, 3)
}

/// `int + num/2^m` écrit en décimal avec une virgule.
fn decimal(int: u64, num: u64, m: u32) -> String {
    if num == 0 {
        return int.to_string();
    }
    let scaled = num * 5u64.pow(m);
    let s = format!("{scaled:0width$}", width = m as usize);
    format!("{int},{}", s.trim_end_matches('0'))
}

/// `int + num/2^m` écrit en binaire avec une virgule.
fn binary(int: u64, num: u64, m: u32) -> String {
    let frac = format!("{num:0width$b}", width = m as usize);
    format!("{int:b},{}", frac.trim_end_matches('0'))
}

/// Tire `int + num/2^m` avec `num` impair : m chiffres binaires après la virgule.
fn dyadic(rng: &mut Rng) -> (u64, u64, u32) {
    let m = rng.range(1, 4) as u32;
    let num = 2 * rng.range(0, (1 << (m - 1)) - 1) as u64 + 1;
    let int = if rng.coin() {
        0
    } else {
        rng.range(1, 12) as u64
    };
    (int, num, m)
}

fn frac_to_bin(rng: &mut Rng) -> Exercise {
    let (int, num, m) = dyadic(rng);
    let den = 1u64 << m;
    let mut lines = Vec::new();
    let mut k = num;
    while k != 0 {
        let before = decimal(0, k, m);
        let doubled = 2 * k;
        let digit = doubled / den;
        k = doubled % den;
        lines.push(format!(
            "{before} × 2 = {}   → {digit}",
            decimal(digit, k, m)
        ));
    }
    let result = binary(int, num, m);
    let mut accepted = vec![result.clone()];
    if int == 0 {
        accepted.push(result.trim_start_matches('0').to_string());
    }
    Exercise::new(
        format!("Écrire {} en binaire.", decimal(int, num, m)),
        Answer::Text { accepted },
        format!(
            "Partie entière : {int} = ({int:b})₂. Partie fractionnaire : multiplications successives par 2, les parties entières donnent les chiffres après la virgule. Résultat : ({result})₂."
        ),
    )
    .with_code(lines.join("\n"))
}

fn bin_to_frac(rng: &mut Rng) -> Exercise {
    let (_, num, m) = dyadic(rng);
    let bits = format!("{num:0width$b}", width = m as usize);
    let terms: Vec<String> = bits
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '1')
        .map(|(i, _)| decimal(0, 1 << (m - 1 - i as u32), m))
        .collect();
    Exercise::new(
        format!("Que vaut (0,{bits})₂ en base 10 ?"),
        Answer::Text {
            accepted: vec![decimal(0, num, m)],
        },
        format!(
            "Chaque 1 placé n rangs après la virgule vaut 2⁻ⁿ : {} = {}.",
            terms.join(" + "),
            decimal(0, num, m)
        ),
    )
}

fn max_on_bits(rng: &mut Rng) -> Exercise {
    let n = rng.range(2, 16) as u32;
    let max = (1i64 << n) - 1;
    Exercise::new(
        format!("Plus grande valeur d'un entier non signé sur {n} bits ?"),
        Answer::Number {
            value: max,
            radix: 10,
        },
        format!("2{} − 1 = {} − 1 = {max}.", sup(n), 1i64 << n),
    )
}

fn bits_needed(rng: &mut Rng) -> Exercise {
    let x = rng.range(3, 5000) as u64;
    let k = 64 - x.leading_zeros();
    Exercise::new(
        format!("Combien de bits faut-il au minimum pour écrire {x} en binaire ?"),
        Answer::Number {
            value: k as i64,
            radix: 10,
        },
        format!(
            "2{} = {} ≤ {x} < 2{} = {} : il faut {k} bits ({x:b}).",
            sup(k - 1),
            1u64 << (k - 1),
            sup(k),
            1u64 << k
        ),
    )
}

fn byte_operands(rng: &mut Rng) -> (u64, u64) {
    (rng.range(20, 240) as u64, rng.range(10, 240) as u64)
}

fn addition_code(a: u64, b: u64) -> String {
    let s = a + b;
    let result = if s > 255 {
        format!(
            "1 {:08b}   ← 9 bits : le bit de poids fort est perdu",
            s & 0xFF
        )
    } else {
        format!("  {s:08b}")
    };
    format!("  {a:08b}   ({a:>3})\n+ {b:08b}   ({b:>3})\n──────────\n{result}")
}

fn byte_addition(rng: &mut Rng) -> Exercise {
    let (a, b) = byte_operands(rng);
    let s = a + b;
    let explain = if s > 255 {
        format!(
            "{a} + {b} = {s} > 255 : dépassement de capacité. On perd 256 et on lit {}.",
            s - 256
        )
    } else {
        format!("{a} + {b} = {s} ≤ 255 : le résultat tient sur un octet.")
    };
    Exercise::new(
        format!("Sur un octet, que lit-on (en base 10) après l'addition {a} + {b} ?"),
        Answer::Number {
            value: (s % 256) as i64,
            radix: 10,
        },
        explain,
    )
    .with_code(addition_code(a, b))
}

fn overflow(rng: &mut Rng) -> Exercise {
    let (a, b) = byte_operands(rng);
    let s = a + b;
    Exercise::new(
        format!("L'addition {a} + {b} sur un octet provoque-t-elle un dépassement de capacité ?"),
        yes_no(s > 255),
        format!("{a} + {b} = {s}, à comparer avec 255, le maximum sur 8 bits."),
    )
    .with_code(format!("  {a:08b}\n+ {b:08b}"))
}
