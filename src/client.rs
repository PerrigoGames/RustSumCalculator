use add_server_lib::MAX_TRANSMISSIONS;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;
    
    // Send the numbers 1-10 to the server
    for i in 1..=MAX_TRANSMISSIONS {
        stream.write_all(i.to_string().as_bytes())?;
        stream.flush()?;
        println!("Sent number {}", i);

        let mut buffer = vec![0; 1024];
        let num_bytes = stream.read(&mut buffer)?;
        let string = String::from_utf8(buffer[0..num_bytes].to_vec()).unwrap();
        println!("Received string: {}", string);  
    }  

    Ok(())
}
