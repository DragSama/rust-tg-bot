**Failed to generate code for following methods**- [ ] getWebhookInfo
```json
{
    "name": "getWebhookInfo",
    "href": "https://core.telegram.org/bots/api#getwebhookinfo",
    "returns": [
        "WebhookInfo"
    ],
    "description": [
        "Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty."
    ]
}
```- [ ] getMe
```json
{
    "name": "getMe",
    "href": "https://core.telegram.org/bots/api#getme",
    "returns": [
        "User"
    ],
    "description": [
        "A simple method for testing your bot's auth token. Requires no parameters. Returns basic information about the bot in form of a User object."
    ]
}
```- [ ] logOut
```json
{
    "name": "logOut",
    "href": "https://core.telegram.org/bots/api#logout",
    "returns": [
        "bool"
    ],
    "description": [
        "Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters."
    ]
}
```- [ ] close
```json
{
    "name": "close",
    "href": "https://core.telegram.org/bots/api#close",
    "returns": [
        "bool"
    ],
    "description": [
        "Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters."
    ]
}
```- [ ] getMyCommands
```json
{
    "name": "getMyCommands",
    "href": "https://core.telegram.org/bots/api#getmycommands",
    "returns": [
        "Vec<BotCommand>"
    ],
    "description": [
        "Use this method to get the current list of the bot's commands. Requires no parameters. Returns Vec<BotCommand> on success.",
        "Methods and objects used in the inline mode are described in the Inline mode section."
    ]
}
```