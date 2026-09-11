# -*- coding: utf-8 -*-
"""Mini arbre DOM + transpileur vers la syntaxe view! de Leptos."""
from html.parser import HTMLParser

VOID = {"br", "hr", "img", "input", "link", "meta", "source", "area", "base", "col"}
# Espaces qui se replient en HTML : NBSP (\xa0) volontairement exclu.
WS = " \t\r\n\f"


class Node:
    __slots__ = ("tag", "attrs", "kids", "parent", "text")

    def __init__(self, tag, attrs=None, text=None, parent=None):
        self.tag = tag           # None => noeud texte
        self.attrs = attrs or []
        self.kids = []
        self.parent = parent
        self.text = text

    # --- helpers ---------------------------------------------------------
    def get(self, name):
        for k, v in self.attrs:
            if k == name:
                return v if v is not None else ""
        return None

    def classes(self):
        c = self.get("class")
        return c.split() if c else []

    def has_class(self, name):
        return name in self.classes()

    def walk(self):
        for k in self.kids:
            yield k
            yield from k.walk()

    def elements(self):
        for n in self.walk():
            if n.tag:
                yield n

    def text_content(self):
        if self.tag is None:
            return self.text or ""
        return "".join(k.text_content() for k in self.kids)

    def select(self, pred):
        return [n for n in self.elements() if pred(n)]

    def first(self, pred):
        for n in self.elements():
            if pred(n):
                return n
        return None

    def path(self):
        out, cur = [], self
        while cur is not None and cur.tag:
            cls = "." + ".".join(cur.classes()) if cur.classes() else ""
            out.append(cur.tag + cls)
            cur = cur.parent
        return " > ".join(reversed(out))


class Builder(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.root = Node("#root")
        self.cur = self.root

    def handle_starttag(self, tag, attrs):
        node = Node(tag, attrs, parent=self.cur)
        self.cur.kids.append(node)
        if tag not in VOID:
            self.cur = node

    def handle_startendtag(self, tag, attrs):
        self.cur.kids.append(Node(tag, attrs, parent=self.cur))

    def handle_endtag(self, tag):
        node = self.cur
        while node is not self.root and node.tag != tag:
            node = node.parent
        if node is not self.root:
            self.cur = node.parent

    def handle_data(self, data):
        self.cur.kids.append(Node(None, text=data, parent=self.cur))

    def handle_comment(self, data):
        pass


def parse(fragment):
    b = Builder()
    b.feed(fragment)
    b.close()
    return b.root


# --- emission Rust -------------------------------------------------------

def rust_str(s):
    out = []
    for ch in s:
        if ch == "\\":
            out.append("\\\\")
        elif ch == '"':
            out.append('\\"')
        elif ch == "\n":
            out.append(r"\n")
        elif ch == "\t":
            out.append(r"\t")
        elif ch == "\r":
            out.append(r"\r")
        elif ch == "\xa0":
            out.append(r"\u{a0}")
        elif ch == "\u202f":
            out.append(r"\u{202f}")
        elif ch == "\u200b":
            out.append(r"\u{200b}")
        else:
            out.append(ch)
    return '"' + "".join(out) + '"'


def collapse(text):
    """Replie les blancs comme le fait le rendu HTML, sans toucher au NBSP."""
    out, in_ws = [], False
    for ch in text:
        if ch in WS:
            in_ws = True
        else:
            if in_ws and out:
                out.append(" ")
            elif in_ws and not out:
                out.append(" ")
            in_ws = False
            out.append(ch)
    if in_ws:
        out.append(" ")
    return "".join(out)


MAX_KIDS = 20  # tachys implemente les tuples jusqu'a 26 elements


def preserves_whitespace(node):
    """Elements dont la feuille de style fige les blancs.

    <pre> par nature; .f est en `white-space:pre-wrap`; et `.ex-body .mono`
    est en `white-space:pre`. Replier les blancs dans ces blocs colle des
    lignes de calcul bout a bout : il faut recopier le texte tel quel.
    """
    if node.tag == "pre":
        return True
    classes = node.classes()
    if "f" in classes:
        return True
    if "mono" in classes:
        cur = node.parent
        while cur is not None and cur.tag:
            if cur.has_class("ex-body"):
                return True
            cur = cur.parent
    return False


class Emitter:
    def __init__(self, replacements=None, drop=None):
        # replacements: fn(node) -> str|None ; remplace un sous-arbre par du code Rust
        self.replacements = replacements or (lambda n: None)
        self.drop = drop or (lambda n: False)
        self.notes = []

    def emit(self, node, indent, pre=False):
        if node.tag is None:
            return self._text(node, indent, pre)
        rep = self.replacements(node)
        if rep is not None:
            return [" " * indent + rep]
        if self.drop(node):
            return []
        return self._element(node, indent, pre)

    def _text(self, node, indent, pre):
        raw = node.text or ""
        if pre:
            if not raw:
                return []
            return [" " * indent + rust_str(raw)]
        if raw.strip(WS) == "":
            if "\n" in raw:
                # blanc issu de l'indentation du source : sans effet au rendu
                self.notes.append(("ws-drop", node.parent.path() if node.parent else "?"))
                return []
            if raw == "":
                return []
            return [" " * indent + '" "']
        return [" " * indent + rust_str(collapse(raw))]

    # Proprietes de composants Leptos : la valeur est du Rust, pas une chaine.
    RAW_ATTRS = {"scroll", "exact", "strict_trailing_slash"}

    def _attrs(self, node):
        parts = []
        for k, v in node.attrs:
            if v is None:
                parts.append(f"{k}=true" if k in ("hidden", "disabled", "checked") else f'{k}=""')
            elif k in self.RAW_ATTRS:
                parts.append(f"{k}={v}")
            else:
                parts.append(f"{k}={rust_str(v)}")
        return (" " + " ".join(parts)) if parts else ""

    def _children(self, node, indent, pre):
        lines = []
        for kid in node.kids:
            lines.extend(self.emit(kid, indent, pre))
        return lines

    def _element(self, node, indent, pre):
        pad = " " * indent
        attrs = self._attrs(node)
        if node.tag in VOID:
            return [f"{pad}<{node.tag}{attrs}/>"]
        pre = pre or preserves_whitespace(node)

        kid_blocks = []
        for kid in node.kids:
            block = self.emit(kid, indent + 4, pre)
            if block:
                kid_blocks.append(block)

        if not kid_blocks:
            return [f"{pad}<{node.tag}{attrs}></{node.tag}>"]

        kid_blocks = self._chunk(kid_blocks, indent + 4)
        lines = [f"{pad}<{node.tag}{attrs}>"]
        for b in kid_blocks:
            lines.extend(b)
        lines.append(f"{pad}</{node.tag}>")
        return lines

    def _chunk(self, blocks, indent):
        """Regroupe en fragments <>...</> tant qu'il y a plus de MAX_KIDS enfants."""
        if len(blocks) <= MAX_KIDS:
            return blocks
        pad = " " * indent
        groups = []
        for i in range(0, len(blocks), MAX_KIDS):
            part = blocks[i:i + MAX_KIDS]
            g = [pad + "<>"]
            for b in part:
                g.extend("    " + line for line in b)
            g.append(pad + "</>")
            groups.append(g)
        return self._chunk(groups, indent)


def render(node, indent=0, replacements=None, drop=None):
    em = Emitter(replacements, drop)
    lines = []
    for kid in node.kids:
        lines.extend(em.emit(kid, indent))
    return "\n".join(lines), em.notes
