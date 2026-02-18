use gpui::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e2e2e))
            .size_full()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .text_xl()
            .child("Hello, Sublime-rust!")
    }
}

fn main() {
    Application::new().run(|cx| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.0), px(100.0)),
                size: size(px(800.0), px(600.0)),
            })),
            ..Default::default()
        };

        cx.open_window(options, |_, cx| {
            cx.new(|_| HelloWorld)
        })
        .expect("failed to open window");
    });
}
