use std::net::TcpListener;

fn main() -> std::io::Result<()>{
    //tcp listener (idiomatic style)
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming(){
        match stream {
            Ok(_stream) => {
                println!("connection established!");
            },
            Err(err) => {
                eprintln!("could not establish connection. Reason: {}", err);
                continue;
            }
        };

        println!("connection established!");
    }

    Ok(())
}
