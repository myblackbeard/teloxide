use crate::core::types::{InputMessageContent, InlineKeyboardMarkup, ParseMode};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultVoice {
    pub id: String,
    pub voice_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub voice_duration: Option<i32>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}