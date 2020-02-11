//! **htmltemplatemod**  
//! html templating for dodrio

use crate::*;

use reader_for_microxml::*;

use dodrio::{
    Node, Listener, Attribute, RenderContext,
    bumpalo::{self, Bump},
    builder::*,
};
use unwrap::unwrap;

/// Svg elements are different because they have a namespace
#[derive(Clone, Copy)]
pub enum HtmlOrSvg {
    /// html element
    Html,
    /// svg element
    Svg,
}

/// get root element Node.   
/// I wanted to use dodrio::Node, but it has only private methods.  
/// I must use dodrio element_builder.  
pub fn get_root_element<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
    html_template: &str,
    html_or_svg_parent: HtmlOrSvg,
) -> Result<Node<'a>, String> {
    let mut reader_for_microxml = ReaderForMicroXml::new(html_template);
    let mut dom_path = Vec::new();
    let mut root_element;
    let mut html_or_svg_local = html_or_svg_parent;
    #[allow(clippy::single_match_else, clippy::wildcard_enum_match_arm)]
    match reader_for_microxml.read_event() {
        Event::StartElement(name) => {
            dom_path.push(name.to_owned());
            let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
            root_element = ElementBuilder::new(bump, name);
            if name == "svg" {
                html_or_svg_local = HtmlOrSvg::Svg;
            }
            if let HtmlOrSvg::Svg = html_or_svg_local {
                //svg elements have this namespace
                root_element = root_element.namespace(Some("http://www.w3.org/2000/svg"));
            }
            // recursive function can return error
            match fill_element_builder(
                rrc,
                &mut reader_for_microxml,
                root_element,
                bump,
                html_or_svg_local,
                &mut dom_path,
            ) {
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
#[allow(clippy::too_many_lines, clippy::type_complexity)]
fn fill_element_builder<'a>(
    rrc: &RootRenderingComponent,
    reader_for_microxml: &mut ReaderForMicroXml,
    mut element: ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >,
    bump: &'a Bump,
    html_or_svg_parent: HtmlOrSvg,
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
    let mut replace_string: Option<String> = None;
    let mut replace_node: Option<Node> = None;
    let mut html_or_svg_local;
    //loop through all the siblings in this iteration
    loop {
        // the children inherits html_or_svg from the parent, but cannot change the parent
        html_or_svg_local = html_or_svg_parent;
        match reader_for_microxml.read_event() {
            Event::StartElement(name) => {
                dom_path.push(name.to_owned());
                //construct a child element and fill it (recursive)
                let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                let mut child_element = ElementBuilder::new(bump, name);
                if name == "svg" {
                    //this tagname changes to svg now
                    html_or_svg_local = HtmlOrSvg::Svg;
                }
                if let HtmlOrSvg::Svg = html_or_svg_local {
                    //this is the
                    //svg elements have this namespace
                    child_element = child_element.namespace(Some("http://www.w3.org/2000/svg"));
                }
                if name == "foreignObject" {
                    //this tagname changes to html for children, not for this element
                    html_or_svg_local = HtmlOrSvg::Html;
                }
                child_element = fill_element_builder(
                    rrc,
                    reader_for_microxml,
                    child_element,
                    bump,
                    html_or_svg_local,
                    dom_path,
                )?;
                if let Some(repl_node) = replace_node {
                    //logmod::debug_write(&format!("child is repl_node {}", ""));
                    element = element.child(repl_node);
                    replace_node = None;
                } else {
                    element = element.child(child_element.finish());
                }
            }
            Event::Attribute(name, value) => {
                if name.starts_with("data-t-") {
                    //the rest of the name does not matter.
                    //The replace_string will always be applied to the next attribute.
                    let fn_name = value;
                    let repl_txt = fncallermod::call_function_string(rrc, fn_name);
                    replace_string = Some(repl_txt);
                } else if name.starts_with("data-on-") {
                    // Only one listener for now because the api does not give me other method.
                    let fn_name = value.to_string();
                    let event_to_listen =
                        bumpalo::format!(in bump, "{}",&unwrap!(name.get(8..))).into_bump_str();
                    //logmod::debug_write(&format!("create listener {}", &fn_name));
                    element = element.on(event_to_listen, move |root, vdom, _event| {
                        let fn_name = fn_name.clone();
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        //call a function from string
                        //logmod::debug_write(&format!("fn_name {}", fn_name));
                        fncallermod::call_listener(&vdom, rrc, &fn_name);
                    });
                } else {
                    let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                    let value2;
                    if let Some(repl) = replace_string {
                        value2 =
                            bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(&repl))
                                .into_bump_str();
                        //empty the replace_string for the next node
                        replace_string = None;
                    } else {
                        value2 =
                            bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(value))
                                .into_bump_str();
                    }
                    element = element.attr(name, value2);
                }
            }
            Event::TextNode(txt) => {
                let txt2;
                if let Some(repl) = replace_string {
                    txt2 = bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(&repl))
                        .into_bump_str();
                    //empty the replace_string for the next node
                    replace_string = None;
                } else {
                    txt2 = bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(txt))
                        .into_bump_str();
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
                    let fn_name = unwrap!(txt.get(2..));
                    let repl_txt = fncallermod::call_function_string(rrc, fn_name);
                    replace_string = Some(repl_txt);
                } else if txt.starts_with("n=") {
                    let fn_name = unwrap!(txt.get(2..));
                    let repl_node = fncallermod::call_function_node(rrc, bump, fn_name);
                    //logmod::debug_write(&format!("n= {:?}", &repl_node));
                    replace_node = Some(repl_node);
                } else {
                    //nothing. it is really a comment
                }
            }
            Event::EndElement(name) => {
                let last_name = unwrap!(dom_path.pop());
                //it can be also auto-closing element
                if last_name == name || name == "" {
                    return Ok(element);
                } else {
                    return Err(format!(
                        "End element not correct: starts <{}> ends </{}>",
                        last_name, name
                    ));
                }
            }
            Event::Error(error_msg) => {
                return Err(error_msg.to_string());
            }
            Event::Eof => {
                return Ok(element);
            }
        }
    }
}

/// get en empty div node
pub fn empty_div<'a>(cx: &mut RenderContext<'a>) -> Node<'a> {
    div(&cx).finish()
}

/// decode 5 xml control characters : " ' & < >  
/// https://www.liquid-technologies.com/XML/EscapingData.aspx
/// I will ignore all html entities, to keep things simple,
/// because all others characters can be written as utf-8 characters.
/// https://www.tutorialspoint.com/html5/html5_entities.htm  
pub fn decode_5_xml_control_characters(input: &str) -> String {
    //I don't know how slow is replace, but I have really small texts.
    let control_character_symbols = vec!["\"", "'", "&", "<", ">"];
    let control_character_names = vec!["&quot;", "&apos;", "&amp;", "&lt;", "&gt;"];
    let mut output = input.to_owned();
    for i in 0..control_character_symbols.len() {
        output = output.replace(
            unwrap!(control_character_names.get(i)),
            unwrap!(control_character_symbols.get(i)),
        )
    }
    //return
    output
}
