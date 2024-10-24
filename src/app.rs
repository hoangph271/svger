// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::fl;
use cosmic::app::{Command, Core, Message};
use cosmic::iced::{self, event, Alignment, Event, Length};
use cosmic::iced_runtime::window;
use cosmic::widget::{self, menu};
use cosmic::{cosmic_theme, style, theme, Application, ApplicationExt, Element};
use std::fs;
use std::path::{Path, PathBuf};

const REPOSITORY: &str = "https://github.com/edfloreshz/cosmic-app-template";
const SVG_DIR: &str = "";
const GRID_ITEM_WIDTH: usize = 256;

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct Svger {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    svg_files: Vec<PathBuf>,
    grid_rows_count: Option<usize>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum SvgerMessage {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),
    UpdateGridRowsCount(Option<usize>),
}

/// Identifies a context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = SvgerMessage;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => SvgerMessage::ToggleContextPage(ContextPage::About),
        }
    }
}

fn list_svg_files(dir: &str) -> Vec<PathBuf> {
    let path = Path::new(dir);
    let mut svg_files = Vec::new();

    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "svg") {
                    svg_files.push(path);
                }
            }
        }
    }

    svg_files
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the async executor that will be used to run your application's commands.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for Svger {
    type Executor = cosmic::executor::Default;

    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

    type Flags = ();

    type Message = SvgerMessage;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = Svger {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            svg_files: list_svg_files(SVG_DIR),
            grid_rows_count: None,
        };

        let command = Command::batch([app.update_titles(), app.update_grid_rows_count()]);

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        let Some(grid_rows_count) = self.grid_rows_count else {
            return widget::text::caption(fl!("welcome")).into();
        };

        let mut svg_grid = widget::grid();

        let mut row_count = 0;

        for path in self.svg_files.iter() {
            svg_grid = svg_grid.push(
                widget::column()
                    .push(
                        widget::svg(widget::svg::Handle::from_path(path))
                            .width(96)
                            .height(96),
                    )
                    .push(widget::text::caption(
                        path.file_name().unwrap().to_str().unwrap(),
                    ))
                    .spacing(8)
                    .align_items(Alignment::Center),
            );

            row_count += 1;

            if row_count == grid_rows_count {
                svg_grid = svg_grid.insert_row();
                row_count = 0;
            }
        }

        widget::container(widget::scrollable(
            svg_grid
                .column_alignment(Alignment::Center)
                .row_alignment(Alignment::Center)
                .row_spacing(16)
                .column_spacing(8)
                .width(Length::Fill),
        ))
        .width(Length::Fill)
        .into()
    }

    /// Application messages are handled here. The application state can be modified based on
    /// what message was received. Commands may be returned for asynchronous execution on a
    /// background thread managed by the application's executor.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            SvgerMessage::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }
            SvgerMessage::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }

                // Set the title of the context drawer.
                self.set_context_title(context_page.title());
            }
            SvgerMessage::UpdateGridRowsCount(grid_rows_count) => {
                self.grid_rows_count = grid_rows_count;
            }
        }
        Command::none()
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
        })
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        event::listen_with(|message, _| match message {
            Event::Window(window_id, window_event) => {
                if window_id == window::Id::MAIN {
                    match window_event {
                        iced::window::Event::Resized { width, height: _ } => {
                            Some(SvgerMessage::UpdateGridRowsCount(Some(
                                width as usize / GRID_ITEM_WIDTH,
                            )))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
    }
}

impl Svger {
    /// The about page for this app.
    pub fn about(&self) -> Element<SvgerMessage> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/128x128/apps/com.example.CosmicAppTemplate.svg")
                [..],
        ));

        let title = widget::text::title3(fl!("app-title"));

        let link = widget::button::link(REPOSITORY)
            .on_press(SvgerMessage::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_titles(&mut self) -> Command<SvgerMessage> {
        let window_title = fl!("app-title");

        self.set_window_title(window_title)
    }

    pub fn update_grid_rows_count(&mut self) -> Command<SvgerMessage> {
        window::fetch_size(window::Id::MAIN, move |size| {
            let grid_rows_count = size.width as usize / GRID_ITEM_WIDTH;

            grid_rows_count
        })
        .map(|grid_rows_count| {
            Message::from(SvgerMessage::UpdateGridRowsCount(Some(grid_rows_count)))
        })
    }
}
