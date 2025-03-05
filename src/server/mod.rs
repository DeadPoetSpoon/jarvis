mod schedule;

use minio::s3::client::Client;
use minio::s3::client::ClientBuilder;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

pub fn serve_jarvis_rocket() -> Rocket<Build> {
    let config = rocket::Config {
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        ..Default::default()
    };
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();
    rocket::custom(config)
        .mount("/schedule", routes![schedule::handle])
        .attach(cors)
        .manage(minio())
}

fn minio() -> Client {
    let provider = StaticProvider::new("minioadmin", "minioadmin", None);
    let base_url = BaseUrl::from_str("http://jarvis:9000").unwrap();
    ClientBuilder::new(base_url)
        .provider(Some(Box::new(provider)))
        .build()
        .unwrap()
}
