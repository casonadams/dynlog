#[cfg(feature = "socket-trigger")]
use {
    tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    tokio::net::UnixListener,
};

#[cfg(feature = "socket-trigger")]
pub async fn enable_socket_trigger() {
    let crate_name = env!("CARGO_CRATE_NAME");
    let path = format!("/tmp/{}.sock", crate_name);
    let _ = std::fs::remove_file(&path);

    let listener = match UnixListener::bind(&path) {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("dynlog: failed to bind unix socket '{}': {}", path, e);
            return;
        }
    };

    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let mut reader = BufReader::new(stream);
                    let mut line = String::new();

                    if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                        continue;
                    }

                    let cmd = line.trim().to_lowercase();
                    match cmd.parse::<log::LevelFilter>() {
                        Ok(level) => {
                            crate::set_level(level);
                            let _ = reader.get_mut().write_all(b"ok\n").await;
                            eprintln!(
                                "{}dynlog: log level set to {}{}",
                                crate::level::COLORS.gray,
                                cmd.to_uppercase(),
                                crate::level::COLORS.reset
                            );
                        }
                        Err(_) => {
                            let _ = reader
                                .get_mut()
                                .write_all(b"error: invalid log level\n")
                                .await;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("dynlog: socket accept error: {}", e);
                }
            }
        }
    });
}
