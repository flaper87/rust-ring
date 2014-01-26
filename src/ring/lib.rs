#[crate_id = "github.com/flaper87/rust-ring#rust-ring:0.1"];
#[desc = "Consistent hashing ring lib for Rust"];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod extra;

extern mod ssl = "github.com/sfackler/rust-openssl#openssl:0.0";
pub mod ring;
