use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length, Result, Settings, Theme, window, Size, Task};

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
        Message::Decrement => *value = value.saturating_sub(1),
        Message::Reset => *value = 0,
    }
}

fn view(value: &u64) -> Element<Message> {
    let counter_text = text(format!("{}", value))
        .size(80)
        .style(|_theme: &Theme| iced::widget::text::Style {
            color: Some(Color::from_rgb(0.3, 0.5, 0.8)),
            ..Default::default()
        });

    let increment_btn = button("+")
        .padding(20)
        .on_press(Message::Increment);

    let decrement_btn = button("-")
        .padding(20)
        .on_press(Message::Decrement);

    let reset_btn = button("Reset")
        .padding(10)
        .on_press(Message::Reset);

    // Layout
    let content = column![
        counter_text,
        row![increment_btn, decrement_btn]
            .spacing(30)
            .align_y(Alignment::Center),
        reset_btn
    ]
    .spacing(40)
    .align_x(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn main() -> Result {
    iced::application(
        "Fancy Counter App",
        update,
        view,
    )
    .theme(|_state: &u64| Theme::Dark)
    .window(window::Settings {
        size: Size::new(300.0, 300.0),
        position: window::Position::Centered,
        ..Default::default()
    })
    .run_with(|| (0, Task::none()))
}
