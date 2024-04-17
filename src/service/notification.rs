use std::thread;

use rocket::http::Status;
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;

use bambangshop_receiver::{compose_error_response, Result, APP_CONFIG, REQWEST_CLIENT};
use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::repository::notification::NotificationRepository;

pub struct NotificationService;

impl NotificationService {
}