use actix_web::{middleware::{Middleware, Started}, HttpResponse, HttpRequest, Result};
use actix_web::middleware::identity::RequestIdentity;
use handler::{Queries, ResponseTypes};
use utils::decode_token;
use actix_web::http::Method;
use futures::future::Future;
use app::AppState;
use actix_web::http::header;

pub struct AuthRoute {
    pub name: &'static str,
    pub method: Method,
    pub roles: Vec<&'static str>,
}
pub struct Auth {
    pub routes: Vec<AuthRoute>,
}
impl Middleware<AppState> for Auth {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        let mut continue_check: bool = true;
        for route in self.routes.iter() {
            if req.resource().name() == route.name && req.method() == route.method {
                if req.headers().contains_key("Authorization") {
                    let auth_key: &header::HeaderValue = req.headers().get("Authorization").unwrap();
                    let token_key = auth_key.to_str().unwrap();
                    match decode_token(token_key) {
                        Ok(slim_user) => {
                            if route.roles.is_empty() {
                                return Ok(Started::Done);
                            } else {
                                req.state().db.send(Queries::ListRoles(slim_user.email))
                                    .and_then(|res| {
                                        match res {
                                            Ok(roles) => {
                                                if let ResponseTypes::Roles(r_roles) = roles {
                                                    for r_role in r_roles.iter() {
                                                        for role in route.roles.iter() {
                                                            if r_role.eq(role.into()) {
                                                                continue_check = true;
                                                                return Ok(Started::Done);
                                                            }
                                                        }
                                                    }
                                                }
                                                continue_check = false;
                                                return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
                                            },
                                            Err(_err) => {
                                                continue_check = false;
                                                return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
                                            },
                                        }
                                    }).wait();
                            }

                        },
                        Err(_) => {
                            continue_check = false;
                            return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
                        }
                    }
                }
                else {
                    if let Some(identity) = req.identity() {
                        match decode_token(&identity) {
                            Ok(slim_user) => {
                                if route.roles.is_empty() {
                                    return Ok(Started::Done);
                                } else {
                                    req.state().db.send(Queries::ListRoles(slim_user.email))
                                        .and_then(|res| {
                                            match res {
                                                Ok(roles) => {
                                                    if let ResponseTypes::Roles(r_roles) = roles {
                                                        for r_role in r_roles.iter() {
                                                            for role in route.roles.iter() {
                                                                if r_role.eq(role.into()) {
                                                                    continue_check = true;
                                                                    return Ok(Started::Done);
                                                                }
                                                            }
                                                        }
                                                    }
                                                    continue_check = false;
                                                    return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
                                                },
                                                Err(_err) => {
                                                    continue_check = false;
                                                    return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
                                                },
                                            }
                                        }).wait();
                                }

                            },
                            Err(_) => {
                                continue_check = false;
                                return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
                            }
                        }
                    } else {
                        continue_check = false;
                        return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
                    }
                }


            }
        }
        if continue_check {
            Ok(Started::Done)
        } else {
            return Ok(Started::Response(HttpResponse::Unauthorized().json("Không có quyền truy cập")));
        }
    }
}


