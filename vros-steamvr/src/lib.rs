use std::{
    env,
    io::self,
    process::Stdio,
};

use miette::{bail, miette, IntoDiagnostic, WrapErr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{ChildStdin, ChildStdout, Command},
    task::JoinHandle,
};
use vros_steamvr_core::{FromAgent, ToAgent};

struct Sender(ChildStdin);

impl Sender {
    async fn send(&mut self, message: &ToAgent) -> Result<(), io::Error> {
        let bytes = bincode::serialize(message).unwrap();
        self.0.write_all(&bytes).await?;
        self.0.flush().await
    }
}

struct Receiver {
    stdout: ChildStdout,
    length: Option<u32>,
    position: u32,
    buffer: Vec<u8>,
}

impl Receiver {
    fn new(stdout: ChildStdout) -> Receiver {
        Receiver {
            stdout,
            length: None,
            position: 0,
            buffer: Vec::new(),
        }
    }

    async fn receive(&mut self) -> Option<bincode::Result<FromAgent>> {
        let length = match self.length {
            None => {
                let length = match self.stdout.read_u32_le().await {
                    Ok(length) => length,
                    Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => return None,
                    Err(err) => return Some(Err(Box::new(bincode::ErrorKind::Io(err)))),
                };
                self.length = Some(length);
                self.buffer.resize(length as usize, 0);
                self.position = 0;
                length
            }
            Some(length) => length,
        };
        while self.position < length {
            self.position += match self
                .stdout
                .read(&mut self.buffer[self.position as usize..])
                .await
            {
                Ok(len) => len as u32,
                Err(err) => return Some(Err(Box::new(bincode::ErrorKind::Io(err)))),
            };
        }
        self.length = None;
        Some(bincode::deserialize(&self.buffer))
    }
}

pub async fn start() -> miette::Result<JoinHandle<miette::Result<()>>> {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push("vros-steamvr-agent.exe");
    let mut child = Command::new(path)
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .into_diagnostic()
        .wrap_err("Agent start failed")?;

    let to_agent = Sender(child.stdin.take().unwrap());
    let mut from_agent = Receiver::new(child.stdout.take().unwrap());
    let init = async {
        match from_agent.receive().await {
            Some(Ok(FromAgent::InitializationCompleted)) => Ok(()),
            Some(Ok(FromAgent::InitializationError(err))) => {
                Err(err).wrap_err("Initialization error")
            }
            _ => Err(miette!("Agent protocol error")),
        }
    };

    tokio::select! {
        _ = child.wait() => bail!("Agent unexpectedly exitted"),
        result = init => result?,
    };

    Ok(tokio::spawn(async move {
        // Move ownership of the child process into the spawned task.

        let _child = child;
        let _to_agent = to_agent;
        loop {
            tokio::select! {
                event_out = from_agent.receive() => match event_out {
                    None => break,
                    Some(Ok(FromAgent::InitializationCompleted)) | Some(Ok(FromAgent::InitializationError(_))) => bail!("Agent protocol error"),
                    Some(Ok(FromAgent::ApplicationName(name))) => dbg!(name),
                    Some(Err(err)) => bail!("Agent failed: {}", err),
                },
            };
        }

        Ok(())

        //let _ = child.kill().await;
    }))
}
