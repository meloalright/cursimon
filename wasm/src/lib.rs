extern crate cursimon;
extern crate glue;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    let mut stack = cursimon::stack_view::StackView::new();
    stack.add_a_child_of_view(
        cursimon::stack_view::View::new().add_button("+haha".to_owned(), move |s: &mut cursimon::stack_view::StackView| {
            s.add_a_child_of_view(
                cursimon::stack_view::View::new().add_button("haha".to_owned(), move |s: &mut cursimon::stack_view::StackView| {
                    s.say();
                    print!("11");
                }),
            );
        }),
    );
    stack.add_a_child_of_view(
        cursimon::stack_view::View::new()
            .add_button("🤔".to_owned(), move |s: &mut cursimon::stack_view::StackView| {
                s.say();
                print!("22");
            })
            .add_button("👻".to_owned(), move |s: &mut cursimon::stack_view::StackView| {
                s.say();
                print!("33");
            }),
    );


    glue::BrowserGlue::setup();
    glue::BrowserGlue::parse(&mut stack);


    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let container = document
        .get_element_by_id("container")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let trigger = document
        .create_element("button")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    trigger.set_inner_html("trigger info 's i and j");
    // // //
    let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
        let info = document
            .get_element_by_id("info")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let i_str = info.get_attribute("i").unwrap();
        let j_str = info.get_attribute("j").unwrap();
        let my_i = i_str.parse::<usize>().unwrap();
        let my_j = j_str.parse::<usize>().unwrap();
        glue::BrowserGlue::trigger(&mut stack, my_i, my_j);
        glue::BrowserGlue::clear_cursimon();
        glue::BrowserGlue::parse(&mut stack);
    }) as Box<dyn FnMut(_)>);
    // st.borrow_mut().batch_fire();
    trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
    container.append_child(&trigger);

    cb.forget();

    Ok({})
}
