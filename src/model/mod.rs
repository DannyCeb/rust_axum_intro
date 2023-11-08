//! Simplistic model layer
//! (With mock-store layer )
//!

use crate::{
    ctx::Ctx,
    error::{Error, Result},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

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
}

impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate, ctx: Ctx) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        Ok(self
            .tickets_store
            .lock()
            .unwrap()
            .iter()
            .filter_map(|t| t.clone())
            .collect())
    }

    pub async fn delete_ticket(&self, id: u64, _ctx: Ctx) -> Result<Ticket> {
        let ticket = self
            .tickets_store
            .lock()
            .unwrap()
            .get_mut(id as usize)
            .and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
