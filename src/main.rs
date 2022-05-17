use clap::Parser;
use dotenv::dotenv;

mod request;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Show hello message
    #[clap(long)]
    hello: bool,

    /// Show env
    #[clap(long)]
    env: bool,

    /// Send get request
    #[clap(long)]
    request: bool,
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();

    if args.hello {
        println!("Hello, world!");
    }

    if args.env {
        let key = "MY_ENV_VAR";
        let val = utils::get_env(key);
        println!("{} = {}", key, val);
    }

    if args.request {
        let url = "https://google.com";
        let res = request::send_get_request(url).await;
        assert_eq!(res.is_ok(), true);
    }

}
