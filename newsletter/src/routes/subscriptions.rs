use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::web::Form;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
//     let name = SubscriberName::parse(form.name)?;
//     let email = SubscriberEmail::parse(form.email)?;
//
//     Ok(NewSubscriber { email, name })
// }

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;

        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(form, connection_pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
    };

    match insert_subscriber(&connection_pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, connection_pool),
    fields(test_label = "test123")
)]
async fn insert_subscriber(
    connection_pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.to_str(),
        new_subscriber.name.to_str(),
        Utc::now(),
    )
    .execute(connection_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);

        e
    })?;

    tracing::info!("Successfully added to the database.");

    Ok(())
}

fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    let is_too_long = s.len() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}
