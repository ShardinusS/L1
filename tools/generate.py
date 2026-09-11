# -*- coding: utf-8 -*-
"""Genere les modules Rust des pages a partir du classeur HTML d'origine."""
import io, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from htmltree import parse, render, rust_str, Node, WS

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(os.path.dirname(ROOT), "classeur-informatique.html")
OUT = os.path.join(ROOT, "src", "pages")

RAW = io.open(SRC, encoding="utf-8").read()
LINES = RAW.split("\n")

PANES = [
    ("home", "home", 358, 503, "Accueil"),
    ("sf",   "sf",   506, 1561, "Matiere 01 - Structures fondamentales"),
    ("algo", "algo", 1564, 1894, "Matiere 02 - Algorithmique 1"),
    ("info", "info", 1896, 2253, "Matiere 03 - Representation de l'information"),
    ("os",   "os",   2255, 2503, "Matiere 04 - Systemes d'exploitation"),
]

COURSE_ENUM = {"sf": "Sf", "algo": "Algo", "info": "Info", "os": "Os"}
COURSE_LABEL = {
    "sf": "Structures fondamentales",
    "algo": "Algorithmique 1",
    "info": "Représentation de l'information",
    "os": "Systèmes d'exploitation",
}
ROUTE = {"sf": "/structures-fondamentales", "algo": "/algorithmique",
         "info": "/information", "os": "/systemes"}


def fragment(a, b):
    return "\n".join(LINES[a - 1:b])


def text_of(node):
    return node.text_content()


def trim(s):
    return s.strip(WS + "\xa0\u202f")


def strip_words(s, words):
    out = s
    for w in words:
        out = re.sub(w, "", out, flags=re.IGNORECASE)
    return trim(out)


# ---------------------------------------------------------------- index ----
def build_index(key, sections):
    """Reproduit les regles de collecte du script d'origine (lignes 2556-2585)."""
    course_label = COURSE_LABEL[key]
    entries = []
    for sec in sections:
        sec_id = sec.get("id")
        head = sec.first(lambda n: n.has_class("sec-head"))
        h2 = head.first(lambda n: n.tag == "h2") if head else None
        stitle = trim(text_of(h2)) if h2 else ""
        if h2:
            entries.append((sec_id, stitle, course_label, False))
        for hx in sec.select(lambda n: n.tag in ("h3", "h4")):
            t = strip_words(text_of(hx), [r"complément", r"annale"])
            if t:
                entries.append((sec_id, t, stitle, False))
        for dt in sec.select(lambda n: n.tag == "dt" and n.parent and n.parent.has_class("def")):
            entries.append((sec_id, trim(text_of(dt)), stitle, False))
        for c in sec.select(lambda n: n.has_class("c") and n.parent and n.parent.has_class("cmd")):
            d = c.parent.first(lambda n: n.has_class("d"))
            entries.append((sec_id, trim(text_of(c)), trim(text_of(d)) if d else stitle, True))
        for x in sec.select(lambda n: n.has_class("exo")):
            n_ = x.first(lambda n: n.has_class("n"))
            t_ = x.first(lambda n: n.has_class("t"))
            if t_:
                sub = stitle + (" · ex. " + trim(text_of(n_)) if n_ else "")
                entries.append((sec_id, trim(text_of(t_)), sub, False))
        for a in sec.select(lambda n: n.tag == "h4" and n.parent and n.parent.has_class("an")):
            y = a.parent.first(lambda n: n.has_class("y"))
            entries.append((sec_id, trim(text_of(a)), trim(text_of(y)) if y else stitle, False))
        for t in sec.select(lambda n: n.has_class("title") and n.parent and n.parent.has_class("note")):
            entries.append((sec_id, strip_words(text_of(t), [r"annale"]), stitle, False))
    return entries


def emit_index(key, entries):
    lines = ["/// Entrées de recherche de la matière, dans l'ordre du cours."]
    lines.append("#[rustfmt::skip]")
    lines.append("pub const INDEX: &[IndexEntry] = &[")
    ce = COURSE_ENUM[key]
    for sec_id, label, sub, mono in entries:
        ctor = "IndexEntry::mono" if mono else "IndexEntry::new"
        lines.append(
            f"    {ctor}(Course::{ce}, {rust_str(sec_id)}, {rust_str(label)}, {rust_str(sub)}),"
        )
    lines.append("];")
    return "\n".join(lines)


# ------------------------------------------------------------------ toc ----
def build_toc(pane):
    nav = pane.first(lambda n: n.tag == "nav" and n.has_class("toc"))
    if not nav:
        return None, []
    label = nav.get("aria-label") or "Sommaire"
    items = []
    for a in nav.select(lambda n: n.tag == "a"):
        href = a.get("href") or ""
        num = a.first(lambda n: n.has_class("num"))
        spans = [n for n in a.kids if n.tag == "span"]
        title = spans[-1] if spans else None
        items.append((href.lstrip("#"), trim(text_of(num)) if num else "",
                      trim(text_of(title)) if title else trim(text_of(a))))
    return label, items


def emit_toc(label, items):
    out = ["/// Sommaire de la matière : (ancre, numéro, titre)."]
    out.append("#[rustfmt::skip]")
    out.append("pub const TOC: &[TocItem] = &[")
    for anchor, num, title in items:
        out.append(f"    TocItem {{ anchor: {rust_str(anchor)}, num: {rust_str(num)}, title: {rust_str(title)} }},")
    out.append("];")
    out.append("")
    out.append("/// Libellé accessible du sommaire.")
    out.append(f"pub const TOC_LABEL: &str = {rust_str(label)};")
    return "\n".join(out)


# -------------------------------------------------------------- rewrite ----
def set_attr(node, name, value):
    node.attrs = [(k, v) for k, v in node.attrs if k != name] + [(name, value)]


def del_attr(node, name):
    node.attrs = [(k, v) for k, v in node.attrs if k != name]


def rewrite_tree(pane, key):
    """Adapte le balisage d'origine aux besoins de l'application Leptos."""
    for node in list(pane.elements()):
        if node.tag == "button" and node.has_class("mat-card"):
            target = node.get("data-pane")
            goto = node.get("data-goto")
            cls = node.get("class")
            # <A> de leptos_router : navigation sans rechargement de la page.
            node.tag = "A"
            node.attrs = []
            if target:
                set_attr(node, "href", ROUTE[target])
            elif goto:
                set_attr(node, "href", ROUTE["sf"] + "#" + goto)
            # Le defilement est pilote par l'effet de app.rs, pas par <A>.
            set_attr(node, "scroll", "false")
            set_attr(node, "attr:class", cls)
    return pane


def replacements(node):
    if node.has_class("conv"):
        return "<Converter/>"
    if node.has_class("search") and node.first(lambda n: n.get("id") == "q"):
        return "<CmdTable/>"
    return None


def drop(node):
    if node.has_class("tw") and node.first(lambda n: n.get("id") == "recap"):
        return True
    return False


# ----------------------------------------------------------------- main ----
def fn_name(sec_id):
    return "sec_" + sec_id.replace("-", "_")


def render_node(node, repl, dr):
    wrapper = Node("#w")
    wrapper.kids = [node]
    return render(wrapper, indent=8, replacements=repl, drop=dr)


def generate(key, pane_key, start, end, title):
    frag = fragment(start, end)
    root = parse(frag)
    pane = root.first(lambda n: n.get("id") == "pane-" + pane_key)
    assert pane is not None, key
    rewrite_tree(pane, key)

    header = pane.first(lambda n: n.tag == "header")
    shell = pane.first(lambda n: n.has_class("shell"))
    main = pane.first(lambda n: n.tag == "main")
    sections = [n for n in main.kids if n.tag == "section"]

    toc_label, toc_items = build_toc(pane)
    entries = build_index(key, sections) if key != "home" else []

    out = []
    out.append("//! " + title + ".")
    out.append("//!")
    out.append("//! Contenu transposé tel quel depuis le classeur HTML d'origine :")
    out.append("//! mêmes textes, mêmes formules, mêmes exemples.")
    out.append("")
    out.append("use leptos::prelude::*;")
    if key == "home":
        out.append("use leptos_router::components::A;")
    crate_uses = []
    if key == "info":
        crate_uses.append("use crate::components::converter::Converter;")
    if key == "os":
        crate_uses.append("use crate::components::cmd_table::CmdTable;")
    if key != "home":
        crate_uses.append("use crate::components::toc::{Toc, TocItem};")
        crate_uses.append("use crate::course::Course;")
        crate_uses.append("use crate::search::IndexEntry;")
    if crate_uses:
        out.append("")
        out.extend(sorted(crate_uses))
    out.append("")

    out.append("/// Ancres des sections de la page, dans l'ordre d'affichage.")
    out.append("#[rustfmt::skip]")
    out.append("pub const SECTIONS: &[&str] = &[")
    for sec in sections:
        out.append("    " + rust_str(sec.get("id")) + ",")
    out.append("];")
    out.append("")

    if key != "home":
        out.append(emit_toc(toc_label, toc_items))
        out.append("")
        out.append(emit_index(key, entries))
        out.append("")

    notes_all = []

    head_body, notes = render_node(header, replacements, drop)
    notes_all += notes
    out.append("fn masthead() -> impl IntoView {")
    out.append("    view! {")
    out.append(head_body)
    out.append("    }")
    out.append("}")
    out.append("")

    for sec in sections:
        body, notes = render_node(sec, replacements, drop)
        notes_all += notes
        out.append("fn " + fn_name(sec.get("id")) + "() -> impl IntoView {")
        out.append("    view! {")
        out.append(body)
        out.append("    }")
        out.append("}")
        out.append("")

    shell_class = " ".join(shell.classes())
    pane_class = " ".join(pane.classes())
    out.append("/// Page complète de la matière.")
    out.append("pub fn page() -> impl IntoView {")
    out.append("    view! {")
    out.append("        <div class=" + rust_str(pane_class) + ">")
    out.append("            {masthead()}")
    out.append("            <div class=" + rust_str(shell_class) + ">")
    if key != "home":
        out.append("                <Toc course=Course::" + COURSE_ENUM[key] + " items=TOC label=TOC_LABEL/>")
    out.append("                <main>")
    groups = [sections[i:i + 12] for i in range(0, len(sections), 12)]
    for g in groups:
        if len(groups) > 1:
            out.append("                    <>")
            pad = "                        "
        else:
            pad = "                    "
        for sec in g:
            out.append(pad + "{" + fn_name(sec.get("id")) + "()}")
        if len(groups) > 1:
            out.append("                    </>")
    out.append("                </main>")
    out.append("            </div>")
    out.append("        </div>")
    out.append("    }")
    out.append("}")
    out.append("")

    path = os.path.join(OUT, key + ".rs")
    io.open(path, "w", encoding="utf-8", newline="\n").write("\n".join(out))
    return key, len(sections), len(entries), notes_all


if __name__ == "__main__":
    os.makedirs(OUT, exist_ok=True)
    total = 0
    allnotes = []
    for key, pane_key, a, b, title in PANES:
        k, nsec, nidx, notes = generate(key, pane_key, a, b, title)
        allnotes += notes
        total += nidx
        print("%-5s sections=%2d index=%3d" % (k, nsec, nidx))
    print("index total:", total)
    if "--notes" in sys.argv:
        from collections import Counter
        for path, n in Counter(p for _, p in allnotes).most_common(40):
            print("%4d  %s" % (n, path))
