use iced::Task;
use log::info;
mod elements;
mod structs;
use simple_logger::SimpleLogger;

use crate::structs::app::App;

fn main() -> iced::Result {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    info!("Logger OK : info visible");
    iced::application("Ping Example", App::update, App::view)
        .subscription(App::subscription)
        .run_with(|| (App::new(), Task::none()))
}
