# Outils du port

Le contenu des pages n'a pas été retapé : il a été transposé mécaniquement
depuis `../../classeur-informatique.html`, ce qui garantit qu'aucun mot,
qu'aucune formule et qu'aucun espace insécable n'a bougé.

| Fichier | Rôle |
| --- | --- |
| `htmltree.py` | Mini arbre DOM et transpileur HTML → syntaxe `view!` de Leptos |
| `generate.py` | Écrit `src/pages/*.rs` : sections, sommaires, index de recherche |
| `compare.py` | Vérifie que le texte rendu est identique à celui de l'original |

```powershell
cd tools
python generate.py     # regénère src/pages/*.rs
python compare.py      # doit afficher « ECARTS : 0 »
cd ..
cargo fmt
```

`generate.py` s'appuie sur les numéros de ligne des cinq panneaux du fichier
d'origine (constante `PANES`) : si ce fichier est modifié, ces bornes sont à
reprendre.

Deux points de vigilance encodés dans `htmltree.py` :

- les blancs sont repliés comme le fait un navigateur, sauf dans les blocs que
  la feuille de style fige (`<pre>`, `.f`, `.ex-body .mono`), recopiés au
  caractère près — sans quoi les lignes de calcul se collent bout à bout ;
- `tachys` n'implémente les tuples que jusqu'à 26 éléments : au-delà, les
  enfants d'un élément sont regroupés dans des fragments `<>…</>`.
