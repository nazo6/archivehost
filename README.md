# Archivehost - Wayback machine downloader & server

Program to batch download and serve the wayback machine archives.

## Pre-requisite

To build and run this program, you need to have the following installed:

- Node.js
- pnpm
- Rust

## Install

Currently, binary is not available. You can install by running:

```bash
$ git clone https://github.com/nazo6/archivehost
$ cd archivehost
$ pnpm i
$ pnpm prisma-generate
$ pnpm inst
```

## Usage

You can see the help message by command:

```bash
$ archivehost --help
```

### Download

```bash
$ archivehost download <url> [--concurrency <concurrency>] [--from <from>] [--to <to>]
```

File will be save to `~/.local/share/archivehost` on Linux and
`~/AppData/Roaming/archivehost` on Windows. This can be changed by using the
`--root` flag.

### Serve

```bash
$ archivehost serve [--port <port>] [--host <host>]
```

By default, the server will run on port 3000. You can change the port by using
the `--port` flag.

#### Serve url

##### `/web/latest/<url>`

Show the latest archive of the url.

##### `/web/<timestamp>/<url>`

Show the archive of the url before the timestamp.

## TODO

- Download
  - [x] Download the archive
  - [x] Save downloaded info to sqlite database
  - [x] Add database cleanup feature
  - [ ] Be able to export the downloaded archive

- Serve
  - [x] Basic server
  - [ ] Add manager web ui frontend and api
  - [ ] Add banner to archive page so that user can see archive info
  - [ ] On-demand download
  - [ ] Return dummy response even if the archive is not found
  - [ ] Better url rewriting
