use std::{
    os::unix::net::{UnixListener, UnixStream},
    process::exit,
    sync::LazyLock,
};

use librtdbg::{
    api::{ReqApi, RespApi},
    comms::{report_error, report_write_error, send_packet},
    packet::Packet,
};
use log::{error, info};

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
        // Read packet from stream
        let packet = match Packet::read_from_stream(&mut stream) {
            Ok(packet) => packet,
            Err(e) => {
                report_error(&mut stream, e);
                return;
            }
        };

        // Parse it into a request
        let req = match ReqApi::try_from(packet) {
            Ok(req) => req,
            Err(e) => {
                report_error(&mut stream, e);
                return;
            }
        };

        match req {
            ReqApi::Disconnect => {
                return;
            }
            ReqApi::AddToQueue { script } => {
                let (mut queue_locked, condvar) = extract_queue();

                // Push script onto the queue
                queue_locked.push_front(script);

                // Notify runtime
                condvar.notify_one();
            }
            ReqApi::RemoveFromQueue { index } => {
                let (mut queue_locked, _) = extract_queue();

                // Remove the script from the queue
                let result = queue_locked.remove(index);

                if result.is_none() {
                    info!("Bad request. No such script");

                    let resp = RespApi::Error(String::from("No such script"));

                    let resp_packet = Packet::from(resp);

                    let write_result = send_packet(&mut stream, resp_packet);

                    if let Err(e) = write_result {
                        // If writing failed then it's likely we won't be able to recover, so we quit the connection
                        report_write_error(e);

                        return;
                    }

                    continue;
                }
            }
        }

        // If we're here then everything went smoothly and we can report that back
        let write_res = send_packet(&mut stream, Packet::from(RespApi::Success));

        if let Err(e) = write_res {
            report_write_error(e);

            return;
        }
    }
}

fn extract_queue() -> (
    std::sync::MutexGuard<'static, std::collections::VecDeque<librtdbg::script::Script>>,
    &'static std::sync::Condvar,
) {
    let (queue, condvar) = &SCRIPT_QUEUE;

    let queue_locked = match queue.lock() {
        Ok(queue) => queue,
        Err(e) => {
            error!("Unable to lock queue, poisoned! Error: {:?}. Exiting...", e);

            exit(1);
        }
    };
    (queue_locked, condvar)
}
