mod schedule;

use rocket::routes;
use rocket::Build;
use rocket::Rocket;
use std::net::{IpAddr, Ipv4Addr};

pub fn serve_jarvis_rocket() -> Rocket<Build> {
    let mut config = rocket::Config::default();
    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();
    rocket::custom(config)
        .mount("/schedule", routes![schedule::hello])
        .attach(cors)
}
