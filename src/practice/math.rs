//! Calculs de référence des corrigés : arithmétique, dénombrement,
//! permutations. Les générateurs s'en servent pour produire la réponse
//! attendue, les tests pour la vérifier.

pub fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

/// `base^exp mod m`, résultat dans `0..m`.
pub fn pow_mod(base: i64, exp: u64, m: i64) -> i64 {
    let m = m as i128;
    let mut b = (base as i128).rem_euclid(m);
    let mut e = exp;
    let mut acc: i128 = 1 % m;
    while e > 0 {
        if e & 1 == 1 {
            acc = acc * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    acc as i64
}

pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Nombre d'arrangements `A(n,p) = n!/(n−p)!`.
pub fn arrangements(n: u64, p: u64) -> u64 {
    if p > n {
        return 0;
    }
    (n - p + 1..=n).product()
}

/// Coefficient binomial `C(n,p)`.
pub fn binom(n: u64, p: u64) -> u64 {
    if p > n {
        return 0;
    }
    let p = p.min(n - p);
    (0..p).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

/// Une étape de l'algorithme d'Euclide : `a = b × q + r`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EuclidStep {
    pub a: i64,
    pub b: i64,
    pub q: i64,
    pub r: i64,
}

/// Étapes de l'algorithme d'Euclide sur `a, b > 0`, jusqu'au reste nul.
pub fn euclid_steps(a: i64, b: i64) -> Vec<EuclidStep> {
    let (mut a, mut b) = (a, b);
    let mut steps = Vec::new();
    while b != 0 {
        let (q, r) = (a.div_euclid(b), a.rem_euclid(b));
        steps.push(EuclidStep { a, b, q, r });
        (a, b) = (b, r);
    }
    steps
}

/// Euclide étendu : `(g, u, v)` avec `u·a + v·b = g = a ∧ b`.
pub fn bezout(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r0, mut r1) = (a, b);
    let (mut u0, mut u1) = (1, 0);
    let (mut v0, mut v1) = (0, 1);
    while r1 != 0 {
        let q = r0.div_euclid(r1);
        (r0, r1) = (r1, r0 - q * r1);
        (u0, u1) = (u1, u0 - q * u1);
        (v0, v1) = (v1, v0 - q * v1);
    }
    if r0 < 0 {
        (-r0, -u0, -v0)
    } else {
        (r0, u0, v0)
    }
}

/// Inverse de `a` dans ℤ/nℤ, s'il existe, dans `0..n`.
pub fn inverse_mod(a: i64, n: i64) -> Option<i64> {
    let (g, u, _) = bezout(a.rem_euclid(n), n);
    (g == 1).then(|| u.rem_euclid(n))
}

/// Décomposition en facteurs premiers : `(p, exposant)`, p croissants.
pub fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        let mut e = 0;
        while n.is_multiple_of(p) {
            n /= p;
            e += 1;
        }
        if e > 0 {
            out.push((p, e));
        }
        p += 1;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

/// Plus petit nombre d'éléments qu'il faut pour qu'une permutation soit
/// d'ordre `k` : la somme des `pᵃ` de la décomposition de `k`.
pub fn min_elements_for_order(k: u64) -> u64 {
    factorize(k).iter().map(|(p, e)| p.pow(*e)).sum()
}

/// Cycles disjoints de longueur ≥ 2, chacun commençant par son plus petit
/// élément, rangés dans l'ordre de ces éléments — la méthode du cours.
pub fn cycles(perm: &[usize]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; perm.len()];
    let mut out = Vec::new();
    for start in 0..perm.len() {
        if seen[start] {
            continue;
        }
        let mut cycle = vec![start];
        seen[start] = true;
        let mut x = perm[start];
        while x != start {
            seen[x] = true;
            cycle.push(x);
            x = perm[x];
        }
        if cycle.len() > 1 {
            out.push(cycle);
        }
    }
    out
}

/// Écriture en cycles : `(0, 12, 2)(1, 4, 6, 10)`, ou `Id`.
pub fn cycles_to_string(perm: &[usize]) -> String {
    let cs = cycles(perm);
    if cs.is_empty() {
        return "Id".to_string();
    }
    cs.iter()
        .map(|c| {
            let inner: Vec<String> = c.iter().map(|x| x.to_string()).collect();
            format!("({})", inner.join(", "))
        })
        .collect()
}

pub fn perm_order(perm: &[usize]) -> i64 {
    cycles(perm)
        .iter()
        .fold(1, |acc, c| lcm(acc, c.len() as i64))
}

/// Nombre de transpositions de la décomposition en chaîne : Σ (P − 1).
pub fn transpositions(perm: &[usize]) -> usize {
    cycles(perm).iter().map(|c| c.len() - 1).sum()
}

pub fn signature(perm: &[usize]) -> i64 {
    if transpositions(perm).is_multiple_of(2) {
        1
    } else {
        -1
    }
}

pub fn inverse_perm(perm: &[usize]) -> Vec<usize> {
    let mut inv = vec![0; perm.len()];
    for (x, &y) in perm.iter().enumerate() {
        inv[y] = x;
    }
    inv
}

/// `σ^k`, calculé après réduction de `k` modulo l'ordre.
pub fn perm_pow(perm: &[usize], k: u64) -> Vec<usize> {
    let r = k % perm_order(perm) as u64;
    let mut out: Vec<usize> = (0..perm.len()).collect();
    for _ in 0..r {
        out = out.iter().map(|&x| perm[x]).collect();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// La permutation de ℕ₁₂ de l'examen 2024, corrigée dans le cours.
    const SIGMA: [usize; 13] = [12, 4, 0, 11, 6, 5, 10, 9, 7, 3, 1, 8, 2];

    #[test]
    fn permutation_de_l_examen_2024() {
        assert_eq!(
            cycles_to_string(&SIGMA),
            "(0, 12, 2)(1, 4, 6, 10)(3, 11, 8, 7, 9)"
        );
        assert_eq!(perm_order(&SIGMA), 60);
        assert_eq!(transpositions(&SIGMA), 9);
        assert_eq!(signature(&SIGMA), -1);
        assert_eq!(
            cycles_to_string(&perm_pow(&SIGMA, 18723)),
            "(1, 10, 6, 4)(3, 7, 11, 9, 8)"
        );
        assert_eq!(
            cycles_to_string(&inverse_perm(&SIGMA)),
            "(0, 2, 12)(1, 10, 6, 4)(3, 9, 7, 8, 11)"
        );
    }

    #[test]
    fn ordres_possibles_sur_treize_elements() {
        assert!(min_elements_for_order(30) <= 13);
        assert!(min_elements_for_order(26) > 13);
        assert_eq!(min_elements_for_order(60), 12);
    }

    #[test]
    fn restes_des_annales() {
        assert_eq!(pow_mod(4007, 1235, 13), 9);
        assert_eq!(pow_mod(3044, 3044, 13), 9);
    }

    #[test]
    fn bezout_de_l_examen_2025() {
        assert_eq!(bezout(124, 47), (1, 11, -29));
        let (g, u, v) = bezout(84, 36);
        assert_eq!(g, 12);
        assert_eq!(84 * u + 36 * v, 12);
    }

    #[test]
    fn denombrement() {
        assert_eq!(binom(20, 3), 1140);
        assert_eq!(arrangements(10, 3), 720);
        assert_eq!(factorial(5), 120);
        assert_eq!(binom(5, 7), 0);
    }

    #[test]
    fn inverses_modulo_12() {
        let inv: Vec<i64> = (0..12).filter(|a| inverse_mod(*a, 12).is_some()).collect();
        assert_eq!(inv, vec![1, 5, 7, 11]);
        assert_eq!(inverse_mod(34, 101).map(|x| 34 * x % 101), Some(1));
    }

    #[test]
    fn division_euclidienne_des_negatifs() {
        assert_eq!((-17i64).div_euclid(5), -4);
        assert_eq!((-17i64).rem_euclid(5), 3);
    }
}
