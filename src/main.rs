extern crate redis;

use redis::cmd;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    
    let mut con = client.get_connection()?;

    let resp:String = cmd("PING").query(&mut con).expect("Failed to receive response\n");

    println!("{}", resp);
    
    Ok(())
}
