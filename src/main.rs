pub mod p_keyboard;
mod handlers;

use teloxide::prelude::*;
use handlers::{start_command_handler, back_button_handler, forward_button_handler};



#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if msg.text().is_none(){
            return Ok(())
        }
        
        match msg.text().unwrap(){
            "/start" => {
                let res = start_command_handler(&bot, &msg).await?;
                return Ok(())
            },
            "Back" => {
                let res = back_button_handler(&bot, &msg).await?;
                return Ok(())
            },
            "Forward" => {
                let res = forward_button_handler(&bot, &msg).await?;
                return Ok(())
            }
            _ => {
                return Ok(())
            }
        };
        // bot.send_dice(msg.chat.id).await?;
        // Ok(())
    })
    .await;
}