mod error;
use error::FakeMcError;

use std::{fs, thread};
use std::net::{TcpListener, TcpStream};

use ozelot::{Server, ClientState};
use ozelot::serverbound::ServerboundPacket;

fn send_status(conn: &mut Server) -> Result<(), FakeMcError>
{
	let data = fs::read_to_string("./server-list.json")?;
	let packet = ozelot::clientbound::StatusResponse::new(data);
	conn.send(packet)?;

	return Ok(());
}

fn send_disconnect(conn: &mut Server) -> Result<(), FakeMcError>
{
	let data = fs::read_to_string("./disconnect.json")?;
	let packet = ozelot::clientbound::LoginDisconnect::new(data);
	conn.send(packet)?;

	return Ok(());
}

fn handle_connection(stream_result: std::io::Result<TcpStream>) -> Result<(), FakeMcError>
{
	let mut conn = Server::from_tcpstream(stream_result?)?;
	let mut had_handshake = false;
	let mut had_ping = false;

	loop
	{
		let packets = conn.read()?;
		for packet in packets
		{
			match packet
			{
				ServerboundPacket::Handshake(ref p) => {
					println!("Received connection to {}:{}", p.get_server_address(), p.get_server_port());

					match p.get_next_clientstate() {
						Some(ClientState::Status) => send_status(&mut conn)?,
						Some(ClientState::Login) => send_disconnect(&mut conn)?,
						_ => {},
					}

					had_handshake = true;
				},
				ServerboundPacket::StatusPing(ref p) => {
					let response = ozelot::clientbound::StatusPong::new(p.get_id().clone());
					conn.send(response)?;

					had_ping = true;
				},
				_ => {},
			}
		}

		if had_handshake && had_ping {
			conn.write()?;
			conn.close()?;
			return Ok(());
		}
	}
}

fn main() -> std::io::Result<()>
{
	let listener = TcpListener::bind("0.0.0.0:25565")?;
	for stream_result in listener.incoming()
	{
		thread::spawn(move|| {
			match handle_connection(stream_result)
			{
				Ok(_) => {},
				Err(e) => println!("Error: {}", e),
			}
		});
	}

	return Ok(());
}
