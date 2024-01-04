use mysql::*;
use mysql::prelude::*;

pub struct DbConn {
    pool: Pool,
}

impl DbConn {
    pub fn new(addr: &str) -> Result<Self, mysql::Error> {
        let pool = Pool::new(addr)?;
        Ok(DbConn { pool })
    }

    pub fn get_conn(&self) -> Result<PooledConn, mysql::Error> {
        self.pool.get_conn()
    }
}