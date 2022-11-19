use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[macro_export]
macro_rules! inner_getters {
    (self.$v:ident: $t:ty) => {
        /// Gets access to the inner view.
        pub fn get_inner(&self) -> &$t {
            &self.$v
        }
        /// Gets mutable access to the inner view.
        pub fn get_inner_mut(&mut self) -> &mut $t {
            &mut self.$v
        }
    };
}

pub struct StackView {
    layers: Vec<ChildView>,
}

impl StackView {
    /// Creates a new empty StackView
    pub fn new() -> Self {
        StackView { layers: Vec::new() }
    }

    pub fn add_a_child_of_view(&mut self, view: View) {
        let boxedView = BoxedView {
            view: Box::new(view),
        };
        self.layers.push(ChildView { view: boxedView })
    }

    pub fn say(&mut self) {
        println!("say {}", self.layers.len());
    }

    pub fn get(&self, i: usize) -> Option<&View> {
        self.layers.get(i).map(|child| &*child.view)
    }
}

pub struct ChildView {
    view: BoxedView,
}

pub struct BoxedView {
    view: Box<View>,
}

impl Deref for BoxedView {
    type Target = View;

    fn deref(&self) -> &View {
        &*self.view
    }
}

// impl BoxedView {
//     inner_getters!(self.view: View);
// }

pub struct View {
    buttons: Vec<ChildButton>,
}

impl View {
    pub fn new() -> Self {
        View {
            buttons: Vec::new(),
        }
    }

    pub fn add_button<F>(mut self, label: String, cb: F) -> Self
    where
        F: 'static + Fn(&mut StackView),
    {
        self.buttons.push(ChildButton::new(label, cb));
        self
    }

    // pub fn get_button_callback(mut self, i: usize) -> Option<Callback> {
    //     self.buttons.get(i).map(|child| child.button.callback)
    // }

    pub fn get(&self, i: usize) -> Option<&Button> {
        self.buttons.get(i).map(|child| &child.button)
    }
}

pub struct ChildButton {
    button: Button,
}

impl ChildButton {
    pub fn new<F>(label: String, cb: F) -> Self
    where
        F: 'static + Fn(&mut StackView),
    {
        ChildButton {
            button: Button {
                label,
                enabled: true,
                callback: Callback::from_fn(cb),
            },
        }
    }
}

impl Deref for ChildButton {
    type Target = Button;

    fn deref(&self) -> &Button {
        &self.button
    }
}

pub struct Button {
    label: String,
    enabled: bool,
    callback: Callback,
}

impl Button {
    pub fn get_cb(&self) -> &Callback {
        &self.callback
    }
}

#[derive(Clone)]
pub struct Callback(Rc<dyn Fn(&mut StackView)>);

impl Callback {
    /// Wraps the given function into a `Callback` object.
    pub fn from_fn<F>(f: F) -> Self
    where
        F: 'static + Fn(&mut StackView),
    {
        Callback(Rc::new(move |s| {
            f(s);
        }))
    }

    /// Returns a dummy callback that doesn't run anything.
    pub fn dummy() -> Self {
        Callback::from_fn(|_| ())
    }

    pub fn handle(self, s: &mut StackView) {
        if let Callback(cb) = self {
            cb(s);
        }
    }
}
