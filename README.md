# Content-Finder-rs
Extremely simple cli tool as if it were ubuntu build-in.
This enables you to:
- filter files which include query string you like.

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

## Installation
``` bash
$ cargo install --git <This Repository Url>
```

## philosophy
This aims to be used as simple as `find` command (built-in cli app that allows you to find file with name that includes query).
``` bash
$ find .zip
a.zip
b.zip
```

