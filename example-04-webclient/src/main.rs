extern crate webplatform;

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    body.html_set("<h1>HELLO FROM RUST</h1> <button>CLICK ME</button>");
    let button = document.element_query("button").unwrap();
    button.on("click", |_| webplatform::alert("WITNESS ME"));
}
