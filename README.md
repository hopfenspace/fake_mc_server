# Fake Minecraft Server

This is a simple program which responds to Minecraft [Server List Pings](https://wiki.vg/Server_List_Ping) and responds with configurable values. This allows to set a custom version name, max players, current players and motd. Additionally when a user tries to connect to the server he is immediately disconnected with a custom message that can also be configured.

## Setup

```sh
git clone https://github.com/hopfenspace/fake_mc_server
cd fake_mc_server
cargo run
```

For Server list customizations edit `server-list.json`, it has the format as defined [at wiki.vg](https://wiki.vg/Server_List_Ping#Response).

For custom disconnect messages edit `disconnect.json`, it is in `/tellraw` format as defined [at wiki.vg](https://wiki.vg/Chat) and the [minecraft wiki](https://minecraft.gamepedia.com/Raw_JSON_text_format). You can use any of the available tellraw generators to generate it, such as [minecraft.tools](https://minecraft.tools/en/tellraw.php).

## Why?

We run a server which hosts multiple minecraft servers. The first one was on port 25565, but users occasionally forgot to enter the port for the server they actually wanted to connect to. We then moved all minecraft servers to custom ports, and are using this tool to tell the user he is missing a port in the address (i.e. they tried connecting to port 25565).

## Screenshots

![](https://i.m4gnus.de/fcbe7.png)

![](https://i.m4gnus.de/fcbe6.png)
