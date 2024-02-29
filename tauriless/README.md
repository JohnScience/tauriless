# tauriless

Run a Tauri-like application without installation.

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
