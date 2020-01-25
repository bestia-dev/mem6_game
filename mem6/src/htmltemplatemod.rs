//! **htmltemplatemod**  
//!

use crate::fncallermod;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::logmod;
use crate::divnicknamemod;

use dodrio::builder::*;
use dodrio::{Node, Listener, Attribute, RenderContext};
use dodrio::bumpalo::{self, Bump};
use unwrap::unwrap;
use reader_for_microxml::*;

/// get root element Node. The html template is in the field rrc.html_template.  
/// I wanted to use dodrio::Node, but it has only private methods.  
/// I must use element_builder.  
pub fn get_root_element<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
) -> Result<Node<'a>, String> {
    let mut pp = ReaderForMicroXml::new(&rrc.html_template);
    let mut dom_path = Vec::new();
    let mut root_element;
    let mut html_or_svg = 0; //0-html, 1-svg
    match pp.read_event() {
        Event::StartElement(name) => {
            dom_path.push(name.to_owned());
            let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
            root_element = ElementBuilder::new(bump, name);
            if name == "svg" {
                html_or_svg = 1; //svg
            }
            if html_or_svg == 1 {
                //svg elements have this namespace
                root_element = root_element.namespace(Some("http://www.w3.org/2000/svg"));
            }
            // recursive function can return error
            match fill_element_builder(rrc, &mut pp, root_element, bump, html_or_svg, &mut dom_path)
            {
                //the methods are move, so I have to return the moved value
                Ok(new_root_element) => root_element = new_root_element,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        _ => {
            //return error
            return Err("Error: no root element".to_owned());
        }
    }
    //return
    Ok(root_element.finish())
}

/// Recursive function to fill the tree with a node.  
/// Moves & Returns ElementBuilder or error.  
/// I must `move` ElementBuilder because its methods are all `move`.  
/// It makes the code less readable. It is only good for chaining and type changing.  
fn fill_element_builder<'a>(
    rrc: &RootRenderingComponent,
    pp: &mut ReaderForMicroXml,
    mut element: ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >,
    bump: &'a Bump,
    mut html_or_svg: usize,
    dom_path: &mut Vec<String>,
) -> Result<
    ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >,
    String,
> {
    let mut replacement: Option<String> = None;
    loop {
        match pp.read_event() {
            Event::StartElement(name) => {
                dom_path.push(name.to_owned());
                //construct a child element and fill it (recursive)
                let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                let mut child_element = ElementBuilder::new(bump, name);
                if name == "svg" {
                    //this tagname changes to svg now
                    html_or_svg = 1; //svg
                }
                if html_or_svg == 1 {
                    //this is the
                    //svg elements have this namespace
                    child_element = child_element.namespace(Some("http://www.w3.org/2000/svg"));
                }
                if name == "foreignObject" {
                    //this tagname changes to html for children, not for this element
                    html_or_svg = 0; //html
                }
                child_element =
                    fill_element_builder(rrc, pp, child_element, bump, html_or_svg, dom_path)?;
                element = element.child(child_element.finish());
            }
            Event::Attribute(name, value) => {
                if name.starts_with("data-t-") {
                    //the rest of the name does not matter.
                    //The replacement will always be applied to the next attribute.
                    let fn_name = value;
                    let repl_txt = fncallermod::call_function_string(rrc, fn_name);
                    replacement = Some(repl_txt);
                } else if name.starts_with("data-on-") {
                    // TODO: add a listener.
                    // Only one listener for now because the api does not give me other method.
                    let event_to_listen = &name[8..];
                    let fn_name = value.to_string();
                    let event_to_listen =
                        bumpalo::format!(in bump, "{}",event_to_listen).into_bump_str();
                        logmod::debug_write(&format!("create listener {}", &fn_name));
                    element = element.on(event_to_listen, move |root, vdom, event| {
                        let fn_name = fn_name.clone();
                        let vdom = vdom.clone();
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        //call a function from string
                        logmod::debug_write(&format!("fn_name {}", fn_name));
                        fncallermod::call_listener(&vdom, rrc, fn_name);
                    });
                } else {
                    let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                    let value2;
                    if let Some(repl) = replacement {
                        value2 =
                            bumpalo::format!(in bump, "{}",decode_5_minimum_html_entities(&repl))
                                .into_bump_str();
                        //empty the replacement for the next node
                        replacement = None;
                    } else {
                        value2 =
                            bumpalo::format!(in bump, "{}",decode_5_minimum_html_entities(value))
                                .into_bump_str();
                    }
                    element = element.attr(name, value2);
                }
            }
            Event::TextNode(txt) => {
                let txt2;
                match replacement {
                    Some(repl) => {
                        txt2 =
                            bumpalo::format!(in bump, "{}",decode_5_minimum_html_entities(&repl))
                                .into_bump_str();
                        //empty the replacement for the next node
                        replacement = None;
                    }
                    None => {
                        txt2 = bumpalo::format!(in bump, "{}",decode_5_minimum_html_entities(txt))
                            .into_bump_str();
                    }
                }
                // here accepts only utf-8.
                // only minimum html entities are decoded
                element = element.child(text(txt2));
            }
            Event::Comment(txt) => {
                //the main goal of comments is to change the value of the next text node
                //with the result of a function
                // it must look like <!--t=get_text-->
                if txt.starts_with("t=") {
                    let fn_name = &txt[2..];
                    let repl_txt = fncallermod::call_function_string(rrc, fn_name);
                    replacement = Some(repl_txt);
                }
                //TODO: replace an entire node ?
            }
            Event::EndElement(name) => {
                let last_name = unwrap!(dom_path.pop());
                //it can be also auto-closing element
                if last_name == name || name == "" {
                    return Ok(element);
                } else {
                    return Err(format!("End element not correct: {} {}", last_name, name));
                }
            }
            Event::Error(error_msg) => {
                return Err(format!("{}", error_msg));
            }
            Event::Eof => {
                return Ok(element);
            }
        }
    }
}

//get en empty div node
pub fn empty_div<'a>(cx: &mut RenderContext<'a>) -> Node<'a> {
    div(&cx).finish()
}

/// decode 5 minimum html entities for html5 : " ' & < >  
/// https://www.tutorialspoint.com/html5/html5_entities.htm  
/// I will ignore all others to keep it simple,
/// because all others can be written as utf-8 characters.
pub fn decode_5_minimum_html_entities(input: &str) -> String {
    //I don't know how slow is replace, but I have really small texts.
    let entity_symbols = vec!["\"", "'", "&", "<", ">"];
    let entity_names = vec!["&quot;", "&apos;", "&amp;", "&lt;", "&gt;"];
    let mut output = input.to_owned();
    for i in 0..entity_symbols.len() {
        output = output.replace(entity_names[i], entity_symbols[i])
    }
    //return
    output
}
