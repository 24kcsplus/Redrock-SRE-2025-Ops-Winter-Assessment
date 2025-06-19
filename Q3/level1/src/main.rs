mod input_and_output;

use crate::input_and_output::read_command_line;
use std::env;
use std::fs::{File, OpenOptions};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};
use ctrlc;

fn main() {

    ctrlc::set_handler(|| {}).expect("设置 Ctrl-C 处理器时出错");

    loop {

        let input = match read_command_line() {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                line
            }
            Err(input_and_output::ReadLineError::Interrupted) => {
                continue;
            }
            Err(input_and_output::ReadLineError::Io(e)) => {
                eprintln!("读取输入错误: {}", e);
                break;
            }
        };

        // 使用临时占位符来避免替换冲突
        // 简单替换的结果太难蚌了，直到大模型指出来之前我都没意识到这连在一起的两个也给替换了
        let input = input
            .replace(">>", "\u{E000}")
            .replace(">", " > ")
            .replace("<", " < ")
            .replace("\u{E000}", " >> ");

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
        let mut failed = false; // 标记管道是否失败

        unsafe {
            while let Some(command_str) = commands.next() {

                // 解析重定向和管道

                if failed {
                    children.clear();
                    break;
                }

                let mut parts = command_str.trim().split_whitespace();
                let command = match parts.next() {
                    Some(cmd) => cmd,
                    None => {
                        eprintln!("执行管道时错误: 出现空命令");
                        failed = true;
                        continue;
                    }
                };

                // 将args改为动态数组
                // 解析重定向符号
                let (args, stdin_file, stdout_file, redirect_failed) = parse_redirections(&mut parts);
                failed = redirect_failed;

                if failed {
                    continue;
                }

                let stdin = if let Some(filename) = stdin_file {
                    match File::open(&filename) {
                        Ok(file) => Stdio::from(file),
                        Err(e) => {
                            eprintln!("无法打开输入文件 {}: {}", filename, e);
                            failed = true;
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
                            failed = true;
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
                    .pre_exec(|| {
                        libc::signal(libc::SIGINT, libc::SIG_DFL);
                        Ok(())
                    })
                    .spawn();

                match child_process {
                    Ok(child) => {
                        children.push(child);
                    }
                    Err(e) => {
                        eprintln!("执行指令错误: {}: {}", command, e);
                        failed = true;
                        continue;
                    }
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

fn parse_redirections(parts: &mut dyn Iterator<Item=&str>) -> (Vec<String>,Option<String>, Option<(String, bool)>,bool) {
    
    // 解析重定向符号
    
    let mut args = Vec::new();
    let mut stdin_file = None;
    let mut stdout_file = None;
    let mut redirect_failed = false;

    while let Some(part) = parts.next() {
        match part {
            "<" => {
                if let Some(filename) = parts.next() {
                    stdin_file = Some(filename.to_string());
                } else {
                    eprintln!("语法错误: `<` 后缺少路径");
                    redirect_failed = true;
                    break;
                }
            },
            ">" => {
                if let Some(filename) = parts.next() {
                    stdout_file = Some((filename.to_string(), false));
                } else {
                    eprintln!("语法错误: `>` 后缺少路径");
                    redirect_failed = true;
                    break;
                }
            },
            ">>" => {
                if let Some(filename) = parts.next() {
                    stdout_file = Some((filename.to_string(), true));
                } else {
                    eprintln!("语法错误: `>>` 后缺少路径");
                    redirect_failed = true;
                    break;
                }
            },
            _ => args.push(part.to_string()),
        }
    }
    (args, stdin_file, stdout_file, redirect_failed)
}