use reqwest::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://www.google.co.jp/search?q={}", "person");
    println!("url = {}", url);
    if let Ok(res) = reqwest::get(&url).await {
        println!("{:?}", res);
        println!("=========");
        match res.status() {
            StatusCode::OK => {
                let body = res.text().await?;
                println!("{}", body);
            },
            _ => {
                println!("Cannot find the page");
            },
        }
    } else {
        println!("Cannot access the site");
    }
    Ok(())
}

