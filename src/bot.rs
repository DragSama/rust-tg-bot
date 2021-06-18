use crate::error::Result;
use crate::methods::{
    AddStickerToSet, AnswerCallbackQuery, AnswerInlineQuery, AnswerPreCheckoutQuery,
    AnswerShippingQuery, CopyMessage, CreateChatInviteLink, CreateNewStickerSet, DeleteChatPhoto,
    DeleteChatStickerSet, DeleteMessage, DeleteStickerFromSet, DeleteWebhook, EditChatInviteLink,
    EditMessageCaption, EditMessageLiveLocation, EditMessageMedia, EditMessageReplyMarkup,
    EditMessageText, ExportChatInviteLink, ForwardMessage, GetChat, GetChatAdministrators,
    GetChatMember, GetChatMembersCount, GetFile, GetGameHighScores, GetStickerSet, GetUpdates,
    GetUserProfilePhotos, KickChatMember, LeaveChat, PinChatMessage, PromoteChatMember,
    RestrictChatMember, RevokeChatInviteLink, SendAnimation, SendAudio, SendChatAction,
    SendContact, SendDice, SendDocument, SendGame, SendInvoice, SendLocation, SendMediaGroup,
    SendMessage, SendPhoto, SendPoll, SendSticker, SendVenue, SendVideo, SendVideoNote, SendVoice,
    SetChatAdministratorCustomTitle, SetChatDescription, SetChatPermissions, SetChatPhoto,
    SetChatStickerSet, SetChatTitle, SetGameScore, SetMyCommands, SetPassportDataErrors,
    SetStickerPositionInSet, SetStickerSetThumb, SetWebhook, StopMessageLiveLocation, StopPoll,
    UnbanChatMember, UnpinAllChatMessages, UnpinChatMessage, UploadStickerFile,
};
use crate::types::*;

use reqwest;

#[derive(Clone)]
pub struct Bot {
    pub token: String,
    base_endpoint: String,
    reqwest_client: reqwest::Client,
}

impl Bot {
    pub fn new(bot_token: &str) -> Self {
        Self {
            token: bot_token.into(),
            base_endpoint: format!("https://api.telegram.org/bot{}/", bot_token).into(),
            reqwest_client: reqwest::Client::new(),
        }
    }
    pub(crate) async fn send(
        &self,
        method: &str,
        args: Option<String>,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_endpoint, method);
        let response = if args.is_some() {
            let form: serde_json::Value = serde_json::from_str(&args.unwrap())?;
            self.reqwest_client.post(url).form(&form).send().await?
        } else {
            self.reqwest_client.post(url).send().await?
        };
        Ok(response)
    }

    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates::new(&self)
    }
    pub fn set_webhook(&self, url: String) -> SetWebhook {
        SetWebhook::new(&self, url)
    }
    pub fn delete_webhook(&self) -> DeleteWebhook {
        DeleteWebhook::new(&self)
    }
    pub fn send_message(&self, chat_id: i64, text: String) -> SendMessage {
        SendMessage::new(&self, chat_id, text)
    }
    pub fn forward_message(
        &self,
        chat_id: i64,
        from_chat_id: i64,
        message_id: i32,
    ) -> ForwardMessage {
        ForwardMessage::new(&self, chat_id, from_chat_id, message_id)
    }
    pub fn copy_message(&self, chat_id: i64, from_chat_id: i64, message_id: i32) -> CopyMessage {
        CopyMessage::new(&self, chat_id, from_chat_id, message_id)
    }
    pub fn send_photo(&self, chat_id: i64, photo: InputFile) -> SendPhoto {
        SendPhoto::new(&self, chat_id, photo)
    }
    pub fn send_audio(&self, chat_id: i64, audio: InputFile) -> SendAudio {
        SendAudio::new(&self, chat_id, audio)
    }
    pub fn send_document(&self, chat_id: i64, document: InputFile) -> SendDocument {
        SendDocument::new(&self, chat_id, document)
    }
    pub fn send_video(&self, chat_id: i64, video: InputFile) -> SendVideo {
        SendVideo::new(&self, chat_id, video)
    }
    pub fn send_animation(&self, chat_id: i64, animation: InputFile) -> SendAnimation {
        SendAnimation::new(&self, chat_id, animation)
    }
    pub fn send_voice(&self, chat_id: i64, voice: InputFile) -> SendVoice {
        SendVoice::new(&self, chat_id, voice)
    }
    pub fn send_video_note(&self, chat_id: i64, video_note: InputFile) -> SendVideoNote {
        SendVideoNote::new(&self, chat_id, video_note)
    }
    pub fn send_media_group(&self, chat_id: i64, media: Vec<InputMediaAudio>) -> SendMediaGroup {
        SendMediaGroup::new(&self, chat_id, media)
    }
    pub fn send_location(&self, chat_id: i64, latitude: f64, longitude: f64) -> SendLocation {
        SendLocation::new(&self, chat_id, latitude, longitude)
    }
    pub fn edit_message_live_location(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> EditMessageLiveLocation {
        EditMessageLiveLocation::new(&self, latitude, longitude)
    }
    pub fn stop_message_live_location(&self) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new(&self)
    }
    pub fn send_venue(
        &self,
        chat_id: i64,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> SendVenue {
        SendVenue::new(&self, chat_id, latitude, longitude, title, address)
    }
    pub fn send_contact(
        &self,
        chat_id: i64,
        phone_number: String,
        first_name: String,
    ) -> SendContact {
        SendContact::new(&self, chat_id, phone_number, first_name)
    }
    pub fn send_poll(&self, chat_id: i64, question: String, options: Vec<String>) -> SendPoll {
        SendPoll::new(&self, chat_id, question, options)
    }
    pub fn send_dice(&self, chat_id: i64) -> SendDice {
        SendDice::new(&self, chat_id)
    }
    pub fn send_chat_action(&self, chat_id: i64, action: String) -> SendChatAction {
        SendChatAction::new(&self, chat_id, action)
    }
    pub fn get_user_profile_photos(&self, user_id: i32) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(&self, user_id)
    }
    pub fn get_file(&self, file_id: String) -> GetFile {
        GetFile::new(&self, file_id)
    }
    pub fn kick_chat_member(&self, chat_id: i64, user_id: i32) -> KickChatMember {
        KickChatMember::new(&self, chat_id, user_id)
    }
    pub fn unban_chat_member(&self, chat_id: i64, user_id: i32) -> UnbanChatMember {
        UnbanChatMember::new(&self, chat_id, user_id)
    }
    pub fn restrict_chat_member(
        &self,
        chat_id: i64,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> RestrictChatMember {
        RestrictChatMember::new(&self, chat_id, user_id, permissions)
    }
    pub fn promote_chat_member(&self, chat_id: i64, user_id: i32) -> PromoteChatMember {
        PromoteChatMember::new(&self, chat_id, user_id)
    }
    pub fn set_chat_administrator_custom_title(
        &self,
        chat_id: i64,
        user_id: i32,
        custom_title: String,
    ) -> SetChatAdministratorCustomTitle {
        SetChatAdministratorCustomTitle::new(&self, chat_id, user_id, custom_title)
    }
    pub fn set_chat_permissions(
        &self,
        chat_id: i64,
        permissions: ChatPermissions,
    ) -> SetChatPermissions {
        SetChatPermissions::new(&self, chat_id, permissions)
    }
    pub fn export_chat_invite_link(&self, chat_id: i64) -> ExportChatInviteLink {
        ExportChatInviteLink::new(&self, chat_id)
    }
    pub fn create_chat_invite_link(&self, chat_id: i64) -> CreateChatInviteLink {
        CreateChatInviteLink::new(&self, chat_id)
    }
    pub fn edit_chat_invite_link(&self, chat_id: i64, invite_link: String) -> EditChatInviteLink {
        EditChatInviteLink::new(&self, chat_id, invite_link)
    }
    pub fn revoke_chat_invite_link(
        &self,
        chat_id: i64,
        invite_link: String,
    ) -> RevokeChatInviteLink {
        RevokeChatInviteLink::new(&self, chat_id, invite_link)
    }
    pub fn set_chat_photo(&self, chat_id: i64, photo: InputFile) -> SetChatPhoto {
        SetChatPhoto::new(&self, chat_id, photo)
    }
    pub fn delete_chat_photo(&self, chat_id: i64) -> DeleteChatPhoto {
        DeleteChatPhoto::new(&self, chat_id)
    }
    pub fn set_chat_title(&self, chat_id: i64, title: String) -> SetChatTitle {
        SetChatTitle::new(&self, chat_id, title)
    }
    pub fn set_chat_description(&self, chat_id: i64) -> SetChatDescription {
        SetChatDescription::new(&self, chat_id)
    }
    pub fn pin_chat_message(&self, chat_id: i64, message_id: i32) -> PinChatMessage {
        PinChatMessage::new(&self, chat_id, message_id)
    }
    pub fn unpin_chat_message(&self, chat_id: i64) -> UnpinChatMessage {
        UnpinChatMessage::new(&self, chat_id)
    }
    pub fn unpin_all_chat_messages(&self, chat_id: i64) -> UnpinAllChatMessages {
        UnpinAllChatMessages::new(&self, chat_id)
    }
    pub fn leave_chat(&self, chat_id: i64) -> LeaveChat {
        LeaveChat::new(&self, chat_id)
    }
    pub fn get_chat(&self, chat_id: i64) -> GetChat {
        GetChat::new(&self, chat_id)
    }
    pub fn get_chat_administrators(&self, chat_id: i64) -> GetChatAdministrators {
        GetChatAdministrators::new(&self, chat_id)
    }
    pub fn get_chat_members_count(&self, chat_id: i64) -> GetChatMembersCount {
        GetChatMembersCount::new(&self, chat_id)
    }
    pub fn get_chat_member(&self, chat_id: i64, user_id: i32) -> GetChatMember {
        GetChatMember::new(&self, chat_id, user_id)
    }
    pub fn set_chat_sticker_set(
        &self,
        chat_id: i64,
        sticker_set_name: String,
    ) -> SetChatStickerSet {
        SetChatStickerSet::new(&self, chat_id, sticker_set_name)
    }
    pub fn delete_chat_sticker_set(&self, chat_id: i64) -> DeleteChatStickerSet {
        DeleteChatStickerSet::new(&self, chat_id)
    }
    pub fn answer_callback_query(&self, callback_query_id: String) -> AnswerCallbackQuery {
        AnswerCallbackQuery::new(&self, callback_query_id)
    }
    pub fn set_my_commands(&self, commands: Vec<BotCommand>) -> SetMyCommands {
        SetMyCommands::new(&self, commands)
    }
    pub fn edit_message_text(&self, text: String) -> EditMessageText {
        EditMessageText::new(&self, text)
    }
    pub fn edit_message_caption(&self) -> EditMessageCaption {
        EditMessageCaption::new(&self)
    }
    pub fn edit_message_media(&self, media: InputMedia) -> EditMessageMedia {
        EditMessageMedia::new(&self, media)
    }
    pub fn edit_message_reply_markup(&self) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::new(&self)
    }
    pub fn stop_poll(&self, chat_id: i64, message_id: i32) -> StopPoll {
        StopPoll::new(&self, chat_id, message_id)
    }
    pub fn delete_message(&self, chat_id: i64, message_id: i32) -> DeleteMessage {
        DeleteMessage::new(&self, chat_id, message_id)
    }
    pub fn send_sticker(&self, chat_id: i64, sticker: InputFile) -> SendSticker {
        SendSticker::new(&self, chat_id, sticker)
    }
    pub fn get_sticker_set(&self, name: String) -> GetStickerSet {
        GetStickerSet::new(&self, name)
    }
    pub fn upload_sticker_file(&self, user_id: i32, png_sticker: InputFile) -> UploadStickerFile {
        UploadStickerFile::new(&self, user_id, png_sticker)
    }
    pub fn create_new_sticker_set(
        &self,
        user_id: i32,
        name: String,
        title: String,
        emojis: String,
    ) -> CreateNewStickerSet {
        CreateNewStickerSet::new(&self, user_id, name, title, emojis)
    }
    pub fn add_sticker_to_set(
        &self,
        user_id: i32,
        name: String,
        emojis: String,
    ) -> AddStickerToSet {
        AddStickerToSet::new(&self, user_id, name, emojis)
    }
    pub fn set_sticker_position_in_set(
        &self,
        sticker: String,
        position: i32,
    ) -> SetStickerPositionInSet {
        SetStickerPositionInSet::new(&self, sticker, position)
    }
    pub fn delete_sticker_from_set(&self, sticker: String) -> DeleteStickerFromSet {
        DeleteStickerFromSet::new(&self, sticker)
    }
    pub fn set_sticker_set_thumb(&self, name: String, user_id: i32) -> SetStickerSetThumb {
        SetStickerSetThumb::new(&self, name, user_id)
    }
    pub fn answer_inline_query(
        &self,
        inline_query_id: String,
        results: Vec<InlineQueryResult>,
    ) -> AnswerInlineQuery {
        AnswerInlineQuery::new(&self, inline_query_id, results)
    }
    pub fn send_invoice(
        &self,
        chat_id: i64,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> SendInvoice {
        SendInvoice::new(
            &self,
            chat_id,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
        )
    }
    pub fn answer_shipping_query(
        &self,
        shipping_query_id: String,
        ok: bool,
    ) -> AnswerShippingQuery {
        AnswerShippingQuery::new(&self, shipping_query_id, ok)
    }
    pub fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: String,
        ok: bool,
    ) -> AnswerPreCheckoutQuery {
        AnswerPreCheckoutQuery::new(&self, pre_checkout_query_id, ok)
    }
    pub fn set_passport_data_errors(
        &self,
        user_id: i32,
        errors: Vec<PassportElementError>,
    ) -> SetPassportDataErrors {
        SetPassportDataErrors::new(&self, user_id, errors)
    }
    pub fn send_game(&self, chat_id: i64, game_short_name: String) -> SendGame {
        SendGame::new(&self, chat_id, game_short_name)
    }
    pub fn set_game_score(&self, user_id: i32, score: i32) -> SetGameScore {
        SetGameScore::new(&self, user_id, score)
    }
    pub fn get_game_high_scores(&self, user_id: i32) -> GetGameHighScores {
        GetGameHighScores::new(&self, user_id)
    }
}
