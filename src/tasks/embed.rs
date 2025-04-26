#![allow(unused_mut, dead_code)]
use crate::signal::SignalReceiver;
use sea_orm::DbConn;
use std::sync::Arc;

// todo: embed
pub async fn run(mut embed_receiver: SignalReceiver, _db_cnn: Arc<DbConn>) {
	let _ = embed_receiver;
	todo!()
}
