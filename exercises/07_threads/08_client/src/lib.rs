use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    sender: Sender<Command>
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        // 20240705: 创建临时channel用于接收服务端的执行结果，这样是最佳实践吗？
        // 看来channel是单向通讯，本例中客户端用channel1向服务端发送请求，服务端回复数据要用另一个channel2。
        // channel传输的数据是静态强类型的，因而也不好做双向通讯吧。
        // 只要channel的创建和传输是轻量级的(官方文档未提及)，引入临时channel也无所谓。
        // 本例中channel定义时就决定了传输类型是T:TicketId，服务端必然发送T，客户端必然接收T，编译器予以保证。
        // 明确写成 channel::<TicketId>() 更好，有了类型推断可以省掉::<TicketId>。
        let (s, r) = std::sync::mpsc::channel();
        self.sender.send(Command::Insert { draft, response_channel: s }).unwrap();
        r.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (s, r) = std::sync::mpsc::channel();
        self.sender.send(Command::Get { id, response_channel: s }).unwrap();
        r.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sender,
    }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
