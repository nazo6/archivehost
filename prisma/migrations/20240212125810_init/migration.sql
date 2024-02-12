-- CreateTable
CREATE TABLE "Archive" (
    "urlScheme" TEXT NOT NULL,
    "urlPath" TEXT NOT NULL,
    "timestamp" DATETIME NOT NULL,
    "mime" TEXT NOT NULL,
    "status" TEXT,
    "savePath" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Archive_urlScheme_urlPath_timestamp_key" ON "Archive"("urlScheme", "urlPath", "timestamp");
