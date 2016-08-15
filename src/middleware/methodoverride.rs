use std::str::FromStr;

use iron::prelude::*;
use iron::BeforeMiddleware;
use iron::method::Method;

pub struct MethodOverride;

impl BeforeMiddleware for MethodOverride {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        use params::{Params, Value};

        let method = match req.get_ref::<Params>().unwrap().get("_method") {
            Some(&Value::String(ref method)) => Some(method),
            _ => None
        }.map(|method| Method::from_str(method));

        if let Some(Ok(method)) = method {
            match method {
                Method::Extension(_) => (),
                _ => req.method = method,
            }
        }

        Ok(())
    }
}
