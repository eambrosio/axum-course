// Simple model layer

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    ctx::Ctx,
    error::{Error, Result},
};

// region:      --Ticket types
#[derive(Clone, Serialize, Debug)]
pub struct Ticket {
    pub id: u64,
    pub user_id: u64,
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

    pub async fn create_ticket(&self, ctx: Ctx, ticket_payload: TicketPayload) -> Result<Ticket> {
        let mut tickets = self.tickets_store.lock().unwrap();

        let ticket = Ticket {
            id: tickets.len() as u64,
            user_id: ctx.user_id(),
            title: ticket_payload.title,
        };
        tickets.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
// endregion:   --Model controller
