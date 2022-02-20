// discordbot
// Daniel Kogan
// 02.20.2022

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio, exit};
use std::path::Path;
use std::fmt;
use std::io::{self, Write, Result};
use chrono;

mod config;

fn main() {
    println!("Hello World!");
}
fn get_day() -> String {
    let now = format!("{:?}", chrono::offset::Local::now());
    let f_now = now.split("T").collect::<Vec<&str>>();
    let disorganized = f_now[0].split("T").collect::<Vec<&str>>();
    let organized = organize_date(disorganized);
    return organized.to_owned()
}
fn organize_date(collection: Vec<&str>) -> String{
    let mut return_str = String::from("");
    return_str += collection[1];
    return_str += ".";
    return_str += collection[2]; 
    return_str += ".";
    return return_str + collection[0];
}

fn create_bot () {
    let mut bot = File::create("bot.py").expect("error handling");
    let template_bot = format!("");
    let contents = fs::read_to_string(&template_bot).expect("an error occured");
    let mut content_new_lines = contents.lines().collect::<Vec<_>>();
    let date = &String::from(get_day())[..];
    content_new_lines[3] = date;
    for line in content_new_lines {
        writeln!(&mut bot, "{}", line);
    }
    println!("Created bot.py");
}
fn create_dotenv() {
    let mut dotenv = File::create(".env").expect("error handling");
    writeln!(&mut dotenv, "TOKEN=\"\"").unwrap();
    println!("Created .env");
}
fn create_venv() {
    let venv = "python3 -m venv venv";
    let venv_stdout = Command::new("sh").arg("-c").arg(venv)
        .stdout(Stdio::piped()).output().unwrap();
    println!("Created venv");
}
fn create_readme() {
    let get_basedir_cmd = format!("echo $(basename \"$PWD\")");
    let get_basedir_spawn = Command::new("sh").arg("-c").arg(get_basedir_cmd).stdout(Stdio::piped()).output().unwrap();
    let get_basedir_str = String::from_utf8(get_basedir_spawn.stdout).unwrap();
    let mut readme = File::create("Readme.md").expect("error handling");
    writeln!(&mut readme, "# {}", get_basedir_str);
    println!("Created Readme");
}
fn create_run() {

}