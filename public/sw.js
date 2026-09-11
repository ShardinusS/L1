/* Service worker du classeur.
 *
 * Trunk ajoute une empreinte au nom du bundle a chaque build : la liste des
 * fichiers a mettre en cache ne peut donc pas etre ecrite en dur. On precache
 * la coquille, puis on ajoute au cache chaque reponse same-origin obtenue.
 * Hors ligne, tout est servi depuis le cache, et une navigation retombe sur la
 * page d'accueil, qui contient l'application complete. */

// A changer a chaque publication : la coquille est servie depuis le cache.
const VERSION = 'classeur-v3';
const SHELL = ['./', './manifest.webmanifest', './icon.svg'];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches
      .open(VERSION)
      .then((cache) => cache.addAll(SHELL))
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
  const request = event.request;
  if (request.method !== 'GET') return;

  const url = new URL(request.url);
  const sameOrigin = url.origin === self.location.origin;
  // Les polices Google sont immuables : on les garde aussi.
  const cacheable = sameOrigin || url.hostname.endsWith('gstatic.com') || url.hostname.endsWith('googleapis.com');
  if (!cacheable) return;

  event.respondWith(
    caches.match(request).then((hit) => {
      if (hit) return hit;
      return fetch(request)
        .then((response) => {
          if (response && response.status === 200) {
            const copy = response.clone();
            caches.open(VERSION).then((cache) => cache.put(request, copy));
          }
          return response;
        })
        .catch(() => {
          // Hors ligne : une navigation retombe sur la coquille.
          if (request.mode === 'navigate') return caches.match('./');
          return Response.error();
        });
    })
  );
});
