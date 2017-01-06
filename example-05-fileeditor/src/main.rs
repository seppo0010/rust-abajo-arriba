extern crate webplatform;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let document_rc = Rc::new(webplatform::init());
    let document = document_rc.clone();
    let body = document.element_query("body").unwrap();
    body.html_set("
        <select>
            <option></option>
            <option value=\"Cargo.toml\">Cargo.toml</option>
            <option value=\"Cargo.lock\">Cargo.lock</option>
            <option value=\"src/main.rs\">src/main.rs</option>
        </select>
        <div>
            <h1>Editing <span></span></h1>
            <textarea cols=\"80\" rows=\"30\"></textarea>
            <button>Save</button>
        </div>
    ");

    let select_rc = Rc::new(document.element_query("select").unwrap());
    let div_rc = Rc::new(document.element_query("div").unwrap());
    let textarea_rc = Rc::new(document.element_query("textarea").unwrap());
    div_rc.clone().style_set_str("display", "none");
    let filename_rc = Rc::new(RefCell::new(None));

    let span = document.element_query("span").unwrap();
    let textarea = textarea_rc.clone();
    let filename = filename_rc.clone();
    let select = select_rc.clone();
    let div = div_rc.clone();
    select_rc.clone().on("change", move |_| {
            let f = select.prop_get_str("value");
            span.html_set(&*f);
            div.style_set_str("display", if f == "" { "none" } else { "block" });
            textarea.style_set_str("display", "none");

            if f != "" {
                let textarea = textarea.clone();
                webplatform::ajax_get(&*document, &*f, move |xhr| {
                    textarea.style_set_str("display", "block");
                    textarea.prop_set_str("value", &*xhr.response_text().unwrap());
                });
                *filename.borrow_mut() = Some(f);
            } else {
                *filename.borrow_mut() = None;
            }
    });

    let textarea = textarea_rc.clone();
    let document = document_rc.clone();
    let filename = filename_rc.clone();
    let select = select_rc.clone();
    let div = div_rc.clone();
    let button = document.element_query("button").unwrap();
    button.on("click", move |_| {
        let value = textarea.prop_get_str("value");
        if let Some(ref s) = *filename.borrow() {
            select.prop_set_str("value", "");
            select.prop_set_str("disabled", "");
            let div = div.clone();
            let select = select.clone();
            webplatform::ajax_post(&*document, s, Some(&*value), move |_| {
                div.style_set_str("display", "none");
                select.prop_del("disabled");
            });
        }
    });

    webplatform::spin();
}
