use crate::{Calculator, Message, Operator};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text};
use iced::{Background, Border, Color, Element, Length, Shadow, Theme, Vector, Task};


pub fn view(calculator: &Calculator) -> Element<'_, Message> {
    let display = create_display(calculator);
    let button_grid = create_button_grid();

    container(
        column![display, button_grid]
            .spacing(20)
            .padding(25)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_theme| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.12))),
        border: Border {
            color: Color::from_rgb(0.3, 0.3, 0.35),
            width: 1.0,
            radius: 15.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 20.0,
        },
        text_color: None,
    })
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn create_display(calculator: &Calculator) -> Element<'_, Message> {
    container(
        text(&calculator.display)
            .size(48)
            .color(Color::BLACK)
            .align_x(Horizontal::Right)
    )
    .width(Length::Fill)
    .height(Length::Fixed(90.0))
    .center_y(Length::Fill)
    .into()
}





fn create_button_grid() -> Element<'static, Message> {
    column![
        // Row 1: Clear and operators
        row![
            special_button("C", Message::ClearPressed, ButtonType::Clear),
            empty_space(),
            empty_space(),
            operator_button(Operator::Divide),
        ]
        .spacing(12),
        
        // Row 2: 7, 8, 9, ×
        row![
            number_button(7),
            number_button(8),
            number_button(9),
            operator_button(Operator::Multiply),
        ]
        .spacing(12),
        
        // Row 3: 4, 5, 6, −
        row![
            number_button(4),
            number_button(5),
            number_button(6),
            operator_button(Operator::Subtract),
        ]
        .spacing(12),
        
        // Row 4: 1, 2, 3, +
        row![
            number_button(1),
            number_button(2),
            number_button(3),
            operator_button(Operator::Add),
        ]
        .spacing(12),
        
        // Row 5: 0, ., =
        row![
            number_button_wide(0),
            special_button(".", Message::DecimalPressed, ButtonType::Function),
            special_button("=", Message::EqualsPressed, ButtonType::Equals),
        ]
        .spacing(12),
    ]
    .spacing(12)
    .into()
}

#[derive(Clone, Copy)]
enum ButtonType {
    Number,
    Operator,
    Function,
    Clear,
    Equals,
}

impl ButtonType {
    fn colors(&self) -> (Color, Color, Color) {
        match self {
            ButtonType::Number => (
                Color::from_rgb(0.2, 0.2, 0.25),     // Normal
                Color::from_rgb(0.25, 0.25, 0.3),    // Hover
                Color::from_rgb(0.15, 0.15, 0.2),    // Pressed
            ),
            ButtonType::Operator => (
                Color::from_rgb(1.0, 0.58, 0.0),     // Orange
                Color::from_rgb(1.0, 0.65, 0.15),    // Lighter orange
                Color::from_rgb(0.9, 0.52, 0.0),     // Darker orange
            ),
            ButtonType::Function => (
                Color::from_rgb(0.65, 0.65, 0.7),    // Light gray
                Color::from_rgb(0.7, 0.7, 0.75),     // Lighter
                Color::from_rgb(0.6, 0.6, 0.65),     // Darker
            ),
            ButtonType::Clear => (
                Color::from_rgb(0.8, 0.3, 0.3),      // Red
                Color::from_rgb(0.85, 0.4, 0.4),     // Lighter red
                Color::from_rgb(0.7, 0.25, 0.25),    // Darker red
            ),
            ButtonType::Equals => (
                Color::from_rgb(0.0, 0.7, 0.4),      // Green
                Color::from_rgb(0.1, 0.75, 0.45),    // Lighter green
                Color::from_rgb(0.0, 0.65, 0.35),    // Darker green
            ),
        }
    }

    fn text_color(&self) -> Color {
        match self {
            ButtonType::Function => Color::BLACK,
            _ => Color::WHITE,
        }
    }
}

fn styled_button(label: String, msg: Message, button_type: ButtonType) -> Element<'static, Message> {
    let (normal_color, hover_color, pressed_color) = button_type.colors();
    
    button(
        text(label)
            .size(22)
            .color(button_type.text_color())
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
    )
    .width(Length::Fill)
    .height(Length::Fixed(65.0))
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => hover_color,
            button::Status::Pressed => pressed_color,
            _ => normal_color,
        })),
        text_color: button_type.text_color(),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 6.0,
        },
    })
    .on_press(msg)
    .into()
}

fn number_button(n: u8) -> Element<'static, Message> {
    styled_button(n.to_string(), Message::NumberPressed(n), ButtonType::Number)
}

fn number_button_wide(n: u8) -> Element<'static, Message> {
    let (normal_color, hover_color, pressed_color) = ButtonType::Number.colors();
    
    button(
        text(n.to_string())
            .size(22)
            .color(Color::WHITE)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
    )
    .width(Length::FillPortion(2)) // Takes up 2 units of space
    .height(Length::Fixed(65.0))
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => hover_color,
            button::Status::Pressed => pressed_color,
            _ => normal_color,
        })),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 6.0,
        },
    })
    .on_press(Message::NumberPressed(n))
    .into()
}

fn operator_button(op: Operator) -> Element<'static, Message> {
    styled_button(op.symbol().to_string(), Message::OperatorPressed(op), ButtonType::Operator)
}

fn special_button(label: &str, msg: Message, button_type: ButtonType) -> Element<'static, Message> {
    styled_button(label.to_string(), msg, button_type)
}

fn empty_space() -> Element<'static, Message> {
    container(text(""))
        .width(Length::Fill)
        .height(Length::Fixed(65.0))
        .into()
}

fn update(calculator: &mut Calculator, message: Message) -> Task<Message> {
    println!("Received message: {:?}", message); // Debug line
    
    match message {
        Message::NumberPressed(number) => {
            println!("Number pressed: {}", number); // Debug line
            calculator.input_number(number);
        },
        Message::OperatorPressed(operator) => {
            println!("Operator pressed: {:?}", operator); // Debug line
            calculator.input_operator(operator);
        },
        Message::EqualsPressed => {
            println!("Equals pressed"); // Debug line
            calculator.input_equals();
        },
        Message::ClearPressed => {
            println!("Clear pressed"); // Debug line
            calculator.clear();
        },
        Message::DecimalPressed => {
            println!("Decimal pressed"); // Debug line
            calculator.input_decimal();
        },
    }
    
    println!("Display after update: '{}'", calculator.display); // Debug line
    Task::none()
}
