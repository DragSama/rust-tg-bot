**Failed to generate code for following types**
- [ ] InputMedia
```json
{
    "name": "InputMedia",
    "href": "https://core.telegram.org/bots/api#inputmedia",
    "description": [
        "This object represents the content of a media message to be sent. It should be one of"
    ],
    "subtypes": [
        "InputMediaAnimation",
        "InputMediaDocument",
        "InputMediaAudio",
        "InputMediaPhoto",
        "InputMediaVideo"
    ]
}
```
- [ ] InputFile
```json
{
    "name": "InputFile",
    "href": "https://core.telegram.org/bots/api#inputfile",
    "description": [
        "This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.",
        "There are three ways to send files (photos, stickers, audio, media, etc.):",
        "Sending by file_id",
        "Sending by URL",
        "Objects and methods used in the inline mode are described in the Inline mode section."
    ]
}
```
- [ ] InlineQueryResult
```json
{
    "name": "InlineQueryResult",
    "href": "https://core.telegram.org/bots/api#inlinequeryresult",
    "description": [
        "This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:"
    ],
    "subtypes": [
        "InlineQueryResultCachedAudio",
        "InlineQueryResultCachedDocument",
        "InlineQueryResultCachedGif",
        "InlineQueryResultCachedMpeg4Gif",
        "InlineQueryResultCachedPhoto",
        "InlineQueryResultCachedSticker",
        "InlineQueryResultCachedVideo",
        "InlineQueryResultCachedVoice",
        "InlineQueryResultArticle",
        "InlineQueryResultAudio",
        "InlineQueryResultContact",
        "InlineQueryResultGame",
        "InlineQueryResultDocument",
        "InlineQueryResultGif",
        "InlineQueryResultLocation",
        "InlineQueryResultMpeg4Gif",
        "InlineQueryResultPhoto",
        "InlineQueryResultVenue",
        "InlineQueryResultVideo",
        "InlineQueryResultVoice"
    ]
}
```
- [ ] InputMessageContent
```json
{
    "name": "InputMessageContent",
    "href": "https://core.telegram.org/bots/api#inputmessagecontent",
    "description": [
        "This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 4 types:"
    ],
    "subtypes": [
        "InputTextMessageContent",
        "InputLocationMessageContent",
        "InputVenueMessageContent",
        "InputContactMessageContent"
    ]
}
```
- [ ] PassportElementError
```json
{
    "name": "PassportElementError",
    "href": "https://core.telegram.org/bots/api#passportelementerror",
    "description": [
        "This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:"
    ],
    "subtypes": [
        "PassportElementErrorDataField",
        "PassportElementErrorFrontSide",
        "PassportElementErrorReverseSide",
        "PassportElementErrorSelfie",
        "PassportElementErrorFile",
        "PassportElementErrorFiles",
        "PassportElementErrorTranslationFile",
        "PassportElementErrorTranslationFiles",
        "PassportElementErrorUnspecified"
    ]
}
```
- [ ] CallbackGame
```json
{
    "name": "CallbackGame",
    "href": "https://core.telegram.org/bots/api#callbackgame",
    "description": [
        "A placeholder, currently holds no information. Use BotFather to set up your game."
    ]
}
```
