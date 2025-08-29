//  SPDX-FileCopyrightText: Copyright 2025 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! socket stream
use {
    crate::mu::{
        env::Env,
        exception::{self, Condition, Exception},
        tag::Tag,
    },
    async_net::{TcpStream, UdpSocket},
    std::io::{self, Read, Write},
    futures_lite::{AsyncReadExt, AsyncWriteExt, future::block_on},
    futures_locks::RwLock,
};

#[derive (Debug, Clone)]
enum Props {
    Tcp((String, u16)),
    Udp((String, u16)),
}

#[derive (Debug)]
pub enum Direction {
    Input,
    Output,
    BiDir,
}

// stream builder
pub struct SocketStreamBuilder {
    pub socket: Option<Props>,
    pub direction: Option<Direction>,
}

impl SocketStreamBuilder {
    pub fn new() -> Self {
        Self {
            socket: None,
            direction: None,
        }
    }

    pub fn udp(&mut self, socket: u16) -> &mut Self {
        self.socket =
            match self.socket {
                None => Some(Props::Udp(("127.0.0.1".to_string(), socket))),
                _ => panic!(),
            };
        
        self
    }
        
    pub fn tcp(&mut self, socket: u16) -> &mut Self {
        self.socket =
            match self.socket {
                None => Some(Props::Tcp(("127.0.0.1".to_string(), socket))),
                _ => panic!(),
            };
        
        self
    }
    
    pub fn input(&mut self) -> &mut Self {
        self.direction = match self.direction {
            None => Some(Direction::Input),
            _ => panic!(),
        };
        
        self
    }

    pub fn output(&mut self) -> &mut Self {
        self.direction = match self.direction {
            None => Some(Direction::Output),
            _ => panic!(),
        };
        
        self
    }

    pub fn bidir(&mut self) -> &mut Self {
        self.direction = match self.direction {
            None => Some(Direction::BiDir),
            _ => panic!(),
        };
        
        self
    }

    pub async fn build(&mut self) -> SocketStream {
        let socket =
            match &self.socket {
                Some(socket) => match socket {
                    Props::Udp((host, socket)) => match UdpSocket::bind(format!("{host}:{socket}")).await {
                        Ok(socket) => Socket::Udp(socket),
                        Err(_) => panic!(),
                    },
                    Props::Tcp((host, socket)) => match TcpStream::connect(format!("{host}:{socket}")).await {
                        Ok(stream) => Socket::Tcp(stream),
                        Err(_) => panic!(),
                    },
                },
                None => panic!(),
            };
        
        match &self.direction {
            Some(dir) => match dir {
                Direction::Input => SocketStream::Reader(RwLock::new(socket)),
                Direction::Output => SocketStream::Writer(RwLock::new(socket)),
                Direction::BiDir => panic!(),
            },
            None => panic!(),
        }
    }
}

// socket stream
#[derive(Debug)]
pub enum Socket {
    Tcp(TcpStream),
    Udp(UdpSocket),
}

#[derive(Debug)]
pub enum SocketStream {
    Reader(RwLock<Socket>),
    Writer(RwLock<Socket>),
    BiDir(RwLock<Socket>),
}

impl SocketStream {
    pub fn read_byte(&self, env: &Env) -> exception::Result<Option<u8>> {
        let mut buf = [0; 1];

        match self {
            Self::Reader(stream) => match stream {
                Socket::Tcp(socket) => {
                    let socket_ref = block_on(socket.write());
                    let task: io::Result<usize> = socket_ref.read(&mut buf);

                    match task {
                        Ok(nread) => {
                            if nread == 0 {
                                Ok(None)
                            } else {
                                Ok(Some(buf[0]))
                            }
                        }
                        Err(_) => Err(Exception::new(
                            env,
                            Condition::Read,
                            "mu:read-byte",
                            Tag::nil(),
                        )),
                    }
                },
                _ => panic!(),
            }
            _ => panic!(),
        }
    }

    /*
    pub fn write_byte(&self, env: &Env, byte: u8) -> exception::Result<Option<()>> {
        let buf = [byte; 1];

        match self {
            Self::Writer(file) => {
                let mut file_ref = block_on(file.write());
                let task: io::Result<()> = block_on(file_ref.write_all(&buf));

                match task {
                    Ok(_) => Ok(None),
                    Err(_) => Err(Exception::new(
                        env,
                        Condition::Write,
                        "mu:write-byte",
                        Tag::nil(),
                    )),
                }
            },
            _ => panic!(),
        }
}
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn stream() {
        assert!(true)
    }
}
