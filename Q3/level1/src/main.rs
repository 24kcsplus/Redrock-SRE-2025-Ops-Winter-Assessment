use std::env;
use std::io::{Write, stdin, stdout};
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split('|').peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command= match parts.next() { 
                Some(cmd) => cmd,
                None => {
                    continue;
                }
            };
            let mut args = parts;

            match command {
                "exit" => return,
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
                    previous_command = None
                }
                "pwd" => match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("pwd: 获取当前目录时错误: {}", e),
                },
                "echo" => {
                    let output = args.collect::<Vec<&str>>().join(" ");
                    println!("{}", output);
                }
                _ => {
                    let stdin = previous_command.take().map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let child = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match child {
                        Ok(child) => {
                            previous_command = Some(child);
                        }
                        Err(e) => {
                            eprintln!("执行指令错误: {}", e);
                            previous_command = None;
                            break;
                        }
                    }
                }
            }
        }
            if let Some(mut final_command) = previous_command {
                if let Err(e) = final_command.wait() {
                    eprintln!("等待子进程时出错: {}", e);
                }
            }
    }
}
