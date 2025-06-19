use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::env;
use std::io::{stdout, Write};
use unicode_width::UnicodeWidthStr;

pub fn read_command_line() -> std::io::Result<String> {

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    let mut buffer: Vec<char> = Vec::new();
    let mut position = 0; // 光标在缓冲区中的位置
    let username = whoami::username();

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

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
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