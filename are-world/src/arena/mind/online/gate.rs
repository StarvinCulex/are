use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread::JoinHandle;

use crate::arena::Cosmos;
use crate::grids::Coord;

use super::{Mind, Player};

pub struct Gate {
    connections: mpsc::Receiver<(TcpStream, SocketAddr)>,
    _listen_thread: JoinHandle<()>,
}

impl Gate {
    pub fn listen<Addr: std::net::ToSocketAddrs + Sync>(address: Addr) -> Gate {
        let (send, recv) = mpsc::channel();

        let listener = TcpListener::bind(address).unwrap();
        let listen_thread = std::thread::spawn(move || loop {
            while let Ok(x) = listener.accept() {
                if send.send(x).is_err() {
                    return;
                }
            }
        });

        Gate {
            connections: recv,
            _listen_thread: listen_thread,
        }
    }
}

impl Mind for Gate {
    fn observe(&mut self, cosmos: &Cosmos) -> Result<(), ()> {
        Ok(())
    }

    fn make_move(&mut self, cosmos: &Cosmos) -> Result<(), ()> {
        loop {
            match self.connections.try_recv() {
                Err(TryRecvError::Disconnected) => return Err(()),
                Err(TryRecvError::Empty) => return Ok(()),
                Ok((stream, addr)) => {
                    let Coord(x, y) = *cosmos.plate.size();
                    if let Ok(player) = Player::new(
                        stream,
                        addr,
                        Coord(0, 0) | Coord(x - 1, y - 1),
                        Coord(x as usize, y as usize),
                    ) {
                        cosmos.angelos.join_lock(Box::new(player))
                    }
                }
            }
        }
    }

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()> {
        Ok(())
    }
}
