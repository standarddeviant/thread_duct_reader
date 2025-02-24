use duct::cmd;
use log::{error, info};
use ringbuf::{traits::*, StaticRb};
use std::{io::Read, thread};

pub const RB_SIZE: usize = 1024;

fn main() {
    let mut buffer = StaticRb::<u8, RB_SIZE>::default();
    let (mut prod, mut cons) = buffer.split();

    // let mut buffer = StaticRb::<u8, 1024>::new();

    let mut reader_thread = thread::spawn(move || {
        let mut reader = cmd("python", ["-u", "./test.py"])
            .stderr_to_stdout()
            .reader()
            .expect("Failed to start reader");

        // let mut reader = reader;

        loop {
            let mut tmpbuf: [u8; 1024] = [0; 1024];
            match reader.read(&mut tmpbuf) {
                Ok(nread) => {
                    if nread > 0 {
                        // wee...
                        println!("Read {} bytes: {:?}", nread, &tmpbuf[0..nread]);
                        prod.push_slice(&tmpbuf[0..nread]);
                    }
                }
                Err(_bad) => {
                    eprintln!("hmmm... {_bad}");
                    break;
                }
            }

            match reader.try_wait() {
                Ok(Some(exit)) => {
                    break;
                    // done!
                }
                Ok(None) => {
                    // still going...
                }
                Err(_bad) => {
                    // ?
                    eprintln!("hmmm... (try_wait) : {_bad}");
                    break;
                }
            }
        }
        // while let Ok(data) = reader.read(&mut buffer) {
        //     if data == 0 {
        //         break;
        //     }
        // }
    });

    println!("hello from the main thread!");
    let mut recv_buf: Vec<u8> = vec![];
    let mut finished_count = 0;
    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        // cons.read(&mut buf);
        match cons.read(&mut buf) {
            Ok(num_bytes) => {
                if num_bytes > 0 {
                    recv_buf.extend_from_slice(&buf[0..num_bytes]);
                    println!("num_bytes = {num_bytes}");
                }
            }
            Err(_bad) => {
                // this isn't actually an error I guess..., but rather "no bytes right now..."
                println!("sleeping 1s in the main thread...");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        if reader_thread.is_finished() {
            finished_count += 1;
            if finished_count > 1 {
                break; // we out!
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    println!("joining reader_thread...");
    reader_thread.join().unwrap();
    println!("all done!");

    if let Ok(recv_str) = String::from_utf8(recv_buf) {
        println!("final output:\n{}", recv_str);
    }
}
