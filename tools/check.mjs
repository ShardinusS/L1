// Verification du site statique — aucune dependance, `node tools/check.mjs`.
//
// Controle :
//   1. chaque lien interne vise un fichier qui existe ;
//   2. chaque ancre citee existe dans la page visee ;
//   3. les fichiers JSON sont lisibles et coherents ;
//   4. chaque page charge bien les feuilles de style et les scripts.

import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';

const ROOT = path.resolve(import.meta.dirname, '..');
const read = (p) => fs.readFileSync(path.join(ROOT, p), 'utf8');

const pages = fs.readdirSync(ROOT).filter((f) => f.endsWith('.html'));
const errors = [];
const fail = (msg) => errors.push(msg);

// ── ancres disponibles par page ──────────────────────────────────────
const anchors = new Map();
for (const page of pages) {
  const html = read(page);
  const ids = new Set([...html.matchAll(/\sid="([^"]+)"/g)].map((m) => m[1]));
  anchors.set(page, ids);
}

// ── liens ────────────────────────────────────────────────────────────
let linkCount = 0;
for (const page of pages) {
  const html = read(page);
  for (const m of html.matchAll(/href="([^"]+)"/g)) {
    const href = m[1];
    if (/^(https?:|mailto:|#|$)/.test(href)) {
      // Ancre interne a la page courante.
      if (href.startsWith('#') && href.length > 1) {
        linkCount++;
        if (!anchors.get(page).has(href.slice(1))) {
          fail(`${page} : ancre interne inconnue « ${href} »`);
        }
      }
      continue;
    }
    linkCount++;
    const [file, frag] = href.split('#');
    const target = file.split('?')[0];
    if (!target) continue;
    if (!fs.existsSync(path.join(ROOT, target))) {
      fail(`${page} : lien vers un fichier absent « ${target} »`);
      continue;
    }
    if (frag && target.endsWith('.html') && !anchors.get(target)?.has(frag)) {
      fail(`${page} : « ${target}#${frag} » vise une ancre inexistante`);
    }
  }
}

// ── ressources referencees par chaque page ───────────────────────────
for (const page of pages) {
  const html = read(page);
  for (const m of html.matchAll(/src="([^"]+)"/g)) {
    const src = m[1];
    if (/^https?:/.test(src)) continue;
    if (!fs.existsSync(path.join(ROOT, src))) fail(`${page} : script absent « ${src} »`);
  }
  if (!html.includes('assets/css/tokens.css')) fail(`${page} : feuilles de style manquantes`);
  if (!html.includes('assets/js/app.js')) fail(`${page} : app.js manquant`);
}

// ── donnees ──────────────────────────────────────────────────────────
const data = {};
for (const name of ['search', 'practice', 'prompts', 'generated', 'courses']) {
  const file = `assets/data/${name}.json`;
  try {
    data[name] = JSON.parse(read(file));
  } catch (e) {
    fail(`${file} : JSON illisible (${e.message})`);
  }
}

const PAGE_OF = {
  sf: 'structures-fondamentales.html',
  mtc: 'methodes-calcul.html',
  algo: 'algorithmique.html',
  info: 'information.html',
  os: 'systemes.html',
};

if (data.search) {
  for (const e of data.search) {
    const page = PAGE_OF[e.c];
    if (!page) fail(`search.json : matière inconnue « ${e.c} »`);
    else if (!anchors.get(page)?.has(e.a)) {
      fail(`search.json : « ${e.l} » vise l'ancre inconnue ${page}#${e.a}`);
    }
  }
}

if (data.prompts) {
  if (data.prompts.length !== 35) fail(`prompts.json : ${data.prompts.length} prompts au lieu de 35`);
  for (const p of data.prompts) {
    const page = PAGE_OF[p.course];
    if (!anchors.get(page)?.has(p.anchor)) {
      fail(`prompts.json : « ${p.title} » vise l'ancre inconnue ${page}#${p.anchor}`);
    }
  }
}

if (data.practice) {
  for (const ch of data.practice) {
    const page = PAGE_OF[ch.course];
    if (!anchors.get(page)?.has(ch.anchor)) {
      fail(`practice.json : chapitre « ${ch.id} » vise l'ancre inconnue ${page}#${ch.anchor}`);
    }
    for (const f of ch.fixed) {
      if (!f.prompt.trim()) fail(`practice.json : énoncé vide dans « ${ch.id} »`);
      if (f.kind === 'choice' && new Set(f.answer).size !== f.answer.length) {
        fail(`practice.json : options en double dans « ${ch.id} » — ${f.prompt.slice(0, 40)}`);
      }
    }
  }
}

if (data.generated && data.practice) {
  const known = new Set(data.practice.map((c) => c.course + '/' + c.id));
  for (const g of data.generated) {
    if (!known.has(g.course + '/' + g.chapter)) {
      fail(`generated.json : chapitre inconnu « ${g.course}/${g.chapter} »`);
      break;
    }
  }
}

// ── verdict ──────────────────────────────────────────────────────────
const counts = {
  pages: pages.length,
  liens: linkCount,
  entrées: data.search?.length ?? 0,
  prompts: data.prompts?.length ?? 0,
  chapitres: data.practice?.length ?? 0,
  exercices:
    (data.practice?.reduce((s, c) => s + c.fixed.length, 0) ?? 0) + (data.generated?.length ?? 0),
};
console.log(
  Object.entries(counts)
    .map(([k, v]) => `${v} ${k}`)
    .join(' · ')
);

if (errors.length) {
  console.error(`\n${errors.length} problème(s) :`);
  for (const e of errors.slice(0, 40)) console.error('  - ' + e);
  process.exit(1);
}
console.log('Tout est cohérent.');
