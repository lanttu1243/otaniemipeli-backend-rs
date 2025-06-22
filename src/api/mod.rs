pub mod games;
mod drinks;

use rouille::{Request, Response};
use deadpool_postgres::Pool;

pub fn router(pool: Pool) -> impl Fn(&Request) -> Response + Send + Sync + 'static {
    
    let games = games::router(pool.clone());
    let drinks = drinks::router(pool.clone());
    
    move |req| {
        let resp = games(req);
        if resp.status_code != 404 {return resp}
        let resp = drinks(req);
        if resp.status_code != 404 {return resp}
        
        Response::empty_404()
    }
}