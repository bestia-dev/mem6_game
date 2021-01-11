# unForGetTable  (development name: mem6)

[comment]: # (lmake_readme version)  

mem6 is a simple drinking game to lose memory. It is made primarily for learning the Rust programming language and Wasm/WebAssembly with Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web App).  

## Idea

Playing the memory game alone is boring.  
Playing it with friends is better. Playing with drinking friends is even better. More friends - more fun.  
I hope you have at least 3 or 4 friends now and all of you are around the same table.  
The first player opens bestia.dev/mem6 and starts the group. Other players scan the QR code and join the same group. Then put all phones on the table near to each other. It will look as a "big" board game.  
The game is hyper simple: every player opens 2 cards. If the cards are the same, you drink. If not you don't drink. Then the next player opens 2 cards. And so on...

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

Constructing a HTML page with Virtual DOM (vdom) is easier because it is scheduled to render completely on the next tick (animation frame). We can use the term here "state machine". The rendering depends only on the state of the data and not on the history of the changes.  
Sometimes is very complex what should change in the UI when some data changes.  
The data can change from many different events and very chaotically (asynchronously).  
It is easier to think how to render the complete DOM for a given state of data.  
The Rust Dodrio library has ticks, time intervals when it does something. If a rendering is scheduled, it will be done on the next tick. If a rendering is not scheduled I believe nothing happens.  
This enables asynchronous changing of data and rendering. They cannot happen theoretically in the
same exact moment. So, no data race here.  
When GameData change and we know it will affect the DOM, then rendering must be scheduled.  
The main component of the Dodrio Virtual Dom is the Root Rendering Component (rrc).  
It is the component that renders the complete user interface (HTML) and contains all the data state.  

## GameData

All the game data state are in this simple struct inside the Root Rendering Component.  

## WebSocket communication

HTML5 has finally bring a true stateful bidirectional communication.  
Most of the programming problems are more easily and effectively solved this way.  
The old unidirectional stateless communication is very good for static html pages, but is terrible for any dynamic page. The WebSocket is very rudimental and often the communication breaks for many different reasons. The programmer must deal with it inside the application.  
I send simple structs text messages in json format between the players. They are all in the WsMsg enum and therefore easy to use by the server and client.  
The WebSocket server is coded especially for this game and recognizes the players string that has a vector of ws_uid to whom send the message.  

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
The WebRTC data_channel sounds great for peer-to-peer communication. Very probably the players will be all on the same wifi network, this solves all latency issues.  
I will add this to version 7.  

## The game flow

In a few words:  
Playing player : Status1 - user action - send msg - await for ack msgs - update game data - Status2  
Other players: Status1 - receive WsMessage - send ack msg - update game data - Status2  

In one moment the game is in a one Game Status for all players.  
One player is the playing player and all others are awaiting.  
The active user then makes an action on the GUI.
This action will eventually change the GameData and the GameStatus. But before that there is communication.  
A message is sent to other players so they can also change their local GameData and GameStatus.  
Because of unreliable networks there must be an acknowledge ack msg to confirm, that the msg is received to continue the game.  
The rendering is scheduled and it will happen shortly (async).  

## Futures and Promises, Rust and JavaScript

JavaScript is all asynchronous. Wasm is nothing else then a shortcut to the JavaScript engine.  
So everything is asynchronous too. This is pretty hard to grasp. Everything is Promises and Futures. Fortunately lately there is async/await for Rust and it is great for dealing with javascript.  
JavaScript does not have a good idea of Rust datatypes. All there is is a generic JSValue type.  
The library `wasm-bindgen` has made a fantastic job of giving Rust the ability to call
anything JavaScript can call, but the way of doing it is sometimes cumbersome.  

## Html templating

In the past I wrote html inside Rust code with the macro `html!` from the `crate typed-html`  
<https://github.com/bodil/typed-html>  
It has also a macro `dodrio !` created exclusively for the dodrio vdom.  
I had two main problems with this approach:  

1. Any change to the html required a recompiling. And that is very slow in Rust.  
2. I could not add new html elements, that the macro don't recognize. I wanted to use SVG. There was not support for that.  

I reinvented "html templating".  
First a graphical designer makes a html/css page that looks nice. No javascript, nothing is dynamic. It is just a graphical template.  
Then I insert in it html comments and "data-" attributes that I can later replace in my code.  
The html is not changed graphically because of it. So both the graphical designer and the programmer are still happy.  
In my code I parse the html template as a microXml file. Basically they are the same with small effort. When I find a comment or "data-" attribute then the value of the next node is replaced.  
I can replace attributes, strings and entire nodes. And I can insert event for behavior with "data-t".  
When developing, the html template is loaded and parsed and a dodrio node is created. That is not very fast. But I can change the html in real time and see it rendered without compiling the Rust code. This is super efficient for development.  
I have in plans to add a Rust code generator, that creates the Rust code for the dodrio node before compile time. In that case nothing is parsed in runtime and I expect great speeds. But the flexibility of easily changing the html template is gone. For every change I must recompile the Rust code.  

## Browser console

At least in modern browsers (Firefox and Chrome) we have the developer tools F12 and there is a
console we can output to. So we can debug what is going on with our Wasm program.
But not on smartphones !! I save the error and log messages in session_storage and this is displayed on the screen.  

## Safari on iOS and FullScreen

Apple is very restrictive and does not allow fullscreen Safari on iPhones.  
The workaround is to `Add to HomeScreen` the webapp.  

## PWA (Progressive Web App)

On both android and iPhone is possible to use PWA.  
To be 100% PWA it must be secured with TLS and must have a service worker.  
I added also the PWA manifest and png images for icons and now the game is a full PWA.  

**Very important :**
On Android Chrome to `Clear & reset` the cached data of the website you must click on the icon of the URL address (the lock) and choose `Site Settings`.  
Sometimes even that does not work. Than I go in the Menu to Settings - Privacy - Clear browser data and delete all. Very aggressive, but the only way I found that works.  

## Modules

Rust code is splitted into modules. They are not exactly like classes, but can be similar.  
Rust has much more freedom to group code in different ways. So that is best suits the problem.  
I splitted the rendering pages and that into sub-components.  
And then I splitted the User Actions by the Status1 to easy follow the flow of the game.  

## State machine

I try to use the philosophy of "state machine" because it is easier to follow.  
All is dependent on the state of the data and not on the chronological events.  
Any event can change the state/data. Then another piece of code will do the rest
based on the state/data. The data can come from different places: user input, fetch from
web, url/hash, local_storage. It doesn't matter. All the data must first go into the state/data.  
Then some other code makes decision based on the state/data.  
The app is divided in 2 ways:

- visually it is divided in pages and page components
- behaviorally is divided into pages and game_state

## Clippy

Clippy is very useful to teach us how to program Rust in a better way.  
These are not syntax errors, but hints how to do it in a more Rusty way (idiomatic).  
Some lints are problematic and they are explicitly allowed here.

## font-size

Browsers have 2 types of zoom:

- zoom everything proportionally (I like this.)
- zoom only the text (I hate this: it breaks the layout completely.)

When the font-size in android is increased (accessibility) it applies somehow also to the browser rendering.  
I have tried many different things, but it looks this cannot be overridden from the css or javascript. Only the user can change this setting in his phone.  

## SVG

This is why I chose to use SVG for my html templates. Svg promises that the user cannot ruin the layout completely. But also SVG has its set of complication small and big.  
It is annoying that SVG must use namespaces for all the elements and subelements.  
I will use percents to define x, y, width and height. Because for the game is only logical to be always full screen.

## font-family

The size of the font depends a lot of the font-family. Every browser show different fonts
even when they call them the same. I need to use a third-party web font. Google seems to
be a good source of free fonts. I choose Roboto. Having them download every time from google is time consuming. So I will download them and host them locally on my website.  
I use the <https://google-webfonts-helper.herokuapp.com> to download fonts.  

## favicon.ico

Crazy stuff. I used the website <https://www.favicon-generator.org/> to generate
all the different imgs, sizes and code. And than add all this into index.html. There is more lines for icons than anything else now. Just crazy world.  

## audio normalize

Different audio files had very different volume levels. That was not nice.
I used Audacity with the ffmpeg plugin. I opened the audio file and then used
Effect normalize. The songs were normalized at -4dB and the voice moniker at -1 dB.
So the songs are not too loud, like they were before.  

## big img

Some types of images are simply too small (like bottles). You cannot see any details.
I added a second big image. It opens with a smooth animation on click on the small image.
So the details are nice visible.  

## Easter egg

Click on the title unForGetTable opens a new browser tab with music about memory and forgetting.  

## text to speech

When creating a new kind of game I prepare images and labels.  
Then usually I use an online "text to speech" to produce the sound from the labels.  
The best so far I found is:  
<https://text-to-speech-demo.ng.bluemix.net/>  
This is the IBM Watson text to speech service.  
I prepare the text so every label is in a separate line and be sure to put a full stop on every line.  
I copy/paste it and press Speak.  
Then on the right side I click on the Kebap Button and click on Download.  
The downloaded file has the name "synthesize" with no extension. I think it is in reality an ".mp3", but it doesn't really matter.  
I open the file in Audacity. Ctrl+A to select the track. Then Analyze - Silence Finder. Tweak a little with the settings.  
When I'm happy with the result then File - Export -Export Multiple as mp3.  
Finally I rename the files to match the names in the game_config.json file.  
