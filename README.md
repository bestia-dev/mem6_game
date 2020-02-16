# "unForGetTable" (development name: mem6)

![loc](https://img.shields.io/badge/lines_of_Rust_code-3009-success)
![loc](https://img.shields.io/badge/lines_of_docs/comments-1260-informational)

a drinking game to lose memory  

**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web Apps) - part six**  
*Things are changing fast. This is the situation on 2020-02-16.*

## Documentation

Documentation generated from source code:  
<https://lucianobestia.github.io/mem6_game/mem6/index.html>  
The workspace mem6_game is made of projects:  

1. Wasm/WebAssembly (for browsers) frontend - mem6  
2. web server Warp backend - mem6_server  
3. common structures - mem6_common  
4. webfolder - contains files copied to the web folder

Every project has its own readme.md.  

- [mem6/README.md](
https://github.com/LucianoBestia/mem6_game/blob/master/mem6/README.md)  
- [mem6_common/README.md](https://github.com/LucianoBestia/mem6_game/blob/master/mem6_common/README.md)  
- [mem6_server/README.md](https://github.com/LucianoBestia/mem6_game/blob/master/mem6_server/README.md)  
  
Read also my `Previous projects` on Github:  
<https://github.com/LucianoBestia/mem5_game>  

## Working game server

You can play the game (mobile only) hosted on google cloud platform:  
<https://bestia.dev/mem6>  

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
