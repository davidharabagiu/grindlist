#[macro_use]
extern crate nickel;

use clap::Parser;
use nickel::{HttpRouter, Nickel};

mod dao;
mod routes;

use dao::DAO;

/// Todo list app
#[derive(Parser)]
#[clap(author, about)]
struct Args {
    /// Server port
    #[clap(short, long, value_parser, default_value = "8080")]
    port: u16,

    /// SQLite database file
    #[clap(short = 'f', long, value_parser, default_value = "./grindlist.db")]
    db_file: String,
}

fn main() {
    let args = Args::parse();

    let dao = DAO::new(&args.db_file).expect("Failed to open database");

    let mut server = Nickel::new();
    server.get("/", middleware!(routes::hello()));

    server
        .listen(format!("127.0.0.1:{}", args.port))
        .expect("Failed to start server");
}
