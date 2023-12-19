use error_chain::error_chain;
use reqwest;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/get").await?;

    println!("{:?}", res.status());
    println!("{:?}", res);
    Ok(())
}
