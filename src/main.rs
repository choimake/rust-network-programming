use std::env;

// 他のcrateに実装されているmacroを使用する
#[macro_use]
// crateのimport
// url: https://doc.rust-jp.rs/rust-by-example-ja/crates/link.html
extern crate log;

// 外部moduleの読み込み
mod tcp_server;
mod tcp_client;

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
                // unwrap_or_else
                // エラー時にpanicを返すのではなく、クロージャーで実行した値を返す
                // ?演算子はOkなら中の値を返し、Errなら「即座」に値をreturnする
                // url: https://qiita.com/nirasan/items/321e7cc42e0e0f238254
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
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
