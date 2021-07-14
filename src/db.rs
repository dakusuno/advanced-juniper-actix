use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use r2d2::Pool;
use dotenv::dotenv;
use std::env;
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool()->PostgresPool{
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no db url found !");
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("DB Connection Failed !")
}