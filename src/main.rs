use csv::{ReaderBuilder, StringRecord, Writer, WriterBuilder};
use std::env::current_dir;
use std::fs::*;
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
                let new_name = name.clone();

                // do the conversion to these files
                if let Ok(mut f_rdr) = ReaderBuilder::new()
                    .has_headers(false)
                    .delimiter(b';')
                    .from_path(name)
                {
                    let mut temp_list = Vec::<StringRecord>::new();
                    for res_line in f_rdr.records() {
                        if let Ok(line) = res_line {
                            let mut new_line = StringRecord::new();
                            for field in line.iter() {
                                if let Ok(val) = field.trim().parse::<f32>() {
                                    match val.is_sign_positive() {
                                        true => {
                                            new_line.push_field(&format!("{:.3}", val / 1000000.0))
                                        }
                                        false => {
                                            new_line.push_field(&format!("{:.0}", val * 0.8686))
                                        }
                                    }
                                }
                            }
                            temp_list.push(new_line);
                        }
                    }
                    let mut f_write = WriterBuilder::new()
                        .delimiter(b',')
                        .has_headers(false)
                        .from_path(new_name)
                        .unwrap();
                    for line in temp_list {
                        f_write.write_record(&line);
                    }
                }
            }
        }
    }
}

fn c_d() -> PathBuf {
    current_dir().unwrap()
}
