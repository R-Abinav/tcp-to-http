use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*}
};

fn main() -> std::io::Result<()>{
    //tcp listener (idiomatic style)
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                let _ = handle_connection(stream);
            },
            Err(err) => {
                eprintln!("could not establish connection. Reason: {}", err);
                continue;
            }
        };
    }

    Ok(())
}

//handler function to handle connection
fn handle_connection(mut stream: TcpStream) -> std::io::Result<()>{
    let buf_reader = BufReader::new(&stream);

    //idomatic way
    let http_request = buf_reader.lines()
                                            .take_while(|line| {
                                                matches!(line, Ok(l) if !l.is_empty())
                                            })
                                            .collect::<Result<Vec<_>, _> >()?;

    println!("request: {http_request:#?}");   

    //structure the response according the HTTP response structure 
    // let body = "Hello there big boss!";
    // let response = format!("HTTP/1.1 200 OK\r\n\
    //                                 Content-Length: {}\r\n\
    //                                 \r\n\
    //                                 {}", body.len(), body);

    //returning HTML
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("example.html")?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}