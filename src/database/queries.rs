use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use super::{ActiveModel, Column, Entity, Model};
type Result<T> = std::result::Result<T, sea_orm::DbErr>;

pub async fn insert_entry(db: &DatabaseConnection, data: ActiveModel) -> Result<Model> {
	data.insert(db).await
}

pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Model>> {
	Entity::find().all(db).await
}

pub async fn find_by_app(db: &DatabaseConnection, app: &str) -> Result<Vec<Model>> {
	Entity::find().filter(Column::AppName.eq(app)).all(db).await
}

pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Model>> {
	Entity::find().filter(Column::Id.eq(id)).one(db).await
}

pub async fn update_entry(db: &DatabaseConnection, data: ActiveModel) -> Result<Model> {
	data.update(db).await
}

pub async fn delete_entry(db: &DatabaseConnection, id: i32) -> Result<()> {
	let _ = Entity::delete_by_id(id).exec(db).await?;
	Ok(())
}

pub async fn delete_by_app(db: &DatabaseConnection, app: &str) -> Result<()> {
	let _ = Entity::delete_many().filter(Column::AppName.eq(app)).exec(db).await?;
	Ok(())
}

pub async fn delete_all(db: &DatabaseConnection) -> Result<()> {
	let _ = Entity::delete_many().exec(db).await?;
	Ok(())
}
