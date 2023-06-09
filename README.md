# PgSdl

A GUI librairy / Widget toolkit built on top of sdl2, in rust. Originally created for the Benday project.

## The concept

PgSdl aims to be easy to use and flexible at the same time. It is basically a widget toolkit and a set of high level Sdl2 functions wrapper.

### The widgets

The librairy comes with builtins widget that are easy to use with the main loop pattern describe down. Under the hood, all the widget are kept in a hashmap, with the name you give name as key. 
You can implement any custom widget as long as you impl the `widget` trait. This will allow you to add it to the list of widget. 
The interesting fallback with this method is that you can't have two mutable references of any widget of that list at the same time (the borrow checker cannot now if you borrowed twice the same widget).
Putting the widgets in the app, and there update and draw will be call automaticaly.
You can also keep them in your app state and use them in the main loop yourself.

### The main loop

You create a struct that will hold the state of your app and implement the two functions `fn update(...)` and `fn draw(...)`. In update you have acces to the inputs and all the widgets you add to your app. 


## Usage

```rust
use pg_sdl::prelude::*;

// Here we define our app-state struct
pub struct MyApp {
    pub draw_circle: bool,
}

// To call the run function of PgSdl, we need to implement the App trait for our app-state struct
impl App for MyApp {
    // The update function is called every frame, and is used to update the app-state
    fn update(&mut self, _delta: f32, _input: &Input, widgets: &mut Widgets) -> bool {
        let mut changed = false;
        if self.draw_circle {
            changed = true;
            self.draw_circle = false;
        }
        let button = widgets.get_button("button");
        if button.state.is_down() {
            self.draw_circle = true;
            changed = true;
        }
        changed
    }

    // The draw function is called every frame, if update returned true or any widget has changed
    // It is called just after the update function
    fn draw(&mut self, canvas: &mut Canvas<Window>, _text_drawer: &mut TextDrawer) {
        // We can put any custom drawing code here
        if self.draw_circle {
            canvas.set_draw_color(Colors::VIOLET);
            draw_circle(canvas, point!(500, 400), 100, 20);
        }
        // All the widgets are drawn automatically by PgSdl
    }
}

fn main() {
    // First we initialize our custom app-state struct
    let mut my_app = MyApp { draw_circle: false };

    // Then we initialize the PgSdl struct
    let mut pd_sdl: PgSdl = PgSdl::init("Benday", 1280, 720, Some(60), true, Colors::SKY_BLUE);

    // We can add widgets to the PgSdl struct (as long as they implement the Widget trait)
    // We will retrieve them later in the update function with the name we gave them
    pd_sdl
        .add_widget(
            "button",
            Box::new(Button::new(
                Colors::ROYAL_BLUE,
                rect!(500, 500, 200, 100),
                Some(9),
                Some(Text::new("Auto !".to_string(), 16, None)),
            )),
        );

    // Finally we run the app, that take a mutable reference to our custom app-state struct
    pd_sdl.run(&mut my_app);
}
```
Find the full example [here](./examples/basic.rs) or run with `cargo run --example basic`

## License
This project is licensed under the MIT License - see the [LICENSE.md](./docs/LICENSE.md) file for details

## Contributing
Please read [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests.
