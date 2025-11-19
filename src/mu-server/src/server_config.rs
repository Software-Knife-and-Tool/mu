//  SPDX-FileCopyrightText: Copyright 2022 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/listener
use {
    futures::executor::block_on,
    getopt::Opt,
    mu::Mu,
    std::net::{SocketAddr, ToSocketAddrs},
};

// runtime configuration
pub struct ServerConfig {
    #[allow(dead_code)]
    socket_addr: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

// options
type OptDef = (OptType, String);

#[derive(Debug, PartialEq)]
enum OptType {
    Config,
    Eval,
    Load,
    Ping,
    Socket,
}

impl ServerConfig {
    fn usage() {
        println!("env-server: {}: [-h?psvcel] [file...]", Mu::version());
        println!("h: usage message");
        println!("?: usage message");
        println!("c: [name:value,...]");
        println!("s: socket [ip-addr:port-number]");
        println!("e: eval [form]");
        println!("l: load [path]");
        println!("p: ping mode, requires -s");
        println!("v: print version and exit");

        std::process::exit(0);
    }

    fn parse_options(mut argv: Vec<String>) -> Option<Vec<OptDef>> {
        let mut opts = getopt::Parser::new(&argv, "h?s:pvc:e:l:q:");
        let mut optv = Vec::new();

        loop {
            let opt = opts.next().transpose();
            match opt {
                Err(_) => {
                    if let Err(error) = opt {
                        eprintln!("runtime: option {error:?}")
                    };
                    return None;
                }
                Ok(None) => break,
                Ok(clause) => match clause {
                    Some(opt) => match opt {
                        Opt('h', None) | Opt('?', None) => Self::usage(),
                        Opt('v', None) => {
                            print!("runtime: {} ", Mu::version());
                            return None;
                        }
                        Opt('p', None) => {
                            optv.push((OptType::Ping, String::from("")));
                        }
                        Opt('e', Some(expr)) => {
                            optv.push((OptType::Eval, expr));
                        }
                        Opt('s', Some(socket)) => {
                            optv.push((OptType::Socket, socket));
                        }
                        Opt('l', Some(path)) => {
                            optv.push((OptType::Load, path));
                        }
                        Opt('c', Some(config)) => {
                            optv.push((OptType::Config, config));
                        }
                        _ => {
                            eprintln!("unmapped switch {}", opt);
                            return None;
                        }
                    },
                    None => return None,
                },
            }
        }

        for file in argv.split_off(opts.index()) {
            optv.push((OptType::Load, file));
        }

        Some(optv)
    }

    pub fn new() -> Self {
        // 49152 to 65535 are dynamically available
        const SERVER_PORT: u16 = 50000;

        let mut config = String::new();
        let mut ping = false;

        let mut socket = format!("localhost:{}", SERVER_PORT);

        match Self::parse_options(std::env::args().collect()) {
            Some(opts) => {
                for opt in &opts {
                    if opt.0 == OptType::Config {
                        config = opt.1.to_string();
                    }
                }

                let env = match Mu::config(Some(config)) {
                    Some(config) => Mu::env(&config),
                    None => {
                        eprintln!("option: configuration error");
                        std::process::exit(-1)
                    }
                };

                for opt in opts {
                    match opt.0 {
                        OptType::Config => (),
                        OptType::Ping => ping = true,
                        OptType::Socket => socket = opt.1.to_string(),
                        OptType::Eval => match Mu::eval_str(env, &opt.1) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!(
                                    "runtime: error {}, {}",
                                    opt.1,
                                    Mu::exception_string(env, e)
                                );
                                std::process::exit(-1);
                            }
                        },
                        OptType::Load => match Mu::load(env, &opt.1) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!(
                                    "runtime: failed to load {}, {}",
                                    &opt.1,
                                    Mu::exception_string(env, e)
                                );
                                std::process::exit(-1);
                            }
                        },
                    }
                }
            }
            None => {
                eprintln!("option: error");
                std::process::exit(-1)
            }
        }

        let socket_addr = match socket.to_socket_addrs() {
            Ok(mut addrs) => match addrs.next() {
                Some(addr) => addr,
                None => {
                    eprintln!("{} is not a legal socket designator", socket);
                    std::process::exit(-1)
                }
            },
            Err(_) => {
                eprintln!("cannot resolve host {}", socket);
                std::process::exit(-1)
            }
        };

        if ping {
            let is_server_port_open =
                block_on(oports::is_port_open(socket_addr.ip(), socket_addr.port()));
            if is_server_port_open {
                println!("server port {} is open", socket);
                std::process::exit(-1)
            } else {
                println!("server port {} is not open", socket);
                std::process::exit(-1)
            }
        }

        ServerConfig { socket_addr }
    }
}
