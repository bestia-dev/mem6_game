# mem6_server

[comment]: # (auto_cargo_toml_to_md start)

**server for the game mem6 http + WebSocket on the same port**  
***version: 2022.1005.1100 date: 2022-10-05 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/mem6_game)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-271-green.svg)](https://github.com/bestia-dev/mem6_game/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-35-blue.svg)](https://github.com/bestia-dev/mem6_game/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-77-purple.svg)](https://github.com/bestia-dev/mem6_game/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/mem6_game/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/mem6_game/)

[comment]: # (auto_lines_of_code end)

**Html and WebSocket server for the mem6 game**  

Primarily made for learning to code Rust for a http + WebSocket server on the same port.  
Using Warp for a simple memory game for kids - mem6.  
On the IP address on port 8086 listens to http and WebSocket.  
Route for http `/` serves static files from folder `/mem6/`.  
Route `/mem6ws/` broadcast all WebSocket msg to all connected clients except sender.  

## Google vm

One working server is installed on my google vm.  
There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8086.
Nginx also redirects all http 80 to https 443.  
You can play the game here (hosted on google cloud platform):  
<https://bestia.dev/mem6>  

## new version of Warp

The new version looks nice, but I had the problem when a user disconnects the websocket without handshake. It happens only on Android Chrome.  
