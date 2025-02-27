use chrono::{DateTime, Utc};
use kernel::model::id::{BookId, CheckoutId, UserId};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutsResponse {
    pub items: Vec<CheckoutResponse>,
}

impl From<Vec<kernel::model::checkout::Checkout>> for CheckoutsResponse {
    fn from(value: Vec<kernel::model::checkout::Checkout>) -> Self {
        Self {
            items: value.into_iter().map(CheckoutResponse::from).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutResponse {
    pub id: CheckoutId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBookResponse,
}

impl From<kernel::model::checkout::Checkout> for CheckoutResponse {
    fn from(value: kernel::model::checkout::Checkout) -> Self {
        let kernel::model::checkout::Checkout {
            id,
            checked_out_by,
            checked_out_at,
            returned_at,
            book,
        } = value;
        Self {
            id,
            checked_out_by,
            checked_out_at,
            returned_at,
            book: book.into(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutBookResponse {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<kernel::model::checkout::CheckoutBook> for CheckoutBookResponse {
    fn from(value: kernel::model::checkout::CheckoutBook) -> Self {
        let kernel::model::checkout::CheckoutBook {
            book_id,
            title,
            author,
            isbn,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
        }
    }
}
