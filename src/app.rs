use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEvent},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols,
    widgets::{Block, Padding, Tabs, Widget, WidgetRef},
};

#[macro_export]
macro_rules! tabs {
    ($($x:expr),+ $(,)?) => {
        [
            $(
                ::std::boxed::Box::new($x) as ::std::boxed::Box<dyn crate::app::Tab>
            ),+
        ]
    };
}

pub trait Tab: WidgetRef {
    fn title(&self) -> String;
    fn color(&self) -> Color;
}

enum State {
    Running,
    Quitting,
    // Popup,
}

pub struct App<const N: usize> {
    state: State,
    selected_tab: usize,
    tabs: [Box<dyn Tab>; N],
    mouse: Option<MouseEvent>,
}

impl<const N: usize> App<N> {
    pub const fn new(tabs: [Box<dyn Tab>; N]) -> Self {
        assert!(
            N > 0,
            "No tabs were provided, but at least one tab is required"
        );

        Self {
            state: State::Running,
            selected_tab: 0,
            tabs,
            mouse: None,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while let State::Running = self.state {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('/') | KeyCode::Char('?') => self.quit(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = self
            .tabs
            .iter()
            .map(|tab| format!("  {}  ", tab.title()).bg(tab.color()));

        Tabs::new(titles)
            .highlight_style((Color::default(), self.current_tab().color()))
            .select(self.selected_tab)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn current_tab(&self) -> &Box<dyn Tab> {
        &self.tabs[self.selected_tab]
    }

    fn next_tab(&mut self) {
        if self.selected_tab < N - 1 {
            self.selected_tab += 1;
        }
    }

    fn previous_tab(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        }
    }

    fn quit(&mut self) {
        self.state = State::Quitting;
    }
}

impl<const N: usize> Widget for &App<N> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Max, Min};

        let [header_area, rest] =
            Layout::vertical([Length(1), Min(0)]).areas(area.centered_horizontally(Max(100)));

        let block = Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.current_tab().color());

        self.render_tabs(header_area, buf);
        self.current_tab().render_ref(block.inner(rest), buf);
        block.render(rest, buf);
    }
}
