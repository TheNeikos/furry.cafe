use iron::prelude::*;
use iron::AfterMiddleware;
use iron::headers::ContentType;
use router::NoRoute;

use error::FurratoriaError;

use views;
use views::layout::LayoutData;

pub struct ErrorHandler;

impl AfterMiddleware for ErrorHandler {
    fn catch(&self, req: &mut Request, mut err: IronError) -> IronResult<Response> {
        let data = LayoutData::from_request(req);

        error!("{}", err.error);
        if let Some(e) = err.error.downcast::<FurratoriaError>() {
            match e {
                &FurratoriaError::Unauthorized(_) => {
                    err.response.headers.set(ContentType::html());
                    Ok(err.response.set(views::shared::unauthorized(&data).unwrap()))
                }
                _ => {
                    err.response.headers.set(ContentType::html());
                    Ok(err.response.set(views::shared::internalerror(&data).unwrap()))
                }
            }
        } else if let Some(_) = err.error.downcast::<NoRoute>() {
            err.response.headers.set(ContentType::html());
            Ok(err.response.set(views::shared::notfound(&data).unwrap()))
        } else {
            Ok(err.response)
        }
    }
}
