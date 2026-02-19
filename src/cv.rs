#![allow(dead_code)]

use crate::app::Tab;
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span, Text},
    widgets::{Paragraph, Row, Table, Widget, WidgetRef},
};
use serde::Deserialize;
use serde_yaml;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub author: String,
    pub contact: Contact,
    pub languages: Vec<String>,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
    pub interests: Vec<Interest>,
}

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub email: String,
    pub phone: String,
    pub github: String,
    pub linkedin: String,
}

#[derive(Deserialize, Debug)]
pub struct Experience {
    pub what: String,
    pub r#where: String,
    pub start: String,
    pub end: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub what: String,
    pub desc: String,
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct Interest {
    pub what: String,
    pub desc: String,
}

impl Data {
    pub fn read(path: &str) -> Result<Data> {
        let file = fs::read_to_string(path)?;
        serde_yaml::from_str(&file).map_err(Into::into)
    }
}

pub struct CV {
    data: Data,
}

impl CV {
    pub fn new(data: Data) -> Self {
        Self { data }
    }
}

impl Tab for CV {
    fn title(&self) -> String {
        "CV".to_string()
    }

    fn color(&self) -> Color {
        Color::Blue
    }
}

impl WidgetRef for CV {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Fill, Length, Min};

        let [header, rest] = Layout::vertical([Length(2), Fill(1)]).areas(area);

        let rows = [
            Row::new([
                &self.data.author as &str,
                &self.data.contact.email,
                &self.data.contact.phone,
            ]),
            Row::new([
                "Curriculum Vitae",
                &self.data.contact.github,
                &self.data.contact.linkedin,
            ]),
        ];

        Table::new(rows, [Fill(1), Min(0), Min(0)]).render(header, buf);

        let [left, right] = Layout::horizontal([Fill(1), Fill(1)]).areas(rest);

        self.data.experience.iter().fold(right, |acc, item| {
            let [top, rest] = Layout::vertical([Length(3), Fill(1)]).areas(acc);

            Text::from(vec![
                item.r#where.clone().into(),
                item.what.clone().into(),
                item.start.clone().into(),
            ])
            .render(top, buf);

            rest
        });
    }
}
