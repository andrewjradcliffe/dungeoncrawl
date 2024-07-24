use std::io::BufRead;
use std::io::Read;
use std::io::{self, Write};

pub fn progress_bar() -> io::Result<()> {
    const N: usize = 10;
    let mut stdout = io::stdout();
    write!(stdout, "loading: ==========")?;
    stdout.flush()?;
    std::thread::sleep(std::time::Duration::from_millis(2000));

    for i in 0..N {
        stdout.write_all(&[0x08u8; N])?;
        // stdout.flush();
        for _ in 0..i + 1 {
            write!(stdout, "█")?;
        }
        for _ in 0..N - i - 1 {
            write!(stdout, "=")?;
        }
        stdout.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Ok(())
}

pub fn read_line() {
    let mut buf: Vec<u8> = Vec::with_capacity(1 << 10);
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    if let Ok(n) = handle.read_until('\n' as u8, &mut buf) {
        println!("read {n} bytes, which are:");
        for (i, b) in buf.iter().enumerate() {
            println!("buf[{i}] = {:#04X} = {b}", b);
        }
    }
}

pub fn read_direction() -> io::Result<()> {
    let mut buf: [u8; 3] = [0u8; 3];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stdout = io::stdout();
    write!(stdout, "XX")?;
    stdout.flush()?;
    loop {
        buf[0] = 0;
        buf[1] = 0;
        buf[2] = 0;
        match handle.read_exact(&mut buf) {
            Ok(()) => {
                match buf {
                    [0x1B, 0x5B, 0x41] => {
                        stdout.write_all(&[0x08_u8; 2])?;
                        write!(stdout, "U!")?;
                    }
                    [0x1B, 0x5B, 0x42] => {
                        stdout.write_all(&[0x08_u8; 2])?;
                        write!(stdout, "D!")?;
                    }
                    [0x1B, 0x5B, 0x43] => {
                        stdout.write_all(&[0x08_u8; 2])?;
                        write!(stdout, "F!")?;
                    }
                    [0x1B, 0x5B, 0x44] => {
                        stdout.write_all(&[0x08_u8; 2])?;
                        write!(stdout, "B!")?;
                    }
                    _ => {
                        stdout.write_all(&[0x08_u8; 2])?;
                        write!(stdout, "ZZ")?;
                    }
                }
                stdout.flush()?;
            }
            Err(e) => {
                println!("Error during read: {:#?}", e);
                break;
            }
        }
    }
    Ok(())
}

pub fn clear_screen() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(&[0x1B, 0x5B, 0x32, 'J' as u8])?;
    stdout.flush()
}
pub fn cursor_topleft() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(&[0x1B, 0x5B, 0x31, ';' as u8, 0x31, 'H' as u8])?;
    stdout.flush()
}
pub fn cursor_up(n: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    for _ in 0..n {
        stdout.write_all(&[0x1B, 0x5B, 0x31, 'A' as u8])?;
    }
    Ok(())
}
pub fn cursor_down(n: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    for _ in 0..n {
        stdout.write_all(&[0x1B, 0x5B, 0x31, 'B' as u8])?;
    }
    Ok(())
}
pub fn clear_line() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(&[0x1B, 0x5B, 0x32, 'K' as u8])?;
    Ok(())
}

pub fn clear_last_n_lines(n: usize) -> io::Result<()> {
    cursor_up(n)?;
    let mut stdout = io::stdout();
    for _ in 0..n {
        stdout.write_all(&[0x1B, 0x5B, 0x32, 'K' as u8])?;
        stdout.write_all(&[0x1B, 0x5B, 0x31, 'B' as u8])?;
    }
    stdout.flush()?;
    cursor_up(n)
}
