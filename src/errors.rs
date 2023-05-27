use rocket::{Request, response};
use rocket::http::Status;
use rocket::response::Responder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Redis(#[from] deadpool::managed::PoolError<deadpool_redis::redis::RedisError>),

    #[error(transparent)]
    Basic(#[from] Box<dyn std::error::Error>),
}


impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);
        Status::InternalServerError.respond_to(req)
    }
}
