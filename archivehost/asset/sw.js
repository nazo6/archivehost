self.addEventListener("install", function (e) {
  console.log("install", e);
});

self.addEventListener("activate", function (e) {
  console.log("activate", e);
});

self.addEventListener("fetch", function (e) {
  const fn = async () => {
    const clientId = e.clientId;
    const client = await clients.get(clientId);
    if (!client) return await fetch(e.request);
    const clientUrl = new URL(client.url);

    console.log("ref", event.request.referrer);

    const timestamp = clientUrl.pathname.split("/")[2];
    const url = new URL(e.request.url);
    url.host = clientUrl.host;
    url.pathname = `/web/${timestamp}${url.pathname}`;

    console.log(`${e.request.url} -> ${url.href} (${client.url})`);

    return await fetch(url.href);
  };

  e.respondWith(fn());
});
