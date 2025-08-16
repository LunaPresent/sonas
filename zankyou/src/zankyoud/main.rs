use std::io;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use zankyou::{Command, Database, server};

use interprocess::local_socket::{
	ListenerOptions,
	tokio::{Listener, Stream, prelude::*},
};

#[derive(Debug, Error)]
enum Error {
	#[error("IO error occured: {0}")]
	IoError(#[from] io::Error),
	#[error("Database error occured: {0}")]
	DatabaseError(#[from] sqlx::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	let listener = ListenerOptions::new()
		.name(server::name()?)
		.create_tokio()?;

	let database = Database::connect("db.sqlite").await?;

	loop {
		let Err(error) = handle_connection(&listener, database.clone()).await else {
			continue;
		};

		eprintln!("Error with incoming connection: {error}");
	}
}

async fn handle_connection(listener: &Listener, database: Database) -> io::Result<()> {
	let connection = listener.accept().await?;

	tokio::spawn(async move {
		let Err(error) = handle_command(connection, &database).await else {
			return;
		};

		eprintln!("Error handling connection: {error}");
	});

	Ok(())
}

async fn handle_command(conn: Stream, database: &Database) -> io::Result<()> {
	let mut recver = BufReader::new(&conn);
	let mut sender = &conn;

	let mut buf = String::with_capacity(128);
	let _ = recver.read_line(&mut buf).await?;

	let result = match buf.parse::<Command>() {
		Ok(command) => command.execute(),
		Err(error) => format!("{error:?}"),
	};

	sender.write_all(&result.into_bytes()).await
}
