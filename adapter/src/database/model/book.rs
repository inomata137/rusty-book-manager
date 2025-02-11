use uuid::Uuid;

pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<BookRow> for kernel::model::book::Book {
    fn from(value: BookRow) -> Self {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
