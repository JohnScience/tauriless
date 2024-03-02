# Tauriless

Run a Tauri-like application without installation as a standalone binary.

See the related discussion for Tauri <https://github.com/tauri-apps/tauri/discussions/3048>.

## Warnings

This workspace is a temporary solution to the problem of running Tauri-like applications without installation. It is not a replacement for Tauri, and it is not a long-term solution. It is a workaround for the time being. Also, a small API breakage is expected soon.

Also, the library was tested only for Windows and is not guaranteed to work on other platforms. If you want to help with testing on other platforms, please open an issue.

Currently, the library can't even guarantee that the binary will work on desktop machines of the majority of the users because of runtime dependencies on `WebView2` and `vcredist`. While `WebView2` comes pre-installed on Windows 10, it is not the case for `vcredist`. The dependency on the latter can be resolved by using [`target-feature=+crt-static`](https://rust-lang.github.io/rfcs/1721-crt-static.html).

## Usage

The public API for `tauriless` consists of two parts:

* `tauriless` crate for Core process of [`wry`](https://crates.io/crates/wry), which is a cross-platform WebView rendering library,
* `tauriless-js` JavaScript/TypeScript `npm` library for the WebView process of `wry` application.

### Tauirless crate

[![Crates.io](https://img.shields.io/crates/v/tauriless)](https://crates.io/crates/tauriless)
[![Downloads](https://img.shields.io/crates/d/tauriless.svg)](https://crates.io/crates/tauriless)
[![Documentation](https://docs.rs/tauriless/badge.svg)](https://docs.rs/tauriless)
[![License](https://img.shields.io/crates/l/tauriless)](https://crates.io/crates/tauriless)

See [tauriless](./tauriless) directory.

### Tauriless-js npm package

[![npm](https://img.shields.io/npm/v/tauriless-js)](https://www.npmjs.com/package/tauriless-js)
[![npm](https://img.shields.io/npm/dt/tauriless-js)](https://www.npmjs.com/package/tauriless-js)
[![npm](https://img.shields.io/npm/l/tauriless-js)](https://www.npmjs.com/package/tauriless-js)

See [tauriless-js](./tauriless-js) directory.
