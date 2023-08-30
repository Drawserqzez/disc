use diesel::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager, PoolError};


pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    let database_url = dotenvy::var("DATABASE_URL").expect("No db url D:");

    init_pool(&database_url).expect("Failed to create pool ):")
}

fn init_pool(db_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}

