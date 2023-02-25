use add_server_lib::MAX_TRANSMISSIONS;
use std::io::{Read, Write};
use std::net::{TcpListener};

fn main() -> std::io::Result<()> {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server started");

    // Listen for incoming connections
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut sum = 0;

        // Receive the numbers from the client
        for _ in 1..=MAX_TRANSMISSIONS {
            let mut buffer = vec![0; 1024];
            let num_bytes = stream.read(&mut buffer)?;
            let string = String::from_utf8(buffer[0..num_bytes].to_vec()).unwrap();
            let num = match string.trim().parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid number received");
                    continue;
                }
            };
            println!("Received number: {}", num);
            sum += num;

            // Send the cumulative sum back to the client
            let response = sum.to_string();
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
            println!("Sent sum {}", sum);
        }
    }

    Ok(())
}
