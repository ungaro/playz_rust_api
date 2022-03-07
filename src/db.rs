
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
/*
extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_rds::{Rds, RdsClient, DescribeDBClustersMessage};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

*/
pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)



 
    
/*
    
    #[test]
    fn should_describe_db_clusters() {
        let _ = env_logger::init();
        let credentials = DefaultCredentialsProvider::new().unwrap();
        let client = RdsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
        let request = DescribeDBClustersMessage::default();
    
        let result = client.describe_db_clusters(&request);
        println!("{:#?}", result);
    }

*/
}

pub fn connect() -> MysqlPool {
    println!("{}",dotenv!("DB_URL"));
    init(dotenv!("DB_URL")).expect("Error")
}



