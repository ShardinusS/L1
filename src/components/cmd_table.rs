//! Aide-mémoire filtrable de la matière 04.
//!
//! Reprend le tableau `#recap` et son filtre : la recherche porte sur la ligne
//! entière (commande, rôle, chapitre), accents ignorés, comme dans l'original.

use leptos::prelude::*;

use crate::search::norm;

pub struct Cmd {
    pub name: &'static str,
    pub role: &'static str,
    pub chapter: &'static str,
    /// Commandes destructrices, affichées en rouge.
    danger: bool,
}

/// Les 32 commandes du cours, dans l'ordre des chapitres. L'entraînement
/// s'en sert aussi pour ses questions « quelle commande pour… ».
///
/// Une ligne par commande : le tableau se relit plus vite ainsi qu'éclaté
/// par rustfmt, d'où le `skip`.
#[rustfmt::skip]
pub const COMMANDS: &[Cmd] = &[
    Cmd { name: "pwd", role: "Emplacement courant dans l'arborescence", chapter: "Se déplacer", danger: false },
    Cmd { name: "ls", role: "Lister les fichiers du répertoire", chapter: "Se déplacer", danger: false },
    Cmd { name: "ls -l", role: "Lister avec les détails (d = dossier, - = fichier)", chapter: "Se déplacer", danger: false },
    Cmd { name: "ls -a", role: "Afficher aussi les fichiers cachés (commençant par .)", chapter: "Se déplacer", danger: false },
    Cmd { name: "cd", role: "Changer de répertoire", chapter: "Se déplacer", danger: false },
    Cmd { name: "cd ..", role: "Remonter dans le répertoire père", chapter: "Se déplacer", danger: false },
    Cmd { name: "file", role: "Déterminer le type d'un fichier", chapter: "Se déplacer", danger: false },
    Cmd { name: "mkdir", role: "Créer un répertoire", chapter: "Créer", danger: false },
    Cmd { name: "touch", role: "Créer un fichier vide", chapter: "Créer", danger: false },
    Cmd { name: "nano", role: "Éditer un fichier", chapter: "Créer", danger: false },
    Cmd { name: "> fichier", role: "Rediriger la sortie en écrasant le fichier", chapter: "Créer", danger: false },
    Cmd { name: ">> fichier", role: "Rediriger la sortie en ajoutant à la fin", chapter: "Créer", danger: false },
    Cmd { name: "cat", role: "Voir le contenu d'un fichier", chapter: "Lire", danger: false },
    Cmd { name: "less", role: "Afficher un fichier page par page", chapter: "Lire", danger: false },
    Cmd { name: "head", role: "Afficher le début d'un fichier", chapter: "Lire", danger: false },
    Cmd { name: "tail", role: "Afficher la fin d'un fichier", chapter: "Lire", danger: false },
    Cmd { name: "head -n 3", role: "Les 3 premières lignes (tail -n 3 : les 3 dernières)", chapter: "Lire", danger: false },
    Cmd { name: "wc", role: "Compter les lignes, les mots, les caractères", chapter: "Lire", danger: false },
    Cmd { name: "cp source destination", role: "Copier un fichier", chapter: "Gérer", danger: false },
    Cmd { name: "mv", role: "Renommer ou déplacer un fichier", chapter: "Gérer", danger: false },
    Cmd { name: "rm", role: "Supprimer un fichier (pas de corbeille)", chapter: "Gérer", danger: true },
    Cmd { name: "rmdir", role: "Supprimer un répertoire vide", chapter: "Gérer", danger: true },
    Cmd { name: "man commande", role: "Manuel d'une commande", chapter: "Aide", danger: false },
    Cmd { name: "apropos", role: "Trouver la commande qui fait quelque chose", chapter: "Aide", danger: false },
    Cmd { name: "cut", role: "Sélectionner des colonnes ou des caractères", chapter: "Composer", danger: false },
    Cmd { name: "paste", role: "Assembler", chapter: "Composer", danger: false },
    Cmd { name: "sort", role: "Trier", chapter: "Composer", danger: false },
    Cmd { name: "sort -r", role: "Trier dans le sens inverse", chapter: "Composer", danger: false },
    Cmd { name: "sort -n", role: "Trier de manière numérique", chapter: "Composer", danger: false },
    Cmd { name: "uniq", role: "Ne garder qu'un exemplaire des lignes identiques consécutives", chapter: "Composer", danger: false },
    Cmd { name: "|", role: "Pipe : combiner deux commandes", chapter: "Composer", danger: false },
    Cmd { name: "echo", role: "Répéter le texte écrit", chapter: "Base", danger: false },
];

/// Vrai si la ligne correspond à la requête (requête vide : tout passe).
fn matches(cmd: &Cmd, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let haystack = norm(&format!("{}{}{}", cmd.name, cmd.role, cmd.chapter));
    haystack.contains(needle)
}

#[component]
pub fn CmdTable() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let needle = Memo::new(move |_| norm(query.get().trim()));
    let visible =
        Memo::new(move |_| needle.with(|n| COMMANDS.iter().filter(|c| matches(c, n)).count()));

    view! {
        <div class="search">
            <label for="q">"Filtrer les commandes"</label>
            <input
                id="q"
                type="search"
                placeholder="ex. : trier, supprimer, ls…"
                autocomplete="off"
                spellcheck="false"
                prop:value=move || query.get()
                on:input=move |ev| query.set(event_target_value(&ev))
            />
        </div>
        <div class="tw">
            <table id="recap">
                <thead>
                    <tr>
                        <th>"Commande"</th>
                        <th>"Rôle"</th>
                        <th>"Chapitre"</th>
                    </tr>
                </thead>
                <tbody>
                    {COMMANDS
                        .iter()
                        .map(|cmd| {
                            view! {
                                <tr
                                    data-danger=cmd.danger.then_some("")
                                    hidden=move || needle.with(|n| !matches(cmd, n))
                                >
                                    <td>{cmd.name}</td>
                                    <td>{cmd.role}</td>
                                    <td class="cat">{cmd.chapter}</td>
                                </tr>
                            }
                        })
                        .collect_view()}
                </tbody>
            </table>
            <p class="empty" id="empty" hidden=move || visible.get() > 0>
                "Aucune commande ne correspond."
            </p>
        </div>
    }
}
