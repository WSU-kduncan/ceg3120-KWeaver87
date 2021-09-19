# Project 2 - Discord Bot

This is a class project Discord Bot written in Rust on top of the [serenity](https://github.com/serenity-rs/serenity) library. Its only purpose is to recite quotes of the character [William Riker](https://memory-alpha.fandom.com/wiki/William_T._Riker) from the TV show Star Trek: The Next Generation.

## Usage

There are only two interactions with the bot. Entering `!riker` into chat will get a single line, and the `/riker` Command will give you an option to select up to 8 lines at once.

## Setup

### Discord Developer Portal

This part of the guide will be abbreviated. A more complete guide is at [RealPython](https://realpython.com/how-to-make-a-discord-bot-python/), and there are many others that can be found via a websearch.

- Need a [Discord](https://discord.com/) account.
- Create an application at the [Developer Portal](https://discord.com/developers/applications)
- Under Bot menu, create a new bot.
- Under OAuth2, you'll need to create a URL with the Scopes `bot` and `applications.commands`, and the Bot Permissions `Manage Webhooks` and `Send Messages`.
- Copy and save the Application ID from General Information menu and Token from Bot menu.

### Compiling the code

Note: This may take up to 2GB of free disk space.

- Have [rustup](https://rustup.rs/) installed and updated.
- On a CLI, clone the repo and CD into the `Discord-Bot` directory.
- Run `cargo build --release` to compile an optimized binary (this will take a few minutes).
- Copy the binary from `target/release/riker-bot` to desired execution directory (this guide will use `/opt/riker-bot` as an example).

### Running the bot

These instructions assume you are using a systemd-based Linux distro (most are, especially Debian, Ubuntu, Arch, etc.). There's no official support for SysVinit-based distros.

- On your server, create a system user for the bot to run under.
  - On a Debian-based system (such as Ubuntu), use `sudo adduser --system --group --home /opt/riker-bot riker-bot`.
- Place your Application ID and Token in a file at `/opt/riker-bot/.env`, following this template:

```sh
DISCORD_TOKEN=YourTokenHere
DISCORD_APP_ID=YourAppIdHere
```

- Copy `riker-bot.service` from this repo to `/etc/systemd/system/`, adapting it as you need.
  - If not already set, change the permissions with `sudo chmod 644 riker-bot.service`.
- Copy the directory `data/` from this repo to `/opt/riker-bot/`
- Use `sudo systemctl enable riker-bot` to enable the program to run on bootup.
- Use `sudo systemctl start riker-bot` to start the program immediately.
- Check that the bot is running (and read any error messages) by viewing the log with `sudo journalctl -u riker-bot`.
- Check on your Discord server/guild that the bot is working as desired!

## Python Data Format Conversion

The original file containing Riker quotes, coming from [RikerIpsum](https://github.com/ben174/rikeripsum/blob/master/rikeripsum/data/riker.pickle), is in a Python pickle file format, which I couldn't find a working Rust library to for. The solution I went with was to CD to the `data` directory and use the python2 REPL to convert the file to JSON.

```py
import os
import pickle
import json

file_in = open('riker.pickle')
loaded_data = pickle.load(file_in)

file_out = open('riker.json', 'w')
file_out.write(json.dumps(loaded_data))
file_out.close()
```
