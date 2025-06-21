pub mod games;

use rouille::{Request, Response};
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> impl Fn(&Request) -> Response + Send + Sync + 'static {
    let games = games::router(pool.clone());

    move |req| {
        let resp = games(req);
        if resp.status_code == 404 { Response::empty_404() } else { resp }
    }
}