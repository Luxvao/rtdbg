use librtdbg::{
    api::ReqApi, comms::send_packet, error::Error, packet::Packet, runtime_connection,
    script::Script,
};

pub fn run_script(pid: u32, script_contents: String) -> Result<(), Error> {
    let mut socket = runtime_connection::connect(pid)?;

    // Send script
    let script = Script::from(script_contents);

    let add_to_queue_req = ReqApi::AddToQueue { script };

    let add_to_queue_packet = Packet::from(add_to_queue_req);

    send_packet(&mut socket, add_to_queue_packet)?;

    // Send disconnect packet
    let disconnect = ReqApi::Disconnect;

    let disconnect_packet = Packet::from(disconnect);

    send_packet(&mut socket, disconnect_packet)?;

    Ok(())
}
