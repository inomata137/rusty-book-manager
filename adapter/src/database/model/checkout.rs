use chrono::{DateTime, Utc};
use kernel::model::id::{BookId, CheckoutId, UserId};

pub struct CheckoutStateRow {
    pub book_id: BookId,
    pub checkout_id: Option<CheckoutId>,
    pub user_id: Option<UserId>,
}

pub struct CheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<CheckoutRow> for kernel::model::checkout::Checkout {
    fn from(value: CheckoutRow) -> Self {
        let CheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            title,
            author,
            isbn,
        } = value;
        Self {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            returned_at: None,
            book: kernel::model::checkout::CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}

pub struct ReturnedCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<ReturnedCheckoutRow> for kernel::model::checkout::Checkout {
    fn from(value: ReturnedCheckoutRow) -> Self {
        let ReturnedCheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            returned_at,
            title,
            author,
            isbn,
        } = value;
        Self {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            returned_at: Some(returned_at),
            book: kernel::model::checkout::CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}
