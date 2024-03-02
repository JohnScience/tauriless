# tauriless-js

[![npm](https://img.shields.io/npm/v/tauriless-js)](https://www.npmjs.com/package/tauriless-js)
[![npm](https://img.shields.io/npm/dt/tauriless-js)](https://www.npmjs.com/package/tauriless-js)
[![npm](https://img.shields.io/npm/l/tauriless-js)](https://www.npmjs.com/package/tauriless-js)

JS WASM bindings for [`tauriless`], a [Tauri](https://tauri.app/)-like cross-platform application framework. Unlike Tauri, `tauriless` **does not require installation** but doesn't support all the features of Tauri, is not a long-term solution, and is not a replacement for Tauri.

For more information, see the [`README` of `tauriless` repository].

## Adding as a dependency

Add `tauriless-js` as a dependency to your project:

```console
npm install tauriless-js
```

If you use a bundler like `Vite`, also ensure that it can work with WASM by using [`vite-plugin-wasm`](https://www.npmjs.com/package/vite-plugin-wasm) with [`vite-plugin-top-level-await`](https://www.npmjs.com/package/vite-plugin-top-level-await).

Example `vite.config.ts`

```ts
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ]
})
```

## Usage

```ts
import init, { invoke, encode } from "tauriless-js";

init().then(() => {
    console.log("tauriless-js initialized!");
    // encode allows you to see how the data is encoded before sending it to the Core process of `wry`
    const encoded = encode({ num: 42 });
    console.log("Result of tauriless_js.encode(): ", encoded);
    // invoke allows you to call a command in the Core process of `wry`.
    //
    // Whether the command is synchronous or asynchronous, it will return a Promise because
    // XHR with binary data must be asynchronous.
    //
    // When the command accepts multiple arguments, you pass a heterogeneous array rather than an object.
    // For example, invoke("do_stuff_with_num_and_str", [42, "hello"]);
    const v: Promise<unknown> = invoke("do_stuff_with_num", { num: 42 });
    v.then((result) => {
      console.log("Result of tauriless_js.invoke(): ", result);
    })
});
```

## Want type-safe bindings specific to your commands?

Send an email to <mailto:demenev.dmitriy1@gmail.com> and I will consider adding that to the library.

[`tauriless`]: https://github.com/JohnScience/tauriless/
[`README` of `tauriless` repository]: https://github.com/JohnScience/tauriless/
