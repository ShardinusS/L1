//! Les cinq matières du classeur : identité, route et libellés.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Course {
    Sf,
    Mtc,
    Algo,
    Info,
    Os,
}

pub const COURSES: [Course; 5] = [
    Course::Sf,
    Course::Mtc,
    Course::Algo,
    Course::Info,
    Course::Os,
];

impl Course {
    /// Chemin de la route, sans barre oblique finale.
    pub const fn route(self) -> &'static str {
        match self {
            Course::Sf => "/structures-fondamentales",
            Course::Mtc => "/methodes-calcul",
            Course::Algo => "/algorithmique",
            Course::Info => "/information",
            Course::Os => "/systemes",
        }
    }

    /// Nom complet, tel qu'il apparaît dans les onglets et les sous-titres.
    pub const fn label(self) -> &'static str {
        match self {
            Course::Sf => "Structures fondamentales",
            Course::Mtc => "Méthodes et techniques de calcul",
            Course::Algo => "Algorithmique 1",
            Course::Info => "Représentation de l'information",
            Course::Os => "Systèmes d'exploitation",
        }
    }

    /// Nom court affiché dans la pastille des résultats de recherche.
    pub const fn short(self) -> &'static str {
        match self {
            Course::Sf => "Maths",
            Course::Mtc => "Calcul",
            Course::Algo => "Algo",
            Course::Info => "Information",
            Course::Os => "Systèmes",
        }
    }

    /// Suffixe des classes CSS : `.badge.sf`, `.tab-sf`, `.c-sf`…
    pub const fn key(self) -> &'static str {
        match self {
            Course::Sf => "sf",
            Course::Mtc => "mtc",
            Course::Algo => "algo",
            Course::Info => "info",
            Course::Os => "os",
        }
    }

    /// Route de l'entraînement de la matière : `/entrainement/systemes`.
    pub fn practice_route(self) -> String {
        format!("/entrainement{}", self.route())
    }

    /// Matière dont la route vaut `/{slug}`.
    pub fn from_slug(slug: &str) -> Option<Course> {
        COURSES
            .into_iter()
            .find(|c| c.route().trim_start_matches('/') == slug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_d_entrainement() {
        assert_eq!(Course::Os.practice_route(), "/entrainement/systemes");
        for c in COURSES {
            let slug = c.practice_route().rsplit('/').next().unwrap().to_string();
            assert_eq!(Course::from_slug(&slug), Some(c));
        }
        assert_eq!(Course::from_slug("melange"), None);
    }
}
