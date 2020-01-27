# mem6

[comment]: # (lmake_readme version)  

mem6 is a simple memory game made primarily for learning the Rust programming language and Wasm/WebAssembly with Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web App).  

## Idea

Playing the memory game alone is boring.  
Playing it with friends is better.  
But if all friends just stare in their smartphones, it is still boring.  
What makes memory games (and other board games) entertaining is the company of friends.  
There must be many friends around the table watching one another and stealing moves and laughing and screaming at each other.  
Today I assume everybody has a decent smartphone. If all friends open the mem6 game and put their smartphones on the center of the table one near the other so that everybody can see them and touch them, this is the closest it gets to a **classic board game**.  
All the phones will have a small card grid (ex. 3x3). But the combined card grid from all these phones together is not so small anymore. It is now much more interesting to play for many players.  
It can be played with as many friends as there are: 3,4,5,...  
More friends - more fun.  

## Rust and Wasm/WebAssembly

Rust is a pretty new language created by Mozilla for really low level programming.  
It is a step forward from the C language with functionality and features that are best practice today.  
It is pretty hard to learn. Some concepts are so different from other languages it makes it
hard for beginners. Lifetimes are the strangest and most confusing concept.  
The Rust language has been made from the ground up with an ecosystem that makes it productive.  
The language and most of the libraries are Open Source. That is good and bad, but mostly good.  
Rust is the best language today to compile into Wasm/WebAssembly.  
That compiled code works inside a browser directly with the JavaScript engine.  
So finally no need for JavaScript to make cross-platform applications inside browsers.  
I have a lot of hope here.  

## Virtual DOM

Constructing a HTML page with Virtual DOM (vdom) is easier because it is rendered completely on every tick (animation frame).  
Sometimes is very complex what should change in the UI when some data changes.  
The data can change from many different events and very chaotically (asynchronously).  
It is easier to think how to render the complete DOM for a given state of data.  
The Rust Dodrio library has ticks, time intervals when it does something. If a rendering is scheduled, it will be done on the next tick. If a rendering is not scheduled I believe nothing happens.  
This enables asynchronous changing of data and rendering. They cannot happen theoretically in the
same exact moment. So, no data race here.  
When GameData change and we know it will affect the DOM, then rendering must be scheduled.  
The main component of the Dodrio Virtual Dom is the Root Rendering Component (rrc).  
It is the component that renders the complete user interface (HTML).  
The root rendering component is easily splitted  into sub-components.  

![subcomponents](https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/subcomponents.png)  

Some subcomponents don't need any extra data and can be coded as simple functions.  
The subcomponent "players and scores" has its own data. This data is cached from the GameData.  
When this data does not match, invalidation is called to cache them.
That also schedules the rendering of the subcomponent.  
If no data has changed, the cached subcomponent Node is used. This is more efficient and performant.  

## GameData

All the game data are in this simple struct.  

## WebSocket communication

HTML5 has finally bring a true stateful bidirectional communication.  
Most of the programming problems are more easily and effectively solved this way.  
The old unidirectional stateless communication is very good for static html pages, but is terrible for any dynamic page. The WebSocket is very rudimental and often the communication breaks for many different reasons. The programmer must deal with it inside the application.  
I send simple structs text messages in json format between the players. They are all in the WsMsg enum and therefore easy to use by the server and client.  
The WebSocket server is coded especially for this game and recognizes 2 types of msg:
TODO: decide on the client what the server will do with the msg.

- msg to broadcast to every other player
- msg to send only to the actual game players

## WebSockets is not reliable

Simple messaging is not reliable. On mobiles it is even worse. There is a lot of possibilities that something goes wrong and the message doesn't reach the destination. The protocol has nothing that can be used to deal with reconnections or lost messages.  
That means that I need additional work on the application level - always reply one acknowledgement "ack" message.  
Workflow:  

- sender sends one message to more players (more ws_uid) with one random number msg_id
    push to a vector (msg queue) more items with ws_uid and msg_id
    blocks the continuation of the workflow until receives all ACK from all players

- receiver on receive send the ACK acknowledge msg with his ws_uid and msg_id

- the sender receives the ACK and removes one item from the vector
    if there is no more items for that msg_id, the workflow can continue.
    TODO: if after 3 seconds the ACK is not received and error message is shown to the player.

This is very similar to a message queue...  

## gRPC, WebRTC datachannel

The new shiny protocol gRPC for web communication is great for server-to-server communication. But it is still very limited inside the browser. When it eventually becomes stable I would like to change WebSockets for gRPC.  
The WebRTC datachannel sounds great for peer-to-peer communication. Very probably the players will be all on the same wifi network, this solves all latency issues. TODO: in version 6.  

## The game flow

In a few words:  
Playing player : Status1 - user action - send msg - await for ack msgs - update game data - Status2  
Other players: Status1 - receive WsMessage - send ack msg - update game data - Status2  
  
Before the actual game there is the `join` flow.  
It is a little bit different from the game flow. The first player broadcast an invitation msg.  
All other players that are in the first status receive that and are asked if they join.  
When the user join it sends a msg to the first player.  
The first player waits to receive msgs from all other users.  
After that he starts the game. This calculates the game_data and send this init data to all other players.  

| Game Status1         | Render               | User action           | GameStatus2 p.p. | Sends Msg       | On rcv Msg o.p.       | GameStatus2 o.p. |
| -------------------- | -------------------- | --------------------- | ---------------- | --------------  | -------------------   | --------------   |
| p.p. StatusStartPage | div_start_page       | on_click_invite       |    | MsgInvite       | on_msg_invite         |     |
| o.p.    |           | on_click_join       | StatusJoined   | MsgJoin       | on_msg_joined         | -                |
| o.p. StatusJoined  | div_invite_joined  |                       |                  |                 |                       | -                |
| p.p.   |          | on_click_start_game   | Status1stCard    | MsgStartGame    | on_msg_start_game     | Status1stCard    |

This starts the game flow, that repeats until the game is over.  
  
In one moment the game is in a one Game Status for all players.  
One player is the playing player and all others are awaiting.  
The active user then makes an action on the GUI.
This action will eventually change the GameData and the GameStatus. But before that there is communication.  
A message is sent to other players so they can also change their local GameData and GameStatus.  
Because of unreliable networks there must be an acknowledge ack msg to confirm, that the msg is received to continue the game.  
The rendering is scheduled and it will happen shortly (async).  

| Game Status1      | Render                     | User action                    | Condition                     | GameStatus2 p.p.    | Sends Msg            | On rcv Msg o.p.             | GameStatus2 o.p.               |
| ----------------  | -------------------------- | ------------------------------ | ----------------------------- | ----------------    | ----------------     | --------------------------  | ----------------------------   |
| Status1stCard     | div_grid_container         | on_click_1st_card()            | -                             | Status2ndCard       | MsgClick1stCard      | on_msg_click_1st_card       | Status2ndCard                  |
| Status2ndCard     | div_grid_container         | on_click_2nd_card()            | If cards match                | Status1stCard       | MsgClick2ndCardPoint | on_msg_click_2nd_card_point | Status1stCard                  |
| -                 | -                          | continues on ack msgs received | if all cards permanently up   | StatusGameOver      | MsgGameOver          | on_msg_game_over            | StatusGameOver                 |
| Status2ndCard     | div_grid_container         | on_click_take_turn_begin       | else cards don't match        | StatusTakeTurnBegin | MsgTakeTurnBegin     | on_msg_take_turn_begin      | MsgTakeTurnBegin               |
| MsgTakeTurnBegin  | div_take_turn_begin        | on_click_take_turn_end         | -                             | Status1stCard       | MsgTakeTurnEnd       | on_msg_take_turn_end        | Status1stCard, the next player |
| StatusGameOver    | div_game_over              | window.location().reload()     | -                             | -                   | -                    | -                           | -                              |
|  |  |  |  |  |  |  |  |

p.p. = playing player,   o.p. = other players,  rrc = rrc, rcv = receive

1. Some actions can have different results. For example the condition if card match or if card don’t match.  
2. one action must be only for one status1. This action changes Status for this player and sends Msg to other players.  
3. on receive msg can produce only one status2.  

## Futures and Promises, Rust and JavaScript

JavaScript is all asynchronous. Wasm is nothing else then a shortcut to the JavaScript engine.  
So everything is asynchronous too. This is pretty hard to grasp. Everything is Promises and Futures.  
There is a constant jumping from thinking in Rust to thinking is JavaScript and back. That is pretty confusing.  
JavaScript does not have a good idea of Rust datatypes. All there is is a generic JSValue type.  
The library `wasm-bindgen` has made a fantastic job of giving Rust the ability to call
anything JavaScript can call, but the way of doing it is sometimes cumbersome.  

## Typed html

Writing html inside Rust code is much easier with the macro `html!` from the `crate typed-html`  
<https://github.com/bodil/typed-html>  
It has also a macro `dodrio!` created exclusively for the dodrio vdom.  
Everything is done in compile time, so the runtime is nothing slower.  
TODO: what if an attribute is not covered by the macro. Can I add it later?

## Browser console

At least in modern browsers (Firefox and Chrome) we have the developer tools F12 and there is a
console we can output to. So we can debug what is going on with our Wasm program.
But not on smartphones !! I save the error and log messages in sessionStorage and this is displayed on the screen.  

## Safari on iOS and FullScreen

Apple is very restrictive and does not allow fullscreen Safari on iPhones.  
The workaround is to `Add to HomeScreen` the webapp.  

## PWA (Progressive Web App)

On both android and iPhone is possible to use PWA.  
To be 100% PWA it must be secured with TLS and must have a service worker.  
I added also the PWA manifest and png images for icons and now the game is a full PWA.  

**Very important :**
On Android Chrome to `Clear & reset` the cached data of the website you must click on the icon of the URL address (the lock) and choose `Site Settings`.  
Sometimes even that does not work. Than I go in the Menu to Settings - Privacy - Clear browser data and delete all. Very aggressive, but the only thing that works.  

## Modules

Rust code is splitted into modules. They are not exactly like classes, but can be similar.  
Rust has much more freedom to group code in different ways. So that is best suits the problem.  
I splitted the rendering into sub-components.  
And then I splitted the User Actions by the Status1 to easy follow the flow of the game.  

## Clippy

Clippy is very useful to teach us how to program Rust in a better way.  
These are not syntax errors, but hints how to do it in a more Rusty way (idiomatic).  
Some lints are problematic and they are explicitly allowed here.

## font-size

Browsers have 2 types of zoom:

- zoom everything proportionally (I like it)
- zoom only the text (this breaks the layout completely)

When the font-size in android is increased (accessibility) it applies somehow also to the browser rendering.  
I have tried many different things, but it looks this cannot be overridden from the css or javascript. Only the user can change this setting in his phone.  
