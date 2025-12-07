use iced::{Task};


mod structs;

use crate::structs::{app::App};

fn main() -> iced::Result {
    

    iced::application("Ping Example", App::update, App::view)
        .subscription(App::subscription).run_with(|| (App::new(), Task::none()))
}
