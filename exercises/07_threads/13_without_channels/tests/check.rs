use std::sync::{Arc, RwLock};
use std::thread::spawn;

use ticket_fields::test_helpers::{ticket_description, ticket_title};
use without_channels::data::TicketDraft;
use without_channels::store::TicketStore;

#[test]
fn works() {
    // wox, 没想到还能改tests代码，前面可没有这个规矩。
    // 这个题我是看了答案的。没看答案前一直在store.rs里折腾代码。
    //let store = TicketStore::new();
    let store = Arc::new(RwLock::new(TicketStore::new()));

    let store1 = store.clone();
    let client1 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        store1.write().unwrap().add_ticket(draft)
    });

    let store2 = store.clone();
    let client2 = spawn(move || {
        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        store2.write().unwrap().add_ticket(draft)
    });

    let ticket_id1 = client1.join().unwrap();
    let ticket_id2 = client2.join().unwrap();

    let reader = store.read().unwrap();

    let ticket1 = reader.get(ticket_id1).unwrap();
    assert_eq!(ticket_id1, ticket1.read().unwrap().id);

    let ticket2 = reader.get(ticket_id2).unwrap();
    assert_eq!(ticket_id2, ticket2.read().unwrap().id);
}
