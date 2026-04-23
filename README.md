# Content-Finder-rs
Extremely simple cli tool as if it were ubuntu build-in.
This enables you to:
- filter files which include given query string in content.

## Usage
```bash
$ cfind "export async function GET"
./app/api/route.ts
./.next/dev/server/chunks/[root-of-the-server]__0~zm8xe._.js.map
./.next/dev/types/routes.d.ts
```
```bash
# Exclude entries with "-e" option.
$ cfind "export async function GET" -e .next
./app/api/route.ts
```
**This can be more powerful when you use it with "grep" command.**
```bash
$ cfind "export async function GET" | grep .ts
./app/api/route.ts
./.next/dev/types/routes.d.ts
```

## Installation
``` bash
$ cargo install --git https://github.com/mtugb/content-finder-rs
```
Or, if you u don't want to make your dir dirty:
``` bash
$ git clone git@github.com:mtugb/content-finder-rs.git
$ cargo run
```
to try it in limited scope.

## philosophy
It aims to be a user-friendly wrapper around `grep -rl`.

$ grep -rl "query" .   # hard to remember
$ cfind "query"        # simple
