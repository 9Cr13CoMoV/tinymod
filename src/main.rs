use csv::ReaderBuilder;
use std::env::current_dir;
use std::fs::*;
use std::io;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // don't really need a config file: just compare read_dir() before and after
    // this script, the app, and the files of concern will all be pre-placed in the same usr dir
    let mut config_check = false;
    let mut exe_check = false;
    for entry_result in read_dir(work_dir()).unwrap() {
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
        config_generator();
    }

    let config_file = read_to_string(config_filepath()).unwrap();
    let mut config_iterator = config_file.lines();
    let mut data_ids = Vec::new();
    for line in config_iterator {
        data_ids.push(line);
    }

    let mut app_path = work_dir();
    app_path.push("tinySA-App.exe");
    let app_return = Command::new(app_path).output();

    // intention: compare the results of read_dir() before and after running the app.
    // problem: the handle to the "after" data doesn't want to be brought outside the scope

    if let Ok(_ret_val) = app_return {
        let mut new_iter = config_file.lines();
        for entry_result in read_dir(work_dir()).unwrap() {
            if let Some(filename) = entry_result.unwrap().path().file_name() {
                if !data_ids.contains(&filename.to_str().unwrap()) {
                    data_ids.push(filename.to_str().unwrap());

                    let mut filepath = work_dir();
                    filepath.push(filename);
                    let mut file_to_convert =
                        ReaderBuilder::new().has_headers(false).from_path(filepath);
                    let mut file = file_to_convert.unwrap().records();
                    for data in file {}
                    //let reader = ReaderBuilder::new()
                    //                .delimiter(b';' )
                    //.from_reader()
                    // Export file without quotes and headers
                }
            }
        }
        // write updated config file
    }
}

fn work_dir() -> PathBuf {
    let mut work_path = PathBuf::new();
    work_path.push(current_dir().unwrap());
    work_path
}

fn config_filepath() -> PathBuf {
    let mut config_path = work_dir();
    config_path.push("tinymod.config");
    config_path
}
fn config_generator() {
    write(config_filepath(), "");
}
