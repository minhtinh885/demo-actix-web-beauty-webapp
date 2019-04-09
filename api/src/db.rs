use actix::prelude::*;
use r2d2_mysql::{MysqlConnectionManager, r2d2::{PooledConnection}};

pub type Pool = r2d2_mysql::r2d2::Pool<MysqlConnectionManager>;
pub type Connection = PooledConnection<MysqlConnectionManager>;
/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool);

// Actors communicate exclusively by exchanging messages.
// The sending actor can optionally wait for the response.
// Actors are not referenced directly, but by means of addresses.
// Any rust type can be an actor, it only needs to implement the Actor trait.
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}