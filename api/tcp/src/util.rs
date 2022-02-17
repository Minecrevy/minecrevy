//! Provides a [`Channel`] type to hold a [`Sender`]/[`Receiver`] pair, as flume doesn't have a nice type for it.

use flume::{Receiver, Sender};

/// A [`Sender`]/[`Receiver`] pair.
#[derive(Clone, Debug)]
pub struct Channel<T> {
    /// The sending side.
    pub send: Sender<T>,
    /// The receiving side.
    pub recv: Receiver<T>,
}

impl<T> Channel<T> {
    /// Constructs a new channel.
    pub fn new(send: Sender<T>, recv: Receiver<T>) -> Self {
        Self {
            send,
            recv,
        }
    }
}

impl<T> From<(Sender<T>, Receiver<T>)> for Channel<T> {
    fn from((send, recv): (Sender<T>, Receiver<T>)) -> Self {
        Self::new(send, recv)
    }
}
