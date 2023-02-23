use std::env;
use load_dotenv::load_dotenv;
// call load_dotenv!() before using dotenv
load_dotenv!();

pub fn bot_file() -> String {
  return String::from(env!("HOMEDIR")) + "bot.txt";
} 

pub fn run_file () -> String {
  return String::from(env!("HOMEDIR")) + "/bot";
}

pub fn requirements() -> String {
  return String::from(env!("HOMEDIR")) + "/requirements.txt";
}

pub fn readme_file() -> String {
  return String::from(env!("HOMEDIR")) + "/readme.txt";
}

pub fn add_token() -> String {
  return String::from("TOKEN=") + &env!("TOKEN");
}
