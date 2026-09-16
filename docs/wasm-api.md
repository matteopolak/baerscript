# Browser Wasm API

## What it is

`baerscript-wasm` is a browser-embeddable wrapper around the Rust interpreter. It accepts source and pre-buffered input, runs synchronously with a deterministic instruction budget, and returns captured stdout and stderr without accessing browser globals.

## How it works

Build the publishable `wasm-bindgen` bundle from the repository root:

```bash
wasm-pack build wasm --target web --release --out-dir pkg
```

The generated bundle is not committed. A publisher should upload the complete `wasm/pkg/` directory, whose consumer-facing files are:

- `wasm/pkg/baerscript_wasm.js` — ES module loader and the `execute` export
- `wasm/pkg/baerscript_wasm_bg.wasm` — interpreter binary
- `wasm/pkg/baerscript_wasm.d.ts` — TypeScript declarations, including `ExecutionResult`
- `wasm/pkg/package.json` — package metadata
- `wasm/pkg/LICENSE` — MIT license included with the distributable

The only interpreter API export is:

```ts
execute(
  source: string,
  input: string,
  ascii: boolean,
  max_steps: number,
): ExecutionResult;

interface ExecutionResult {
  ok: boolean;
  stdout: string;
  stderr: string;
  steps: number;
}
```

`ok` is false for parse errors, input exhaustion, invalid input, arithmetic or movement errors, and instruction-budget exhaustion. `stdout` contains output produced before success or failure. `stderr` is empty on success and otherwise contains a human-readable diagnostic. `steps` counts completed instructions; parse failures report zero.

`input` is consumed one non-newline byte at a time by `<`. With `ascii: true`, each value must be ASCII and is stored as its character code. With `ascii: false`, each value must be one decimal digit. CR and LF bytes are skipped, so newline-separated inputs work naturally.

The instruction budget is independent of a wall-clock timeout. Pass `max_steps` as an integer from `0` through `4_294_967_295`; a non-empty program with `max_steps: 0` fails before executing an instruction.

### Worker integration

`execute` is deliberately synchronous. Import and initialize the module inside a Web Worker, call `execute` there, and have the main thread terminate and replace the worker if its wall-clock timer fires. Wasm cannot cooperatively report a result after its worker has been terminated, so the portfolio should synthesize its own timeout diagnostic in that case.

```js
import init, { execute } from "./baerscript_wasm.js";

await init();

self.onmessage = ({ data }) => {
  const result = execute(data.source, data.input ?? "", data.ascii ?? false, data.maxSteps);
  self.postMessage(result);
};
```

Initialize once per worker. The `--target web` loader resolves the `.wasm` file relative to the JavaScript module by default; a CDN or fingerprinted asset pipeline may instead pass an explicit module or URL to `init`.

For approximate syntax highlighting, recognize the instruction symbols `. + - * < > ^ v [ ]`, treat `#` through the end of its line as a comment, and treat spaces and tabs as insignificant. Highlighting does not need to invoke Wasm.

## How to change it

Interpreter execution and error capture live in `runtime/src/execute.rs` and the bounded loop lives in `runtime/src/parser.rs`. Keep browser-independent behavior there so it remains covered by ordinary Rust tests. `wasm/src/lib.rs` should remain a thin conversion from the Rust result to a JavaScript object.

If the return shape changes, update the custom TypeScript declaration in `wasm/src/lib.rs`, its Wasm test, this document, and any downstream worker protocol together. Avoid adding timers, DOM access, or worker creation to the crate: those policies belong to the embedding application.

Validate changes with:

```bash
cargo test --workspace
cargo check --workspace --target wasm32-unknown-unknown
wasm-pack test --node wasm
wasm-pack build wasm --target web --release --out-dir pkg
```

## Configuration

There are no environment variables or compile-time feature flags. The caller supplies all runtime configuration through `ascii` and `max_steps`. Wall-clock timeout duration is a host-side setting and is not passed into Wasm.

## Dependencies

The wrapper relies on the internal `runtime` crate for parsing and interpretation, `wasm-bindgen` for generated JavaScript bindings, and `js-sys` for constructing the returned JavaScript object. Wasm tests use `wasm-bindgen-test`. Building requires the `wasm32-unknown-unknown` Rust target and `wasm-pack`.
