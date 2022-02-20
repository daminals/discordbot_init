// discordbot
// Daniel Kogan
// 02.20.2022

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

fn main() {
    println!("Hello World!");
}

fn create_bot () -> std::io::Result<()> {
    let mut bot = File::create("bot.py");



    writeln!(&bot, "");
    return Ok(());
}
fn create_dotenv() -> std::io::Result<()> {
    let mut dotenv = File::create(".env");
    writeln!(&dotenv, "TOKEN=\"\"").unwrap();
    return Ok(());
}
