use std::net::SocketAddr;
use std::{collections::HashMap, convert::Infallible};

use cookie::Cookie;
use dav_server::{fakels::FakeLs, localfs::LocalFs, DavHandler};
use http_auth_basic::Credentials;
use hyper::{
    header::{HeaderValue, AUTHORIZATION, COOKIE, LOCATION, SET_COOKIE, WWW_AUTHENTICATE},
    service::{make_service_fn, service_fn},
    Body, HeaderMap, Request, Response, Server, StatusCode,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tokio::sync::mpsc;

use crate::settings::get_settings;
use crate::utils::log_green;
use crate::utils::{delay, log};

use crate::cache::get_sender;
use crate::cache::update_sender;

use crate::settings::get_profile_by_id;
use crate::settings::get_profile_by_login;

static SECRET_KEY: &str = "cenfun-wds";
static TOKEN_KEY: &str = "wds-token";
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
}

impl Claims {
    pub fn new(id: String) -> Self {
        let iat = OffsetDateTime::now_utc();
        let exp = iat + Duration::days(1);

        Self {
            id,
            // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
            exp,
        }
    }
}

mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
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
    let cookies = get_cookies(headers);

    let builder = Response::builder();
    // check token
    if let Some(token) = cookies.get(TOKEN_KEY) {
        //decode token
        let key = DecodingKey::from_secret(SECRET_KEY.as_ref());
        let td = decode::<Claims>(token, &key, &Validation::default());

        //println!("token {} {:?}", token, td);

        if let Ok(d) = td {
            let id = d.claims.id;
            println!("token id {:?}", id);

            let profile = get_profile_by_id(id);
            println!("profile {:?}", profile);

            let res = builder
                .status(StatusCode::OK)
                .body(Body::from("check token success"))
                .unwrap();
            return Ok(res);
        }

        // when token failed, continue check authorization, do not return
    }

    if let Some(authorization) = headers.get(AUTHORIZATION) {
        let auth_header_value = String::from(authorization.to_str().unwrap());
        return match Credentials::from_header(auth_header_value) {
            Ok(credentials) => {
                // check user pass
                let username = credentials.user_id;
                let password = credentials.password;
                //log(format!("got credentials {}/{}", username, password));

                if let Some(profile) = get_profile_by_login(username, password) {
                    // println!("profile {:?}", profile);
                    let my_claims = Claims::new(profile.id);
                    let key = EncodingKey::from_secret(SECRET_KEY.as_ref());
                    let token = encode(&Header::default(), &my_claims, &key);
                    if let Ok(t) = token {
                        //println!("token {:?}", t);
                        return Ok(res_token(t, req));
                    }
                }

                Ok(res_unauthorized())
            }
            Err(e) => Ok(res_bad_request(e.to_string())),
        };
    }

    Ok(res_unauthorized())
}

fn get_cookies(headers: &HeaderMap) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    if let Some(hv) = headers.get(COOKIE) {
        let s = hv.to_str().unwrap();
        Cookie::split_parse(s).for_each(|item| {
            if let Ok(c) = item {
                cookies.insert(c.name().to_string(), c.value().to_string());
            }
        })
    }
    cookies
}

fn res_token(token: String, req: Request<Body>) -> Response<Body> {
    log("moved permanently");

    let uri = req.uri().to_string();
    let cookie = format!("{}={}; HttpOnly", TOKEN_KEY, token);

    Response::builder()
        .header(LOCATION, HeaderValue::from_str(&uri).unwrap())
        .header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
        .status(StatusCode::MOVED_PERMANENTLY)
        .body(Body::empty())
        .unwrap()
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
