use std::env;

// 他のcrateに実装されているmacroを使用する
#[macro_use]
// crateのimport
// url: https://doc.rust-jp.rs/rust-by-example-ja/crates/link.html
extern crate log;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // 引数を受け取る
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [top|upd] [server|client] [addr:port].");
        std::process::exit(1);
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => {
                println!("tcp server");
            }
            _ => {
                missing_role();
            }
        }
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
            std::process::exit(1);
        }
    }
}

// roleが不正な値であるときにエラーを出す
fn missing_role(){
    error!("Please specify server or client on the 2nd argument.");
    std::process::exit(1);
}
