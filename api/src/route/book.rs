use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::{
    book::{delete_book, register_book, show_book, show_book_list, update_book},
    checkout::{checkout_book, return_book, show_checked_out_list},
};

pub fn build_book_routers() -> Router<AppRegistry> {
    let book_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book))
        .route("/:book_id", put(update_book))
        .route("/:book_id", delete(delete_book));

    let checkout_router = Router::new()
        .route("/checkouts", get(show_checked_out_list))
        .route("/:book_id/checkouts", post(checkout_book))
        .route("/:book_id/checkouts/:checkout_id", put(return_book))
        .route("/:book_id/checkout-history", get(show_checked_out_list));

    Router::new().nest("/books", book_routers.merge(checkout_router))
}
