extern crate reqwest;
extern crate env_logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut res = reqwest::get("http://httpbin.org/get")?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    std::io::copy(&mut res, &mut std::io::stdout())?;

    Ok(())
}
