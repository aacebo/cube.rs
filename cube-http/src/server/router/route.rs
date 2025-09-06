use std::marker::PhantomData;

use cube_url::template::Template;

use crate::{
    Method,
    server::{Request, Response},
};

pub struct Route<ReqBody, ResBody, Handler>
where
    Handler: Fn(&Request<ReqBody>, &mut Response<ResBody>),
{
    __phantom_req_body__: PhantomData<ReqBody>,
    __phantom_res_body__: PhantomData<ResBody>,
    method: Option<Method>,
    path: Template,
    handler: Handler,
}

impl<ReqBody, ResBody, Handler> Route<ReqBody, ResBody, Handler>
where
    Handler: Fn(&Request<ReqBody>, &mut Response<ResBody>),
{
    pub fn new(path: Template, handler: Handler) -> Self {
        return Self {
            __phantom_req_body__: PhantomData::<ReqBody>,
            __phantom_res_body__: PhantomData::<ResBody>,
            method: None,
            path,
            handler,
        };
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = Some(method);
        return self;
    }

    pub fn is_match(&self, req: &Request<ReqBody>) -> bool {
        if let Some(method) = self.method {
            if method != req.method {
                return false;
            }
        }

        return match self.path.eval(&req.url.to_string()) {
            Err(_) => false,
            Ok(_) => true,
        };
    }

    pub fn invoke(&self, req: &Request<ReqBody>, res: &mut Response<ResBody>) {
        (self.handler)(req, res);
    }
}
