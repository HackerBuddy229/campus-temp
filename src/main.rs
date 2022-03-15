use regex::Regex;

#[tokio::main]
async fn main() {
    let body = reqwest::get("https://temp.campus.ltu.se")
    .await.unwrap()
    .text()
    .await.unwrap();

    //println!("test: {}", body);

    let re = Regex::new("(-)*[0-99]+,[0-99]").unwrap();

    let cap = re.captures(body.as_str());
    match cap {
        Some(x) => println!("{}", x.get(0).unwrap().as_str()),
        _ => ()
    }
    
}
