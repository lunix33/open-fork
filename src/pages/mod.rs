use actix_web::web::ServiceConfig;

mod home;

pub fn configure(config: &mut ServiceConfig) {
    config.configure(home::configure);
}
