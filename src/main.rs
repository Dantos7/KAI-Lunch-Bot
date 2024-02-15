use teloxide::prelude::*;
use regex::Regex;

// #[tokio::main]
// async fn main() {
//     pretty_env_logger::init();
//     log::info!("Starting throw dice bot...");

//     dotenvy::from_filename(".secret.env").unwrap();

//     let bot = Bot::from_env();

//     teloxide::repl(bot, |bot: Bot, msg: Message| async move {
//         bot.send_dice(msg.chat.id).await?;
//         Ok(())
//     }).await;
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get HTML from page containing the menu image
    let resp = reqwest::get("https://gasthof-hopf.com/aktuelles/").await?;
    let text = resp.text().await?;
    // Retrieve links to menu images by finding regex matches
    // (they are contained in wp-content/uploads/ and have a fixed naming scheme)
    let re = Regex::new(r"https://gasthof-hopf\.com/wp-content/uploads/[0-9]{4}/[0-9]{2}/Mittagskarte-neu-[0-9]{4}-[0-9\-]{2,8}scaled.jpg").unwrap();
    for re_match in re.find_iter(&text) {
        println!("{:#?}", re_match.as_str());
    }
    Ok(())
}
