// use wasm_bindgen::prelude::*;
// use web_sys::{window, Document, Element, HtmlElement};

// #[wasm_bindgen]
// pub fn add_border() {
//     // Get the current document and body element
//     let document = window().unwrap().document().unwrap();
//     let body = document.body().unwrap();

//     // Create a new <style> element and set its CSS
//     let style = document.create_element("style").unwrap().unchecked_into::<HtmlElement>();
//     style.set_inner_html("body { border: 5px solid red; }");

//     // Append the <style> element to the <body>
//     body.append_child(&style).unwrap();
// }
