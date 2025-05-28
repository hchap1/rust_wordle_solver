mod wordle;
mod utility;
mod application;

use application::Application;

fn main() -> iced::Result {
    iced::application("Wordle Solver", Application::update, Application::view)
        .subscription(Application::subscription)
        .run()
}
