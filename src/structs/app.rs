use iced::{Background,Color};
use iced::widget::{button, text, container,row};
use iced::{Alignment, Element, Subscription, Task, time::{self},};
use crate::structs::ping_controller::{PingController, PingControllerError};
use crate::structs::{ping_settings::{PingSettings}};

pub struct App {
    ping_settings: PingSettings,
    ping_controller: PingController
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
            return Subscription::none()
        }
        
        if self.ping_controller.had_receive() {
            return Subscription::none()
        }

        time::every(self.ping_settings.get_interval_ping()).map(|_| Message::NextPing)

    }

    pub fn view(&self) -> Element<Message> {
        // Texte à afficher en haut à droite
        let ping_result = match self.ping_controller.get_last_ping() {
            Ok(v) => v.to_string(),
            Err(PingControllerError::NoLastPing) => {
                "Aucun".to_string()
            }
            Err(PingControllerError::ErrConnection) => {
                "Erreur".to_string()
            }
        };


        let ping_display = text(
            ping_result
        )
        .size(16);

    let ping_to_display = match self.ping_controller.get_last_ping() {
        Ok(v) => v,
        Err(PingControllerError::NoLastPing) => 0,
        Err(PingControllerError::ErrConnection) => 1000000
    };

    let ping_color = if self.ping_controller.is_pinging() && ping_to_display > 0 {
        if ping_to_display < 100 {
            Color::from_rgb8(0x28, 0xA7, 0x45) // vert
        } else if ping_to_display < 200 {
            Color::from_rgb8(0xFF, 0xC1, 0x07) // orange
        } else {
            Color::from_rgb8(0xDC, 0x35, 0x45) // rouge
        }
    } else {
        Color::from_rgb8(0x6C, 0x75, 0x7D) // gris pour ancien ping
    };

        let ping_button = button(ping_display)
        .on_press(Message::StartStopPingButtonPressed)
        .style(move |_,_| button::Style {
            background: Some(Background::Color(ping_color)),
            ..Default::default()
        })
        ;



        // Row pour mettre le ping et le bouton en haut à droite
        let top_right = row![ping_button]
            .spacing(10)
            .align_y(Alignment::Center);

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