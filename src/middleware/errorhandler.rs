use iron::prelude::*;
use iron::AfterMiddleware;
use iron::headers::ContentType;
use router::NoRoute;

use error::UnauthorizedError;

use views;
use views::layout::LayoutData;

pub struct ErrorHandler;

impl AfterMiddleware for ErrorHandler {
    fn catch(&self, req: &mut Request, mut err: IronError) -> IronResult<Response> {
        let data = LayoutData::from_request(req);

        if let Some(_) = err.error.downcast::<UnauthorizedError>() {
            err.response.headers.set(ContentType::html());
            Ok(err.response.set(views::shared::unauthorized(&data).unwrap()))
        } else if let Some(_) = err.error.downcast::<NoRoute>() {
            err.response.headers.set(ContentType::html());
            Ok(err.response.set(views::shared::notfound(&data).unwrap()))
        } else {
            Ok(err.response)
        }
    }
}
