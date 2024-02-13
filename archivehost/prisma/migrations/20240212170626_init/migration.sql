-- CreateTable
CREATE TABLE "Archive" (
    "urlScheme" TEXT NOT NULL,
    "urlHost" TEXT NOT NULL,
    "urlPath" TEXT NOT NULL,
    "timestamp" DATETIME NOT NULL,
    "mime" TEXT NOT NULL,
    "status" INTEGER,
    "savePath" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Archive_urlScheme_urlHost_urlPath_timestamp_key" ON "Archive"("urlScheme", "urlHost", "urlPath", "timestamp");
