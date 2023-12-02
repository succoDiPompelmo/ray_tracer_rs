mod camera;
mod canvas;
mod core;
mod margin;
mod materials;
mod rays;
mod scenarios;
mod shapes;

use std::f64::consts::PI;

use actix_cors::Cors;
use actix_web::{error, get, post, web, App, HttpServer, Responder, Result};
use scenarios::Scenario;
use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    core::{transformations::Transformation, tuples::Tuple},
    scenarios::lights::PointLight,
};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(greet)
            .service(list_scenarios)
            .service(render_scenario)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/scenarios")]
async fn list_scenarios() -> Result<impl Responder> {
    let obj = Scenarios {
        values: Scenario::list(),
    };
    Ok(web::Json(obj))
}

#[post("/render/{scenario}")]
async fn render_scenario(
    scenario: web::Path<String>,
    parameters: web::Json<ScenarioParameters>,
) -> Result<impl Responder> {
    if !Scenario::list().contains(&scenario) {
        return Err(error::ErrorBadRequest("err.name"));
    }

    let mut scenario = Scenario::get(&scenario);

    scenario.get_world().set_light(PointLight::new(
        Tuple::white(),
        Tuple::new_point(
            parameters.light_position.x,
            parameters.light_position.y,
            parameters.light_position.z,
        ),
    ));

    let mut camera = Camera::new(1000, 500, PI / 2.0);
    camera.set_transform(Transformation::view_transform(
        Tuple::new_point(
            parameters.camera_position.from.x,
            parameters.camera_position.from.y,
            parameters.camera_position.from.z,
        ),
        Tuple::new_point(
            parameters.camera_position.to.x,
            parameters.camera_position.to.y,
            parameters.camera_position.to.z,
        ),
        Tuple::new_point(
            parameters.camera_position.up.x,
            parameters.camera_position.up.y,
            parameters.camera_position.up.z,
        ),
    ));
    camera.precompute_inverse_transform();

    let canvas = camera.render(scenario.get_world());
    let image = Image {
        base64_image: canvas.base64(),
    };

    Ok(web::Json(image))
}

#[derive(Serialize)]
struct Scenarios {
    values: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ScenarioParameters {
    camera_position: CameraPosition,
    light_position: LightPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LightPosition {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct CameraPosition {
    from: FromPosition,
    to: ToPosition,
    up: UpPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct FromPosition {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct ToPosition {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct UpPosition {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Serialize)]
struct Image {
    base64_image: String,
}
