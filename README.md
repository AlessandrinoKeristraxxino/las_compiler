# LampScript Compiler (las_compiler)

This repo is the official compiler for LampScript.

## How to install

Is simple.

### Windows

## Usage

Create a project and start its local development server:

```text
las new My-Project
cd My-Project
las serve
```

Open `http://127.0.0.1:8000` in a browser. The `serve` command builds the
project and serves `target/index.html`, `target/glue.js`, and `target/main.wasm`
over HTTP. Opening `target/index.html` directly with a `file://` URL is not
supported because browsers block `fetch()` requests for WebAssembly files from
local file origins.

