use http::StatusCode;
use std::default::Default;

use crate::app::user;
use crate::service::dto;
use crate::service::test::{constants::*, protocol, ServiceType, TestService};
use crate::test_name;

#[actix_web::test]
async fn list_users_requires_admin() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::list_users();

	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	service.login().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn list_users_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login_admin().await;
	let request = protocol::list_users();
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
}

#[actix_web::test]
async fn create_user_requires_admin() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::create_user(dto::NewUser {
		name: "Walter".into(),
		password: "secret".into(),
		admin: false,
	});

	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	service.login().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn create_user_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login_admin().await;

	let new_user = dto::NewUser {
		name: "Walter".into(),
		password: "secret".into(),
		admin: false,
	};
	let request = protocol::create_user(new_user);
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
}

#[actix_web::test]
async fn update_user_requires_admin() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::update_user("Walter", dto::UserUpdate::default());

	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	service.login().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn update_user_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::update_user("Walter", dto::UserUpdate::default());

	service.login_admin().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
}

#[actix_web::test]
async fn update_user_cannot_unadmin_self() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::update_user(
		TEST_USERNAME_ADMIN,
		dto::UserUpdate {
			new_is_admin: Some(false),
			..Default::default()
		},
	);

	service.login_admin().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[actix_web::test]
async fn delete_user_requires_admin() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::delete_user("Walter");

	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	service.login().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn delete_user_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::delete_user("Walter");

	service.login_admin().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
}

#[actix_web::test]
async fn delete_user_cannot_delete_self() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	let request = protocol::delete_user(TEST_USERNAME_ADMIN);
	service.login_admin().await;
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[actix_web::test]
async fn get_preferences_requires_auth() {
	let mut service = ServiceType::new(&test_name!()).await;
	let request = protocol::get_preferences();
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn get_preferences_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login().await;

	let request = protocol::get_preferences();
	let response = service.fetch_json::<_, user::Preferences>(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
}

#[actix_web::test]
async fn put_preferences_requires_auth() {
	let mut service = ServiceType::new(&test_name!()).await;
	let request = protocol::put_preferences(user::Preferences::default());
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn put_preferences_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login().await;

	let request = protocol::put_preferences(user::Preferences::default());
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
}
