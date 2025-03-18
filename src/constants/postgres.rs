use dotenvy::dotenv;

pub async fn load_env() {
    dotenv().ok();
}

pub async fn get_postgres_url() -> String {
    std::env::var("DATABASE_URL").expect("POSTGRES_URL must be set")
}
