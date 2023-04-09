use actix_web::web::ServiceConfig;

mod censor;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(censor::censor);
    }
}
