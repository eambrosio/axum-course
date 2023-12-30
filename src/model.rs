// Simple model layer

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

// region:      --Ticket types
#[derive(Clone, Serialize, Debug)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketPayload {
    pub title: String,
}
// endregion:   --Ticket types

// region:      --Model controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    pub async fn create_ticket(&self, ticket_payload: TicketPayload) -> Result<Ticket> {
        let mut tickets = self.tickets_store.lock().unwrap();

        let ticket = Ticket {
            id: tickets.len() as u64,
            title: ticket_payload.title,
        };
        tickets.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
// endregion:   --Model controller
