use crate::{commands::list::PinkColor, Arguments};

pub fn execute(_: Arguments) {
    println!("uki {} by {}", env!("CARGO_PKG_VERSION").accent(), "smokingplaya".accent());
    println!("github {}", "https://github.com/smokingplaya/uki".accent());
    println!("made with {} <3", "love".accent());
}