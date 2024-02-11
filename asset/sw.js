self.addEventListener("install", function (e) {
  console.log("install", e);
});

self.addEventListener("activate", function (e) {
  console.log("activate", e);
});

self.addEventListener("fetch", function (e) {
  console.log("fetch", e);

  const referer = e.request.headers.get("Referer");
  const url = new URL(e.request.url);
  url.host = "{{host}}";

  console.log(`${e.request.url} -> ${url.href} (${referer})`);

  e.respondWith(fetch(url.href));
});
