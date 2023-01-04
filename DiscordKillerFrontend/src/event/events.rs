use crossterm::event::EventStream;
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use std::time::Duration;

pub enum Event {
    CrosstermEvent(crossterm::event::Event),
    Tick,
}

pub struct EventManager {
    pub ces: EventStream,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            ces: crossterm::event::EventStream::new(),
        }
    }

    pub async fn get(&mut self, tick_rate: Duration) -> Event {
        let mut delay = Delay::new(tick_rate).fuse();
        let mut crossterm_event = self.ces.next().fuse();

        select! {
            _ = delay => {
                Event::Tick
            }

            maybe_crossterm_event = crossterm_event => {
                match maybe_crossterm_event {
                    Some(Ok(event)) => {
                        Event::CrosstermEvent(event)
                    }
                    Some(Err(e)) => {
                        panic!("{e}")
                    }
                    None => panic!("Something went terribly wrong")
                }
            }
        }
    }
}
