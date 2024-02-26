use tauriless_macro::command;

mod tauriless {
    use std::borrow::Cow;
    use postcard::{from_bytes, to_stdvec};

    pub(crate) trait SyncCommand {
        type Args: for<'a> serde::Deserialize<'a>;
        type RetTy: serde::Serialize;
        const NAME: &'static str;

        fn command(args: Self::Args) -> Self::RetTy;

        fn custom_protocol(
            req: wry::http::request::Request<Vec<u8>>,
        ) -> wry::http::response::Response<Cow<'static, [u8]>> {
            let (_parts, body): (wry::http::request::Parts, Vec<u8>) = req.into_parts();
            let args: Self::Args = match from_bytes(&body) {
                Ok(args) => args,
                Err(_e) => {
                    return wry::http::response::Response::builder()
                        .status(wry::http::StatusCode::BAD_REQUEST)
                        .body(Cow::<'static, [u8]>::Borrowed(b"Bad request: failed to deserialize the body."))
                        .unwrap();
                }
            };
            let ret: Self::RetTy = Self::command(args);
            let resp_body = match to_stdvec(&ret) {
                Ok(resp_body) => resp_body,
                Err(_e) => {
                    return wry::http::response::Response::builder()
                        .status(wry::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Cow::<'static, [u8]>::Borrowed(b"Internal server error: failed to serialize the response."))
                        .unwrap();
                }
            };
            wry::http::response::Response::builder()
                .status(wry::http::StatusCode::OK)
                .body(Cow::Owned(resp_body))
                .unwrap()
        }
    }
}

#[command]
fn argsless_command() {}

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
    argsless_command();
    command_with_args(1, 2);
    let _ = command_with_args_and_return_type(1, 2);
}
