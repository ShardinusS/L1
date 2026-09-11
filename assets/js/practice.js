/* Classeur L1 — entrainement.
 *
 * Les questions rediges et les exercices tires au hasard sont livres en JSON
 * (assets/data/practice.json et generated.json) : le navigateur n'a qu'a
 * piocher, poser, corriger et compter. Les statistiques restent dans le
 * localStorage de ce navigateur.
 */
(() => {
  'use strict';

  const $ = (sel, root = document) => root.querySelector(sel);
  const el = (tag, cls, text) => {
    const node = document.createElement(tag);
    if (cls) node.className = cls;
    if (text != null) node.textContent = text;
    return node;
  };

  const COURSES = {
    sf: { label: 'Structures fondamentales', page: 'structures-fondamentales.html', num: '01' },
    mtc: { label: 'Méthodes et techniques de calcul', page: 'methodes-calcul.html', num: '02' },
    algo: { label: 'Algorithmique 1', page: 'algorithmique.html', num: '03' },
    info: { label: "Représentation de l'information", page: 'information.html', num: '04' },
    os: { label: "Systèmes d'exploitation", page: 'systemes.html', num: '05' },
  };
  const ORDER = ['sf', 'mtc', 'algo', 'info', 'os'];

  /* ─────────────────────────── correction ─────────────────────────── */

  const norm = (s) =>
    s
      .toLowerCase()
      .normalize('NFD')
      .replace(/[̀-ͯ]/g, '');

  /** Uniformise les variantes typographiques du signe moins. */
  const asciiMinus = (s) => s.replace(/[−–—]/g, '-');

  /** Forme de comparaison d'une reponse libre : sans espaces ni guillemets. */
  const clean = (s) =>
    asciiMinus(norm(s))
      .replace(/[\s"'«»“”’]/g, '')
      .replace(/,/g, '.');

  const DIGITS = '0123456789abcdef';

  /** Lit un entier en base `radix`, en acceptant (1011)₂, 0x3F, les espaces. */
  function parseInt_(s, radix) {
    let t = asciiMinus(s)
      .toLowerCase()
      .replace(/[\s()_₀-₉]/g, '');
    let negative = false;
    if (t.startsWith('-') || t.startsWith('+')) {
      negative = t[0] === '-';
      t = t.slice(1);
    }
    const prefix = { 2: '0b', 8: '0o', 16: '0x' }[radix];
    if (prefix && t.startsWith(prefix)) t = t.slice(prefix.length);
    if (!t) return null;
    let value = 0;
    for (const ch of t) {
      const d = DIGITS.indexOf(ch);
      if (d < 0 || d >= radix) return null;
      value = value * radix + d;
    }
    return negative ? -value : value;
  }

  /** Tous les entiers d'une chaine, signes compris. */
  const ints = (s) => (asciiMinus(s).match(/-?\d+/g) || []).map(Number);

  const sortedUnique = (xs) => [...new Set(xs)].sort((a, b) => a - b);

  function parseSet(s) {
    const t = s.trim();
    if (!t || t === '∅' || clean(t) === 'vide' || t === '{}') return [];
    return sortedUnique(ints(t));
  }

  /** Lit une permutation ecrite en cycles : "(0 3 5)(1 2)", "Id" pour l'identite. */
  function parseCycles(s, n) {
    const perm = Array.from({ length: n }, (_, i) => i);
    const t = s.trim();
    if (!t) return null;
    if (/^(id|identite|identité|e)$/i.test(clean(t))) return perm;
    const groups = t.match(/\(([^)]*)\)/g);
    if (!groups) return null;
    const seen = new Set();
    for (const g of groups) {
      const cycle = ints(g);
      if (cycle.length < 2) return null;
      for (const x of cycle) {
        if (x < 0 || x >= n || seen.has(x)) return null;
        seen.add(x);
      }
      for (let i = 0; i < cycle.length; i++) {
        perm[cycle[i]] = cycle[(i + 1) % cycle.length];
      }
    }
    return perm;
  }

  /** Vrai si `response` repond correctement a `answer`. */
  function grade(answer, response) {
    switch (answer.kind) {
      case 'choice':
        return response.index === answer.correct;
      case 'card':
        return response.knew === true;
      case 'text':
        return answer.accepted.some((a) => clean(a) === clean(response.text));
      case 'number':
        return parseInt_(response.text, answer.radix) === answer.value;
      case 'set': {
        const got = parseSet(response.text);
        const want = sortedUnique(answer.elems);
        return got !== null && got.length === want.length && got.every((v, i) => v === want[i]);
      }
      case 'cycles': {
        const got = parseCycles(response.text, answer.perm.length);
        return got !== null && got.every((v, i) => v === answer.perm[i]);
      }
      case 'bezout': {
        const [u, v] = ints(response.text);
        return Number.isFinite(u) && Number.isFinite(v) && u * answer.a + v * answer.b === answer.g;
      }
      default:
        return false;
    }
  }

  const isTyped = (answer) => answer.kind !== 'choice' && answer.kind !== 'card';

  const PLACEHOLDERS = {
    set: 'ex. {1, 3, 5}  ou  ∅',
    cycles: 'ex. (0 3 5)(1 2)  — Id si identité',
    bezout: 'u, v',
  };

  function placeholderFor(answer) {
    if (answer.kind === 'number') {
      return (
        { 2: 'en binaire, ex. 101101', 8: 'en octal, ex. 157', 16: 'en hexadécimal, ex. 3F' }[
          answer.radix
        ] || 'un entier'
      );
    }
    return PLACEHOLDERS[answer.kind] || 'ta réponse';
  }

  const minus = (n) => (n < 0 ? '−' + -n : String(n));

  function expectedText(answer) {
    switch (answer.kind) {
      case 'choice':
        return answer.options[answer.correct];
      case 'text':
        return answer.accepted[0];
      case 'number': {
        const sign = answer.value < 0 ? '−' : '';
        const digits = Math.abs(answer.value).toString(answer.radix).toUpperCase();
        return sign + digits;
      }
      case 'set': {
        const e = sortedUnique(answer.elems);
        return e.length ? '{' + e.map(minus).join(', ') + '}' : '∅';
      }
      case 'card':
        return answer.back;
      default:
        return answer.expected || '';
    }
  }

  /* ──────────────────────── statistiques ──────────────────────────── */

  const STORE = 'classeur-practice';

  function loadProgress() {
    try {
      return JSON.parse(localStorage.getItem(STORE)) || {};
    } catch (e) {
      return {};
    }
  }

  function saveProgress(p) {
    try {
      localStorage.setItem(STORE, JSON.stringify(p));
    } catch (e) {
      /* navigation privee : les statistiques ne seront pas retenues */
    }
  }

  function record(progress, course, chapterId, ok) {
    const key = course + '/' + chapterId;
    const t = progress[key] || { asked: 0, right: 0 };
    t.asked += 1;
    if (ok) t.right += 1;
    progress[key] = t;
    saveProgress(progress);
  }

  const rate = (t) => (t && t.asked ? Math.round((t.right * 100) / t.asked) : null);

  function answersLabel(n) {
    return n <= 1 ? `${n} réponse` : `${n} réponses`;
  }

  /* ───────────────────────── tirage ───────────────────────────────── */

  const shuffle = (arr) => {
    for (let i = arr.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [arr[i], arr[j]] = [arr[j], arr[i]];
    }
    return arr;
  };

  /** Poids d'un exercice tire au hasard : il ne s'epuise pas. */
  const GENERATED_WEIGHT = 3;

  function buildPool(chapters, generated, filter) {
    const pool = [];
    for (const ch of chapters) {
      if (filter.course && ch.course !== filter.course) continue;
      if (filter.chapter && ch.id !== filter.chapter) continue;
      for (const f of ch.fixed) pool.push({ ch, fixed: f });
    }
    for (const g of generated) {
      if (filter.course && g.course !== filter.course) continue;
      if (filter.chapter && g.chapter !== filter.chapter) continue;
      const ch = chapters.find((c) => c.course === g.course && c.id === g.chapter);
      if (!ch) continue;
      for (let i = 0; i < GENERATED_WEIGHT; i++) pool.push({ ch, gen: g });
    }
    return pool;
  }

  /** Transforme une question rediges en exercice posable (QCM melange). */
  function materialise(entry) {
    const { ch } = entry;
    if (entry.gen) {
      const g = entry.gen;
      return {
        course: ch.course,
        chapterId: ch.id,
        chapterTitle: ch.title,
        anchor: ch.anchor,
        prompt: g.prompt,
        code: g.code,
        answer: g.answer,
        explain: g.explain,
        expected: g.expected,
        placeholder: g.placeholder,
      };
    }
    const f = entry.fixed;
    let answer;
    if (f.kind === 'choice') {
      const right = f.answer[0];
      const options = shuffle([...new Set(f.answer)]);
      answer = { kind: 'choice', options, correct: options.indexOf(right) };
    } else if (f.kind === 'vf') {
      answer = { kind: 'choice', options: ['Vrai', 'Faux'], correct: f.answer ? 0 : 1 };
    } else if (f.kind === 'text') {
      answer = { kind: 'text', accepted: f.answer };
    } else if (f.kind === 'number') {
      answer = { kind: 'number', value: f.answer, radix: 10 };
    } else if (f.kind === 'set') {
      answer = { kind: 'set', elems: f.answer };
    } else {
      answer = { kind: 'card', back: f.answer };
    }
    return {
      course: ch.course,
      chapterId: ch.id,
      chapterTitle: ch.title,
      anchor: ch.anchor,
      prompt: f.prompt,
      code: f.code,
      answer,
      explain: f.explain,
      expected: expectedText(answer),
      placeholder: placeholderFor(answer),
    };
  }

  function draw(pool, count) {
    if (!pool.length) return [];
    const bag = shuffle([...pool]);
    const out = [];
    for (let i = 0; i < bag.length * 2 && out.length < count; i++) {
      const ex = materialise(bag[i % bag.length]);
      if (out.some((q) => q.prompt === ex.prompt && q.code === ex.code)) continue;
      out.push(ex);
    }
    return out;
  }

  /* ─────────────────────────── interface ──────────────────────────── */

  const state = {
    chapters: [],
    generated: [],
    course: null, // null = les cinq matieres
    chapter: null,
    length: 10,
    series: [],
    at: 0,
    answered: null, // { ok, response }
    missed: [],
    progress: loadProgress(),
  };

  const root = $('#train');

  function chaptersOf(course) {
    return state.chapters.filter((c) => c.course === course);
  }

  function start() {
    const pool = buildPool(state.chapters, state.generated, {
      course: state.course,
      chapter: state.chapter,
    });
    state.series = draw(pool, state.length);
    state.at = 0;
    state.answered = null;
    state.missed = [];
    render();
  }

  function answer(response) {
    const q = state.series[state.at];
    const ok = grade(q.answer, response);
    state.answered = { ok, response };
    record(state.progress, q.course, q.chapterId, ok);
    if (!ok) state.missed.push(q);
    render();
  }

  function next() {
    state.at += 1;
    state.answered = null;
    render();
  }

  /* ── panneau lateral ── */

  function sidebar() {
    const side = el('aside', 'qz-side');

    const scope = el('div', 'qz-block');
    scope.append(el('div', 'qz-label', 'Sur quoi ?'));
    const chips = el('div', 'qz-chips');
    const list = state.course ? chaptersOf(state.course) : [];
    const all = el('button', 'qz-chip');
    all.type = 'button';
    all.setAttribute('aria-pressed', String(state.chapter === null));
    all.append(el('span', null, state.course ? 'Tout le cours' : 'Les cinq matières'));
    all.append(el('span', 'qz-rate'));
    all.addEventListener('click', () => {
      state.chapter = null;
      start();
    });
    chips.append(all);
    for (const ch of list) {
      const chip = el('button', 'qz-chip');
      chip.type = 'button';
      chip.setAttribute('aria-pressed', String(state.chapter === ch.id));
      chip.append(el('span', null, ch.title));
      const r = rate(state.progress[ch.course + '/' + ch.id]);
      chip.append(el('span', 'qz-rate', r === null ? '' : r + ' %'));
      chip.addEventListener('click', () => {
        state.chapter = ch.id;
        start();
      });
      chips.append(chip);
    }
    scope.append(chips);
    side.append(scope);

    const len = el('div', 'qz-block');
    len.append(el('div', 'qz-label', 'Longueur de la série'));
    const seg = el('div', 'qz-seg');
    seg.setAttribute('role', 'group');
    seg.setAttribute('aria-label', 'Nombre de questions');
    for (const n of [5, 10, 20]) {
      const b = el('button', null, String(n));
      b.type = 'button';
      b.setAttribute('aria-pressed', String(state.length === n));
      b.addEventListener('click', () => {
        state.length = n;
        start();
      });
      seg.append(b);
    }
    len.append(seg);
    side.append(len);

    const stats = el('div', 'qz-block');
    stats.append(el('div', 'qz-label', 'Mes statistiques'));
    const scoped = Object.entries(state.progress).filter(
      ([k]) => !state.course || k.startsWith(state.course + '/')
    );
    const asked = scoped.reduce((s, [, t]) => s + t.asked, 0);
    const right = scoped.reduce((s, [, t]) => s + t.right, 0);
    stats.append(
      el(
        'p',
        'qz-total',
        asked
          ? `${answersLabel(asked)} · ${Math.round((right * 100) / asked)} % de réussite`
          : 'Pas encore de réponse enregistrée.'
      )
    );

    const ul = el('ul', 'qz-stats');
    const rows = state.course ? chaptersOf(state.course) : state.chapters;
    for (const ch of rows) {
      const t = state.progress[ch.course + '/' + ch.id];
      const r = rate(t);
      const li = el('li');
      li.append(el('span', 'lab', ch.title));
      const track = el('span', 'track');
      const fill = el('span', 'fill');
      fill.style.width = (r === null ? 0 : r) + '%';
      track.append(fill);
      li.append(track);
      li.append(el('span', 'n', r === null ? '—' : r + ' %'));
      ul.append(li);
    }
    stats.append(ul);

    const reset = el('button', 'qz-reset', 'Effacer mes statistiques');
    reset.type = 'button';
    reset.addEventListener('click', () => {
      state.progress = {};
      saveProgress(state.progress);
      render();
    });
    stats.append(reset);
    side.append(stats);
    return side;
  }

  /* ── carte de question ── */

  function questionCard(q) {
    const card = el('article', 'qz-card c-' + q.course);

    const head = el('header', 'qz-head');
    head.append(el('span', null, `Question ${state.at + 1} / ${state.series.length}`));
    head.append(el('span', 'badge ' + q.course, q.chapterTitle));
    card.append(head);

    const body = el('div', 'qz-body');
    body.append(el('p', 'qz-prompt', q.prompt));
    if (q.code) body.append(el('pre', 'qz-code', q.code));

    const done = state.answered !== null;

    if (q.answer.kind === 'choice') {
      const opts = el('div', 'qz-options');
      opts.setAttribute('role', 'group');
      opts.setAttribute('aria-label', 'Réponses proposées');
      q.answer.options.forEach((text, i) => {
        const b = el('button', 'qz-opt');
        b.type = 'button';
        if (done) {
          b.disabled = true;
          if (i === q.answer.correct) b.classList.add('right');
          if (state.answered.response.index === i) {
            b.classList.add('picked');
            if (!state.answered.ok) b.classList.add('wrong');
          }
        }
        b.append(el('span', 'qz-key', String(i + 1)));
        b.append(el('span', 'qz-opt-text', text));
        b.addEventListener('click', () => answer({ index: i }));
        opts.append(b);
      });
      body.append(opts);
    } else if (q.answer.kind === 'card') {
      const actions = el('div', 'qz-actions');
      if (!done) {
        const show = el('button', 'btn primary', 'Retourner la carte');
        show.type = 'button';
        show.addEventListener('click', () => {
          state.answered = { ok: null, response: { reveal: true } };
          render();
        });
        actions.append(show);
      } else if (state.answered.response.reveal) {
        const knew = el('button', 'btn primary', 'Je savais');
        knew.type = 'button';
        knew.addEventListener('click', () => answer({ knew: true }));
        const nope = el('button', 'btn', 'Je ne savais pas');
        nope.type = 'button';
        nope.addEventListener('click', () => answer({ knew: false }));
        actions.append(knew, nope);
      }
      body.append(actions);
    } else {
      const form = el('form', 'qz-form');
      const input = el('input', 'qz-input');
      input.type = 'text';
      input.autocomplete = 'off';
      input.spellcheck = false;
      input.placeholder = q.placeholder;
      if (done) {
        input.disabled = true;
        input.value = state.answered.response.text;
      }
      const check = el('button', 'btn primary', 'Vérifier');
      check.type = 'submit';
      const skip = el('button', 'btn', 'Je ne sais pas');
      skip.type = 'button';
      skip.addEventListener('click', () => answer({ text: '' }));
      if (!done) form.append(input, check, skip);
      else form.append(input);
      form.addEventListener('submit', (ev) => {
        ev.preventDefault();
        answer({ text: input.value });
      });
      body.append(form);
      if (!done) requestAnimationFrame(() => input.focus());
    }

    if (done && state.answered.ok !== null) {
      const fb = el('div', 'qz-feedback ' + (state.answered.ok ? 'ok' : 'ko'));
      fb.setAttribute('role', 'status');
      fb.append(el('p', 'qz-verdict', state.answered.ok ? '✓ Juste' : '✗ Pas tout à fait'));
      if (!state.answered.ok || q.answer.kind !== 'choice') {
        const exp = el('p', 'qz-expected');
        exp.append(document.createTextNode('Réponse attendue : '));
        exp.append(el('span', 'mono', q.expected));
        fb.append(exp);
      }
      if (q.explain) fb.append(el('p', 'qz-explain', q.explain));
      const link = el('a', 'qz-link', 'Revoir la fiche du cours →');
      link.href = `${COURSES[q.course].page}#${q.anchor}`;
      fb.append(link);
      body.append(fb);

      const actions = el('div', 'qz-actions');
      const last = state.at + 1 >= state.series.length;
      const btn = el('button', 'btn primary', last ? 'Voir le bilan →' : 'Question suivante →');
      btn.type = 'button';
      btn.addEventListener('click', next);
      actions.append(btn);
      body.append(actions);
    }

    card.append(body);
    return card;
  }

  /* ── bilan ── */

  function summary() {
    const card = el('article', 'qz-card');
    const body = el('div', 'qz-body qz-summary');
    const total = state.series.length;
    const good = total - state.missed.length;
    body.append(el('h3', null, `${good} / ${total}`));
    const pct = total ? Math.round((good * 100) / total) : 0;
    body.append(
      el(
        'p',
        'qz-message',
        pct === 100
          ? 'Série parfaite. Change de chapitre, ou allonge la série.'
          : pct >= 70
            ? 'Bonne série. Les questions manquées sont listées ci-dessous.'
            : 'À reprendre : relis les fiches citées, puis relance une série.'
      )
    );

    if (state.missed.length) {
      const ul = el('ul', 'qz-miss');
      for (const q of state.missed) {
        const li = el('li');
        li.append(el('span', 'qz-miss-q', q.prompt));
        const a = el('span', 'qz-miss-a');
        a.append(document.createTextNode('Réponse : '));
        a.append(el('span', 'mono', q.expected));
        li.append(a);
        const link = el('a', 'qz-link', 'Revoir la fiche →');
        link.href = `${COURSES[q.course].page}#${q.anchor}`;
        li.append(link);
        ul.append(li);
      }
      body.append(ul);
    }

    const actions = el('div', 'qz-actions');
    const again = el('button', 'btn primary', 'Nouvelle série');
    again.type = 'button';
    again.addEventListener('click', start);
    actions.append(again);
    body.append(actions);
    card.append(body);
    return card;
  }

  /* ── rendu complet ── */

  function render() {
    root.textContent = '';

    const course = state.course ? COURSES[state.course] : null;
    const header = el('header', 'masthead');
    const inner = el('div', 'masthead-inner');
    inner.append(
      el('div', 'eyebrow', course ? `Entraînement · Matière ${course.num}` : 'Entraînement')
    );
    inner.append(el('h1', null, course ? course.label : 'S’exercer, matière par matière'));
    inner.append(
      el(
        'p',
        'lede',
        course
          ? "Choisis un chapitre ou garde tout le cours. Chaque correction renvoie à la fiche qui l'explique."
          : 'Les cinq matières dans une même série. Choisis une matière à gauche pour cibler un chapitre.'
      )
    );
    const meta = el('div', 'meta');
    const sel = el('select');
    sel.setAttribute('aria-label', 'Matière');
    const optAll = el('option', null, 'Les cinq matières');
    optAll.value = '';
    sel.append(optAll);
    for (const key of ORDER) {
      const o = el('option', null, COURSES[key].label);
      o.value = key;
      sel.append(o);
    }
    sel.value = state.course || '';
    sel.addEventListener('change', () => {
      state.course = sel.value || null;
      state.chapter = null;
      const url = new URL(location.href);
      if (state.course) url.searchParams.set('m', COURSES[state.course].page.replace('.html', ''));
      else url.searchParams.delete('m');
      history.replaceState({}, '', url);
      start();
    });
    meta.append(sel);
    if (course) {
      const open = el('a', null, 'Ouvrir le cours');
      open.href = course.page;
      meta.append(open);
    }
    inner.append(meta);
    header.append(inner);
    root.append(header);

    const shell = el('div', 'shell wide');
    const main = el('main');
    const qz = el('div', 'qz');
    qz.append(sidebar());

    const col = el('div', 'qz-main');
    if (!state.series.length) {
      col.append(el('p', 'qz-total', 'Aucun exercice pour cette sélection.'));
    } else if (state.at >= state.series.length) {
      col.append(summary());
    } else {
      const bar = el('ol', 'qz-bar');
      bar.setAttribute('aria-hidden', 'true');
      state.series.forEach((_, i) => {
        const li = el('li');
        if (i === state.at) li.className = 'now';
        else if (i < state.at) li.className = 'done';
        bar.append(li);
      });
      col.append(bar);
      col.append(questionCard(state.series[state.at]));
    }
    qz.append(col);
    main.append(qz);
    shell.append(main);
    root.append(shell);
  }

  /* ── raccourcis clavier ── */

  document.addEventListener('keydown', (ev) => {
    const tag = (document.activeElement?.tagName || '').toLowerCase();
    if (tag === 'input' || tag === 'select' || tag === 'textarea') return;
    const q = state.series[state.at];
    if (!q) return;
    if (state.answered && (ev.key === 'Enter' || ev.key === ' ')) {
      ev.preventDefault();
      next();
      return;
    }
    if (!state.answered && q.answer.kind === 'choice') {
      const i = Number(ev.key) - 1;
      if (i >= 0 && i < q.answer.options.length) {
        ev.preventDefault();
        answer({ index: i });
      }
    }
  });

  /* ── demarrage ── */

  const params = new URLSearchParams(location.search);
  const slug = params.get('m');
  if (slug) {
    const found = ORDER.find((k) => COURSES[k].page === slug + '.html');
    if (found) state.course = found;
  }

  Promise.all([
    fetch('assets/data/practice.json').then((r) => r.json()),
    fetch('assets/data/generated.json').then((r) => r.json()),
  ])
    .then(([chapters, generated]) => {
      state.chapters = chapters;
      state.generated = generated;
      start();
    })
    .catch(() => {
      root.textContent = '';
      const p = el('p', 'qz-total', "Les exercices n'ont pas pu être chargés.");
      root.append(p);
    });
})();
