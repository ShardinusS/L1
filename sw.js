/* Service worker du classeur — consultation hors ligne.
 *
 * Strategie volontairement prudente :
 *   - les pages HTML passent par le reseau d'abord, avec repli sur le cache
 *     (on ne sert jamais une vieille page quand le site vient d'etre mis a jour) ;
 *   - le reste (CSS, JS, JSON, images) est servi depuis le cache d'abord,
 *     puis rafraichi en arriere-plan.
 *
 * Les chemins sont relatifs au dossier du service worker : le site marche donc
 * a la racine d'un domaine comme dans un sous-dossier GitHub Pages.
 */

const VERSION = 'classeur-2026-09-11';

const SHELL = [
  './',
  './index.html',
  './structures-fondamentales.html',
  './methodes-calcul.html',
  './algorithmique.html',
  './information.html',
  './systemes.html',
  './prompts.html',
  './entrainement.html',
  './assets/css/tokens.css',
  './assets/css/base.css',
  './assets/css/components.css',
  './assets/css/subject-03.css',
  './assets/css/practice.css',
  './assets/css/prompts.css',
  './assets/css/print.css',
  './assets/js/app.js',
  './assets/js/practice.js',
  './assets/data/search.json',
  './icon.svg',
  './manifest.webmanifest',
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches
      .open(VERSION)
      // `addAll` echoue en bloc a la moindre ressource manquante : on prend
      // chaque fichier separement pour qu'une absence ne casse pas l'install.
      .then((cache) => Promise.all(SHELL.map((url) => cache.add(url).catch(() => null))))
      .then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) => Promise.all(keys.filter((k) => k !== VERSION).map((k) => caches.delete(k))))
      .then(() => self.clients.claim())
  );
});

self.addEventListener('fetch', (event) => {
  const { request } = event;
  if (request.method !== 'GET') return;
  const url = new URL(request.url);
  if (url.origin !== self.location.origin) return;

  const isPage = request.mode === 'navigate' || url.pathname.endsWith('.html');

  if (isPage) {
    event.respondWith(
      fetch(request)
        .then((response) => {
          const copy = response.clone();
          caches.open(VERSION).then((c) => c.put(request, copy));
          return response;
        })
        .catch(() => caches.match(request).then((hit) => hit || caches.match('./index.html')))
    );
    return;
  }

  event.respondWith(
    caches.match(request).then((hit) => {
      const network = fetch(request)
        .then((response) => {
          const copy = response.clone();
          caches.open(VERSION).then((c) => c.put(request, copy));
          return response;
        })
        .catch(() => hit);
      return hit || network;
    })
  );
});
