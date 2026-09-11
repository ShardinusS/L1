# Classeur d'informatique — L1

Les cinq matières du semestre réunies dans un site statique : HTML, CSS et
JavaScript, sans framework, sans build, sans dépendance. Le dépôt *est* le
site — GitHub Pages le sert tel quel.

## Ce qu'il y a dedans

| Page | Contenu |
| --- | --- |
| `index.html` | Accueil : les cinq matières, les révisions, les documents dépouillés |
| `structures-fondamentales.html` | Logique, ensembles, applications, dénombrement, nombres, complexes, structures algébriques, 4 TD, annales et corrigés |
| `methodes-calcul.html` | Suites et limites, récurrence, fonctions, dérivées, primitives et intégrales, 3 TD, formulaire |
| `algorithmique.html` | Algorithme, cycle de développement, types et variables, TD série 1 |
| `information.html` | Binaire, numération, conversions (avec convertisseur), entiers non signés |
| `systemes.html` | Linux en ligne de commande, aide-mémoire |
| `prompts.html` | 35 prompts de révision, un par chapitre, à copier dans un assistant |
| `entrainement.html` | Séries d'exercices corrigés automatiquement |

## Consulter le site

Ouvrir `index.html` suffit pour lire les fiches. Pour que la recherche et
l'entraînement fonctionnent, il faut un serveur HTTP (les navigateurs
interdisent `fetch` depuis `file://`) :

```bash
python -m http.server 8000
# puis http://localhost:8000
```

Tous les liens sont **relatifs** : le site marche à la racine d'un domaine, dans
un sous-dossier (`/L1/` sur GitHub Pages), sur une clé USB derrière un serveur
local, ou dans n'importe quel hébergeur statique. Rien à configurer.

## Publier sur GitHub Pages

Dans *Settings → Pages*, choisir **Deploy from a branch**, branche `main`,
dossier `/ (root)`. Il n'y a rien à construire : les fichiers poussés sont ceux
qui sont servis. Le `.nojekyll` à la racine évite que GitHub passe le site à
Jekyll, et `404.html` sert de page d'erreur.

## Organisation

```
index.html, *.html          les pages, contenu inclus
assets/css/                 les feuilles de style, une par domaine
assets/js/app.js            thème, recherche, sommaire, convertisseur, copie des prompts
assets/js/practice.js       l'entraînement : tirage, correction, statistiques
assets/data/*.json          index de recherche, prompts, banques d'exercices
sw.js                       consultation hors ligne
tools/check.mjs             vérification des liens et des données
```

### Les données

Quatre fichiers JSON, chargés seulement par les pages qui en ont besoin :

| Fichier | Rôle |
| --- | --- |
| `search.json` | 348 entrées d'index, chacune pointant vers une ancre de section |
| `prompts.json` | les 35 prompts, leur niveau et leur calibrage |
| `practice.json` | 33 chapitres d'entraînement et leurs 285 questions rédigées |
| `generated.json` | 1 634 exercices pré-tirés, pour que chaque série soit différente |

`generated.json` pèse environ 530 Ko et n'est chargé que par
`entrainement.html` : les pages de cours restent légères.

## Vérifier

```bash
node tools/check.mjs
```

Contrôle que chaque lien interne vise un fichier existant, que chaque ancre
citée existe réellement dans la page visée, que les JSON sont lisibles et
cohérents entre eux, et que chaque page charge bien ses feuilles de style et ses
scripts. La CI GitHub lance cette même commande à chaque poussée.

## Modifier

Le contenu des cours est écrit directement dans les fichiers `.html` : pour
corriger une fiche, on édite la page et c'est tout.

Pour ajouter une section, lui donner un `id`, l'ajouter au sommaire (`nav.toc`)
de la page, et — si elle doit être trouvable — ajouter une entrée dans
`assets/data/search.json`. `node tools/check.mjs` signale les oublis.

Pour ajouter une question d'entraînement, l'écrire dans le chapitre voulu de
`assets/data/practice.json`. Pour un QCM (`"kind": "choice"`), la **première**
option du tableau est la bonne : l'ordre est mélangé à l'affichage.

## Prompts de révision

`prompts.html` réunit un prompt sur mesure par chapitre, à coller dans un
assistant. Chacun borne explicitement le programme du chapitre et porte un
niveau — `points faciles`, `cœur du barème`, `exigeant`, `jamais tombé` — déduit
du dépouillement des cinq sujets de Structures fondamentales disponibles depuis
la réforme : les deux partiels, les deux examens finals et le rattrapage. Pour
les matières sans annales, le calibrage vient du volume que les TD consacrent à
la notion.

Deux chapitres portent la mention `jamais tombé` : **dénombrement** et **nombres
complexes**, absents des cinq sujets alors que le TD 3 est entièrement consacré
aux seconds. Leur prompt le dit en ouverture et borne la séance.

## Raccourcis

| Touche | Effet |
| --- | --- |
| `/` | Focus sur la recherche |
| `↑` `↓` | Parcourir les résultats |
| `Entrée` | Ouvrir le résultat |
| `Échap` | Fermer la recherche |
| `1` `2` `3`… | Répondre à un QCM d'entraînement |

## Historique

Ce classeur a d'abord existé en Rust (Leptos compilé en WebAssembly). Cette
version reste consultable dans l'historique git ; elle a été remplacée par le
site statique, plus rapide à charger et déployable sans build.

---

**Sources.** Structures fondamentales et Méthodes et techniques de calcul —
cours, TD, sujets et corrigés de Fabien Durand, UPJV. Algorithmique 1 — cours de
Jordan Caracotte et Léo Robert. Représentation de l'information et Systèmes
d'exploitation — notes de cours personnelles.
