use std::io::{stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};
use unicode_width::UnicodeWidthStr;

pub fn read_command_line() -> std::io::Result<String> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    let mut buffer: Vec<char> = Vec::new();
    let mut position = 0; // 光标在缓冲区中的位置

    loop {
        
        let display_buffer: String = buffer.iter().collect();

        // 计算光标位置前的字符串的显示宽度
        let prefix_slice: String = buffer[..position].iter().collect();
        let cursor_col = 2 + UnicodeWidthStr::width(prefix_slice.as_str());
        
        // 将光标移动到行首，打印提示符和缓冲区内容
        execute!(
            stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine),
        )?;
        print!("> {}", display_buffer);
        // 将光标移动到正确的位置 ('> ' 占2个字符)
        execute!(stdout, cursor::MoveToColumn(cursor_col as u16))?;
        stdout.flush()?;

        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                },
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    terminal::disable_raw_mode()?;
                    println!();
                    std::process::exit(0);
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