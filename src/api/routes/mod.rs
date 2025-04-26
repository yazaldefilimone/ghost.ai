use sea_orm::DbConn;
use std::sync::Arc;

pub mod chat;

#[derive(Clone)]
pub struct AppState {
	pub db: Arc<DbConn>,
}
