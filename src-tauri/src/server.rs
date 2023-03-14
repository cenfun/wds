use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, convert::Infallible};

use cookie::Cookie;
use dav_server::fs::DavFileSystem;
use dav_server::{
    body::Body as DBody, davpath::DavPath, fakels::FakeLs, localfs::LocalFs, memfs::MemFs,
    DavHandler,
};
use http_auth_basic::Credentials;
use hyper::server::conn::AddrStream;
use hyper::{
    header::{HeaderValue, AUTHORIZATION, COOKIE, LOCATION, SET_COOKIE, WWW_AUTHENTICATE},
    service::{make_service_fn, service_fn},
    Body as HBody, HeaderMap, Request, Response, Server, StatusCode,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use urlencoding::{decode as url_decode, encode as url_encode};

use crate::settings::get_settings;
use crate::utils::{delay, log, log_green, log_magenta, log_red};

use crate::cache::get_secret_key;
use crate::cache::get_sender;
use crate::cache::remove_cache;
use crate::cache::update_sender;

use crate::settings::get_profile_by_id;
use crate::settings::get_profile_by_login;

use crate::settings::ProfileItem;

static TOKEN_KEY: &str = "wds-token";
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: u64,
}

impl Claims {
    pub fn new(id: String) -> Self {
        // exp Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
        let now = SystemTime::now();
        let start = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // 12 hours
        let dur = 12 * 60 * 60;
        let exp = start + dur;

        //println!("exp {}", exp);

        Self { id, exp }
    }
}

//===============================================================================

pub async fn restart() -> bool {
    log_magenta("restart server ...");
    if let Some(sender) = get_sender() {
        let _ = sender.send(1).await;
        delay(100);
        start_server();
        return true;
    }
    false
}

pub fn start_server() {
    remove_cache();
    tokio::spawn(async {
        let settings = get_settings();
        //println!("{:?}", settings);

        let addr: SocketAddr = ([0, 0, 0, 0], settings.port).into();

        let make_service = make_service_fn(move |conn: &AddrStream| {
            let remote_addr = conn.remote_addr();
            async move { Ok::<_, Infallible>(service_fn(move |req| request_handle(req, remote_addr))) }
        });

        log_green(format!("start server listening on {:?}", addr));

        let (tx, mut rx) = mpsc::channel::<i32>(100);
        update_sender(tx);

        let server = Server::bind(&addr).serve(make_service);

        let graceful = server.with_graceful_shutdown(async {
            rx.recv().await;
            log_red("shutdown server ...");
        });

        //Await the `server` receiving the signal...
        if let Err(e) = graceful.await {
            eprintln!("server error: {}", e);
        }
    });
}

//===============================================================================

async fn request_handle(
    req: Request<HBody>,
    addr: SocketAddr,
) -> Result<Response<DBody>, Infallible> {
    let profile = match check_token(&req, &addr) {
        Ok(p) => p,
        Err(res) => return Ok(res),
    };

    let user = get_current_user(&profile);
    let method = req.method().to_string();
    let path = req.uri().path();
    log(format!(
        "[{}] {} {} {}",
        addr,
        user,
        method,
        url_decode(&path).unwrap()
    ));

    let builder = DavHandler::builder();
    let dav_config = match path {
        "/" => builder.filesystem(create_root_fs(&profile).await),
        _ => match check_permission(&req, &addr, &profile) {
            Ok((base, prefix)) => {
                //println!("base: {} prefix: {}", base, prefix);
                builder
                    .filesystem(LocalFs::new(base, false, false, false))
                    .strip_prefix(prefix)
            }
            Err(res) => return Ok(res),
        },
    };

    let dav_server = dav_config.locksystem(FakeLs::new()).build_handler();

    let res = dav_server.handle(req).await;
    Ok(res)
}

//===============================================================================

fn check_permission(
    req: &Request<HBody>,
    addr: &SocketAddr,
    profile: &ProfileItem,
) -> Result<(String, String), Response<DBody>> {
    let path = req.uri().path();
    let list: Vec<&str> = path.split("/").collect();

    // println!("path {:?} list {:?}", path, list);

    //always starts with /
    let first_name = list[1];

    //Virtual root name
    let root_name = match url_decode(first_name) {
        Ok(p) => p.to_string(),
        Err(e) => return Err(res_bad_request(e.to_string())),
    };

    let dir = profile.dir_list.iter().find(|item| item.name == root_name);
    if let Some(dir) = dir {
        let base = dir.path.clone();
        let prefix = format!("/{}", &root_name);
        // println!("list {:?} prefix {}", list, prefix);

        // check method
        let method = req.method().to_string();
        let permission = dir.permission.clone();
        if let Some(res) = check_method_permission(&method, &permission) {
            log_red(format!("[{}] no permission: {} {}", addr, method, prefix));
            return Err(res);
        }

        return Ok((base, prefix));
    }

    // if from root then no permission
    log_red(format!("[{}] not found: {}", addr, path));
    Err(res_not_found(path))
}

fn check_method_permission(
    method: &String,
    permission: impl Into<String>,
) -> Option<Response<DBody>> {
    if permission.into() == "read" {
        // println!("method {:?} permission {}", method, permission);
        let readonly = ["get", "head", "options", "propfind"];
        let m = method.to_lowercase();
        if let None = readonly.into_iter().find(|it| it.to_string() == m) {
            return Some(res_forbidden());
        }
    }
    None
}

fn check_token(req: &Request<HBody>, addr: &SocketAddr) -> Result<ProfileItem, Response<DBody>> {
    let headers = req.headers();
    // println!("headers: {:?}", headers);
    let cookies = get_cookies(headers);
    if let Some(token) = cookies.get(TOKEN_KEY) {
        //decode token
        let key = DecodingKey::from_secret(get_secret_key().as_ref());
        let td = decode::<Claims>(token, &key, &Validation::default());
        //println!("token {} {:?}", token, td);

        match td {
            Ok(d) => {
                if let Some(profile) = get_profile_by_id(d.claims.id) {
                    return Ok(profile);
                }
            }
            Err(e) => {
                // when token failed, continue check authorization, do not return
                log_red(format!("[{}] {}", addr, e.to_string()));
            }
        }
    }
    return Err(check_login(req, addr));
}

fn check_login(req: &Request<HBody>, addr: &SocketAddr) -> Response<DBody> {
    let headers = req.headers();
    if let Some(authorization) = headers.get(AUTHORIZATION) {
        let auth_header_value = String::from(authorization.to_str().unwrap());
        if let Ok(credentials) = Credentials::from_header(auth_header_value) {
            // check user pass
            let username = credentials.user_id;
            let password = credentials.password;
            // log(format!("got credentials {}/{}", username, password));

            if let Some(profile) = get_profile_by_login(username, password) {
                // println!("profile {:?}", profile);
                let my_claims = Claims::new(profile.id);
                let key = EncodingKey::from_secret(get_secret_key().as_ref());
                let token = encode(&Header::default(), &my_claims, &key);
                if let Ok(t) = token {
                    //println!("token {:?}", t);
                    log_green(format!("[{}] login successful", addr));
                    let location = req.uri().path();
                    return res_token(t, location);
                }
            }
        }
    }

    log_red(format!("[{}] login failed", addr));
    res_unauthorized()
}

//===============================================================================

async fn create_root_fs(profile: &ProfileItem) -> Box<MemFs> {
    let fs = MemFs::new();
    for item in profile.dir_list.iter() {
        let src = format!("/{}", url_encode(&item.name));
        // println!("src {:?}", src);
        match DavPath::new(&src) {
            Ok(dp) => {
                let _ = fs.create_dir(&dp).await;
            }
            Err(e) => println!("{}", e),
        }
    }
    fs
}

//===============================================================================

fn res_token(token: String, location: &str) -> Response<DBody> {
    // log("moved permanently");
    let cookie = format!("{}={}; HttpOnly", TOKEN_KEY, token);
    Response::builder()
        .header(LOCATION, HeaderValue::from_str(location).unwrap())
        .header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
        .status(StatusCode::MOVED_PERMANENTLY)
        .body(DBody::empty())
        .unwrap()
}

fn res_not_found(str: impl Into<String>) -> Response<DBody> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(DBody::from(str.into()))
        .unwrap()
}

fn res_bad_request(str: impl Into<String>) -> Response<DBody> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(DBody::from(str.into()))
        .unwrap()
}

fn res_unauthorized() -> Response<DBody> {
    // log("unauthorized");
    Response::builder()
        .header(
            WWW_AUTHENTICATE,
            HeaderValue::from_static("Basic realm=wds"),
        )
        .status(StatusCode::UNAUTHORIZED)
        .body(DBody::from("unauthorized"))
        .unwrap()
}

fn res_forbidden() -> Response<DBody> {
    // log("forbidden");
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(DBody::from("forbidden"))
        .unwrap()
}

//===============================================================================

fn get_current_user(profile: &ProfileItem) -> String {
    let username = profile.username.clone();
    if username.is_empty() {
        return "anonymous".into();
    }
    username
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
