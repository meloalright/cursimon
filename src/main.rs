extern crate cursimon;

#[cfg(test)]
mod test {

    #[test]
    fn hello() {    let mut stack = cursimon::stack_view::StackView::new();
        stack.add_a_child_of_view(
            cursimon::stack_view::View::new().add_button("cc".to_owned(), move |s: &mut cursimon::stack_view::StackView| {
                s.say();
                print!("11");
            }),
        );
        stack.add_a_child_of_view(
            cursimon::stack_view::View::new()
                .add_button("qq".to_owned(), move |s: &mut cursimon::stack_view::StackView| {
                    s.say();
                    print!("22");
                })
                .add_button("zz".to_owned(), move |s: &mut cursimon::stack_view::StackView| {
                    s.say();
                    print!("33");
                }),
        );
        let cb = stack.get(0).unwrap().get(0).unwrap().get_cb();
        cb.clone().handle(&mut stack);
        let cb = stack.get(1).unwrap().get(0).unwrap().get_cb();
        cb.clone().handle(&mut stack);
        let cb = stack.get(1).unwrap().get(1).unwrap().get_cb();
        cb.clone().handle(&mut stack);
        assert_eq!(1+1, 2);
    }
}

fn main() {
    ()
}
