///javascript function to do full screen and return a promise
export function javascriptdofullscreen() {
    //just an example how to import javascript function to rust
    //this is a javascript function because of different vendor prefixes
    //this is called a jsSnippet and the file must be placed relative to the cargo.toml file
    //wasm-bindgen will then move it around.
    //No need to do anything in index.html like <script> or anything.

    var element = document.documentElement;
    //This is not supported on Safari for iPhone.
    //there is only one workaround: Add to Home Screen this webapp
    if (element.webkitRequestFullscreen) {
        element.webkitRequestFullscreen();
    } else if (element.requestFullscreen) {
        element.requestFullscreen();
    } else if (element.mozRequestFullScreen) {
        element.mozRequestFullScreen();
    } else if (element.msRequestFullscreen) {
        element.msRequestFullscreen();
    }
}