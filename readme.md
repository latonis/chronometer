# Chronometer 🕒
Welcome to this wonderful repository where I try out an application in rust with an actual UI (is it usable? who knows)!

This was built on stream at https://twitch.tv/justtoocloudy

## UI Crate
This application was built leveraging the Slint UI Toolkit
- Website: https://slint.dev/
- Crate: https://crates.io/crates/slint
## Usage

### Setting Time via CLI
`chronometer -h <hours> -m <minutes> -s <seconds>`

### Setting Time via GUI
`chronometer`

![chronometer application GUI](/images/image.png)

### Counting Down

Once started, the timer will count down until it is finished.

![chronometer application once countdown started](/images/countdown.png)

Once finished, the timer will cease counting down and allow to enter another time if desired.

![achronometer application once countdown finished](/images/fin.png)

## Other
Found a bug? Submit an issue or a PR to fix it! :)
