// json parser lib for use in data
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

// terminal display library
extern crate termion;

pub mod resource;
pub mod io;
pub mod state;