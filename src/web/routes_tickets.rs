use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    error::Result,
    model::{ModelController, Ticket, TicketPayload},
};

pub fn routes(state: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(state)
}

// region:      REST handlers

async fn create_ticket(
    State(state): State<ModelController>,
    Json(ticket_payload): Json<TicketPayload>,
) -> Result<Json<Ticket>> {
    println!("->> {:<15} - create ticket", "HANDLER");

    let ticket = state.create_ticket(ticket_payload).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(state): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<15} - list tickets", "HANDLER");

    let tickets = state.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(state): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<15} - delete ticket", "HANDLER");

    let ticket = state.delete_ticket(id).await?;

    Ok(Json(ticket))
}
// endregion:   REST handlers
