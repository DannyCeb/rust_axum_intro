use axum::extract::Path;
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, Json};

use crate::ctx::Ctx;
use crate::error::Result;
use crate::model::{ModelController, Ticket, TicketForCreate};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}
async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    Ok(Json(mc.create_ticket(ticket_fc, ctx).await?))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    Ok(Json(mc.list_tickets(ctx).await?))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
    ctx: Ctx,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete ticket", "HANDLER");

    Ok(Json(mc.delete_ticket(id, ctx).await?))
}
