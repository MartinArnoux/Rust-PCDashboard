use crate::structs::app::Message;
use crate::structs::ping_controller::{PingController, PingControllerError};
use iced::Color;
use iced::Event;
use iced::advanced::Clipboard;
use iced::alignment::Horizontal;
use iced::widget::text::{LineHeight, Shaping, Wrapping};
use iced::{
    Border, Rectangle, Shadow, Size, Theme,
    advanced::{
        Layout, Shell, Text, Widget,
        graphics::core::event,
        layout, mouse,
        renderer::{self, Quad},
        widget::Tree,
    },
};
use iced::{Element, Length};

pub struct PingButton {
    label: String,
    color: Color,
}

impl PingButton {
    pub fn new(controller: &PingController) -> Self {
        let label = match controller.get_last_ping() {
            Ok(v) => v.to_string(),
            Err(PingControllerError::NoLastPing) => "Aucun".to_string(),
            Err(PingControllerError::ErrConnection) => "Erreur".to_string(),
        };
        let ping_to_display = match controller.get_last_ping() {
            Ok(v) => v,
            Err(PingControllerError::NoLastPing) => 0,
            Err(PingControllerError::ErrConnection) => 1000000,
        };

        let color = if controller.is_pinging() && ping_to_display > 0 {
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
        Self { label, color }
    }
}

impl<Renderer> Widget<crate::structs::app::Message, Theme, Renderer> for PingButton
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(iced::Size {
            width: 50.0,
            height: 25.0,
        })
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.8, 1.0),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            self.color,
        );
        let bounds = layout.bounds();

        // Dessin du texte au centre du bouton
        renderer.fill_text(
            Text {
                content: self.label.clone(),
                bounds: bounds.size(),
                size: renderer.default_size(), // taille du texte
                font: renderer.default_font(), // ou une police custom
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                line_height: LineHeight::default(),
                shaping: Shaping::default(),
                wrapping: Wrapping::default(),
            },
            bounds.center(),
            Color::WHITE,
            *_viewport,
        );
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        match event {
            //Event : Click on the widget
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let bounds = _layout.bounds();
                if cursor.is_over(bounds) {
                    _shell.publish(Message::StartStopPingButtonPressed);
                    return event::Status::Captured;
                }
                event::Status::Ignored
            }
            _ => event::Status::Ignored,
        }
    }
}

impl<'a, Renderer> From<PingButton> for Element<'a, crate::structs::app::Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn from(widget: PingButton) -> Self {
        Self::new(widget)
    }
}
