#![allow(dead_code)]

use sea_orm::{entity::prelude::*, Database};
use serde::{Deserialize, Serialize};
pub mod queries;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::utils::database_file_path;

pub async fn connect() -> DatabaseConnection {
	let path = database_file_path();
	match Database::connect(format!("sqlite://{}?mode=rwc", path.display())).await {
		Ok(db) => db,
		Err(e) => {
			panic!("Error connecting to database: {:?}", e);
		}
	}
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "entry")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub app_name: Option<String>,
	pub window_title: Option<String>,
	pub extracted_text: String,
	pub created_at: DateTime,
	pub updated_at: DateTime,
}

impl Model {
	pub async fn insert_entry(db: &DatabaseConnection, data: ActiveModel) -> Model {
		data.insert(db).await.unwrap_or_else(|_| panic!("Failed to insert entry"))
	}

	pub async fn get_all(db: &DatabaseConnection) -> Vec<Model> {
		Entity::find().all(db).await.unwrap_or_else(|_| panic!("Failed to insert entry"))
	}

	pub async fn find_by_app(db: &DatabaseConnection, app: &str) -> Vec<Model> {
		Entity::find()
			.filter(Column::AppName.eq(app))
			.all(db)
			.await
			.unwrap_or_else(|_| panic!("Failed to insert entry"))
	}

	pub async fn find_by_window_title(db: &DatabaseConnection, window_title: &str) -> Vec<Model> {
		Entity::find()
			.filter(Column::WindowTitle.eq(window_title))
			.all(db)
			.await
			.unwrap_or_else(|_| panic!("Failed to insert entry"))
	}
	pub async fn find_by_window_title_and_app(
		db: &DatabaseConnection,
		window_title: &str,
		app: &str,
	) -> Vec<Model> {
		Entity::find()
			.filter(Column::WindowTitle.eq(window_title))
			.filter(Column::AppName.eq(app))
			.all(db)
			.await
			.unwrap_or_else(|_| panic!("Failed to insert entry"))
	}
	pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Option<Model> {
		Entity::find()
			.filter(Column::Id.eq(id))
			.one(db)
			.await
			.unwrap_or_else(|_| panic!("Failed to insert entry"))
	}

	pub async fn update_entry(db: &DatabaseConnection, data: ActiveModel) -> Model {
		data.update(db).await.unwrap_or_else(|_| panic!("Failed to insert entry"))
	}

	pub async fn delete_entry(db: &DatabaseConnection, id: i32) {
		Entity::delete_by_id(id).exec(db).await.unwrap_or_else(|_| panic!("Failed to insert entry"));
	}

	pub async fn delete_by_app(db: &DatabaseConnection, app: &str) {
		Entity::delete_many()
			.filter(Column::AppName.eq(app))
			.exec(db)
			.await
			.unwrap_or_else(|_| panic!("Failed to insert entry"));
	}

	pub async fn delete_all(db: &DatabaseConnection) {
		Entity::delete_many().exec(db).await.unwrap_or_else(|_| panic!("Failed to insert entry"));
	}
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
