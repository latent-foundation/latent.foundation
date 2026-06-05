use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Active,
    Research,
    Archived,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Status::Active => "ACTIVE",
            Status::Research => "RESEARCH",
            Status::Archived => "ARCHIVED",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Project {
    pub id: &'static str,
    pub splash_text: &'static str,
    pub status: Status,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub year: u16,
}

pub static PROJECTS: &[Project] = &[
    Project {
        id: "ido",
        splash_text: "井戸",
        status: Status::Active,
        description: "A quiet local-first knowledge system built around depth, memory, and long-term organization.",
        tags: &["KNOWLEDGE", "LOCAL-FIRST", "RUST"],
        year: 2026,
    },
    Project {
        id: "logos",
        splash_text: "λόγος",
        status: Status::Research,
        description: "A research system for reasoning under uncertainty.",
        tags: &["MARKETS", "BACKTESTING", "RUST"],
        year: 2026,
    },
    Project {
        id: "proteus",
        splash_text: "",
        status: Status::Archived,
        description: "Change-point detection via a Markov-switching model for commodities.",
        tags: &["MARKOV", "CHANGE-POINT", "THESIS", "RUST"],
        year: 2025,
    },
];
