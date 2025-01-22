use combine::{
    any,
    parser::{sequence, token::Token},
    token, EasyParser, Parser,
};
use pulldown_cmark::Options;
#[cfg(test)]
mod tests {
    use combine::satisfy;

    use super::*;

    #[test]
    fn from_todo_file() {
        let input = include_str!("../TODO.md");
        in_n_out(&input);
    }

    #[test]
    fn from_task() {
        let input = "* [ ] Sign the timesheet";
        in_n_out(input);
    }

    fn in_n_out(input: &str) {
        let events =
            pulldown_cmark::Parser::new_ext(input, Options::ENABLE_TASKLISTS).collect::<Vec<_>>();
        println!("{:#?}", events);
        let mut back = String::new();
        pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut back).unwrap();
        println!("{}", back);
    }

    #[test]
    fn test() {
        let board = Board::new()
            .add(Task::new("Sign the timesheet").unchecked())
            .add(Task::new("Learn how to use pulldown-cmark").checked())
            .add(Task::new("Push the current state to origin").checked())
            .add(Task::new("Make it read a flie from inside Obsidian"));

        let json = serde_json::to_string_pretty(&board).unwrap();
        println!("Serde_json: {json}");

        let events: Vec<Event> = board.clone().into();

        let markdown = events_to_markdown(events);
        println!("Markdown:\n{markdown}");

        let board = board.log_checked();
        let json = serde_json::to_string_pretty(&board).unwrap();
        println!("Serde_json: {json}");

        let events: Vec<Event> = board.into();

        let markdown = events_to_markdown(events);
        println!("Markdown:\n{markdown}");
    }

    #[test]
    fn combinator() {
        let input = [
            Event::Start(Tag::Item),
            Event::TaskListMarker(false),
            Event::Text(CowStr::from("Sign the timesheet")),
            Event::End(TagEnd::Item),
        ];
        let parsed = token(Event::Start(Tag::Item))
            .with(satisfy(|event| matches!(event, Event::TaskListMarker(_))).map(|event| {
                if let Event::TaskListMarker(done) = event {
                    done
                } else {
                    panic!("Unexpected event in task done");
                }
            }))
            .parse(input.as_slice());
        println!("{:?}", parsed);
    }
}

use itertools::{Either, Itertools};
use pulldown_cmark::{CowStr, Event, Tag, TagEnd};

impl<'a> From<Task> for Vec<Event<'a>> {
    fn from(task: Task) -> Self {
        let name = task.name;
        vec![
            Event::Start(Tag::Item),
            Event::TaskListMarker(task.checked),
            Event::Text(CowStr::from(name)),
            Event::End(TagEnd::Item),
        ]
    }
}

impl<'a> From<Board> for Vec<Event<'a>> {
    fn from(board: Board) -> Self {
        let mut events = vec![Event::Start(Tag::List(None))];
        for task in board.tasks {
            events.extend_from_slice(&Vec::<Event<'a>>::from(task));
        }
        events.push(Event::End(TagEnd::List(false)));
        events
    }
}

pub(crate) fn events_to_markdown<'e, I: IntoIterator<Item = Event<'e>>>(events: I) -> String {
    let mut string = String::new();
    let _: pulldown_cmark_to_cmark::State =
        pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut string)
            .expect("error converting Event sequent to Markdown string");

    string
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Board {
    tasks: Vec<Task>,
    log: Vec<Task>,
}

impl Board {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }

    pub fn log_checked(self) -> Self {
        let (tasks, mut log): (Vec<_>, Vec<_>) = self.tasks.into_iter().partition_map(|task| {
            if task.checked {
                Either::Right(task)
            } else {
                Either::Left(task)
            }
        });
        log.extend(self.log);
        Self { tasks, log }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    name: String,
    checked: bool,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            checked: false,
        }
    }

    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub fn unchecked(mut self) -> Self {
        self.checked = false;
        self
    }

    pub(crate) fn from_markdown(src: &str) -> Option<Self> {
        let events =
            pulldown_cmark::Parser::new_ext(src, Options::ENABLE_TASKLISTS).collect::<Vec<_>>();
        let task = events.windows(4).find(|window| {
            if window[0] != Event::Start(Tag::Item) {
                return false;
            }
            if let Event::TaskListMarker(_) = window[1] {
            } else {
                return false;
            };
            if let Event::Text(CowStr::Borrowed(_)) = window[2] {
            } else {
                return false;
            };
            if let Event::End(TagEnd::Item) = window[3] {
            } else {
                return false;
            };
            true
        });
        task.map(|task| {
            let name = if let Event::Text(name) = &task[2] {
                name.to_string()
            } else {
                panic!("Unexpected event in task name");
            };
            let done = if let Event::TaskListMarker(done) = &task[1] {
                *done
            } else {
                panic!("Unexpected event in task done");
            };
            Self {
                name,
                checked: done,
            }
        })
    }
}

fn parse<'a>(src: pulldown_cmark::Parser<'a>) {}
