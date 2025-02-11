use chrono::{DateTime, Utc};
use kernel::model::{
    id::{BookId, CheckoutId, UserId},
    user::BookOwner,
};

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owned_by: UserId,
    pub owner_name: String,
}

impl BookRow {
    pub fn into_book(
        self,
        checkout: Option<kernel::model::book::Checkout>,
    ) -> kernel::model::book::Book {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
            owned_by,
            owner_name,
        } = self;
        kernel::model::book::Book {
            id: book_id,
            title,
            author,
            isbn,
            description,
            owner: BookOwner {
                id: owned_by,
                name: owner_name,
            },
            checkout,
        }
    }
}

pub struct PaginatedBookRow {
    pub total: i64,
    pub id: BookId,
}

pub struct BookCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub user_name: String,
    pub checked_out_at: DateTime<Utc>,
}

impl From<BookCheckoutRow> for kernel::model::book::Checkout {
    fn from(value: BookCheckoutRow) -> Self {
        let BookCheckoutRow {
            checkout_id,
            user_id,
            checked_out_at,
            ..
        } = value;
        kernel::model::book::Checkout {
            checkout_id,
            checked_out_by: user_id,
            checked_out_at,
        }
    }
}
