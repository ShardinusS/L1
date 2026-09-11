//! Logique du convertisseur de bases (matière 03), sans dépendance au DOM.

/// Chiffres utilisables, dans l'ordre : l'alphabet de la base `b` est
/// `DIGITS[..b]`.
const DIGITS: &str = "0123456789ABCDEF";

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Conversion {
    pub bin: String,
    pub oct: String,
    pub dec: String,
    pub hex: String,
    pub message: String,
    /// Vrai lorsque le message doit être affiché en rouge (`.msg.warn`).
    pub warn: bool,
}

impl Conversion {
    /// Sorties vides, affichées comme des tirets cadratins.
    fn empty(message: impl Into<String>, warn: bool) -> Self {
        Self {
            bin: "—".into(),
            oct: "—".into(),
            dec: "—".into(),
            hex: "—".into(),
            message: message.into(),
            warn,
        }
    }
}

/// Alphabet de la base sous la forme affichée dans les messages d'erreur.
pub fn alphabet(base: u32) -> String {
    DIGITS
        .chars()
        .take(base as usize)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Convertit `input` depuis `base` vers les quatre bases affichées.
///
/// Reprend les messages et les cas limites de la version JavaScript : entrée
/// vide, chiffre impossible dans la base choisie, nombre hors capacité, et le
/// rappel sur la tenue en un octet.
pub fn convert(input: &str, base: u32) -> Conversion {
    let raw: String = input
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_uppercase())
        .collect();

    if raw.is_empty() {
        return Conversion::empty("Saisir un nombre entier positif.", false);
    }

    let allowed = &DIGITS[..base as usize];
    if let Some(bad) = raw.chars().find(|c| !allowed.contains(*c)) {
        return Conversion::empty(
            format!(
                "Chiffre « {bad} » impossible en base {base} : C = {{{}}}.",
                alphabet(base)
            ),
            true,
        );
    }

    let Ok(n) = u64::from_str_radix(&raw, base) else {
        return Conversion::empty("Nombre trop grand pour être converti ici.", true);
    };

    let bin = format!("{n:b}");
    let message = if n <= 255 {
        format!("Tient sur 1 octet (8 bits) : {:0>8}.", bin)
    } else {
        format!(
            "Dépasse la capacité d\u{2019}un octet (max 255) : il faut {} bits.",
            bin.len()
        )
    };

    Conversion {
        oct: format!("{n:o}"),
        dec: format!("{n}"),
        hex: format!("{n:X}"),
        warn: n > 255,
        bin,
        message,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hexadecimal_ff() {
        let c = convert("FF", 16);
        assert_eq!(c.dec, "255");
        assert_eq!(c.bin, "11111111");
        assert_eq!(c.oct, "377");
        assert_eq!(c.hex, "FF");
        assert!(!c.warn);
        assert_eq!(c.message, "Tient sur 1 octet (8 bits) : 11111111.");
    }

    #[test]
    fn binaire_1010() {
        let c = convert("1010", 2);
        assert_eq!(c.dec, "10");
        assert_eq!(c.hex, "A");
        assert_eq!(c.message, "Tient sur 1 octet (8 bits) : 00001010.");
    }

    #[test]
    fn entree_vide() {
        let c = convert("   ", 10);
        assert_eq!(c.dec, "—");
        assert!(!c.warn);
        assert_eq!(c.message, "Saisir un nombre entier positif.");
    }

    #[test]
    fn chiffre_impossible_dans_la_base() {
        let c = convert("2", 2);
        assert!(c.warn);
        assert_eq!(c.bin, "—");
        assert_eq!(
            c.message,
            "Chiffre « 2 » impossible en base 2 : C = {0, 1}."
        );
    }

    #[test]
    fn alphabet_de_la_base_16() {
        assert_eq!(
            alphabet(16),
            "0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, F"
        );
        assert_eq!(alphabet(8), "0, 1, 2, 3, 4, 5, 6, 7");
    }

    #[test]
    fn depassement_d_un_octet() {
        let c = convert("256", 10);
        assert!(c.warn);
        assert_eq!(c.bin, "100000000");
        assert_eq!(
            c.message,
            "Dépasse la capacité d\u{2019}un octet (max 255) : il faut 9 bits."
        );
    }

    #[test]
    fn minuscules_et_espaces_acceptes() {
        assert_eq!(convert(" ff ", 16).dec, "255");
        assert_eq!(convert("1111 1111", 2).dec, "255");
        assert_eq!(convert("dead", 16).dec, "57005");
    }

    #[test]
    fn nombre_trop_grand() {
        let c = convert(&"F".repeat(20), 16);
        assert!(c.warn);
        assert_eq!(c.dec, "—");
        assert_eq!(c.message, "Nombre trop grand pour être converti ici.");
    }

    #[test]
    fn zero_tient_sur_un_octet() {
        let c = convert("0", 10);
        assert_eq!(c.bin, "0");
        assert_eq!(c.message, "Tient sur 1 octet (8 bits) : 00000000.");
    }
}
