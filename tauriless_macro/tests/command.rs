use tauriless_macro::command;

#[command]
fn argsless_sync_command() {}

#[command]
async fn argsless_async_command() -> i32 {
    42
}

#[command]
fn command_with_args(_a: i32, _b: i32) {}

#[command]
fn command_with_args_and_return_type(_a: i32, _b: i32) -> i32 {
    42
}

// struct NotDeserializable;
// #[command]
// fn command_with_non_deserializable_arg(b: i32, _a: NotDeserializable) {}

fn main() {
    argsless_sync_command();
    command_with_args(1, 2);
    let _ = command_with_args_and_return_type(1, 2);
}
