use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;

use rocket_sync_db_pools::{diesel, database};

#[database("diesel")]
pub struct Db(pub diesel::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("./migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgresql Stage", |rocket| async {
	rocket.attach(Db::fairing())
	    .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
