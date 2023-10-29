use axum::{extract::State, Json};

use crate::error::Result;
use crate::model::{ModelController, Ticket, TicketForCreate};

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    todo!()
}