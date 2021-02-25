#[derive(Debug, Deserialize)]
pub struct ReplyKeyboardRemove{
    pub remove_keyboard: bool,
    pub selective: Option<bool>
}