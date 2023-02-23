// discordbot
// Daniel Kogan
// 02.20.2022

use chrono;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Result, Write};
use std::path::Path;
use std::process::{exit, Command, Stdio};

mod config;

fn main() {
    // dotenv::from_path("~/bin/rust-discordbot-init/.env").expect("Encountered an error reading .env");
    create_discord_bot_project();
}

fn create_discord_bot_project() {
    println!("Creating discord bot project⏳");
    println!("[1]: Create bot.py");
    println!("[2]: Create .env and .env.example");
    println!("[3]: Create Readme.md");
    println!("[4]: Create run.sh");
    println!("[5]: Create requirements.txt");
    println!("[6]: Run git init");
    println!("[7]: Create venv");
    println!("[8]: Install requirements.txt");
    println!("[9]: Give run.sh permissions");

    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template("[{elapsed}][{wide_bar:.green}]({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    create_bot();
    bar.inc(10);
    create_dotenv();
    bar.inc(6);
    create_readme();
    bar.inc(2);
    create_run();
    bar.inc(2);
    create_requirements();
    bar.inc(5);
    git_init();
    bar.inc(10);
    create_venv();
    bar.inc(40);
    install_requirements();
    bar.inc(20);
    bar.finish();
}

fn get_day() -> String {
    let now = format!("{:?}", chrono::offset::Local::now());
    let f_now = now.split("T").collect::<Vec<&str>>();
    let disorganized = f_now[0].split("-").collect::<Vec<&str>>();
    let organized = organize_date(disorganized);
    return organized.to_owned();
}

fn organize_date(collection: Vec<&str>) -> String {
    let mut return_str = String::from("");
    return_str += collection[1];
    return_str += ".";
    return_str += collection[2];
    return_str += ".";
    return return_str + collection[0];
}

fn create_bot() {
    let mut bot = File::create("bot.py").expect("error handling");
    let template_bot = config::bot_file();
    let contents = fs::read_to_string(&template_bot).expect("an error occured");
    let mut content_new_lines = contents.lines().collect::<Vec<_>>();
    let date = &String::from(format!("# {}\n", get_day()))[..];
    content_new_lines[2] = date;
    for line in content_new_lines {
        writeln!(&mut bot, "{}", line);
    }
    // println!("Created bot.py");
}
fn create_dotenv() {
    let mut dotenv = File::create(".env").expect("error handling");
    let token = config::add_token();
    writeln!(&mut dotenv, "{}", token).unwrap();
    // println!("Created .env");
    let mut dotenv_example = File::create(".env.example").expect("error handling");
    writeln!(&mut dotenv_example, "TOKEN=token").unwrap();
    // println!("Created .env.example");
}
fn create_venv() {
    let venv = "python3 -m venv venv";
    let venv_stdout = Command::new("sh")
        .arg("-c")
        .arg(venv)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // println!("Created venv");
}

fn install_requirements() {
    let download_py = "source venv/bin/activate && pip install -r requirements.txt";
    let download_py_stdout = Command::new("sh")
        .arg("-c")
        .arg(download_py)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // println!("Installed requirements.txt");
}

fn create_readme() {
    let mut readme = File::create("README.md").expect("error handling");
    let readme_file = config::readme_file();
    let contents = fs::read_to_string(&readme_file).expect("an error occured");
    let content_new_lines = contents.lines().collect::<Vec<_>>();
    for line in content_new_lines {
        writeln!(&mut readme, "{}", line);
    }
    // println!("Created README.md");
}
fn create_run() {
    let mut run = File::create("run.sh").expect("error handling");
    let run_file = config::run_file();
    let contents = fs::read_to_string(&run_file).expect("an error occured");
    let content_new_lines = contents.lines().collect::<Vec<_>>();
    for line in content_new_lines {
        writeln!(&mut run, "{}", line);
    }
    let give_perms = format!("chmod 777 run.sh");
    let give_perms_spawn = Command::new("sh")
        .arg("-c")
        .arg(give_perms)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // println!("Created run.sh");
}
fn create_requirements() {
    let mut req = File::create("requirements.txt").expect("error handling");
    let req_file = config::requirements();
    let contents = fs::read_to_string(&req_file).expect("an error occured");
    let content_new_lines = contents.lines().collect::<Vec<_>>();
    for line in content_new_lines {
        writeln!(&mut req, "{}", line);
    }
    // println!("Created requirements.txt");
}
fn git_init() {
    let git_init_cmd = format!("git init");
    let git_init_spawn = Command::new("sh")
        .arg("-c")
        .arg(git_init_cmd)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // println!("initialized git")
}
