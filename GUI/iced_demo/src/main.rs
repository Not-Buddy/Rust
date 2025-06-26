mod calc;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Result, Theme, window, Size, Task};

#[derive(Debug, Clone)]
pub enum Message {
    NumberPressed(u8),
    OperatorPressed(Operator),
    EqualsPressed,
    ClearPressed,
    DecimalPressed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn symbol(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "×",
            Operator::Divide => "÷",
        }
    }
}

#[derive(Debug, Default)]
pub struct Calculator {
    pub display: String,
    pub first_operand: Option<f64>,
    pub operator: Option<Operator>,
    pub waiting_for_operand: bool,
}

impl Calculator {
    fn new() -> Self {
        Self {
            display: "0".to_string(),
            first_operand: None,
            operator: None,
            waiting_for_operand: false,
        }
    }

    fn input_number(&mut self, number: u8) {
        if self.waiting_for_operand || self.display == "0" {
            self.display = number.to_string();
            self.waiting_for_operand = false;
        } else {
            self.display.push_str(&number.to_string());
        }
    }

    fn input_decimal(&mut self) {
        if self.waiting_for_operand {
            self.display = "0.".to_string();
            self.waiting_for_operand = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    fn input_operator(&mut self, operator: Operator) {
        if let Ok(current) = self.display.parse::<f64>() {
            if let Some(first) = self.first_operand {
                if let Some(op) = self.operator {
                    let result = self.calculate(first, current, op);
                    self.display = self.format_result(result);
                    self.first_operand = Some(result);
                } else {
                    self.first_operand = Some(current);
                }
            } else {
                self.first_operand = Some(current);
            }
        }

        self.operator = Some(operator);
        self.waiting_for_operand = true;
    }

    fn calculate(&self, first: f64, second: f64, operator: Operator) -> f64 {
        match operator {
            Operator::Add => first + second,
            Operator::Subtract => first - second,
            Operator::Multiply => first * second,
            Operator::Divide => {
                if second != 0.0 {
                    first / second
                } else {
                    0.0
                }
            }
        }
    }

    fn input_equals(&mut self) {
        if let (Some(first), Some(operator)) = (self.first_operand, self.operator) {
            if let Ok(second) = self.display.parse::<f64>() {
                let result = self.calculate(first, second, operator);
                self.display = self.format_result(result);
                self.first_operand = None;
                self.operator = None;
                self.waiting_for_operand = true;
            }
        }
    }

    fn format_result(&self, result: f64) -> String {
        if result.fract() == 0.0 && result.abs() < 1e15 {
            format!("{}", result as i64)
        } else {
            let formatted = format!("{:.10}", result);
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.first_operand = None;
        self.operator = None;
        self.waiting_for_operand = false;
    }
}

// THIS IS THE CRITICAL FUNCTION - MAKE SURE IT'S HERE IN MAIN.RS
fn update(calculator: &mut Calculator, message: Message) -> Task<Message> {
    println!("🔥 Received message: {:?}", message); // Debug line
    
    match message {
        Message::NumberPressed(number) => {
            println!("🔢 Number pressed: {}", number);
            calculator.input_number(number);
        },
        Message::OperatorPressed(operator) => {
            println!("➕ Operator pressed: {:?}", operator);
            calculator.input_operator(operator);
        },
        Message::EqualsPressed => {
            println!("🟰 Equals pressed");
            calculator.input_equals();
        },
        Message::ClearPressed => {
            println!("🗑️ Clear pressed");
            calculator.clear();
        },
        Message::DecimalPressed => {
            println!("🔘 Decimal pressed");
            calculator.input_decimal();
        },
    }
    
    println!("📟 Display after update: '{}'", calculator.display);
    Task::none()
}

fn view(calculator: &Calculator) -> Element<Message> {
    calc::view(calculator)
}

fn main() -> Result {
    iced::application("Calculator", update, view)
        .theme(|_state: &Calculator| Theme::Light)
        .window(window::Settings {
            size: Size::new(350.0, 500.0),
            position: window::Position::Centered,
            resizable: false,
            ..Default::default()
        })
        .run_with(|| (Calculator::new(), Task::none()))
}
