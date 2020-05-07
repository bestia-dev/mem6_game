# mem6_server

[comment]: # (lmake_readme cargo.toml data start)

[comment]: # (lmake_readme cargo.toml data end)  

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

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://bestia.dev/cargo_crev_web/query/num-traits>  
