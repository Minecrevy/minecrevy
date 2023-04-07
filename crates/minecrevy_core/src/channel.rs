use flume::{Receiver, Sender};

pub struct Channel<T> {
    pub send: Sender<T>,
    pub recv: Receiver<T>,
}

impl<T> From<(Sender<T>, Receiver<T>)> for Channel<T> {
    fn from((send, recv): (Sender<T>, Receiver<T>)) -> Self {
        Self { send, recv }
    }
}
