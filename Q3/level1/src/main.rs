use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main(){
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;

        match command {
            "exit" => break,
            "cd" => {
                let target_dir = match args.next() {
                    None | Some("~") => match home::home_dir() {
                        Some(path) => path,
                        None => {
                            eprintln!("cd: 找不到主目录");
                            continue;
                        }
                    },
                    Some(path) => {
                        if path.starts_with('~') {
                            let home_dir = match home::home_dir() {
                                Some(path) => path,
                                None => {
                                    eprintln!("cd: 找不到主目录");
                                    continue;
                                }
                            };
                            home_dir.join(&path[2..])
                        } else {
                            std::path::PathBuf::from(path)
                        }
                    }
                };

                if let Err(e) = env::set_current_dir(&target_dir) {
                    eprintln!("cd: {}: {}", target_dir.display(), e);
                }
            },
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("pwd: 获取当前目录时错误: {}", e),
                }
            },
            "echo" => {
                let output = args.collect::<Vec<&str>>().join(" ");
                println!("{}", output);
            },
            _ => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => {
                        let _ = child.wait();
                    },
                    Err(e) => {
                        eprintln!("执行指令错误: {}", e);
                    }
                }
            }
        }
    }
}