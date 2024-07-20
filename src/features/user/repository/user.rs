use actix_web::http::StatusCode;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::core::error::CustomError;
use crate::core::types::{AppResult, PsqlConn};
use crate::features::post::models::post::{NewPost, Post, UpdatePost};
use crate::{
    features::post::entity::post::PostEntity,
    schema::posts::{self, dsl::*},
};

#[derive(Clone)]
pub struct PostRepository {
    source: PsqlConn,
}

impl PostRepository {
    pub fn new(source: PsqlConn) -> Self {
        PostRepository { source }
    }
}

pub trait IPostRepository: Send + Sync {
    fn create(&self, params: NewPost) -> AppResult<String>;
    fn get_posts(&self) -> AppResult<Vec<Post>>;
    fn get_post_by_id(&self, post_id: i32) -> AppResult<PostEntity>;
    fn update_post(&self, post_id: i32, params: UpdatePost) -> AppResult<PostEntity>;
    fn delete(&self, post_id: i32) -> AppResult<String>;
}

impl IPostRepository for PostRepository {
    fn create(&self, params: NewPost) -> AppResult<String> {
        diesel::insert_into(posts)
            .values(&params)
            .execute(&mut self.source.get().unwrap())
            .map(|_| StatusCode::OK.as_u16().to_string())
            .map_err(|_| CustomError::InternalError)
    }

    fn get_posts(&self) -> AppResult<Vec<Post>> {
        posts
            .load::<Post>(&mut self.source.get().unwrap())
            .map_err(|_| CustomError::InternalError)
    }

    fn get_post_by_id(&self, post_id: i32) -> AppResult<PostEntity> {
        posts::table
            .filter(id.eq(post_id))
            .get_result::<Post>(&mut self.source.get().unwrap())
            .map(|p| PostEntity {
                id: p.id,
                user_id: p.user_id,
                title: p.title,
                body: p.body,
                published: p.published,
            })
            .map_err(|_| CustomError::PostNotFoundError)
    }

    fn update_post(&self, post_id: i32, params: UpdatePost) -> AppResult<PostEntity> {
        let now = Utc::now().naive_utc();

        self.get_post_by_id(post_id)
            .map(|p| {
                diesel::update(posts.find(p.id))
                    .set(UpdatePost {
                        updated_at: Some(now),
                        ..params
                    })
                    .get_result::<Post>(&mut self.source.get().unwrap())
                    .map(|p| PostEntity {
                        id: p.id,
                        user_id: p.user_id,
                        title: p.title,
                        body: p.body,
                        published: p.published,
                    })
                    .map_err(|e| match e {
                        diesel::result::Error::NotFound => CustomError::PostNotFoundError,
                        _ => CustomError::InternalError,
                    })
            })
            .map_err(|_| CustomError::PostNotFoundError)?
    }

    fn delete(&self, post_id: i32) -> AppResult<String> {
        self.get_post_by_id(post_id)
            .map(|p| {
                diesel::delete(posts.find(p.id))
                    .execute(&mut self.source.get().unwrap())
                    .map(|_| format!("post with ID {} deleted successfully", p.id))
                    .map_err(|_| CustomError::InternalError)
            })
            .map_err(|_| CustomError::PostNotFoundError)?
    }
}
