use actix_web::{
    rt::time::{interval_at, Instant},
    web::{Bytes, Data},
    Error,
};
use futures::{Stream, StreamExt};
use std::{
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

type StdReceiver<T> = std::sync::mpsc::Receiver<T>;

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    pub fn create(file_content_receiver: StdReceiver<String>) -> Data<Mutex<Self>> {
        let me = Data::new(Mutex::new(Broadcaster::new()));
        Broadcaster::spawn_ping(me.clone());
        Broadcaster::watch(me.clone(), file_content_receiver);
        me
    }

    fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    fn watch(me: Data<Mutex<Self>>, file_content_receiver: StdReceiver<String>) {
        actix_web::rt::spawn(async move {
            loop {
                match file_content_receiver.recv() {
                    Ok(string) => me.lock().unwrap().send(string.as_str()),
                    Err(e) => me.lock().unwrap().send(e.to_string().as_str()),
                }
            }
        });
    }

    fn spawn_ping(me: Data<Mutex<Self>>) {
        actix_web::rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(10));
            while task.next().await.is_some() {
                me.lock().unwrap().remove_stale_clients();
            }
        });
    }

    fn remove_stale_clients(&mut self) {
        self.clients = self
            .clients
            .iter()
            .filter_map(|client| {
                match client.try_send(Bytes::from("event: ping\ndata: ping\n\n")) {
                    Ok(_) => Some(client.clone()),
                    Err(_) => None,
                }
            })
            .collect();
    }

    pub fn new_client(&mut self) -> Client {
        let (bytes_sender, bytes_receiver) = channel(100);

        bytes_sender
            .try_send(Bytes::from("event: connected\ndata: connected\n\n"))
            .unwrap();

        self.clients.push(bytes_sender);
        Client(bytes_receiver)
    }

    pub fn send(&self, msg: &str) {
        let msg = Bytes::from(["event: message\ndata: ", msg, "\n\n"].concat());

        for client in self.clients.iter() {
            client.try_send(msg.clone()).unwrap_or(());
        }
    }
}

pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
