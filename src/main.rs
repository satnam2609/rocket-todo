use rocket_db_pools::Database;

mod auth;
mod model;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                //TODO ROUTES
                rocket_routes::todo::get_todo,
                rocket_routes::todo::create_todos,
                rocket_routes::todo::get_todos,
                rocket_routes::todo::update_todo,
                rocket_routes::todo::delete_todo,
                //USER ROUTES
                rocket_routes::auth::register,
                rocket_routes::auth::login,
            ],
        )
        .attach(rocket_routes::DbConn::init())
        .attach(rocket_routes::CacheConn::init())
        .launch()
        .await;
}
