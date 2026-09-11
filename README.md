# Classeur d'informatique — L1

Les cinq matières du semestre réunies dans une application Leptos rendue côté
client (Rust compilé en WebAssembly), buildée par Trunk. La sortie est un
dossier statique : rien à faire tourner côté serveur.

Le fichier d'origine `../classeur-informatique.html` est conservé tel quel comme
référence ; l'application reprend son contenu à l'identique.

## Prérequis

```powershell
rustup default stable
rustup target add wasm32-unknown-unknown
rustup component add clippy rustfmt
cargo install trunk --locked
```

## Développer

```powershell
trunk serve --open        # http://127.0.0.1:8080, rechargement à chaud
```

## Vérifier

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Les tests tournent sur la machine hôte, sans navigateur : ils couvrent le
convertisseur de bases, le moteur de recherche, la cohérence de l'index et des
sommaires, et la redirection des anciens liens.

## Publier

```powershell
trunk build --release
```

Le résultat est dans `dist/`. Il faut le servir en HTTP : un navigateur refuse
de charger un module WebAssembly depuis `file://`. N'importe quel serveur
statique convient.

Comme l'application utilise de vraies routes (`/systemes`, `/information`…),
l'hébergeur doit renvoyer `index.html` pour les chemins inconnus. Le script
`scripts/post-build.ps1`, appelé par Trunk après chaque build, écrit déjà
`dist/404.html` — ce qui suffit à GitHub Pages. En release, ce même script
passe le bundle WebAssembly à `wasm-opt -Oz` (environ 1,3 Mo → 700 Ko).

## Organisation

| Chemin | Rôle |
| --- | --- |
| `index.html` | Coquille Trunk : polices, métadonnées, feuilles de style |
| `style/` | Le CSS d'origine, découpé, plus `print.css` |
| `src/app.rs` | Routes, redirection des anciens liens, défilement vers l'ancre |
| `src/course.rs` | Les cinq matières : route, libellés, classes |
| `src/search/` | Index typé et moteur de recherche |
| `src/convert.rs` | Convertisseur de bases (logique pure, testée) |
| `src/scroll.rs` | Défilement vers une section et surbrillance |
| `src/theme.rs` | Thème clair / sombre / automatique |
| `src/components/` | Barre supérieure, recherche, sommaire, convertisseur, aide-mémoire |
| `src/pages/` | Une page par matière, plus l'accueil, l'entraînement et les prompts |
| `src/practice/` | Entraînement : banques d'exercices, générateurs, correction, statistiques |
| `src/prompts/` | Un prompt de révision par chapitre, calibré sur les annales |
| `src/components/quiz.rs` | Séance d'entraînement (série, correction, bilan) |
| `public/` | Manifeste, icône, service worker |
| `tools/` | Transpileur HTML → Leptos et contrôle de fidélité (voir `tools/README.md`) |

### Les pages sont transposées, pas réécrites

Les modules de `src/pages/` reprennent le balisage du classeur HTML : mêmes
textes, mêmes classes CSS, mêmes ancres. Chaque page déclare aussi :

- `SECTIONS` — les ancres de ses sections ;
- `TOC` — son sommaire ;
- `INDEX` — ses entrées de recherche.

C'est le changement de fond par rapport à la version HTML, qui reconstruisait
son index en parcourant le DOM au chargement : l'index est maintenant écrit à
côté du contenu, et des tests vérifient qu'aucune entrée ne pointe vers une
ancre inexistante.

Pour ajouter un chapitre : écrire la fonction de section dans le module de la
matière, ajouter son ancre à `SECTIONS`, sa ligne au `TOC`, et ses entrées à
`INDEX`. `cargo test` signale tout oubli.

## Entraînement

`/entrainement` propose des séries d'exercices corrigés, une page par matière
(`/entrainement/systemes`…) et une série mélangée (`/entrainement/melange`).
Chaque chapitre d'entraînement renvoie à une section du cours; il réunit des
questions rédigées et des générateurs qui tirent de nouvelles valeurs à chaque
série (conversions, traces, restes modulo, permutations…). Les statistiques
restent dans le `localStorage` du navigateur.

Pour ajouter une question : l'écrire dans le tableau du chapitre, dans
`src/practice/<matière>.rs` (la première option d'un QCM est la bonne).
`cargo test` vérifie que chaque chapitre vise une ancre existante et que la
réponse attendue de chaque générateur passe le correcteur, sur 400 tirages.

## Prompts de révision

`/prompts` réunit un prompt sur mesure par chapitre des cinq matières, à copier
dans un assistant. Chaque prompt borne explicitement le programme du chapitre,
et porte un niveau (`points faciles`, `cœur du barème`, `exigeant`,
`jamais tombé`) déduit du dépouillement des cinq sujets de Structures
fondamentales disponibles depuis la réforme — les deux partiels, les deux
examens finals et le rattrapage — ou, pour les matières sans annales, du volume
que les TD consacrent à la notion.

Deux chapitres portent la mention `jamais tombé` : **dénombrement** et
**nombres complexes**, absents des cinq sujets alors que le TD 3 est entièrement
consacré aux seconds. Leur prompt le dit en ouverture et borne la séance.

Les prompts vivent dans `src/prompts/mod.rs`, une constante par matière. Le
numéro affiché n'y est pas stocké : il est lu dans le sommaire de la matière,
et `cargo test` vérifie que chaque prompt vise une section qui existe.

## Raccourcis

| Touche | Effet |
| --- | --- |
| `/` | Focus sur la recherche |
| `↑` `↓` | Parcourir les résultats |
| `Entrée` | Ouvrir le résultat |
| `Échap` | Fermer la recherche |
