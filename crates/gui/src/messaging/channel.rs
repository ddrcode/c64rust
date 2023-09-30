use crossbeam_channel::{bounded, Receiver, Sender};
use machine::client::{ NonInteractiveClient, ClientEvent };

lazy_static! {
    static ref CHANNEL: (Sender<ClientEvent>, Receiver<ClientEvent>) = bounded(10);

    // FIXME ther must be better way of doing it
    static ref CLIENT_CHANNEL_SENDER: &'static Sender<ClientEvent> = &CHANNEL.0;
    static ref CLIENT_CHANNEL_RECEIVER: &'static Receiver<ClientEvent> = &CHANNEL.1;
}

pub fn send_client_event(event: ClientEvent) {
    CLIENT_CHANNEL_SENDER.send(event).unwrap_or_else(|e|{
        log::error!("Sending message to the client channel failed: {e}");
    });
}

pub fn connect_client<T: NonInteractiveClient>(client: &mut T) {
    client.set_receiver(CLIENT_CHANNEL_RECEIVER.clone());
}
