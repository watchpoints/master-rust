use headless_chrome::{protocol::cdp::types::Event, Browser, LaunchOptions};
// 定义一个对外接口函数
pub fn add_numbers(a: i32, b: i32) -> i32 {
    
    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://scrapeme.live/shop/").unwrap();

    a +b
}


//rustc
fn main() {
    println!("{}", add_numbers(1, 2));

}