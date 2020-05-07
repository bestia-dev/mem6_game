# unForGetTable (development name: mem6)

![loc](https://img.shields.io/badge/lines_of_Rust_code-3129-success)
![loc](https://img.shields.io/badge/lines_of_docs/comments-1335-informational)

a drinking game to lose memory  

**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication, PWA (Progressive Web Apps) and WebRtc DataChannel - part six**  
*Things are changing fast. This is the situation on 2020-04-22.*

## Documentation

Documentation generated from source code:  
<https://lucianobestia.github.io/mem6_game/mem6/index.html>  

## CodeTour

For better understanding of the code I use the VSCode extension CodeTour.  
The Tours are a sequence of steps with a description and jump to a line in the code.  
That way is very natural and easy to understand the main parts how the code flows.  

## Workspace

The workspace mem6_game is made of projects:  

1. mem6 - Wasm/WebAssembly (for browsers) frontend  
2. mem6_server - web server Warp backend  
3. mem6_common - common structures  
4. webfolder - contains files copied to the web folder

Every project has its own readme.md.  

- [mem6/README.md](
https://github.com/LucianoBestia/mem6_game/blob/master/mem6/README.md)  
- [mem6_common/README.md](https://github.com/LucianoBestia/mem6_game/blob/master/mem6_common/README.md)  
- [mem6_server/README.md](https://github.com/LucianoBestia/mem6_game/blob/master/mem6_server/README.md)  
  
Read also my `Previous projects` on Github:  
<https://github.com/LucianoBestia/mem5_game>  

## other crates

The projects use also other libraries of mine  
(micro crates available in GitHub and crates.io):

- rust_wasm_websys_utils
- rust_wasm_dodrio_templating
- rust_wasm_websocket
- rust_wasm_web_rtc
- reader_for_microxml
- qrcode53bytes

## Working game server

You can play the game (mobile only) hosted on google cloud platform:  
<https://bestia.dev/mem6>  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://bestia.dev/cargo_crev_web/query/num-traits>  

## Cargo make

I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make audit` - cargo audit warnings about dependencies  
`cargo make fmt` - format source code  
`cargo make doc` - copies readme.md into lib.rs doc-comments, build the `/target/doc` folder and copy to the `/docs` folder  
`cargo make sshadd` - adds identity to ssh-agent for git and publish operations  
`cargo make gitpush` - push the commits to github, uses ssh agent  
`cargo make publish` - publish the webfolder to google vm  
`cargo make udeps` - lists unused dependencies  
`cargo make loc` - Lines Of Rust Code and comments with tokei  
`cargo make depver` - list of not latest dependencies  

## TODO and CHANGELOG

Read files [TODO.md](https://github.com/LucianoBestia/mem6_game/blob/master/TODO.md) and [CHANGELOG.md](https://github.com/LucianoBestia/mem6_game/blob/master/CHANGELOG.md).  
