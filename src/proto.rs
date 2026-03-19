pub mod iterm2 {
    include!(concat!(env!("OUT_DIR"), "/iterm2.rs"));
}

pub use iterm2::*;
