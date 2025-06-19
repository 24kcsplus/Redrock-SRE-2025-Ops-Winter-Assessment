use std::env;
use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            continue;
        }

        let first_command_str = input.trim().split('|').next().unwrap_or("").trim();
        let mut parts = first_command_str.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match command {
            "" => continue,
            "exit" => return,
            "cd" => {
                let target_dir = match args.get(0) {
                    None | Some(&"~") => home::home_dir(),
                    Some(path) => {
                        if path.starts_with("~/") {
                            home::home_dir().map(|mut p| {
                                p.push(&path[2..]);
                                p
                            })
                        } else {
                            Some(std::path::PathBuf::from(*path))
                        }
                    }
                };

                if let Some(dir) = target_dir {
                    if let Err(e) = env::set_current_dir(&dir) {
                        eprintln!("cd: {}: {}", dir.display(), e);
                    }
                } else {
                    eprintln!("cd: 找不到主目录或路径无效");
                }
                continue;
            }
            _ => {}
        }

        let mut commands = input.trim().split('|').peekable();
        let mut children: Vec<Child> = vec![];

        while let Some(command_str) = commands.next() {
            let mut parts = command_str.trim().split_whitespace();
            let command = match parts.next() {
                Some(cmd) => cmd,
                None => {
                    eprintln!("执行管道时错误: 出现空命令");
                    children.clear();
                    break;
                }
            };
            let args = parts;

            let stdin = if let Some(last_child) = children.last_mut() {
                if let Some(stdout) = last_child.stdout.take() {
                    Stdio::from(stdout)
                } else {
                    Stdio::inherit()
                }
            } else {
                Stdio::inherit()
            };

            let stdout = if commands.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };

            let child_process = Command::new(command)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();

            match child_process {
                Ok(child) => {
                    children.push(child);
                }
                Err(e) => {
                    eprintln!("执行指令错误: {}: {}", command, e);
                    children.clear();
                    break;
                }
            }
        }

        for mut child in children {
            if let Err(e) = child.wait() {
                eprintln!("等待子进程时出错: {}", e);
            }
        }
    }
}