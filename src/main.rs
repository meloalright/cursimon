mod stack_view;
use stack_view::StackView;
use stack_view::View;


fn main() {
    let mut stack = StackView::new();

    stack.add_a_child_of_view(View::new().add_button("cc".to_owned(), move |s: &mut StackView| {
        s.say();
        print!("11");
    }));
    stack.add_a_child_of_view(View::new().add_button("qq".to_owned(), move |s: &mut StackView| {
        s.say();
        print!("22");
    }));
    let cb = stack.get(0).unwrap().get(0).unwrap().get_cb();
    cb.clone().handle(&mut stack);
    let cb = stack.get(0).unwrap().get(0).unwrap().get_cb();
    cb.clone().handle(&mut stack);
}
