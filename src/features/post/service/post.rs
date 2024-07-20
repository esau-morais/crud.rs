use std::sync::Arc;

use crate::core::error::CustomError;
use crate::core::types::AppResult;
use crate::features::post::entity::post::PostEntity;
use crate::features::post::models::post::{NewPost, Post, UpdatePost};
use crate::features::post::repository::post::IPostRepository;

#[derive(Clone)]
pub struct PostService {
    pub post_repo: Arc<dyn IPostRepository>,
}

impl PostService {
    pub fn new(post_repo: Arc<dyn IPostRepository>) -> Self {
        Self { post_repo }
    }
}

pub trait IPostService: Send + Sync {
    fn create_post(&self, params: NewPost) -> AppResult<String>;
    fn get_posts(&self) -> AppResult<Vec<Post>>;
    fn get_post_by_id(&self, post_id: i32) -> AppResult<PostEntity>;
    fn update_post(&self, post_id: i32, params: UpdatePost) -> AppResult<PostEntity>;
    fn delete_post(&self, post_id: i32) -> AppResult<String>;
}

impl IPostService for PostService {
    fn create_post(&self, params: NewPost) -> AppResult<String> {
        self.post_repo
            .create(params)
            .map_err(|e| CustomError::BadRequest {
                message: e.to_string(),
            })
    }

    fn get_posts(&self) -> AppResult<Vec<Post>> {
        self.post_repo.get_posts()
    }

    fn get_post_by_id(&self, post_id: i32) -> AppResult<PostEntity> {
        self.post_repo.get_post_by_id(post_id)
    }

    fn update_post(&self, post_id: i32, params: UpdatePost) -> AppResult<PostEntity> {
        self.post_repo.update_post(post_id, params)
    }

    fn delete_post(&self, post_id: i32) -> AppResult<String> {
        self.post_repo.delete(post_id)
    }
}
