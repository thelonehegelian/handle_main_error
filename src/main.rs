use error_chain::error_chain;

use std::fs::File;

use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")
        .and_then(|mut f| f.read_to_string(&mut uptime))
        .unwrap();

    Ok(uptime
        .split('.')
        .next()
        .ok_or("Could not read uptime")?
        .parse()?)
}

fn main() -> Result<()> {
    match read_uptime() {
        Ok(uptime) => println!("Uptime: {} seconds", uptime),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
