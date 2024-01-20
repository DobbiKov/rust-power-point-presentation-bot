use teloxide::types::{ KeyboardButton, KeyboardMarkup };

pub fn get_p_keyboard() -> KeyboardMarkup{
    let back_button = KeyboardButton{
        text: "Back".to_string(),
        request: None
    };
    let forward_button = KeyboardButton{
        text: "Forward".to_string(),
        request: None
    };
    let rows = vec![
        vec![ back_button, forward_button ]
    ];
    
    let markup = KeyboardMarkup{
        keyboard: rows,
        is_persistent: false,
        resize_keyboard: None,
        one_time_keyboard: None,
        input_field_placeholder: None,
        selective: None
    };
    markup
}