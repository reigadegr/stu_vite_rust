use anyhow::{Error, Result, anyhow};
use redis::{Client, Commands, RedisResult};
use tokio::sync::OnceCell;
const REDIS_URI: &str = "redis://127.0.0.1:6379/";
pub static REDIS_DB: OnceCell<Client> = OnceCell::const_new();
pub async fn init_redis_conn() {
    REDIS_DB
        .get_or_init(|| async {
            let client = Client::open(REDIS_URI).expect("redis连接失败");
            client.get_connection().expect("redis连接失败");
            client
        })
        .await;
}

#[inline]
fn get_db_con() -> Client {
    REDIS_DB
        .get()
        .ok_or_else(|| anyhow!("DB not found"))
        .unwrap()
        .clone()
}

#[inline]
pub fn redis_write_and_rm<T: redis::ToRedisArgs>(
    key: &str,
    value: T,
    time: i64,
) -> Result<(), Error> {
    // connect to redis
    println!("开始连接redis_write_and_rm");
    let mut con = get_db_con();
    println!("成功连接");
    // throw away the result, just make sure it does not fail
    let lock_key = format!("{key}:lock");
    let acquire_lock: RedisResult<()> = con.set_ex(&lock_key, "", 20);
    // Check if the lock was acquired
    if acquire_lock.is_err() {
        println!("不能获取锁redis_write_and_rm");
        return Err(Error::msg("Failed to acquire lock"));
    }
    // let mut con = get_db_con();
    let _: () = con.set(key, value)?;
    let _: () = con.expire(key, time)?;
    // Release the lock
    let _: () = con.del(lock_key)?;
    Ok(())
}

#[inline]
pub fn redis_read(key: &str) -> Result<String> {
    // connect to redis
    println!("开始连接redis_read");
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = get_db_con();
    println!("成功连接");
    // throw away the result, just make sure it does not fail
    let lock_key = format!("{key}:lock");
    let acquire_lock: RedisResult<()> = con.set_ex(&lock_key, "", 20);
    // Check if the lock was acquired
    if acquire_lock.is_err() {
        println!("不能获取锁redis_read");
        return Err(Error::msg("Failed to acquire lock"));
    }
    let rs = con.get(key)?;
    let _: () = con.del(lock_key)?;
    Ok(rs)
}
#[allow(dead_code)]
pub fn redis_delete(key: &str) -> Result<()> {
    // connect to redis
    let mut con = get_db_con();
    // delete the key
    let key = String::from(key);
    let _: () = con.del(key)?;

    Ok(())
}

#[allow(dead_code)]
pub fn redis_write<T: redis::ToRedisArgs>(key: &str, value: T) -> Result<()> {
    // Connect to Redis
    let mut con = get_db_con();
    // Throw away the result, just make sure it does not fail
    let _: () = con.set(key, value)?;
    Ok(())
}

#[inline]
#[allow(dead_code)]
pub fn redis_read_vec(key: &str) -> Result<Vec<u8>> {
    // connect to redis
    println!("开始连接redis_read");
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = get_db_con();
    println!("成功连接");
    // throw away the result, just make sure it does not fail
    let lock_key = format!("{key}:lock");
    let acquire_lock: RedisResult<()> = con.set_ex(&lock_key, "", 20);
    // Check if the lock was acquired
    if acquire_lock.is_err() {
        println!("不能获取锁redis_read");
        return Err(Error::msg("Failed to acquire lock"));
    }
    let rs = con.get(key)?;
    let _: () = con.del(lock_key)?;
    Ok(rs)
}
