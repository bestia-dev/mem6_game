//! **htmltemplatemod**  
//! Html templating for dodrio, generic code for a standalone library.
//! The implementation is in another file where RootRenderingComponents
//! implement the trait HtmlTemplating

// region: use
use reader_for_microxml::*;
use dodrio::{
    Node, Listener, Attribute, RenderContext, RootRender,
    bumpalo::{self},
    builder::{ElementBuilder, text},
};
// use crate::*;
use unwrap::unwrap;
// endregion: use

/// Svg elements are different because they have a namespace
#[derive(Clone, Copy)]
pub enum HtmlOrSvg {
    /// html element
    Html,
    /// svg element
    Svg,
}

/// the RootRenderingComponent struct must implement this trait
/// it must have the fields for local_route and html_template fields
pub trait HtmlTemplating {
    // region: specific implementation code. while rendering, cannot mut rrc
    fn call_fn_string(&self, fn_name: &str) -> String;
    fn call_fn_boolean<'a>(&self, fn_name: &str) -> bool;
    fn call_fn_node<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Node<'a>;
    fn call_fn_vec_nodes<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Vec<Node<'a>>;
    fn call_fn_listener(
        &self,
        fn_name: String,
    ) -> Box<dyn Fn(&mut dyn RootRender, dodrio::VdomWeak, web_sys::Event) + 'static>;
    // endregion: specific implementation code

    // region: generic code (in trait definition)

    /// get root element Node.   
    /// I wanted to use dodrio::Node, but it has only private methods.  
    /// I must use dodrio element_builder.  
    fn render_template<'a>(
        &self,
        cx: &mut RenderContext<'a>,
        html_template: &str,
        html_or_svg_parent: HtmlOrSvg,
    ) -> Result<Node<'a>, String> {
        let mut reader_for_microxml = ReaderForMicroXml::new(html_template);
        let mut dom_path = Vec::new();
        let mut root_element;
        let mut html_or_svg_local = html_or_svg_parent;
        let bump = cx.bump;
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
                    // svg elements have this namespace
                    root_element = root_element.namespace(Some("http://www.w3.org/2000/svg"));
                }
                // recursive function can return error
                match self.fill_element_builder(
                    &mut reader_for_microxml,
                    root_element,
                    cx,
                    html_or_svg_local,
                    &mut dom_path,
                ) {
                    // the methods are move, so I have to return the moved value
                    Ok(new_root_element) => root_element = new_root_element,
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            _ => {
                // return error
                return Err("Error: no root element".to_owned());
            }
        }
        // return
        Ok(root_element.finish())
    }
    /// Recursive function to fill the Element with attributes and sub-nodes(Element, Text, Comment).  
    /// Moves & Returns ElementBuilder or error.  
    /// I must `move` ElementBuilder because its methods are all `move`.  
    /// It makes the code less readable. It is only good for chaining and type changing.  
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn fill_element_builder<'a>(
        &self,
        reader_for_microxml: &mut ReaderForMicroXml,
        mut element: ElementBuilder<
            'a,
            bumpalo::collections::Vec<'a, Listener<'a>>,
            bumpalo::collections::Vec<'a, Attribute<'a>>,
            bumpalo::collections::Vec<'a, Node<'a>>,
        >,
        cx: &mut RenderContext<'a>,
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
        let mut replace_vec_nodes: Option<Vec<Node>> = None;
        let mut replace_boolean: Option<bool> = None;
        let mut html_or_svg_local;
        let bump = cx.bump;
        // loop through all the siblings in this iteration
        loop {
            // the children inherits html_or_svg from the parent, but cannot change the parent
            html_or_svg_local = html_or_svg_parent;
            match reader_for_microxml.read_event() {
                Event::StartElement(name) => {
                    dom_path.push(name.to_owned());
                    // construct a child element and fill it (recursive)
                    let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                    let mut child_element = ElementBuilder::new(bump, name);
                    if name == "svg" {
                        // this tagname changes to svg now
                        html_or_svg_local = HtmlOrSvg::Svg;
                    }
                    if let HtmlOrSvg::Svg = html_or_svg_local {
                        // this is the
                        // svg elements have this namespace
                        child_element = child_element.namespace(Some("http://www.w3.org/2000/svg"));
                    }
                    if name == "foreignObject" {
                        // this tagname changes to html for children, not for this element
                        html_or_svg_local = HtmlOrSvg::Html;
                    }
                    child_element = self.fill_element_builder(
                        reader_for_microxml,
                        child_element,
                        cx,
                        html_or_svg_local,
                        dom_path,
                    )?;
                    // if the boolean is empty or true then render the next node
                    if replace_boolean.unwrap_or(true) {
                        if let Some(repl_node) = replace_node {
                            element = element.child(repl_node);
                            replace_node = None;
                        } else if let Some(repl_vec_nodes) = replace_vec_nodes {
                            for repl_node in repl_vec_nodes {
                                element = element.child(repl_node);
                            }
                            replace_vec_nodes = None;
                        } else {
                            element = element.child(child_element.finish());
                        }
                    }
                    if replace_boolean.is_some() {
                        replace_boolean = None;
                    }
                }
                Event::Attribute(name, value) => {
                    if name.starts_with("data-t-") {
                        // the rest of the name does not matter.
                        // The replace_string will always be applied to the next attribute.
                        let fn_name = value;
                        let repl_txt = self.call_fn_string(fn_name);
                        replace_string = Some(repl_txt);
                    } else if name.starts_with("data-on-") {
                        // Only one listener for now because the api does not give me other method.
                        let fn_name = value.to_string();
                        let event_to_listen = unwrap!(name.get(8..)).to_string();
                        //websysmod::debug_write("&event_to_listen");
                        //websysmod::debug_write(&event_to_listen);
                        let event_to_listen =
                            bumpalo::format!(in bump, "{}",&event_to_listen).into_bump_str();
                        element = element.on(event_to_listen, self.call_fn_listener(fn_name));
                    } else {
                        let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                        let value2;
                        if let Some(repl) = replace_string {
                            value2 =
                            bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(&repl))
                                .into_bump_str();
                            // empty the replace_string for the next node
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
                        txt2 =
                            bumpalo::format!(in bump, "{}",decode_5_xml_control_characters(&repl))
                                .into_bump_str();
                        // empty the replace_string for the next node
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
                    // the main goal of comments is to change the value of the next text node
                    // with the result of a function
                    // it must look like <!--t=get_text-->
                    if txt.starts_with("t=") {
                        let fn_name = unwrap!(txt.get(2..));
                        let repl_txt = self.call_fn_string(fn_name);
                        replace_string = Some(repl_txt);
                    } else if txt.starts_with("n=") {
                        let fn_name = unwrap!(txt.get(2..));
                        let repl_node = self.call_fn_node(cx, fn_name);
                        replace_node = Some(repl_node);
                    } else if txt.starts_with("v=") {
                        let fn_name = unwrap!(txt.get(2..));
                        // vector of nodes
                        let repl_vec_nodes = self.call_fn_vec_nodes(cx, fn_name);
                        replace_vec_nodes = Some(repl_vec_nodes);
                    } else if txt.starts_with("b=") {
                        // boolean if this is true than render the next node, else don't render
                        let fn_name = unwrap!(txt.get(2..));
                        replace_boolean = Some(self.call_fn_boolean(fn_name));
                    } else {
                        // nothing. it is really a comment
                    }
                }
                Event::EndElement(name) => {
                    let last_name = unwrap!(dom_path.pop());
                    // it can be also auto-closing element
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
    // endregion: generic code
}

/// get en empty div node
pub fn empty_div<'a>(cx: &mut RenderContext<'a>) -> Node<'a> {
    let bump = cx.bump;
    ElementBuilder::new(bump, "div").finish()
}

/// decode 5 xml control characters : " ' & < >  
/// https://www.liquid-technologies.com/XML/EscapingData.aspx
/// I will ignore all html entities, to keep things simple,
/// because all others characters can be written as utf-8 characters.
/// https://www.tutorialspoint.com/html5/html5_entities.htm  
pub fn decode_5_xml_control_characters(input: &str) -> String {
    // TODO: I don't know how slow is replace(), but I have really small texts.
    let control_character_names = vec!["&quot;", "&apos;", "&amp;", "&lt;", "&gt;"];
    let control_character_symbols = vec!["\"", "'", "&", "<", ">"];
    let mut output = input.to_owned();
    for i in 0..control_character_symbols.len() {
        output = output.replace(
            unwrap!(control_character_names.get(i)),
            unwrap!(control_character_symbols.get(i)),
        )
    }
    // return
    output
}
