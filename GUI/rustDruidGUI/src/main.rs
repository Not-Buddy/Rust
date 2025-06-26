use druid::widget::{Label, Button, Flex};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui)
        .title(LocalizedString::new("Rust GUI App"))
        .window_size((300.0, 150.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch("Hello, Druid!".to_string())?;

    Ok(())
}

fn build_ui() -> impl Widget<String> {
    let label = Label::new(|data: &String, _env: &druid::Env| data.to_string());
    let button = Button::new("Click me").on_click(|_ctx, data: &mut String, _env| {
        data.push_str(" Rust!");
    });
    let reset_button = Button::new("Reset Text").on_click(|_ctx, data: &mut String, _env| {
        *data = "Hello, Druid!".to_string();
    });

    Flex::column()
        .with_child(label)
        .with_spacer(10.0)
        .with_child(button)
        .with_child(reset_button)
}
