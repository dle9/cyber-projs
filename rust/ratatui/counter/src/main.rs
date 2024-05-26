mod app;
mod tui;

fn main() -> std::io::Result<()> {
    let mut terminal = tui::init()?;

    // run the app, capture status
    let app_result = app::App::default().run(&mut terminal);

    tui::restore()?;
    app_result
}