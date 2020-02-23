# dodrio_templating

## Html templating

In the past I wrote html inside Rust code with the macro `html!` from the `crate typed-html`  
<https://github.com/bodil/typed-html>  
It has also a macro `dodrio!` created exclusively for the dodrio vdom.  
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
