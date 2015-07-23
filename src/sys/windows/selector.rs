use io;
use event::IoEvent;
use std::ptr;
use sys::windows;

#[derive(Debug)]
pub struct Selector {
    poll: windows::Poll,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        let poll = try!(windows::Poll::global());
        Ok(Selector { poll: poll })
    }

    pub fn select(&mut self, events: &mut Events, timeout_ms: usize) -> io::Result<()> {
        self.poll.poll()
    }
}

#[derive(Debug)]
pub struct Events;

impl Events {
    pub fn new() -> Events {
        Events
    }

    pub fn len(&self) -> usize {
        0
    }

    pub fn get(&self, idx: usize) -> IoEvent {
        unimplemented!();
    }
}
