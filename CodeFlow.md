
# CodeFlow for dodrio::vdom + router + htmltemplate

It is used in a funny game "unForGetTable" or mem6.  
It is a PWA - progressive web app.  
<https://github.com/LucianoBestia/mem6_game>  
The web server is not really important.  
It is mostly a web file server and for the game a websocket server
just to send messages to other players. No logic on the server.  

## index.html

All starts with `index.html`. It contains this parts:  

- classic header for a web page
- a lot of metadata for PWA: manifest.json, mobile-web-app, icons
- a call to start service worker for PWA
- warning if the browser cannot run javascript
- a text to display while loading because it can take some time
- import and init wasm code

## mem6.css

- roboto google font saved on my server
- html defaults
- screen width between 300 and 600 px is ok for the game
- background color black like a dark theme
- the game should be full screen especially when started from Home Screen
- css reset
- svg elements, not-clickable by default
- colors separated in css classes
- use of h1, h2,... as font size even in svg elements as class
- all style is in css, nothing is is html, so is easier to modify
- blinker animation

## lib.rs - wasm_bindgen_start()

This is a "single page" app so the start of wasm is only one time here:

- console_error_panic_hook
- websocketmod::setup_ws_connection - the main way of communication is ws
- RootRenderingComponent::new - all the data is here and the Render trait
- dodrio::Vdom::new - the main object of dodrio always present everywhere
- fetch data from server: game_config, videos, audio,
- start_router - run immediately and on every hash_change

## Router (routermod + routerimplmod)

- `start_router`: the Closure takes `location.hash`. This is a short_route ex. `#p03`
- `update_rrc_local_route` - updates the `rrc.local_route` with the filename ex. `p03_join_a_group.html`
- `async_spwloc_fetch_text` - fetch the html template
- `between_body_tag()` - the html_template is a complete html file. It can be viewed correctly in the browser. It does not yet have any dynamic parts. This is great because the graphical designer can make changes on a true html file. The programmer after that adds comments that are actions for the templating engine. For the html_template we need only the body part.
- `update_rrc_html_template` - updates `rrc.html_template`

## Render (rootrenderingcomponentmod)

Only one function Render() in `impl Render for RootRenderingComponent`.  

- takes `rrc.html_template` and start the templating to `get_root_node()`.
- after that the dodrio vdom will make its magic: find the diffs and update the real dom.

## HtmlTemplate (htmltemplatemod, htmltemplateimplmod)

- get_root_node() returns a complete single `dodrio::Node`
- parses the html_template with `ReaderForMicroXml`
- create `dodrio::Nodes` with `ElementBuilder::new`
- there is a difference between Html nodes and Svg nodes. The latter must have a namespace.
- calls `fill_element_builder` that recursively goes through all nodes
- adds attributes with element.attr()
- if it finds `data-t-` attributes then calls `call_fn_string`with the value. The resulting string is saved to `replace_string`. Then goes to the next attribute and *replace* the value with the result.
- if it finds `data-on-` attribute then calls `call_fn_listener`with the value. The result is a `Closure` that is added to the event listener named in event name like`data-on-EventName` ex. `data-on-click`.
- the TextNode is `decoded_for_xml`
- if it finds a comment like `<!--t=` it will `call_fn_string` and update `replace_string`. The next TextNode will be replaced with this result.
- if the comment is like `<!--n=` it will `call_fn_node` and update `replace_node`. The next Node will be replaced with this result.
- if the comment is like `<!--b=` it will `call_fn_boolean` and update `replace_boolean`. The next node will NOT be rendered if the result is false.

