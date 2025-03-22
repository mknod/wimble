# Wimble

## Wimble is an early-stage experimental project that lets Twitch chat control a web browser in real-time. Think of it as a chaotic remote control for the internet, powered by your stream community.

⚙️ Getting Started

    git clone https://github.com/mknod/wimble.git
    cd wimble
    cargo build --release

Wimble uses chromedriver under the hood, which must be running on port 9515 before you start.

💡 Chromedriver Examples
Start chromedriver with:

macOS/Linux:

    ./chromedriver --port=9515

Windows:
    chromedriver.exe --port=9515
🧪 Example config.toml


```
[global]
placeholder = true

[streambot]
enabled = true
channel = "twitch_user_name"
command_symbol = "!"
client_id = "your-client-id"
access_token = "your-access-token"
refresh_token = "your-refresh-token"
username = "your-twitch-username"

[browser]
enabled = true
start_url = "https://twitch.tv"
start_cmd = "space"

[browser.goto]
youtube = "https://youtube.com"
google = "https://google.com"

[browser.elements]
youtube_url = { element = "//*[@id='movie_player']/div[3]/div[2]/div/a", attribute = "href" }
```

🎮 Chat Commands

Your Twitch chat can send commands with the prefix (e.g. !) to control the browser:

!up, !down, !left, !right, !space, !enter, !esc, !delete
!<char> sends a single character (e.g. !a)
!get_url prints the current browser URL back into the chat
!<key> from [browser.goto] will navigate the browser to the associated URL

Example:
```
[browser.goto]
youtube = "https://youtube.com"
google = "https://google.com"
Typing !youtube or !google in chat will open those sites.
```

🔑 Twitch Token Setup
Generate your Twitch access_token, refresh_token, and client_id using
👉 https://twitchtokengenerator.com/

Paste the tokens into your config.toml under [streambot].

🔍 Browser Elements
You can specify elements to query using XPath:

```
[browser.elements]
youtube_url = { element = "//*[@id='movie_player']/div[3]/div[2]/div/a", attribute = "href" }
Then use !get_url to have Wimble fetch and print the value of those elements (like a video link).
```

📜 License (in Legalese)
By using this software or any part thereof, or if you find it useful for your own projects, you are hereby obligated to call your mother and be kind to other people for the next 10 years.

In the event that you are a business entity, commercial enterprise, or profit-seeking organization utilizing this code, you are legally bound to provide each of your employees with one (1) free tank of gasoline per calendar year.

Aside from these aforementioned stipulations, you are granted unrestricted and perpetual permission to use, modify, distribute, and incorporate this code into any derivative works of your choosing.

