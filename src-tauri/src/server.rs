use std::convert::Infallible;
use std::net::SocketAddr;

use dav_server::{fakels::FakeLs, localfs::LocalFs, DavHandler};
use http_auth_basic::Credentials;
use hyper::{
    header::{HeaderValue, AUTHORIZATION, WWW_AUTHENTICATE},
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::utils::log_green;
use crate::utils::{delay, log};
use crate::{common::Error, settings::get_settings};

use crate::cache::get_sender;
use crate::cache::update_sender;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String,
    company: String,
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            name: "wds".into(),
            company: "cenfun".into(),
        }
    }
}

//===============================================================================

pub async fn restart() -> bool {
    log("restart server ...");
    if let Some(sender) = get_sender() {
        let _ = sender.send(1).await;
        delay(100);
        start_server();
        return true;
    }
    false
}

pub fn start_server() {
    tokio::spawn(async {
        let settings = get_settings();
        //println!("{:?}", settings);

        let addr: SocketAddr = ([0, 0, 0, 0], settings.port).into();

        let make_service =
            make_service_fn(
                move |_conn| async move { Ok::<_, Infallible>(service_fn(request_handle)) },
            );

        log_green(format!("start server listening on {:?}", addr));

        let (tx, mut rx) = mpsc::channel::<i32>(100);
        update_sender(tx);

        let server = Server::bind(&addr).serve(make_service);

        let graceful = server.with_graceful_shutdown(async {
            rx.recv().await;
            log("shutdown server ...");
        });

        //Await the `server` receiving the signal...
        if let Err(e) = graceful.await {
            eprintln!("server error: {}", e);
        }
    });
}

async fn request_handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let headers = req.headers();
    // println!("headers: {:?}", headers);

    let builder = Response::builder();
    // check token
    if let Some(token) = headers.get("token") {
        //decode token
        // let td = decode::<Claims>(
        //     &t,
        //     &DecodingKey::from_secret("secret".as_ref()),
        //     &Validation::default(),
        // );
        log("got token");

        let res = builder
            .status(StatusCode::OK)
            .body(Body::from("got token"))
            .unwrap();

        return Ok(res);
    }

    if let Some(authorization) = headers.get(AUTHORIZATION) {
        let auth_header_value = String::from(authorization.to_str().unwrap());
        return match Credentials::from_header(auth_header_value) {
            Ok(credentials) => {
                // check user pass

                let user_id = credentials.user_id;
                let password = credentials.password;
                log("got credentials");

                let res = builder
                    .status(StatusCode::OK)
                    .body(Body::from("got credentials"))
                    .unwrap();
                Ok(res)
            }
            Err(e) => Ok(res_bad_request(e.to_string())),
        };
    }

    Ok(res_unauthorized())
}

fn res_bad_request(str: impl Into<String>) -> Response<Body> {
    log("bad request");
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(str.into()))
        .unwrap()
}

fn res_unauthorized() -> Response<Body> {
    log("unauthorized");
    Response::builder()
        .header(
            WWW_AUTHENTICATE,
            HeaderValue::from_static("Basic realm=wds"),
        )
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("unauthorized"))
        .unwrap()
}

// async fn response_handle(req: Request<Body>) -> Response<()> {
//     match (req.method(), req.uri().path()) {
//         // (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(full(INDEX))),
//         // (&Method::GET, "/test.html") => client_request_response().await,
//         // (&Method::POST, "/json_api") => api_post_response(req).await,
//         // (&Method::GET, "/json_api") => api_get_response().await,
//         _ => {
//             // Return 404 not found response.
//             Ok(Response::builder()
//                 .status(StatusCode::NOT_FOUND)
//                 .body(())
//                 .unwrap())
//         }
//     }
// }
