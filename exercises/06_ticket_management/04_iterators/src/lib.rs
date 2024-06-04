use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

pub struct TicketStoreIter {
    store: TicketStore,
    index: usize,
}

impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = TicketStoreIter;

    fn into_iter(self) -> Self::IntoIter {
        TicketStoreIter {
            store: self,
            index: 0,
        }
    }
}

impl Iterator for TicketStoreIter {
    type Item = Ticket;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.store.tickets.len() {
            self.index += 1;
            Some(self.store.tickets[self.index - 1].clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
