#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate json_gettext;

use json_gettext::JSONGetText;

lazy_static! {
    static ref CTX: JSONGetText<'static> = static_json_gettext_build!(
        "en_GB",
        "en_GB",
        "lang/en_GB.json",
        "de_DE",
        "lang/de_DE.json"
    )
    .unwrap();
}

pub mod connection;
pub(crate) mod discord;
pub mod models;
pub mod schema;
pub(crate) mod util;
pub(crate) mod vtm;

fn main() {}
