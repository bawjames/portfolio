use crate::app::App;
use color_eyre::Result;

mod app;
mod cv;
mod overview;

fn main() -> Result<()> {
    color_eyre::install()?;

    let data = cv::Data::read("cv.yml")?;

    let terminal = ratatui::init();
    let tabs = tabs![overview::Overview {}, cv::CV::new(data)];
    // let tabs = [];
    let app_result = App::new(tabs).run(terminal);

    ratatui::restore();
    app_result
}
