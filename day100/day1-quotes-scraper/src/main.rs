fn main() {
    //1. Get Your Target Webpage
    // download the target HTML document
    let response = reqwest::blocking::get("https://scrapeme.live/shop/");
    // get the HTML content from the request response
    // and print it
    let html_content = response.unwrap().text().unwrap();
    println!("{html_content}");
}
