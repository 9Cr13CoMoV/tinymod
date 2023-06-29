use csv::ReaderBuilder;
use std::env::current_dir;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::*;
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // make list of files in dir
    let mut pre_dir = Vec::new();
    for entry_result in read_dir(c_d()).unwrap() {
        if let Ok(fname) = entry_result {
            pre_dir.push(fname.path());
        }
    }

    // run the app
    let mut app_path = c_d();
    app_path.push("tinySA-App.exe");
    let app_return = Command::new(app_path).output();

    // make new list of files in the dir
    let mut post_dir = Vec::new();
    for entry_result in read_dir(c_d()).unwrap() {
        if let Ok(fname) = entry_result {
            post_dir.push(fname.path());
        }
    }

    if let Ok(_ret_val) = app_return {

        //compare pre and post
        for name in post_dir {
            if !pre_dir.contains(&name) {

                // do the conversion to files made during the app session
                
            }
        }
    }
}

fn c_d() -> PathBuf {
    current_dir().unwrap()
}
