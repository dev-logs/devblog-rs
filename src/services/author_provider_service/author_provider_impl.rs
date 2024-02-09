use crate::entities::author::Author;
use crate::services::author_provider_service::author_provider::AuthorProviderService;

pub struct AuthorProviderServiceImpl {}

impl AuthorProviderServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl AuthorProviderService for AuthorProviderServiceImpl {
    fn tiendang(&self) -> Author {
        Author::new("tiendvlp@gmail.com", "/assets/images/authors/author_tiendang.png", "Đặng Minh Tiến", Some("Tien Dang".to_string()), )
    }

    fn list(&self) -> Vec<Author> {
        vec![
            Self::tiendang(self)
        ]
    }
}
