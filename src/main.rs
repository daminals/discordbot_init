// discordbot
// Daniel Kogan
// 02.20.2022

use chrono;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::fs::File;
// use std::io::prelude::*;
// use std::io::{self, Result, Write};
use std::io::Write;
// use std::path::Path;
use std::process::{exit, Command, Stdio};
// use clap::Command as cliCommand;
use clap::Parser;
use clap::{Arg, ArgAction, ArgMatches};

mod config;

static RED: &str = "\u{001b}[31m";
static GREEN: &str = "\u{001b}[32m";
static BOLD: &str = "\u{001b}[1m";
static UNDERLINE: &str = "\u{001b}[4m";
static CLEAR_FORMAT: &str = "\u{001b}[0m";

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Daniel Kogan",
    about = "Tool to create a simple discord bot project"
)]

struct Arguments {
    // Quiet mode - No output of which step is being done
    #[clap(short, long, help = "Suppresses output of which step is being done", action=ArgAction::SetTrue)]
    quiet: bool, // default is false

    #[clap(short, long, help = "Suppresses output of which step is being done", action=ArgAction::SetTrue)]
    traditional: bool, // default is false

}

fn main() {
    let args = Arguments::parse();
    // dotenv::from_path("~/bin/rust-discordbot-init/.env").expect("Encountered an error reading .env");
    ctrlc::set_handler(move || {
        // exit program early
        println!("{} Exiting Program...{}", RED, CLEAR_FORMAT);
        exit(0); // actually exit program
    })
    .expect("Error setting Ctrl-C handler");

    create_discord_bot_project(!args.quiet);
}

fn create_discord_bot_project(verbose: bool) {
    if verbose {
        println!("+================================+");
    }
    println!("+  {}Creating discord bot project{}  +", BOLD, CLEAR_FORMAT);
    if verbose {
        println!("+================================+");
    }
    // println!("[1]: Create bot.py");
    // println!("[2]: Create .env and .env.example");
    // println!("[3]: Create Readme.md");
    // println!("[4]: Create run.sh");
    // println!("[5]: Create requirements.txt");
    // println!("[6]: Run git init");
    // println!("[7]: Create venv");
    // println!("[8]: Install requirements.txt");
    // println!("[9]: Give run.sh permissions");

    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template("\u{001b}[1m[{elapsed}]\u{001b}[0m[{wide_bar:.green}]\u{001b}[1m({eta})\u{001b}[0m")
            .unwrap()
            .progress_chars("#>-"),
    );
    if verbose {
        bar.println(format!("{}[1]:{} Create bot.py", BOLD, CLEAR_FORMAT));
    }
    create_bot();
    bar.inc(6);

    if verbose {
        bar.println(format!(
            "{}[2]:{} Create .env and .env.example",
            BOLD, CLEAR_FORMAT
        ));
    }
    create_dotenv();
    bar.inc(6);

    if verbose {
        bar.println(format!("{}[3]:{} Create Readme.md", BOLD, CLEAR_FORMAT));
    }
    create_readme();
    bar.inc(2);

    if verbose {
        bar.println(format!("{}[4]:{} Create run.sh", BOLD, CLEAR_FORMAT));
    }
    create_run();
    bar.inc(2);

    if verbose {
        bar.println(format!(
            "{}[5]:{} Create requirements.txt",
            BOLD, CLEAR_FORMAT
        ));
    }
    create_requirements();
    bar.inc(5);

    if verbose {
        bar.println(format!("{}[6]:{} Run git init", BOLD, CLEAR_FORMAT));
    }

    git_init();
    bar.inc(10);

    if verbose {
        bar.println(format!("{}[7]:{} Create venv", BOLD, CLEAR_FORMAT));
    }
    create_venv();
    bar.inc(40);

    if verbose {
        bar.println(format!(
            "{}[8]:{} Install requirements.txt",
            BOLD, CLEAR_FORMAT
        ));
    }
    install_requirements();
    bar.inc(20);

    if verbose {
        bar.println(format!(
            "{}[9]:{} Give run.sh permissions",
            BOLD, CLEAR_FORMAT
        ));
    }
    give_permissions();
    bar.inc(4);

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
    let date: &str = &String::from(format!("# {}\n", get_day()))[..];
    content_new_lines[2] = date;
    for line in content_new_lines {
        if let Err(e) = writeln!(&mut bot, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
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
    // let venv_stdout = Command::new("sh")
    Command::new("sh")
        .arg("-c")
        .arg(venv)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // println!("Created venv");
}

fn install_requirements() {
    let download_py = "source venv/bin/activate && pip install -r requirements.txt";
    // let download_py_stdout = Command::new("sh")
    Command::new("sh")
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
        if let Err(e) = writeln!(&mut readme, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    // println!("Created README.md");
}
fn create_run() {
    let mut run = File::create("run.sh").expect("error handling");
    let run_file = config::run_file();
    let contents = fs::read_to_string(&run_file).expect("an error occured");
    let content_new_lines = contents.lines().collect::<Vec<_>>();
    for line in content_new_lines {
        if let Err(e) = writeln!(&mut run, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    // println!("Created run.sh");
}
fn give_permissions() {
    let give_perms = format!("chmod 777 run.sh");
    // let give_perms_spawn = Command::new("sh")
    Command::new("sh")
        .arg("-c")
        .arg(give_perms)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // println!("Gave run.sh permissions");
}

fn create_requirements() {
    let mut req = File::create("requirements.txt").expect("error handling");
    let req_file = config::requirements();
    let contents = fs::read_to_string(&req_file).expect("an error occured");
    let content_new_lines = contents.lines().collect::<Vec<_>>();
    for line in content_new_lines {
        if let Err(e) = writeln!(&mut req, "{}", line) {
            eprintln!("Couldn't write to file: {}", e.to_string());
        }
    }
    // println!("Created requirements.txt");
}
fn git_init() {
    let git_init_cmd = format!("git init");
    // let git_init_spawn = Command::new("sh")
    Command::new("sh")
        .arg("-c")
        .arg(git_init_cmd)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    // println!("initialized git")
}
