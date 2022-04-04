use std::collections::VecDeque;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};

use crate::arena::{cosmos, Cosmos};
use crate::conencode::NVTer;
use crate::cui::Monitor;
use crate::{ConEncoder, Coord, Interval, SWord, Window};

use super::{resource, Mind};

pub struct Player {
    tcp_stream: TcpStream,
    socket_addr: SocketAddr,

    con: NVTer,
    monitor: Monitor<String, fn(&String, usize) -> [[char; 1]; 5], 5, 1>,

    mode: Mode,
    query_result: Vec<Vec<u8>>,
    /// 观察的区域
    watch_area: Coord<Interval<isize>>,
    /// 暂存的信息
    input_buffer: Vec<u8>,
    output_buffer: Vec<u8>,

    /// set_cosmos时执行的命令
    mut_commands: VecDeque<MutCommand>,
}

enum Mode {
    Monitor,
    Blind,
}

include!("execute.rs");

impl Player {
    pub(crate) fn new(
        tcp_stream: TcpStream,
        socket_addr: SocketAddr,
        watch_area: Coord<Interval<isize>>,
        grid_size: Coord<usize>,
    ) -> Result<Player, ()> {
        tcp_stream.set_nonblocking(true).or(Err(()))?;
        Ok(Player {
            tcp_stream,
            socket_addr,
            watch_area,
            con: NVTer::new(),
            monitor: Monitor::new(grid_size, resource::resource),
            mut_commands: VecDeque::new(),
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
            mode: Mode::Monitor,
            query_result: Vec::new(),
        })
    }
}

impl Mind for Player {
    fn observe(&mut self, cosmos: &Cosmos) -> Result<(), ()> {
        let mut data = Vec::new();
        // output monitor
        if let Mode::Monitor = self.mode {
            data.append(&mut self.render(cosmos)?);
        }

        // output result
        for mut result in std::mem::take(&mut self.query_result) {
            data.extend_from_slice(b"\r\n< ");
            data.append(&mut result);
        }

        // output input buffer
        if !self.input_buffer.is_empty() {
            data.extend_from_slice(b"\r\n> ");
            data.extend_from_slice(self.input_buffer.as_slice());
        }

        self.send(data)
    }

    fn make_move(&mut self, cosmos: &Cosmos) -> Result<(), ()> {
        // read input
        let data = self.receive()?;

        let remain_string = String::from_utf8(data).or(Err(()))?;
        let mut remain = remain_string.as_str();
        // for each command
        while let Some(x) = Player::split_next_command(remain)? {
            remain = x.2;
            let command = x.0;
            let args = x.1;
            // parse command
            match Player::parse_command(command, args) {
                // do nothing
                Err(_) => (),
                // save command
                Ok(Command::Mut(mut_cmd)) => self.mut_commands.push_back(mut_cmd),
                // execute command
                Ok(Command::Const(const_cmd)) => const_cmd.exec(self, cosmos),
            }
        }
        self.input_buffer.extend_from_slice(remain.as_bytes());

        Ok(())
    }

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()> {
        // execute commands
        let cmds = std::mem::take(&mut self.mut_commands);
        for cmd in cmds {
            cmd.exec(self, cosmos);
        }

        Ok(())
    }
}

impl Player {
    const INPUT_BUFFER_MAX_SIZE: usize = 4096;
    const OUTPUT_BUFFER_MAX_SIZE: usize = 1024 * 1024;
    const LINE_CMD_BEGIN: u8 = b'/';
    const LINE_CMD_END: &'static str = "\r\n";

    fn render(&mut self, cosmos: &Cosmos) -> Result<Vec<u8>, ()> {
        let window = cosmos.plate.area(self.watch_area).map(|b| b.ground.name());
        println!("~{}", window);

        self.monitor.put(window);
        let display = self.monitor.render();
        let chars = self.con.flush(&display);
        let bytes: Vec<u8> = chars.into_iter().collect::<String>().into();
        Ok(bytes)
    }

    fn send(&mut self, mut data: Vec<u8>) -> Result<(), ()> {
        if self.output_buffer.len() + data.len() > Player::OUTPUT_BUFFER_MAX_SIZE {
            return Err(());
        }
        if self.output_buffer.is_empty() {
            self.output_buffer = data;
        } else {
            self.output_buffer.append(&mut data);
        }

        match self.tcp_stream.write(self.output_buffer.as_slice()) {
            Ok(n) => {
                if n == self.output_buffer.len() {
                    self.output_buffer.clear();
                } else {
                    self.output_buffer = self.output_buffer.as_slice()[n..].to_vec();
                }
                Ok(())
            }
            Err(e) => match e.kind() {
                ErrorKind::WouldBlock | ErrorKind::Interrupted | ErrorKind::TimedOut => Ok(()),
                _ => Err(()),
            },
        }
    }

    fn receive(&mut self) -> Result<Vec<u8>, ()> {
        let mut buf_vec = vec![0u8; Player::INPUT_BUFFER_MAX_SIZE - self.input_buffer.len()];
        let buf = buf_vec.as_mut_slice();

        match self.tcp_stream.read(buf) {
            Ok(n) => self.input_buffer.append(&mut buf[..n].to_vec()),
            Err(e) => match e.kind() {
                ErrorKind::WouldBlock | ErrorKind::Interrupted | ErrorKind::TimedOut => {}
                _ => return Err(()),
            },
        }

        Ok(std::mem::take(&mut self.input_buffer))
    }

    /// 返回 `Ok(None)` 意味着命令不完整，需要继续读  
    /// 返回 `Err(())` 意味输入有误  
    /// 返回 `Ok(Some(a, b, c))`  
    fn split_next_command(
        command: &str,
    ) -> Result<Option<(&str, std::str::SplitAsciiWhitespace, &str)>, ()> {
        let command = command.trim_start();
        if command.is_empty() {
            return Ok(None);
        }

        let ch = command.as_bytes()[0];
        if !ch.is_ascii() {
            return Err(());
        }

        match ch {
            Player::LINE_CMD_BEGIN => {
                if let Some((cmd, remain)) = command.split_at(1).1.split_once(Player::LINE_CMD_END)
                {
                    let mut args = cmd.split_ascii_whitespace();
                    let cmd_head = args.next().unwrap_or(cmd);
                    Ok(Some((cmd_head, args, remain)))
                } else {
                    Ok(None)
                }
            }
            _ => {
                let (cmd, remain) = command.split_at(1);
                let mut args = cmd.split_ascii_whitespace();
                args.next();
                Ok(Some((cmd, args, remain)))
            }
        }
    }
}
