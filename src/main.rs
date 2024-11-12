use iced::widget::{button, column, container, text};
use iced::{Element, Fill, Theme};

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn title(&self) -> String {
        String::from("Counter")
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("Counter").size(100);

        let increment_btn = button(text("+").size(25))
            .padding(10)
            .on_press(Message::Increment);

        let label = text(self.value).size(50);

        let decrement_btn = button(text("-").size(25))
            .padding(10)
            .on_press(Message::Decrement);

        let content = column![increment_btn, label, decrement_btn]
            .spacing(10)
            .padding(10);

        container(column![title, content])
            .padding(10)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(Counter::title, Counter::update, Counter::view)
        .theme(Counter::theme)
        .centered()
        .run()
}
