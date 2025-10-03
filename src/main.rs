use dioxus::prelude::*;
mod frontend;
mod server_db;
use frontend::route_handler;
fn main() {
    launch(route_handler);
}
