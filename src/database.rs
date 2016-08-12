use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use r2d2;

use std::env;

lazy_static! {
    static ref CONNECTION: r2d2::Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::new(config, manager).expect("Failed to create pool")
    };
}

pub fn connection() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    CONNECTION.clone()
}
