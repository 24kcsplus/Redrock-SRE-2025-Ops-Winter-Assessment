use std::io::{stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
};

pub fn read_command_line() -> std::io::Result<String> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    let mut buffer = String::new();
    let mut position = 0; // 光标在 buffer 中的位置

    loop {
        // 移动到行首，打印提示符和缓冲区内容
        execute!(
            stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine),
        )?;
        print!("> {}", buffer);
        // 将光标移动到正确的位置 ('> ' 占2个字符)
        execute!(stdout, cursor::MoveToColumn(2 + position as u16))?;
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
    println!(); // 确保下一个输出从新行开始
    Ok(buffer)
}