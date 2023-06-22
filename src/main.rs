use std::env;
use std::env::current_dir;
use std::fs::*;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use fltk::{prelude::*, *};

#[derive(Debug)]
enum TSARS_ERR {
    OK,
    MissingConfig,
    WrongAppPath,
    WrongDataPath,
    ScriptFailed,
    BAD_DATA,
}
fn main() {
    let work_dir = current_dir().unwrap();

    let mut config_check = false;
    let mut exe_check = false;
    for entry_result in read_dir(work_dir).unwrap() {
        if let Some(filename) = entry_result.unwrap().path().file_name() {
            match filename.to_str() {
                Some("tinymod.config") => config_check = true,
                Some("tinySA-App.exe") => exe_check = true,
                _ => (),
            }
        }
    }
    if !exe_check {
        println!("TinySA-App.exe not located.");
        return;
    }
    if !config_check {
        config_prompt();
    }

    let mut config_path = PathBuf::new();
    config_path.push(current_dir().unwrap());
    config_path.push("tinymod.config");
    let config_file = read_to_string(config_path).unwrap();
    let mut config_iterator = config_file.lines();
    let data_path = config_iterator.next().unwrap();
    let mut data_ids = Vec::new();
    for line in config_iterator {
        data_ids.push(line);
    }

}

fn config_prompt() {
    println!("tinymod needs to be configured");
    println!("Enter the directory where CSVs will be saved: ");
    let mut usr_in = String::new();
    if let Ok(in_n) = io::stdin().read_line(&mut usr_in) {
        write("tinymod.config", usr_in);
    }
}
