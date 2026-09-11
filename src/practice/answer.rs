//! Réponses attendues et correction.
//!
//! La correction est tolérante sur la forme : casse, accents, espaces,
//! virgule décimale, zéros de tête, notations `(1011)₂` ou `0x3F`, ordre des
//! éléments d'un ensemble, point de départ d'un cycle. Elle ne l'est jamais
//! sur le fond.

use super::math;
use crate::search::norm;

#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    /// Choix multiple; `correct` indexe `options`.
    Choice {
        options: Vec<String>,
        correct: usize,
    },
    /// Réponse libre comparée après normalisation à l'une des formes admises.
    Text { accepted: Vec<String> },
    /// Entier écrit dans la base `radix`.
    Number { value: i64, radix: u32 },
    /// Ensemble fini d'entiers : l'ordre et les répétitions sont ignorés.
    Set { elems: Vec<i64> },
    /// Permutation à écrire en cycles disjoints.
    Cycles { perm: Vec<usize> },
    /// Un couple `(u, v)` quelconque vérifiant `u·a + v·b = g`.
    Bezout { a: i64, b: i64, g: i64 },
    /// Carte à retourner : l'élève dit lui-même s'il savait.
    Card { back: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Response {
    Choice(usize),
    Text(String),
    Card(bool),
}

impl Answer {
    /// Vrai si la réponse se tape dans un champ de saisie.
    pub fn is_typed(&self) -> bool {
        !matches!(self, Answer::Choice { .. } | Answer::Card { .. })
    }

    pub fn grade(&self, response: &Response) -> bool {
        match (self, response) {
            (Answer::Choice { correct, .. }, Response::Choice(i)) => i == correct,
            (Answer::Card { .. }, Response::Card(knew)) => *knew,
            (Answer::Text { accepted }, Response::Text(s)) => {
                let s = clean(s);
                accepted.iter().any(|a| clean(a) == s)
            }
            (Answer::Number { value, radix }, Response::Text(s)) => {
                parse_int(s, *radix) == Some(*value)
            }
            (Answer::Set { elems }, Response::Text(s)) => {
                parse_set(s).as_deref() == Some(&sorted(elems)[..])
            }
            (Answer::Cycles { perm }, Response::Text(s)) => {
                parse_cycles(s, perm.len()).as_deref() == Some(&perm[..])
            }
            (Answer::Bezout { a, b, g }, Response::Text(s)) => match ints(s)[..] {
                [u, v] => u * a + v * b == *g,
                _ => false,
            },
            _ => false,
        }
    }

    /// La réponse attendue, telle qu'on l'affiche dans le corrigé.
    pub fn expected(&self) -> String {
        match self {
            Answer::Choice { options, correct } => options[*correct].clone(),
            Answer::Text { accepted } => accepted[0].clone(),
            Answer::Number { value, radix } => format_int(*value, *radix),
            Answer::Set { elems } => format_set(elems),
            Answer::Cycles { perm } => math::cycles_to_string(perm),
            Answer::Bezout { a, b, g } => {
                let (_, u, v) = math::bezout(*a, *b);
                let k = g / math::gcd(*a, *b);
                format!("u = {}, v = {}", minus(u * k), minus(v * k))
            }
            Answer::Card { back } => back.clone(),
        }
    }

    /// Indication affichée dans le champ vide.
    pub fn placeholder(&self) -> &'static str {
        match self {
            Answer::Number { radix: 2, .. } => "en binaire, ex. 101101",
            Answer::Number { radix: 8, .. } => "en octal, ex. 157",
            Answer::Number { radix: 16, .. } => "en hexadécimal, ex. 3F",
            Answer::Number { .. } => "un entier",
            Answer::Set { .. } => "ex. {1, 3, 5}  ou  ∅",
            Answer::Cycles { .. } => "ex. (0 3 5)(1 2)  — Id si identité",
            Answer::Bezout { .. } => "u, v",
            _ => "ta réponse",
        }
    }
}

/// Signe moins typographique, pour l'affichage.
pub fn minus(n: i64) -> String {
    if n < 0 {
        format!("−{}", -n)
    } else {
        n.to_string()
    }
}

pub fn format_int(value: i64, radix: u32) -> String {
    let sign = if value < 0 { "−" } else { "" };
    let v = value.unsigned_abs();
    let digits = match radix {
        2 => format!("{v:b}"),
        8 => format!("{v:o}"),
        16 => format!("{v:X}"),
        _ => v.to_string(),
    };
    format!("{sign}{digits}")
}

pub fn format_set(elems: &[i64]) -> String {
    let e = sorted(elems);
    if e.is_empty() {
        return "∅".to_string();
    }
    let inner: Vec<String> = e.iter().map(|x| minus(*x)).collect();
    format!("{{{}}}", inner.join(", "))
}

fn sorted(elems: &[i64]) -> Vec<i64> {
    let mut e = elems.to_vec();
    e.sort_unstable();
    e.dedup();
    e
}

/// Uniformise les variantes de signe moins.
fn ascii_minus(s: &str) -> String {
    s.chars()
        .map(|c| {
            if matches!(c, '−' | '–' | '—') {
                '-'
            } else {
                c
            }
        })
        .collect()
}

/// Forme de comparaison d'une réponse libre.
fn clean(s: &str) -> String {
    ascii_minus(&norm(s))
        .chars()
        .filter(|c| !c.is_whitespace() && !matches!(c, '"' | '\'' | '«' | '»' | '“' | '”' | '’'))
        .map(|c| if c == ',' { '.' } else { c })
        .collect()
}

/// Lit un entier en base `radix`, en acceptant `(1011)₂`, `0b1011`, `0x3F`,
/// les espaces et les zéros de tête.
pub fn parse_int(s: &str, radix: u32) -> Option<i64> {
    let mut t: String = ascii_minus(s)
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && !matches!(c, '(' | ')' | '_' | '₀'..='₉'))
        .collect();
    let negative = t.starts_with('-');
    if negative || t.starts_with('+') {
        t.remove(0);
    }
    let prefix = match radix {
        2 => "0b",
        8 => "0o",
        16 => "0x",
        _ => "",
    };
    if !prefix.is_empty() {
        if let Some(rest) = t.strip_prefix(prefix) {
            t = rest.to_string();
        }
    }
    if t.is_empty() || t.starts_with(['+', '-']) {
        return None;
    }
    let v = i64::from_str_radix(&t, radix).ok()?;
    Some(if negative { -v } else { v })
}

/// Tous les entiers relatifs présents dans le texte, dans l'ordre.
pub fn ints(s: &str) -> Vec<i64> {
    let s = ascii_minus(s);
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let digits: String = chars[start..i].iter().collect();
            if let Ok(v) = digits.parse::<i64>() {
                let negative = start > 0 && chars[start - 1] == '-';
                out.push(if negative { -v } else { v });
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Ensemble lu dans la réponse, trié et sans doublon; `None` si illisible.
fn parse_set(s: &str) -> Option<Vec<i64>> {
    let values = ints(s);
    if !values.is_empty() {
        return Some(sorted(&values));
    }
    let c = clean(s);
    let empty = c.contains('∅') || c == "{}" || c.contains("vide") || c == "o/";
    empty.then(Vec::new)
}

/// Permutation de `0..n` lue en cycles disjoints; `None` si l'écriture est
/// incohérente (élément hors bornes ou répété).
fn parse_cycles(s: &str, n: usize) -> Option<Vec<usize>> {
    let mut perm: Vec<usize> = (0..n).collect();
    let c = clean(s);
    if c == "id" || c == "identite" {
        return Some(perm);
    }
    let mut used = vec![false; n];
    let mut any = false;
    for group in s.split('(').skip(1) {
        let body = group.split(')').next()?;
        let cycle: Vec<usize> = ints(body)
            .into_iter()
            .map(|x| usize::try_from(x).ok().filter(|x| *x < n))
            .collect::<Option<_>>()?;
        for &x in &cycle {
            if std::mem::replace(&mut used[x], true) {
                return None;
            }
        }
        for (i, &x) in cycle.iter().enumerate() {
            perm[x] = cycle[(i + 1) % cycle.len()];
        }
        any = true;
    }
    any.then_some(perm)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(s: &str) -> Response {
        Response::Text(s.to_string())
    }

    #[test]
    fn entiers_dans_toutes_les_notations() {
        let a = Answer::Number {
            value: 11,
            radix: 2,
        };
        for ok in ["1011", "00001011", "(1011)₂", "0b1011", " 10 11 "] {
            assert!(a.grade(&t(ok)), "{ok}");
        }
        assert!(!a.grade(&t("1010")));
        assert!(!a.grade(&t("11")));
        let h = Answer::Number {
            value: 0xEB,
            radix: 16,
        };
        assert!(h.grade(&t("eb")) && h.grade(&t("0xEB")));
        let n = Answer::Number {
            value: -4,
            radix: 10,
        };
        assert!(n.grade(&t("−4")) && n.grade(&t("-4")) && !n.grade(&t("4")));
    }

    #[test]
    fn texte_sans_casse_ni_accents_ni_guillemets() {
        let a = Answer::Text {
            accepted: vec!["abracadabra".into()],
        };
        assert!(a.grade(&t("\"Abracadabra\"")));
        let r = Answer::Text {
            accepted: vec!["0,101".into()],
        };
        assert!(r.grade(&t("0.101")));
        let e = Answer::Text {
            accepted: vec!["réel".into()],
        };
        assert!(e.grade(&t("Reel")));
    }

    #[test]
    fn ensembles_dans_le_desordre() {
        let a = Answer::Set {
            elems: vec![3, 1, 5],
        };
        assert!(a.grade(&t("{5, 1, 3}")));
        assert!(a.grade(&t("1 3 5 5")));
        assert!(!a.grade(&t("{1, 3}")));
        let vide = Answer::Set { elems: vec![] };
        assert!(vide.grade(&t("∅")) && vide.grade(&t("{ }")) && vide.grade(&t("ensemble vide")));
        assert!(!vide.grade(&t("{0}")));
        assert_eq!(a.expected(), "{1, 3, 5}");
        assert_eq!(Answer::Set { elems: vec![-1, 1] }.expected(), "{−1, 1}");
    }

    #[test]
    fn cycles_ecrits_de_plusieurs_facons() {
        let perm = vec![12, 4, 0, 11, 6, 5, 10, 9, 7, 3, 1, 8, 2];
        let a = Answer::Cycles { perm };
        assert!(a.grade(&t("(0, 12, 2)(1, 4, 6, 10)(3, 11, 8, 7, 9)")));
        assert!(a.grade(&t("(4 6 10 1) (12 2 0) (9 3 11 8 7) (5)")));
        assert!(!a.grade(&t("(0, 2, 12)(1, 4, 6, 10)(3, 11, 8, 7, 9)")));
        assert!(!a.grade(&t("(0, 12, 2)(2, 4)")));
        assert!(!a.grade(&t("(0, 13)")));
        let id = Answer::Cycles {
            perm: vec![0, 1, 2],
        };
        assert!(id.grade(&t("Id")) && id.expected() == "Id");
    }

    #[test]
    fn bezout_accepte_tout_couple_valable() {
        let a = Answer::Bezout {
            a: 124,
            b: 47,
            g: 1,
        };
        assert!(a.grade(&t("11, -29")));
        assert!(a.grade(&t("u = 58, v = −153")));
        assert!(!a.grade(&t("11, 29")));
        assert!(a.grade(&t(&a.expected())));
    }

    #[test]
    fn la_reponse_attendue_est_toujours_acceptee() {
        let answers = [
            Answer::Number {
                value: 255,
                radix: 16,
            },
            Answer::Number {
                value: -17,
                radix: 10,
            },
            Answer::Set { elems: vec![] },
            Answer::Cycles {
                perm: vec![1, 0, 3, 2],
            },
            Answer::Text {
                accepted: vec!["/home/kenzo".into()],
            },
        ];
        for a in answers {
            assert!(a.grade(&t(&a.expected())), "{a:?}");
        }
    }
}
