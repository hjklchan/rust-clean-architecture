use crate::application::article_repository::ArticleRepository;

pub struct CreateArticleUseCase<R: ArticleRepository> {
    article_repo: R,
}

impl<R: ArticleRepository> CreateArticleUseCase<R> {
    pub async fn execute(&self) {
        let _user = self.article_repo.create(todo!()).await;
    }
}