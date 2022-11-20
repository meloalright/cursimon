extern crate web_sys;
extern crate cursimon;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct BrowserGlue {
    pub html_canvas: web_sys::HtmlCanvasElement,
}

static CONTAINER_ID: &'static str = "container";
static INFO_ID: &'static str = "info";
static CURSIMON_ID: &'static str = "cursimon";

impl BrowserGlue {
    fn create_container() -> () {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let container = document
            .get_element_by_id(CONTAINER_ID)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let div = document
            .create_element("div")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        div.set_attribute("style", "position: relative; user-select: none;");
        div.set_attribute("class", "cursimon-rs-content");
        div.set_attribute("id", CURSIMON_ID);
        div.set_inner_html("Hello Cursimon!");
        container.append_child(&div);
    }

    fn create_info() -> () {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let container = document
            .get_element_by_id(CONTAINER_ID)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let div = document
            .create_element("div")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        div.set_attribute("id", INFO_ID);
        container.append_child(&div);
    }

    fn append_a_layer_to_container() -> web_sys::HtmlElement {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let container = document
            .get_element_by_id(CURSIMON_ID)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let layer = document
            .create_element("div")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        layer.set_attribute("class", "cursimon-rs-layer");
        container.append_child(&layer);
        layer
    }

    fn handle_event(i: usize, j: usize) {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let info = document
            .get_element_by_id(INFO_ID)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let mut st = String::from("You clicked ");
        let mine_count: u8 = 8; // or i8 or usize
        st.insert(st.len(), char::from_digit(i as u32, 10).unwrap());
        st += "-";
        st.insert(st.len(), char::from_digit(j as u32, 10).unwrap());
        info.set_inner_html(st.as_str());

        let mut i_string = String::from("");
        i_string.insert(0, char::from_digit(i as u32, 10).unwrap());

        let mut j_string = String::from("");
        j_string.insert(0, char::from_digit(j as u32, 10).unwrap());
        info.set_attribute("i", i_string.as_str());
        info.set_attribute("j", j_string.as_str());
    }

    fn create_button_to_layer(layer_element: &web_sys::HtmlElement, label: String, i: usize, j: usize) -> Result<(), JsValue> {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let button = document
            .create_element("button")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        button.set_inner_html(label.as_str());
        // // //
        let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
            //
            BrowserGlue::handle_event(i, j);
        }) as Box<dyn FnMut(_)>);
        // st.borrow_mut().batch_fire();
        button.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;

        cb.forget();
        // // //
        // // //

        button.set_attribute("class", "cursimon-rs-button");
        layer_element.append_child(&button);
        Ok(())
    }

    pub fn setup() {
        BrowserGlue::create_container();
        BrowserGlue::create_info();
    }

    pub fn trigger(s: &mut cursimon::stack_view::StackView, i: usize, j: usize) {
        let cb = s.get(i).unwrap().get(j).unwrap().get_cb();
        cb.clone().handle(s);
    }

    pub fn clear_cursimon() {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let cursimon = document
            .get_element_by_id(CURSIMON_ID)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        cursimon.set_inner_html("Clear!");
    }

    pub fn get_info() {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let info = document
            .get_element_by_id(INFO_ID)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let html = info.get_attribute("cc");
  
    }

    pub fn parse(s: &mut cursimon::stack_view::StackView) {

        let layers_len = s.layers_len();
        for i in 0..layers_len {
            let layer_element = BrowserGlue::append_a_layer_to_container();
            let this_view: &cursimon::stack_view::View = s.get(i).unwrap();
            let buttons_len = this_view.buttons_len();
            for j in 0..buttons_len {
                let this_button: & cursimon::stack_view::Button = this_view.get(j).unwrap();
                BrowserGlue::create_button_to_layer(&layer_element, this_button.get_label(), i, j);
            }
        }
    }
}