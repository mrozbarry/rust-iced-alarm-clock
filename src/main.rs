use iced::{Column, Text, Settings, Align, Element, Button, Sandbox, button};

pub fn main() -> iced::Result {
    Application::run(Settings::default())
}

struct Application {
    count: i32,
    message: String,
    button_state: button::State,
    noop_button_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPress,
    NoOp,
}

impl Sandbox for Application {
    type Message = Message;

    fn new() -> Application {
        Application {
            count: 0,
            message: "Press Me".to_string(),
            button_state: button::State::default(),
            noop_button_state: button::State::default(),
        }
    }

    fn title(&self) -> String {
        format!("Button Presser - {}", self.count)
    }

    fn update(
        &mut self,
        message: Message,
    ) {
        match message {
            Message::ButtonPress => {
                self.count += 1;
            },
            Message::NoOp => {
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .padding(20)
            .push(
                Button::new(&mut self.button_state, Text::new(&self.message.to_string())).on_press(Message::ButtonPress)
            )
            .push(
                Button::new(&mut self.noop_button_state, Text::new("This button does nothing")).on_press(Message::NoOp)
            )
            .into()
    }
}
