use actix_files::{self as fs, NamedFile};
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder, cookie::{Cookie, SameSite}};
use spotify_lib::modules::oauth::{client::{OauthClient, Scopes}, config::ClientConfig, ExchangeToken};


// This struct represents state
struct AppState {
  oauth_config: ClientConfig,
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

  println!("Open -> http://localhost:9000");

  HttpServer::new(|| {
    App::new()
      .app_data(web::Data::new(AppState {
        oauth_config: ClientConfig::from_env().unwrap()
      }))
      // .service(index)
      .service(redirect)
      .service(jwt_callback)
      .service(cookie_callback)
      .service(exchange)
      .service(app)
      .service(index)
  })
  .bind(("127.0.0.1", 9000)).unwrap()
  .run().await  
}


#[get("/")]
async fn index() -> Result<fs::NamedFile, Error> {
  Ok(NamedFile::open("static/index.html")?)
}

#[get("/jwt/callback")]
async fn jwt_callback() -> Result<fs::NamedFile, Error>  {
  Ok(NamedFile::open("static/jwt/callback.html")?)
}

#[get("/cookie/callback")]
async fn cookie_callback(info: web::Query<ExchangeToken>, data: web::Data<AppState>) -> impl Responder {
  let client = OauthClient::new(&data.oauth_config);
  match client.exchange_token(&info.code, "cookie").await {
    Ok(tkn) => {
      let cookie = Cookie::build("spty-token", tkn.to_string())
      .domain("localhost")
      .path("/cookie/app")
      .http_only(true)
      .same_site(SameSite::Strict)
      .finish();

      HttpResponse::PermanentRedirect()
      .append_header(("Location", "/cookie/app"))
        .cookie(cookie)
        .finish()
    }, 
    Err(e) => {
      HttpResponse::Unauthorized().body(e.to_string())
    }
  }
}

#[get("/{auth_method}/app")]
async fn app(path: web::Path<(String,)>) -> Result<fs::NamedFile, Error> {
  let (auth_method,) = path.into_inner();
  if auth_method == "jwt" {
    return Ok(NamedFile::open("static/jwt/app.html")?)
  }
  Ok(NamedFile::open("static/cookie/app.html")?)
}

#[get("/oauth/redirect/{auth_method}")]
async fn redirect(path: web::Path<(String,)>, data: web::Data<AppState>) -> impl Responder {
  let (auth_method,) = path.into_inner();
  let client = OauthClient::new(&data.oauth_config);
  let scopes: Vec<Scopes> = vec![
    Scopes::UserReadEmail, 
    Scopes::UserReadPrivate, 
    Scopes::PlaylistReadPrivate,
    Scopes::PlaylistReadCollaborative
  ];
  let state = uuid::Uuid::new_v4().to_string();
  
  let redirect_url: String;

  if auth_method == "jwt" {
    redirect_url = client.get_redirect_url(scopes, &state, "jwt");
  } else {
    redirect_url = client.get_redirect_url(scopes, &state, "cookie");
  }

  web::Redirect::to(redirect_url).permanent()
}

#[post("/oauth/exchange")]
async fn exchange(payload: web::Json<ExchangeToken>, data: web::Data<AppState>) -> impl Responder {
  let client = OauthClient::new(&data.oauth_config);
  match client.exchange_token(&payload.code, "jwt").await {
    Ok(tkn) => {
      HttpResponse::Ok().json(tkn)
    }, 
    Err(e) => {
      HttpResponse::Unauthorized().body(e.to_string())
    }
  }
}