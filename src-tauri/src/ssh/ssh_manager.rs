use ssh2::Session;

use std::error::Error;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
pub struct SshConnectionManager {
    session: Option<Session>,
}

impl SshConnectionManager {
    pub fn new() -> Self {
        SshConnectionManager { session: None }
    }

    pub fn connect(
        &mut self,
        addr: &str,
        username: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        let tcp = TcpStream::connect(addr)?;
        let mut session = Session::new()?;

        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(username, password)?;

        if session.authenticated() {
            self.session = Some(session);
            Ok(())
        } else {
            Err("Authentication failed".into())
        }
    }

    pub fn disconnect(&mut self) {
        self.session = None;
    }

    pub fn exec_command(&self, command: &str) -> Result<String, Box<dyn Error>> {
        if let Some(session) = &self.session {
            let mut channel = session.channel_session()?;
            channel.exec(command)?;
            let mut s = String::new();
            channel.read_to_string(&mut s)?;
            channel.wait_close()?;
            Ok(s)
        } else {
            Err("Not connected to any server".into())
        }
    }

    pub fn exec_command_on_shell(&self, command: &str) -> Result<String, Box<dyn Error>> {
        if let Some(session) = &self.session {
            let mut channel = session.channel_session()?;
            channel.request_pty("xterm", None, Some((800, 600, 0, 0)))?;
            channel.exec(command)?;
            let mut s = String::new();
            channel.read_to_string(&mut s)?;
            channel.wait_close()?;
            Ok(s)
        } else {
            Err("Not connected to any server".into())
        }
    }

    pub fn send_file(&self, local_path: &str, remote_path: &str) -> Result<(), Box<dyn Error>> {
        if let Some(session) = &self.session {
            let sftp = session.sftp()?;

            let mut file = std::fs::File::open(local_path)?;
            let metadata = file.metadata()?;
            let mut contents = Vec::with_capacity(metadata.len() as usize);
            file.read_to_end(&mut contents)?;

            let mut remote_file = sftp.create(Path::new(remote_path))?;
            remote_file.write_all(&contents)?;
            Ok(())
        } else {
            Err("Send file failed".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;
    use std::{thread, time::Duration};

    #[test]
    fn test_ssh_manager() {
        dotenv::from_path("../.env").ok();

        let mut manager = SshConnectionManager::new();

        let host = env::var("VITE_HOST").unwrap();
        let port = env::var("VITE_PORT").unwrap();
        let user = env::var("VITE_USER").unwrap();
        let passwd = env::var("VITE_PASSWORD").unwrap();

        manager.connect(
            format!("{}:{}", host, port).as_str(),
            user.as_str(),
            passwd.as_str(),
        );

        let output = manager.exec_command("ls").unwrap();
        println!("Output: {}", output);

        manager.disconnect();
    }

    #[test]
    fn test_send_file() {
        dotenv::from_path("../.env").ok();

        let host = env::var("VITE_HOST").unwrap();
        let port = env::var("VITE_PORT").unwrap();
        let user = env::var("VITE_USER").unwrap();
        let passwd = env::var("VITE_PASSWORD").unwrap();

        let mut manager = SshConnectionManager::new();

        manager.connect(
            format!("{}:{}", host, port).as_str(),
            user.as_str(),
            passwd.as_str(),
        );
        manager.send_file("C:\\Users\\Administrator\\Desktop\\1.txt", "/1.txt");
        manager.disconnect();
    }
}
