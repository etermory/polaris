use http::{header, HeaderValue, StatusCode};
use std::path::PathBuf;

use crate::service::dto::ThumbnailSize;
use crate::service::test::{constants::*, protocol, ServiceType, TestService};
use crate::test_name;

#[actix_web::test]
async fn audio_requires_auth() {
	let mut service = ServiceType::new(&test_name!()).await;

	let path: PathBuf = [TEST_MOUNT_NAME, "Khemmis", "Hunted", "02 - Candlelight.mp3"]
		.iter()
		.collect();

	let request = protocol::audio(&path);
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn audio_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login_admin().await;
	service.index().await;
	service.login().await;

	let path: PathBuf = [TEST_MOUNT_NAME, "Khemmis", "Hunted", "02 - Candlelight.mp3"]
		.iter()
		.collect();

	let request = protocol::audio(&path);
	let response = service.fetch_bytes(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
	assert_eq!(response.body().len(), 24_142);
	assert_eq!(
		response.headers().get(header::CONTENT_LENGTH).unwrap(),
		"24142"
	);
}

#[actix_web::test]
async fn audio_does_not_encode_content() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login_admin().await;
	service.index().await;
	service.login().await;

	let path: PathBuf = [TEST_MOUNT_NAME, "Khemmis", "Hunted", "02 - Candlelight.mp3"]
		.iter()
		.collect();

	let mut request = protocol::audio(&path);
	let headers = request.headers_mut();
	headers.append(
		header::ACCEPT_ENCODING,
		HeaderValue::from_str("gzip, deflate, br").unwrap(),
	);

	let response = service.fetch_bytes(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
	assert_eq!(response.body().len(), 24_142);
	assert_eq!(response.headers().get(header::TRANSFER_ENCODING), None);
	assert_eq!(
		response.headers().get(header::CONTENT_LENGTH).unwrap(),
		"24142"
	);
}

#[actix_web::test]
async fn audio_partial_content() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login_admin().await;
	service.index().await;
	service.login().await;

	let path: PathBuf = [TEST_MOUNT_NAME, "Khemmis", "Hunted", "02 - Candlelight.mp3"]
		.iter()
		.collect();

	let mut request = protocol::audio(&path);
	let headers = request.headers_mut();
	headers.append(
		header::RANGE,
		HeaderValue::from_str("bytes=100-299").unwrap(),
	);

	let response = service.fetch_bytes(&request).await;
	assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
	assert_eq!(response.body().len(), 200);
	assert_eq!(
		response.headers().get(header::CONTENT_LENGTH).unwrap(),
		"200"
	);
}

#[actix_web::test]
async fn audio_bad_path_returns_not_found() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login().await;

	let path: PathBuf = ["not_my_collection"].iter().collect();

	let request = protocol::audio(&path);
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn thumbnail_requires_auth() {
	let mut service = ServiceType::new(&test_name!()).await;

	let path: PathBuf = [TEST_MOUNT_NAME, "Khemmis", "Hunted", "Folder.jpg"]
		.iter()
		.collect();

	let size = None;
	let pad = None;
	let request = protocol::thumbnail(&path, size, pad);
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn thumbnail_golden_path() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login_admin().await;
	service.index().await;
	service.login().await;

	let path: PathBuf = [TEST_MOUNT_NAME, "Khemmis", "Hunted", "Folder.jpg"]
		.iter()
		.collect();

	let size = None;
	let pad = None;
	let request = protocol::thumbnail(&path, size, pad);
	let response = service.fetch_bytes(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
}

#[actix_web::test]
async fn thumbnail_bad_path_returns_not_found() {
	let mut service = ServiceType::new(&test_name!()).await;
	service.complete_initial_setup().await;
	service.login().await;

	let path: PathBuf = ["not_my_collection"].iter().collect();

	let size = None;
	let pad = None;
	let request = protocol::thumbnail(&path, size, pad);
	let response = service.fetch(&request).await;
	assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn thumbnail_size_default() {
	thumbnail_size(&test_name!(), None, None, 400).await;
}

#[actix_web::test]
async fn thumbnail_size_small() {
	thumbnail_size(&test_name!(), Some(ThumbnailSize::Small), None, 400).await;
}

#[actix_web::test]
#[cfg(not(tarpaulin))]
async fn thumbnail_size_large() {
	thumbnail_size(&test_name!(), Some(ThumbnailSize::Large), None, 1200).await;
}

#[actix_web::test]
#[cfg(not(tarpaulin))]
async fn thumbnail_size_native() {
	thumbnail_size(&test_name!(), Some(ThumbnailSize::Native), None, 1423).await;
}

async fn thumbnail_size(name: &str, size: Option<ThumbnailSize>, pad: Option<bool>, expected: u32) {
	let mut service = ServiceType::new(name).await;
	service.complete_initial_setup().await;
	service.login_admin().await;
	service.index().await;
	service.login().await;

	let path: PathBuf = [TEST_MOUNT_NAME, "Tobokegao", "Picnic", "Folder.png"]
		.iter()
		.collect();

	let request = protocol::thumbnail(&path, size, pad);
	let response = service.fetch_bytes(&request).await;
	assert_eq!(response.status(), StatusCode::OK);
	let thumbnail = image::load_from_memory(response.body()).unwrap().to_rgb8();
	assert_eq!(thumbnail.width(), expected);
	assert_eq!(thumbnail.height(), expected);
}
