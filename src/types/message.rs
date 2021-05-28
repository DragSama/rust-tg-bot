use crate::types::{
    Animation, Audio, Chat, Contact, Dice, Document, Game, InlineKeyboardMarkup, Invoice, Location,
    MessageEntity, PassportData, PhotoSize, Poll, ProximityAlertTriggered, Sticker,
    SuccessfulPayment, User, Venue, Video, VideoNote, Voice,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i32,
    /// Optional. Sender, empty for messages sent to channels
    pub from: Option<User>,
    /// Optional. Sender of the message, sent on behalf of a chat. The channel itself for channel messages. The supergroup itself for messages from anonymous group administrators. The linked channel for messages automatically forwarded to the discussion group
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i32,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Optional. For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// Optional. For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    pub forward_from_chat: Option<Chat>,
    /// Optional. For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i32>,
    /// Optional. For messages forwarded from channels, signature of the post author if present
    pub forward_signature: Option<String>,
    /// Optional. Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    pub forward_sender_name: Option<String>,
    /// Optional. For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<i32>,
    /// Optional. For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Optional. Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Optional. Date the message was last edited in Unix time
    pub edit_date: Option<i32>,
    /// Optional. The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Optional. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,
    /// Optional. For text messages, the actual UTF-8 text of the message, 0-4096 characters
    pub text: Option<String>,
    /// Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    pub animation: Option<Animation>,
    /// Optional. Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Optional. Message is a general file, information about the file
    pub document: Option<Document>,
    /// Optional. Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Optional. Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Optional. Message is a video, information about the video
    pub video: Option<Video>,
    /// Optional. Message is a video note, information about the video message
    pub video_note: Option<VideoNote>,
    /// Optional. Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Optional. Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
    pub caption: Option<String>,
    /// Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Optional. Message is a dice with random value
    pub dice: Option<Dice>,
    /// Optional. Message is a game, information about the game. More about games
    pub game: Option<Game>,
    /// Optional. Message is a native poll, information about the poll
    pub poll: Option<Poll>,
    /// Optional. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    pub venue: Option<Venue>,
    /// Optional. Message is a shared location, information about the location
    pub location: Option<Location>,
    /// Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// Optional. A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// Optional. A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// Optional. A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Optional. Service message: the chat photo was deleted
    pub delete_chat_photo: Option<bool>,
    /// Optional. Service message: the group has been created
    pub group_chat_created: Option<bool>,
    /// Optional. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// Optional. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    pub channel_chat_created: Option<bool>,
    /// Optional. The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i32>,
    /// Optional. The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<i32>,
    /// Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// Optional. Message is an invoice for a payment, information about the invoice. More about payments
    pub invoice: Option<Invoice>,
    /// Optional. Message is a service message about a successful payment, information about the payment. More about payments
    pub successful_payment: Option<SuccessfulPayment>,
    /// Optional. The domain name of the website on which the user has logged in. More about Telegram Login
    pub connected_website: Option<String>,
    /// Optional. Telegram Passport data
    pub passport_data: Option<PassportData>,
    /// Optional. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
