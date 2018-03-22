use ::std::fs::{File, remove_file};
use ::std::io::{Read, Write, Seek, SeekFrom};
use ::cipher::cipher;

pub fn handle(key: &[u8], mut file: File, path: String) {
	println!("[STATUS] Handling: {}", path);
	let mut tempfile = ::tempfile::tempfile().expect("couldn't create temp file");
	let mut buf = [0u8; 1024 * 1024 * 16384];
	loop {
		let read = file.read(&mut buf);
		if read.is_err() {
			panic!(read.err().unwrap());
		}
		let read = read.unwrap();
		if read <= 0 {
			break;
		}
		let ciphered = cipher(key, &buf[..read]);
		if ciphered.is_err() {
			panic!(ciphered.err().unwrap());
		}
		let ciphered = ciphered.unwrap();
		tempfile.write_all(ciphered.as_slice()).expect("couldnt write data");
		tempfile.sync_data().expect("couldnt sync data");
	}
	drop(file); // close it
	remove_file(&path).expect("couldnt delete original file");
	let file = File::create(&path);
	if file.is_err() {
		panic!(file.err().unwrap());
	}
	let mut file = file.unwrap();
	tempfile.seek(SeekFrom::Start(0)).expect("can't reverse in tempfile");

	loop {
		let read = tempfile.read(&mut buf);
		if read.is_err() {
			panic!(read.err().unwrap());
		}
		let read = read.unwrap();
		if read <= 0 {
			break;
		}
		file.write_all(&buf[..read]).expect("couldnt write data");
		file.sync_data().expect("couldnt sync data");
	}
}