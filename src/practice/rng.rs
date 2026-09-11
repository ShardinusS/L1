//! Générateur pseudo-aléatoire SplitMix64.
//!
//! Aucune dépendance : les tests natifs fixent la graine pour rejouer un
//! tirage, le navigateur l'initialise avec `Math.random()` et l'horloge.

#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    /// Graine fixe, pour rejouer un tirage dans les tests.
    #[cfg(test)]
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// Graine tirée du navigateur. Ne pas appeler hors WebAssembly.
    pub fn from_entropy() -> Self {
        let a = (js_sys::Math::random() * 9_007_199_254_740_992.0) as u64;
        let b = js_sys::Date::now() as u64;
        Self(a ^ b.rotate_left(32))
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Entier uniforme dans `0..n` (`n > 0`).
    pub fn below(&mut self, n: usize) -> usize {
        debug_assert!(n > 0);
        (self.next_u64() % n as u64) as usize
    }

    /// Entier uniforme dans `lo..=hi`.
    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        debug_assert!(lo <= hi);
        lo + (self.next_u64() % (hi - lo + 1) as u64) as i64
    }

    pub fn coin(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }

    /// Mélange de Fisher-Yates.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
    }

    /// `k` éléments distincts de `0..n`, dans un ordre aléatoire.
    pub fn sample(&mut self, n: usize, k: usize) -> Vec<usize> {
        let mut all: Vec<usize> = (0..n).collect();
        self.shuffle(&mut all);
        all.truncate(k);
        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meme_graine_meme_suite() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn bornes_respectees() {
        let mut r = Rng::new(7);
        for _ in 0..10_000 {
            let x = r.range(-3, 5);
            assert!((-3..=5).contains(&x));
            assert!(r.below(4) < 4);
        }
    }

    #[test]
    fn le_melange_garde_les_elements() {
        let mut r = Rng::new(1);
        let mut v: Vec<u32> = (0..50).collect();
        r.shuffle(&mut v);
        v.sort_unstable();
        assert_eq!(v, (0..50).collect::<Vec<_>>());
    }
}
