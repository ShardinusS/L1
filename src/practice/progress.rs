//! Statistiques d'entraînement : réponses données et réussies, par chapitre.
//!
//! Conservées dans `localStorage` sous une forme texte lisible, une ligne
//! par chapitre : `sf/permutations 12 9`. Une ligne illisible est ignorée
//! plutôt que de faire perdre tout l'historique.

use std::collections::BTreeMap;

use super::{chapters, Chapter};
use crate::course::{Course, COURSES};

const STORAGE_KEY: &str = "classeur-entrainement";

/// Nombre de réponses en dessous duquel un taux n'est pas significatif.
const MIN_ANSWERS: u32 = 3;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Tally {
    pub asked: u32,
    pub correct: u32,
}

impl Tally {
    /// Taux de réussite en pourcentage arrondi; `None` sans réponse.
    pub fn rate(self) -> Option<u32> {
        (self.asked > 0).then(|| (self.correct * 100 + self.asked / 2) / self.asked)
    }

    fn add(&mut self, other: Tally) {
        self.asked += other.asked;
        self.correct += other.correct;
    }
}

/// « 1 réponse », « 12 réponses ».
pub fn answers_label(n: u32) -> String {
    if n > 1 {
        format!("{n} réponses")
    } else {
        format!("{n} réponse")
    }
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Progress(BTreeMap<String, Tally>);

fn key(course: Course, chapter: &str) -> String {
    format!("{}/{chapter}", course.key())
}

impl Progress {
    pub fn record(&mut self, course: Course, chapter: &str, correct: bool) {
        let t = self.0.entry(key(course, chapter)).or_default();
        t.asked += 1;
        if correct {
            t.correct += 1;
        }
    }

    pub fn chapter(&self, course: Course, chapter: &str) -> Tally {
        self.0
            .get(&key(course, chapter))
            .copied()
            .unwrap_or_default()
    }

    pub fn course(&self, course: Course) -> Tally {
        let prefix = format!("{}/", course.key());
        let mut total = Tally::default();
        for (_, t) in self.0.iter().filter(|(k, _)| k.starts_with(&prefix)) {
            total.add(*t);
        }
        total
    }

    pub fn total(&self) -> Tally {
        let mut total = Tally::default();
        for course in COURSES {
            total.add(self.course(course));
        }
        total
    }

    /// Chapitre au plus faible taux de réussite, parmi ceux qui ont assez de
    /// réponses pour que le taux veuille dire quelque chose.
    pub fn weakest(&self, course: Course) -> Option<(&'static Chapter, u32)> {
        chapters(course)
            .iter()
            .filter_map(|ch| {
                let t = self.chapter(course, ch.id);
                (t.asked >= MIN_ANSWERS).then(|| (ch, t.rate().unwrap_or(0)))
            })
            .min_by_key(|(_, rate)| *rate)
    }

    /// Efface les statistiques d'une matière, ou de toutes.
    pub fn reset(&mut self, course: Option<Course>) {
        match course {
            Some(c) => {
                let prefix = format!("{}/", c.key());
                self.0.retain(|k, _| !k.starts_with(&prefix));
            }
            None => self.0.clear(),
        }
    }

    pub fn serialize(&self) -> String {
        self.0
            .iter()
            .map(|(k, t)| format!("{k} {} {}", t.asked, t.correct))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn parse(raw: &str) -> Self {
        let mut map = BTreeMap::new();
        for line in raw.lines() {
            let mut parts = line.split_whitespace();
            let (Some(k), Some(a), Some(c), None) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            else {
                continue;
            };
            let (Ok(asked), Ok(correct)) = (a.parse::<u32>(), c.parse::<u32>()) else {
                continue;
            };
            if k.contains('/') && correct <= asked {
                map.insert(k.to_string(), Tally { asked, correct });
            }
        }
        Self(map)
    }

    pub fn load() -> Self {
        storage()
            .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
            .map(|raw| Self::parse(&raw))
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(s) = storage() {
            let _ = s.set_item(STORAGE_KEY, &self.serialize());
        }
    }
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enregistrement_et_totaux() {
        let mut p = Progress::default();
        p.record(Course::Sf, "logique", true);
        p.record(Course::Sf, "logique", false);
        p.record(Course::Sf, "permutations", true);
        p.record(Course::Os, "aide", true);
        assert_eq!(
            p.chapter(Course::Sf, "logique"),
            Tally {
                asked: 2,
                correct: 1
            }
        );
        assert_eq!(
            p.course(Course::Sf),
            Tally {
                asked: 3,
                correct: 2
            }
        );
        assert_eq!(
            p.total(),
            Tally {
                asked: 4,
                correct: 3
            }
        );
        assert_eq!(p.course(Course::Algo).rate(), None);
        assert_eq!(p.course(Course::Sf).rate(), Some(67));
    }

    #[test]
    fn relu_a_l_identique() {
        let mut p = Progress::default();
        for i in 0..7 {
            p.record(Course::Info, "conversion", i % 3 != 0);
        }
        p.record(Course::Algo, "traces", false);
        assert_eq!(Progress::parse(&p.serialize()), p);
    }

    #[test]
    fn lignes_illisibles_ignorees() {
        let p = Progress::parse(
            "sf/logique 4 3\nn'importe quoi\nsf/ensembles 2 5\nos/aide x 1\ninfo/entiers 3 1 9",
        );
        assert_eq!(
            p.chapter(Course::Sf, "logique"),
            Tally {
                asked: 4,
                correct: 3
            }
        );
        assert_eq!(
            p.total(),
            Tally {
                asked: 4,
                correct: 3
            }
        );
    }

    #[test]
    fn point_faible() {
        let mut p = Progress::default();
        assert!(p.weakest(Course::Sf).is_none());
        for _ in 0..4 {
            p.record(Course::Sf, "logique", true);
            p.record(Course::Sf, "permutations", false);
        }
        p.record(Course::Sf, "complexes", false);
        let (ch, rate) = p.weakest(Course::Sf).unwrap();
        assert_eq!((ch.id, rate), ("permutations", 0));
    }

    #[test]
    fn remise_a_zero_d_une_matiere() {
        let mut p = Progress::default();
        p.record(Course::Sf, "logique", true);
        p.record(Course::Os, "aide", true);
        p.reset(Some(Course::Sf));
        assert_eq!(p.course(Course::Sf).asked, 0);
        assert_eq!(p.course(Course::Os).asked, 1);
        p.reset(None);
        assert_eq!(p.total().asked, 0);
    }
}
