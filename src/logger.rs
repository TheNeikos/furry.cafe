
use iron::{AfterMiddleware, BeforeMiddleware};
use iron::{IronResult, Request, Response, IronError};
use iron::method::Method;
use iron::typemap::Key;

use std::fmt::Write;
use std::time::Instant;


struct TimeLog;
impl Key for TimeLog { type Value = Instant; }

pub struct Logger;

impl Logger {
    fn initialise(&self, req: &mut Request) {
        req.extensions.insert::<TimeLog>(Instant::now());
    }

    fn log(&self, req: &mut Request, res: &Response, err: Option<&IronError>) -> IronResult<()> {
        let elapsed = req.extensions.get::<TimeLog>().unwrap().elapsed();
        let elapsed_ms = (elapsed.subsec_nanos() as u64 * 1000 + elapsed.as_secs() / 1000) as f64 / 1_000_000_000.0;

        let mut out = String::new();

        itry!(write!(&mut out, "total_time={:.6} ", elapsed_ms));

        itry!(write!(&mut out, "resp_status="));
        if let Some(status) = res.status {
            itry!(write!(&mut out, "'{}'", status));
        } else {
            itry!(write!(&mut out, "'?'"));
        }
        itry!(write!(&mut out, " "));

        itry!(write!(&mut out, "method='"));
        match &req.method {
            &Method::Extension(_) => itry!(write!(&mut out, "EXTENSION")),
            t => itry!(write!(&mut out, "{}", t)),
        };
        itry!(write!(&mut out, "' "));

        itry!(write!(&mut out, "path='/{}' ", req.url.path().join("/")));

        if let Some(err) = err {
            itry!(write!(&mut out, "error='{}' ", err));
        }

        info!("{}", out);
        Ok(())
    }
}

impl BeforeMiddleware for Logger {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        self.initialise(req);
        Ok(())
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<()> {
        self.initialise(req);
        Err(err)
    }
}

impl AfterMiddleware for Logger {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        try!(self.log(req, &res, None));
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        try!(self.log(req, &err.response, Some(&err)));
        Err(err)
    }
}

