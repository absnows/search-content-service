use std::env;

pub mod app;

fn main() {
    // environment log configration
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }

    env_logger::init();

    let _ = app::start();
}
