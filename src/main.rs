extern crate redis;


use redis::cmd;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io::prelude::*;

fn open_pipe(_pipe_path: &str, readable: bool, writable: bool) -> std::io::Result<File> {
    let pipe_path = Path::new(_pipe_path);

    let mut fo = File::options().read(readable).write(writable).open(pipe_path);

    return fo;
}


fn send_recv(redis_cmd: &redis::Cmd) -> redis::RedisResult<String>{
    let packed_cmd = redis_cmd.get_packed_command();

    let mut retcmd = String::new();
    match open_pipe("/tmp/redis-pipe0", false, true) {
        Err(why) => {
            panic!("Failed to open pipe: {}", why);
        }
        Ok(mut pipe0) => {
            pipe0.write_all(&packed_cmd)?;
        }
    };

    match open_pipe("/tmp/redis-pipe1", true, false) {
        Err(why) => {
            panic!("Failed to open pipe: {}", why);
        }
        Ok(mut pipe1) => {
            pipe1.read_to_string(&mut retcmd)?;

            println!("readed response: {}", retcmd);
        }
    };

    Ok(retcmd)
}


fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    let mut con = client.get_connection()?;

    let ping_cmd = cmd("PING");

    // let resp:String = ping_cmd.query(&mut con)?;
    // println!("{}", resp);

    let packed_cmd = ping_cmd.get_packed_command();

    println!("{}", String::from_utf8(packed_cmd.clone()).unwrap());
    
    let _ = send_recv(&ping_cmd);

    Ok(())
}
