use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent,KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::env;
use std::io::{stdout, Write};
use unicode_width::UnicodeWidthStr;

// 定义一个自定义错误类型，以区分 I/O 错误和用户中断
#[derive(Debug)]
pub enum ReadLineError {
    Io(std::io::Error),
    Interrupted, // 代表 Ctrl+C
}

// 允许 `?` 运算符将 io::Error 自动转换为 ReadLineError::Io
impl From<std::io::Error> for ReadLineError {
    fn from(err: std::io::Error) -> ReadLineError {
        ReadLineError::Io(err)
    }
}

pub fn read_command_line(historys: &[String]) -> Result<String, ReadLineError> {

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    let mut buffer: Vec<char> = Vec::new();
    let mut position = 0; // 光标在缓冲区中的位置
    let username = whoami::username();
    let mut history_index = historys.len();

    loop {

        let current_dir = env::current_dir().unwrap_or_default();
        let path_str = current_dir.to_string_lossy();
        let display_path = match home::home_dir() {
            Some(home) => {
                let home_str = home.to_string_lossy();
                if path_str.starts_with(home_str.as_ref()) {
                    // 使用 ~ 替换主目录路径
                    path_str.replacen(home_str.as_ref(), "~", 1)
                } else {
                    path_str.to_string()
                }
            }
            None => path_str.to_string(),
        };

        let prompt = format!("[{}@{}]$ ", username, display_path);
        let prompt_width = UnicodeWidthStr::width(prompt.as_str());

        let display_buffer: String = buffer.iter().collect();

        // 计算光标位置前的字符串的显示宽度
        let prefix_slice: String = buffer[..position].iter().collect();
        let cursor_col = prompt_width + UnicodeWidthStr::width(prefix_slice.as_str());

        // 将光标移动到行首，打印提示符和缓冲区内容
        execute!(
            stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine),
        )?;
        print!("{}{}", prompt, display_buffer);
        execute!(stdout, cursor::MoveToColumn(cursor_col as u16))?;
        stdout.flush()?;

        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                },
                KeyCode::Char('l' | 'L') if modifiers.contains(KeyModifiers::CONTROL) => {
                    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
                    stdout.flush()?;
                },
                KeyCode::Char('c' | 'C') if modifiers.contains(KeyModifiers::CONTROL) => {
                    terminal::disable_raw_mode()?;
                    println!("^C");
                    return Err(ReadLineError::Interrupted);
                },
                KeyCode::Char('u' | 'U') if modifiers.contains(KeyModifiers::CONTROL) => {
                    buffer.drain(0..position);
                    position = 0;
                },
                KeyCode::Char(c) => {
                    buffer.insert(position, c);
                    position += 1;
                },
                KeyCode::Backspace => {
                    if position > 0 {
                        position -= 1;
                        buffer.remove(position);
                    }
                },
                KeyCode::Delete => {
                    if position < buffer.len() {
                        buffer.remove(position);
                    }
                },
                KeyCode::Up => {
                    if history_index > 0 {
                        history_index -= 1;
                        buffer = historys[history_index].chars().collect();
                        position = buffer.len();
                    }
                },
                KeyCode::Down => {
                    if history_index < historys.len() {
                        history_index += 1;
                        if history_index == historys.len() {
                            buffer.clear();
                        } else {
                            buffer = historys[history_index].chars().collect();
                        }
                        position = buffer.len();
                    }
                },
                KeyCode::Left => {
                    if position > 0 {
                        position -= 1;
                    }
                },
                KeyCode::Right => {
                    if position < buffer.len() {
                        position += 1;
                    }
                },
                KeyCode::Home => {
                    position = 0;
                },
                KeyCode::End => {
                    position = buffer.len();
                },
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    println!();
    Ok(buffer.into_iter().collect())
}