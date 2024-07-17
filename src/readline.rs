use std::io::BufRead;
use std::io::Read;
use std::io::{self, Write};

pub fn progress_bar() {
    const N: usize = 10;
    let mut stdout = io::stdout();
    write!(stdout, "loading: ==========");
    stdout.flush();
    std::thread::sleep(std::time::Duration::from_millis(2000));

    for i in 0..N {
        stdout.write_all(&[0x08u8; N]);
        // stdout.flush();
        for _ in 0..i + 1 {
            write!(stdout, "█");
        }
        for _ in 0..N - i - 1 {
            write!(stdout, "=");
        }
        stdout.flush();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

pub fn multiline_progress_bar() {
    const N: usize = 10;
    let mut stdout = io::stdout();
    writeln!(stdout, "loading: ==========");
    write!(stdout, "==========");
    stdout.flush();
    std::thread::sleep(std::time::Duration::from_millis(2000));

    const RUBOUT: [u8; 2 * N + 1] = [0x08u8; 2 * N + 1];

    for i in 0..(2 * N + 1) {
        stdout.write_all(&RUBOUT);
        let q = i / N;
        let m = i - q * N;
        if q == 0 {
            for _ in 0..m + 1 {
                write!(stdout, "█");
            }
            for _ in 0..N - m {
                write!(stdout, "=");
            }
            write!(stdout, "\n");
            for _ in 0..N + 1 {
                write!(stdout, "=");
            }
        } else if q == 1 {
            for _ in 0..N + 1 {
                write!(stdout, "█");
            }
            write!(stdout, "\n");
            for _ in 0..m {
                write!(stdout, "█");
            }
            for _ in 0..N - m {
                write!(stdout, "=");
            }
        } else
        /* q == 2 */
        {
            for _ in 0..N + 1 {
                write!(stdout, "█");
            }
            write!(stdout, "\n");
            for _ in 0..N + 1 {
                write!(stdout, "█");
            }
        }
        stdout.flush();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

pub fn read_line() {
    let mut buf: Vec<u8> = Vec::with_capacity(1 << 10);
    let mut stdin = io::stdin();
    let mut handle = stdin.lock();
    if let Ok(n) = handle.read_until('\n' as u8, &mut buf) {
        println!("read {n} bytes, which are:");
        for (i, b) in buf.iter().enumerate() {
            println!("buf[{i}] = {:#04X} = {b}", b);
        }
    }
}

pub fn read_direction() {
    let mut buf: [u8; 3] = [0u8; 3];
    let mut stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stdout = io::stdout();
    write!(stdout, "XX");
    stdout.flush();
    loop {
        buf[0] = 0;
        buf[1] = 0;
        buf[2] = 0;
        match handle.read_exact(&mut buf) {
            Ok(()) => {
                match buf {
                    [0x1B, 0x5B, 0x41] => {
                        stdout.write_all(&[0x08_u8; 2]);
                        write!(stdout, "U!");
                    }
                    [0x1B, 0x5B, 0x42] => {
                        stdout.write_all(&[0x08_u8; 2]);
                        write!(stdout, "D!");
                    }
                    [0x1B, 0x5B, 0x43] => {
                        stdout.write_all(&[0x08_u8; 2]);
                        write!(stdout, "F!");
                    }
                    [0x1B, 0x5B, 0x44] => {
                        stdout.write_all(&[0x08_u8; 2]);
                        write!(stdout, "B!");
                    }
                    _ => {
                        stdout.write_all(&[0x08_u8; 2]);
                        write!(stdout, "ZZ");
                    }
                }
                stdout.flush();
            }
            Err(e) => println!("Error during read: {:#?}", e),
        }
    }
}

pub fn read_direction_wasd() {
    let mut buf: Vec<u8> = Vec::new();
    let mut stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stdout = io::stdout();

    write!(stdout, "XX");
    stdout.flush();
    loop {
        buf.clear();
        match handle.read_until(0x0A, &mut buf) {
            Ok(2) => {
                // println!("{:#04X?}", buf);
                match buf[0] {
                    0x57 | 0x77 => {
                        stdout.write_all(&[0x08_u8; 4]);
                        write!(stdout, "U!");
                    }
                    0x53 | 0x73 => {
                        stdout.write_all(&[0x08_u8; 4]);
                        write!(stdout, "D!");
                    }
                    0x44 | 0x64 => {
                        stdout.write_all(&[0x08_u8; 4]);
                        write!(stdout, "F!");
                    }
                    0x41 | 0x61 => {
                        stdout.write_all(&[0x08_u8; 4]);
                        write!(stdout, "B!");
                    }
                    _ => {
                        stdout.write_all(&[0x08_u8; 4]);
                        write!(stdout, "ZZ");
                    }
                }
                stdout.flush();
            }
            Ok(_) => (),
            Err(e) => println!("Error during read: {:#?}", e),
        }
    }
}

pub fn clear_screen() {
    let mut stdout = io::stdout();
    // println!("clear screen: begin");
    // println!("{}", 'J' as u8);
    stdout.write_all(&[0x1B, 0x5B, 0x32, 'J' as u8, 0x0A]);
    stdout.flush();
    // println!("clear screen: end");
}
