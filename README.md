# unForGetTable (development name: mem6_game)

**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web Apps) - part six**  
**A drinking game to lose memory...**  
<<<<<<< HEAD
<<<<<<< HEAD
***version: 6.0  date: 2020-04-11 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/mem6)***  
=======
***version: 6.0  date: 2020-04-11 author: [Dev_Bestia](https://bestia.dev) repository: [GitHub](https://github.com/LucianoBestia/mem6_game)***  
>>>>>>> a7a7e54a633b59e8a5deeaa947d7da1dee520ffc
=======
***version: 6.0  date: 2020-04-11 author: [Dev_Bestia](https://bestia.dev) repository: [GitHub](https://github.com/LucianoBestia/mem6_game)***  
>>>>>>> a7a7e54a633b59e8a5deeaa947d7da1dee520ffc

![loc](https://img.shields.io/badge/lines_of_Rust_code-3129-success)
![loc](https://img.shields.io/badge/lines_of_docs/comments-1335-informational)

## Documentation

Documentation generated from source code:  
<https://bestia-dev.github.io/mem6_game/mem6/index.html>  
The workspace mem6_game is made of projects:  

1. Wasm/WebAssembly (for browsers) frontend - mem6  
2. web server Warp backend - mem6_server  
3. common structures - mem6_common  
4. webfolder - contains files copied to the web folder

Every project has its own readme.md.  

- [mem6/README.md](
https://github.com/bestia-dev/mem6_game/blob/master/mem6/README.md)  
- [mem6_common/README.md](https://github.com/bestia-dev/mem6_game/blob/master/mem6_common/README.md)  
- [mem6_server/README.md](https://github.com/bestia-dev/mem6_game/blob/master/mem6_server/README.md)  
  
Read also my `Previous projects` on Github:  
<https://github.com/bestia-dev/mem5_game>  

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

Read files [TODO.md](https://github.com/bestia-dev/mem6_game/blob/master/TODO.md) and [CHANGELOG.md](https://github.com/bestia-dev/mem6_game/blob/master/CHANGELOG.md).  
