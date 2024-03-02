# tauriless

[![Crates.io](https://img.shields.io/crates/v/tauriless)](https://crates.io/crates/tauriless)
[![Downloads](https://img.shields.io/crates/d/tauriless.svg)](https://crates.io/crates/tauriless)
[![Documentation](https://docs.rs/tauriless/badge.svg)](https://docs.rs/tauriless)
[![License](https://img.shields.io/crates/l/tauriless)](https://crates.io/crates/tauriless)

Run a Tauri-like application without installation.

## Warning

This crate is a temporary solution to the problem of running Tauri-like applications without installation. It is not a replacement for Tauri, and it is not a long-term solution. It is a workaround for the time being.

Also, the library was tested only for Windows and is not guaranteed to work on other platforms. If you want to help with testing on other platforms, please open an issue.

Currently, the library can't even guarantee that the binary will work on desktop machines of the majority of the users because of runtime dependencies on `WebView2` and `vcredist`. While `WebView2` comes pre-installed on Windows 10, it is not the case for `vcredist`. The dependency on the latter can be resolved by using [`target-feature=+crt-static`](https://rust-lang.github.io/rfcs/1721-crt-static.html).

## Usage

 ```rust, no_run
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use tauriless::{command, commands, WebViewBuilderExt};

#[command]
fn argsless_sync_command() {}

#[command]
async fn async_command_with_args(n: i32) -> i32 {
   // some async code
   n * 2
}

fn main() -> wry::Result<()>     {
   let rt = tokio::runtime::Builder::new_multi_thread()
       .enable_all()
       .build()
       .unwrap();
   // This allows us to use tokio::spawn inside wry asynchronous custom protocol handlers.
   // Since wry doesn't allow us to pass a runtime to the WebViewBuilder, we have to use a global runtime.
   let _rt_guard = rt.enter();

   let event_loop = EventLoop::new();
   let window = WindowBuilder::new()
       .with_title("My Tauriless App")
       .build(&event_loop)
       .unwrap();

    // ...
    
    let _webview = WebViewBuilder::new(&window)
        // ...
        .with_tauriless_commands(commands![argsless_sync_command, async_command_with_args])
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => (),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
 ```

## Eliminating the dependency on vcredist on Windows

Usually, to run a Rust executable on a Windows machine, the user must have `vcredist` installed. See <https://stackoverflow.com/questions/52153676/what-is-the-requirements-for-running-a-rust-compiled-program-on-another-windows>.

In order to avoid the problem it's recommended to add [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) with the following content:

```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-Ctarget-feature=+crt-static"]

[target.i686-pc-windows-msvc]
rustflags = ["-Ctarget-feature=+crt-static"]

[target.i586-pc-windows-msvc]
rustflags = ["-Ctarget-feature=+crt-static"]
```

It will ensure that the Rust compiler links the C runtime statically, so the user won't need to install `vcredist`. It increases the size of the binary[^1], but it's a reasonable trade-off for a standalone application.

[^1]: What's the the size increase?
