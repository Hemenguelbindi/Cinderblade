mod app;
mod player;
mod plugins;
mod prelude;
mod states;
mod ui;

use app::build_app;


fn main() {
    build_app().run();
}
