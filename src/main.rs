use dioxus::prelude::*;
mod frontend;
mod server_db;
use frontend::App;
fn main() {
    launch(App);
}
