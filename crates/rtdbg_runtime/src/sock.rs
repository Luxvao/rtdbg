use std::{
    io::{Error, Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    process::exit,
    sync::LazyLock,
};

use librtdbg::sock::{
    Api, FAILED_TO_READ_SOCKET, MALFORMED_PACKET, SCRIPT_NOT_UTF8, SUCCESS, UNABLE_TO_ADD_TO_QUEUE,
    UNABLE_TO_REMOVE_FROM_QUEUE, UNABLE_TO_REMOVE_FROM_QUEUE_NO_SUCH_SCRIPT, notify_error,
    notify_warning,
};
use log::error;

use crate::SCRIPT_QUEUE;

pub static SOCKET_PATH: LazyLock<String> = LazyLock::new(setup_sock);

fn setup_sock() -> String {
    let pid = std::process::id();

    format!("/tmp/rtdbg-{}.sock", pid)
}

pub fn io() {
    let Ok(listener) = UnixListener::bind(SOCKET_PATH.clone()) else {
        error!("Failed to bind the socket, exiting!");

        exit(1);
    };

    for stream in listener.incoming().flatten() {
        std::thread::spawn(|| handle_client(stream));
    }
}

fn handle_client(mut stream: UnixStream) {
    loop {
        let mut action_buffer: [u8; 1] = [0; 1];

        // Read the requested action from the stream
        println!("TEST - RAN");
        let read_result = stream.read_exact(&mut action_buffer);

        if let Err(e) = read_result {
            notify_error(
                "A: Failed to read from socket! Error: ",
                &mut stream,
                e,
                FAILED_TO_READ_SOCKET,
            );

            return;
        }

        let Ok(action) = Api::try_from(action_buffer[0]) else {
            notify_error(
                "Malformed packet!",
                &mut stream,
                Error::other(""),
                MALFORMED_PACKET,
            );

            return;
        };

        // Execute the action
        match action {
            Api::AddToQueue => {
                // We'll read the payload size into this buffer
                let mut payload_size_buffer: [u8; 8] = [0; 8];

                let read_result = stream.read_exact(&mut payload_size_buffer);

                if let Err(e) = read_result {
                    notify_error(
                        "B: Failed to read from socket! Error: ",
                        &mut stream,
                        e,
                        FAILED_TO_READ_SOCKET,
                    );

                    return;
                }

                // The size of the payload
                let payload_size = usize::from_le_bytes(payload_size_buffer);

                let mut payload_buffer = vec![0; payload_size];

                let read_result = stream.read_exact(&mut payload_buffer);

                if let Err(e) = read_result {
                    notify_error(
                        "C: Failed to read from socket! Error: ",
                        &mut stream,
                        e,
                        FAILED_TO_READ_SOCKET,
                    );

                    return;
                }

                let Ok(script) = String::from_utf8(payload_buffer) else {
                    notify_error(
                        "Script wasn't UTF8!",
                        &mut stream,
                        Error::other(""),
                        SCRIPT_NOT_UTF8,
                    );

                    return;
                };

                {
                    let (queue, condvar) = &SCRIPT_QUEUE;

                    let Ok(mut queue) = queue.lock() else {
                        notify_warning(
                            "Unable to add to queue!",
                            &mut stream,
                            UNABLE_TO_ADD_TO_QUEUE,
                        );

                        return;
                    };

                    queue.push_back(script);

                    condvar.notify_one();
                }
            }
            Api::RemoveFromQueue => {
                // We'll read the index the client wishes to remove
                let mut index_buffer: [u8; 8] = [0; 8];

                let read_result = stream.read_exact(&mut index_buffer);

                if let Err(e) = read_result {
                    notify_error(
                        "D: Failed to read from socket! Error: ",
                        &mut stream,
                        e,
                        FAILED_TO_READ_SOCKET,
                    );

                    return;
                }

                let index = usize::from_le_bytes(index_buffer);

                {
                    let (queue, _) = &SCRIPT_QUEUE;

                    let Ok(mut queue) = queue.lock() else {
                        notify_warning(
                            "Unable to remove from queue!",
                            &mut stream,
                            UNABLE_TO_REMOVE_FROM_QUEUE,
                        );

                        return;
                    };

                    let result = queue.remove(index);

                    if result.is_none() {
                        notify_warning(
                            "Unable to remove from queue! Index out of bounds!",
                            &mut stream,
                            UNABLE_TO_REMOVE_FROM_QUEUE_NO_SUCH_SCRIPT,
                        );

                        return;
                    }
                }
            }
            Api::Disconnect => {
                // Disconnect by dropping the TcpStream
                return;
            }
        }

        // Send back the response code 0 - SUCCESS!
        let write_result = stream.write_all(&SUCCESS);

        if let Err(e) = write_result {
            error!("Unable to write to socket! Error: {:?}", e);

            return;
        }
    }
}
