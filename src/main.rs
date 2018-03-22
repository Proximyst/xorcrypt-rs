extern crate crypto;
extern crate termion;
extern crate tempfile;

mod cipher;
mod arghandler;

use ::termion::input::TermRead;
use ::std::io::{stdin, stdout, Write};
use ::std::fs::OpenOptions;

fn main() { // panicking is okay as this is not an enterprise app
    let password: String;
    {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        let stdin = stdin();
        let mut stdin = stdin.lock();

        stdout
            .write_all(b"Password: ")
            .expect("Couldn't write to stdout!");
        stdout.flush().expect("Couldn't flush stdout!");

        password = match match stdin.read_passwd(&mut stdout) {
            Ok(password) => password,
            Err(err) => panic!("error occurred: {}", err),
        } {
            Some(password) => password,
            None => panic!("no password input"),
        };

        stdout.write_all(b"Re-enter the password: ").unwrap();
        stdout.flush().unwrap();

        let _ = match match stdin.read_passwd(&mut stdout) {
            Ok(password) => password,
            Err(err) => panic!("an error occurred: {}", err),
        } {
            Some(input) => if input != password {
                panic!("the passwords don't match");
            },
            None => panic!("no password input"),
        };
    }
	let key = password[..].as_bytes();

    for argv in std::env::args() {
        let mut open_options = OpenOptions::new();
        let file = open_options.write(true).append(true).read(true).open(&argv);
        if file.is_err() {
			let dir = ::std::fs::read_dir(&argv);
			if dir.is_err() {
				continue;
			}
			let dir = dir.unwrap();
			dir.filter(|entry| entry.is_ok()).for_each(|entry| {
				let entry = entry.unwrap();
				let path = entry.path();
				let file = open_options.write(true).append(true).read(true).open(&path);
				if file.is_ok() {
					arghandler::handle(key, file.unwrap(), format!("{:?}", path));
				}
			});
			continue;
		}
		arghandler::handle(key, file.unwrap(), argv);
    }
}
