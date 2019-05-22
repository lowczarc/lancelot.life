use mysql::Pool;

pub fn print_database_request() -> &'static str{
    concat!("mysql://", env!("MYSQL_USER"), ":", "???", "@localhost:3307/", env!("MYSQL_DATABASE"))
}