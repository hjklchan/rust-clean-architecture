use crate::domain::entities::article::Article;


#[async_trait::async_trait]
pub trait ArticleRepository {
    async fn create(&self, article: Article) -> u64;
    async fn update(&self, article: Article);
    async fn get_by_id(&self, id: u64) -> Article;
    async fn soft_delete(&self, id: u64);
    async fn delete(&self, id: u64);
}