use std::future::Future;

use teloxide::{RequestError, Bot, types::Message, requests::Requester};
use teloxide::prelude::*;
use crate::p_keyboard::get_p_keyboard;
use enigo::*;


pub async fn start_command_handler(bot: &Bot, msg: &Message) -> Result<(), RequestError>{
    let chat_id = msg.chat.id;
    let markup = get_p_keyboard();
    let mut send_mess = bot.send_message(chat_id, "Here are your buttons:");
    send_mess = send_mess.reply_markup(markup);
    send_mess.await?;
    Ok(())
}

pub async fn back_button_handler(bot: &Bot, msg: &Message) -> Result<(), RequestError>{
    let mut enigo = Enigo::new();
    enigo.key_click(Key::LeftArrow);
    log::info!("pressing left arrow key");
    Ok(())
}
pub async fn forward_button_handler(bot: &Bot, msg: &Message) -> Result<(), RequestError>{
    let mut enigo = Enigo::new();
    enigo.key_click(Key::RightArrow);
    log::info!("pressing right arrow key");
    Ok(())
}