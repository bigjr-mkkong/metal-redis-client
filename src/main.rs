extern crate redis;

use redis::cmd;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    
    let mut con = client.get_connection()?;

    let ping_cmd = cmd("PING");

    let resp:String = ping_cmd.query(&mut con)?;

    println!("{}", resp);

    let packed_cmd = ping_cmd.get_packed_command();

    println!("{}", String::from_utf8(packed_cmd).unwrap());
    
    Ok(())
}
