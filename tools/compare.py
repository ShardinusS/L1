# -*- coding: utf-8 -*-
"""Compare le texte de chaque section d'origine a celui du module Rust genere.

Deux controles :
  1. texte complet, blancs replies  -> rien ne doit disparaitre ni apparaitre ;
  2. blocs a blancs figes (<pre>, .f, .ex-body .mono) -> egalite exacte.

Les sous-arbres remplaces par un composant (<Converter/>, <CmdTable/>) sont
exclus des deux controles : leur contenu est ecrit a la main.
"""
import io, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from htmltree import parse, collapse, preserves_whitespace, WS
import generate as G

RS_DIR = G.OUT

STRING_RE = re.compile(r'"((?:[^"\\]|\\.)*)"')


def unescape(s):
    out, i = [], 0
    while i < len(s):
        c = s[i]
        if c != "\\":
            out.append(c)
            i += 1
            continue
        n = s[i + 1]
        if n == "u":
            j = s.index("}", i)
            out.append(chr(int(s[i + 3:j], 16)))
            i = j + 1
        else:
            out.append({"n": "\n", "t": "\t", "r": "\r", "\\": "\\", '"': '"'}[n])
            i += 2
    return "".join(out)


def rust_text_nodes(path):
    """Litteraux de texte des view!, dans l'ordre; les attributs sont ecartes."""
    texts = []
    for line in io.open(path, encoding="utf-8"):
        stripped = line.strip()
        # Une ligne de texte est un litteral seul; un attribut vit dans une balise.
        if not stripped.startswith('"'):
            continue
        m = STRING_RE.fullmatch(stripped)
        if not m:
            continue
        texts.append(unescape(m.group(1)))
    return texts


def replaced(node):
    return G.replacements(node) is not None or G.drop(node)


def visible_text(node):
    """textContent, en sautant les sous-arbres rendus par un composant."""
    if node.tag is None:
        return node.text or ""
    if node.tag and replaced(node):
        return ""
    return "".join(visible_text(k) for k in node.kids)


def frozen_blocks(node, inside=False):
    """Textes des blocs a blancs figes, non imbriques."""
    out = []
    if node.tag and replaced(node):
        return out
    if node.tag and not inside and preserves_whitespace(node):
        out.append(visible_text(node))
        inside = True
    for k in node.kids:
        if k.tag:
            out.extend(frozen_blocks(k, inside))
    return out


def norm_all(s):
    """Compare le contenu, pas la mise en forme : tous les blancs sautent.

    Les blancs entre deux blocs ne se voient pas au rendu; ceux qui comptent
    (blocs figes) sont verifies exactement par le second controle."""
    return "".join(ch for ch in s if ch not in WS)


def main():
    problems = 0
    for key, pane_key, a, b, _title in G.PANES:
        root = parse(G.fragment(a, b))
        pane = root.first(lambda n: n.get("id") == "pane-" + pane_key)
        G.rewrite_tree(pane, key)
        header = pane.first(lambda n: n.tag == "header")
        main_el = pane.first(lambda n: n.tag == "main")
        sections = [n for n in main_el.kids if n.tag == "section"]

        expected = norm_all(visible_text(header) + "".join(visible_text(s) for s in sections))
        actual = norm_all("".join(rust_text_nodes(os.path.join(RS_DIR, key + ".rs"))))

        if expected == actual:
            print(f"{key:5s} texte  OK  ({len(expected)} caracteres)")
        else:
            problems += 1
            print(f"{key:5s} texte  ECART")
            i = next((i for i in range(min(len(expected), len(actual)))
                      if expected[i] != actual[i]), min(len(expected), len(actual)))
            print("   attendu :", repr(expected[max(0, i - 60):i + 60]))
            print("   obtenu  :", repr(actual[max(0, i - 60):i + 60]))

        # blocs figes
        blocks = []
        for s in [header] + sections:
            blocks.extend(frozen_blocks(s))
        rust_join = "".join(rust_text_nodes(os.path.join(RS_DIR, key + ".rs")))
        missing = [blk for blk in blocks if blk and blk not in rust_join]
        if missing:
            problems += 1
            print(f"{key:5s} blocs  {len(missing)} bloc(s) fige(s) alteres")
            for blk in missing[:2]:
                print("   ", repr(blk[:160]))
        else:
            print(f"{key:5s} blocs  OK  ({len(blocks)} blocs figes)")

    print("\nECARTS :", problems)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
