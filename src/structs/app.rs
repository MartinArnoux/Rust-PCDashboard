use crate::elements::ping_button::PingButton;
use crate::structs::ping_controller::PingController;
use crate::structs::ping_settings::PingSettings;
use iced::widget::{container, row};
use iced::{
    Alignment, Element, Subscription, Task,
    time::{self},
};

pub struct App {
    ping_settings: PingSettings,
    ping_controller: PingController,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartStopPingButtonPressed,
    NextPing,
}

impl App {
    pub fn new() -> Self {
        App {
            ping_settings: PingSettings::new(),
            ping_controller: PingController::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartStopPingButtonPressed => self.ping_controller.start_stop_pinging(),
            Message::NextPing => self.ping_controller.ping_once(self.ping_settings),
        }
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if !self.ping_controller.is_pinging() {
            return Subscription::none();
        }

        if !self.ping_controller.had_receive() {
            return Subscription::none();
        }
        time::every(self.ping_settings.get_interval_ping()).map(|_| Message::NextPing)
    }

    pub fn view(&self) -> Element<Message> {
        let ping_button: PingButton = PingButton::new(&self.ping_controller);
        // Row pour mettre le ping et le bouton en haut à droite
        let top_right = row![ping_button].spacing(10).align_y(Alignment::Center);

        // Container pour positionner en haut à droite
        let content = container(top_right)
            .padding(10)
            .align_x(iced::alignment::Horizontal::Right)
            .align_y(iced::alignment::Vertical::Top)
            .width(iced::Fill)
            .height(iced::Fill);

        content.into()
    }
}
