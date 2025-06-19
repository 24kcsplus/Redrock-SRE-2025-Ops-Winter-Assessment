use std::env;
use std::fs::{File, OpenOptions};
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
            },
            _ => {}
        }

        let mut commands = input.trim().split('|').peekable();
        let mut children: Vec<Child> = vec![];
        let mut pipeline_failed = false; // 标记管道是否失败
        
        while let Some(command_str) = commands.next() {
            
            if pipeline_failed {
                children.clear();
                break;
            }
            
            let mut parts = command_str.trim().split_whitespace();
            let command = match parts.next() {
                Some(cmd) => cmd,
                None => {
                    eprintln!("执行管道时错误: 出现空命令");
                    pipeline_failed = true;
                    continue;
                }
            };
            
            // 将args改为动态数组
            let mut args = Vec::new();
            let mut stdin_file: Option<String> = None;
            let mut stdout_file: Option<(String, bool)> = None; // (filename, is_append)

            while let Some(part) = parts.next() {
                match part {
                    "<" => {
                        if let Some(filename) = parts.next() {
                            stdin_file = Some(filename.to_string());
                        } else {
                            eprintln!("语法错误: `<` 后缺少路径");
                            pipeline_failed = true;
                            break;
                        }
                    },
                    ">" => {
                        if let Some(filename) = parts.next() {
                            stdout_file = Some((filename.to_string(), false));
                        } else {
                            eprintln!("语法错误: `>` 后缺少路径");
                            pipeline_failed = true;
                            break;
                        }
                    },
                    ">>" => {
                        if let Some(filename) = parts.next() {
                            stdout_file = Some((filename.to_string(), true));
                        } else {
                            eprintln!("语法错误: `>>` 后缺少路径");
                            pipeline_failed = true;
                            break;
                        }
                    },
                    _ => args.push(part.to_string()),
                }
            }

            if pipeline_failed {
                continue;
            }
            
            let stdin = if let Some(filename) = stdin_file {
                match File::open(&filename) {
                    Ok(file) => Stdio::from(file),
                    Err(e) => {
                        eprintln!("无法打开输入文件 {}: {}", filename, e);
                        pipeline_failed = true;
                        continue;
                    }
                }
            } else if let Some(last_child) = children.last_mut() {
                last_child.stdout.take().map_or(Stdio::inherit(), Stdio::from)
            } else {
                Stdio::inherit()
            };
            
            let stdout = if let Some((filename, append)) = stdout_file {
                let result = if append {
                    OpenOptions::new().create(true).append(true).open(&filename)
                } else {
                    File::create(&filename)
                };
                match result {
                    Ok(file) => Stdio::from(file),
                    Err(e) => {
                        eprintln!("无法创建/打开输出文件 {}: {}", filename, e);
                        pipeline_failed = true;
                        continue;
                    }
                }
            } else if commands.peek().is_some() {
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
                    pipeline_failed = true;
                    continue;
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