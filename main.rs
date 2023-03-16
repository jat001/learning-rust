use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Write, BufReader, BufWriter, BufRead};


fn handle_connection(stream: TcpStream) {
    let stdout = io::stdout();
    let _ = writeln!(&mut stdout.lock(), "New connection: {}", stream.peer_addr().unwrap());


    let stream_clone = stream.try_clone().unwrap();
    let mut writer = BufWriter::new(stream_clone);

    let mut reader = BufReader::new(stream);
    let mut name = String::new();
    loop {
        let r = reader.read_line(&mut name).unwrap();
        if r < 3 { //detect empty line
            break;
        }
    }
    let _ = writeln!(&mut stdout.lock(), "{}", &name.trim());
    let mut size = 0;
    let linesplit = name.split("\n");
    for l in linesplit {
        if l.starts_with("Content-Length") {
                let sizeplit = l.split(":");
                for s in sizeplit {
                    if !(s.starts_with("Content-Length")) {
                        size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
                }
            }
        }
    }
    let mut buffer = vec![0; size]; //New Vector with size of Content
    reader.read_exact(&mut buffer).unwrap(); //Get the Body Content.

    if buffer.len() > 0 {
        let _ = writeln!(&mut stdout.lock(), "{}", std::str::from_utf8(&buffer).unwrap());
    }

    let _ = writeln!(&mut stdout.lock(), "\r\n");

    let r = "HTTP/1.1 204 No Content\r\nServer: MyServer\r\n\r\n";
    writer.write(r.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_connection(stream.unwrap());
        });
    }
}
