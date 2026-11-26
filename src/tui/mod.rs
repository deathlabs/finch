mod files;
mod view;

use std::collections::BTreeSet;
use std::error::Error;
use std::io::{self, IsTerminal};
use std::path::PathBuf;
use std::time::Duration;

use crossterm::terminal;
use rusty_bubbletea::key::{Key, KeyMod};
use rusty_bubbletea::model::Model;
use rusty_bubbletea::{
    Cmd, KeyPressMsg, MouseButton, MouseClickMsg, MouseWheelMsg, Msg, PasteMsg, Program, View,
    WindowSizeMsg, batch, quit, request_window_size, tick,
};
use uuid::Uuid;

use self::files::{Component, ExportDocument, Plan, ToolRules};

const DEFAULT_SSP_PATH: &str = "ssp.json";
const DEFAULT_TITLE: &str = "WAF";
const DEFAULT_DESCRIPTION: &str = "Web App Firewall";
const DEFAULT_PURPOSE: &str = "Filter web traffic";
const COMPONENT_TYPES: &[&str] = &["software", "service", "hardware"];
const SECURITY_TOOLS: &[&str] = &["ZAP", "Grype", "Semgrep"];
const ZAP_RULES: &[&str] = &[
    "10020", "10021", "10038", "10062", "10202", "40012", "40003", "40018", "90019", "90022",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Plan,
    Title,
    ComponentType,
    Description,
    Purpose,
    Controls,
    Tool,
    Rules,
    AnotherComponent,
    Output,
    Saved,
}

#[derive(Default)]
struct Selection {
    cursor: usize,
    selected: BTreeSet<String>,
}

impl Selection {
    fn toggle(&mut self, value: &str) {
        if !self.selected.remove(value) {
            self.selected.insert(value.to_owned());
        }
        return;
    }

    fn visible(&self, count: usize, height: usize, reserved: usize) -> std::ops::Range<usize> {
        let limit: usize = height.saturating_sub(reserved).max(1);
        let start: usize = (self.cursor + 1).saturating_sub(limit);
        return start..(start + limit).min(count);
    }
}

#[derive(Default)]
struct Completion {
    request: usize,
    matches: Vec<String>,
    index: usize,
}

struct TuiModel {
    step: Step,
    input: String,
    path: Option<PathBuf>,
    initial_path: Option<PathBuf>,
    plan: Option<Plan>,
    controls: Vec<String>,
    components: Vec<Component>,
    component: Component,
    tool: usize,
    component_type: usize,
    control_selection: Selection,
    zap_selection: Selection,
    zap_manual: bool,
    completion: Completion,
    message: String,
    busy: bool,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Loaded {
    path: PathBuf,
    result: Result<Plan, String>,
}

#[derive(Debug)]
struct Completed {
    request: usize,
    result: Result<Vec<String>, String>,
}

#[derive(Debug)]
struct Saved {
    path: PathBuf,
    result: Result<(), String>,
}

#[derive(Debug)]
struct ResizeCheck;

fn check_resize() -> Cmd {
    return tick(Duration::from_millis(300), |_| {
        return Some(Box::new(ResizeCheck));
    });
}

impl TuiModel {
    fn new(initial_path: Option<PathBuf>) -> Self {
        return Self {
            step: Step::Plan,
            input: String::new(),
            busy: initial_path.is_some(),
            initial_path,
            path: None,
            plan: None,
            controls: Vec::new(),
            components: Vec::new(),
            component: Component::default(),
            tool: 0,
            component_type: 0,
            control_selection: Selection::default(),
            zap_selection: Selection::default(),
            zap_manual: false,
            completion: Completion::default(),
            message: String::new(),
            width: 80,
            height: 24,
        };
    }

    fn load(path: PathBuf) -> Cmd {
        return Some(Box::new(move || {
            let result = files::load(&path).map_err(|error| {
                format!(
                    "Expected an OSCAL System Security Plan at {}: {error}",
                    path.display()
                )
            });
            return Some(Box::new(Loaded { path, result }));
        }));
    }

    fn complete_path(&mut self) -> Cmd {
        if !self.completion.matches.is_empty() {
            self.completion.index = (self.completion.index + 1) % self.completion.matches.len();
            self.input
                .clone_from(&self.completion.matches[self.completion.index]);
            return None;
        }

        self.completion.request += 1;
        let request: usize = self.completion.request;
        let prefix: String = self.input.clone();
        return Some(Box::new(move || {
            let result = files::completions(&prefix)
                .map_err(|error| format!("Could not list JSON files: {error}"));
            return Some(Box::new(Completed { request, result }));
        }));
    }

    fn clear_completions(&mut self) {
        self.completion.request += 1;
        self.completion.matches.clear();
        return;
    }

    fn reset_component(&mut self) {
        self.component = Component::default();
        self.component_type = 0;
        self.control_selection = Selection::default();
        self.tool = 0;
        self.reset_rule_selection();
        return;
    }

    fn reset_rule_selection(&mut self) {
        self.zap_selection = Selection::default();
        self.zap_manual = false;
        return;
    }

    fn enter(&mut self, step: Step, input: String) {
        self.step = step;
        self.input = input;
        self.message.clear();
        return;
    }

    fn back(&mut self) {
        let (step, input): (Step, String) = match self.step {
            Step::Title => (
                Step::Plan,
                self.path
                    .as_deref()
                    .map_or_else(String::new, |path| path.to_string_lossy().into_owned()),
            ),
            Step::ComponentType => (Step::Title, self.component.title.clone()),
            Step::Description => (Step::ComponentType, String::new()),
            Step::Purpose => (Step::Description, self.component.description.clone()),
            Step::Controls => (Step::Purpose, self.component.purpose.clone()),
            Step::Tool => (Step::Controls, String::new()),
            Step::Rules => (Step::Tool, String::new()),
            Step::AnotherComponent => (Step::Rules, String::new()),
            Step::Plan | Step::Output | Step::Saved => return,
        };
        self.enter(step, input);
        return;
    }

    fn submit(&mut self) -> Cmd {
        let input: &str = self.input.trim();
        self.message.clear();
        let next: Step = match self.step {
            Step::Plan => {
                self.busy = true;
                return Self::load(PathBuf::from(default_value(input, DEFAULT_SSP_PATH)));
            }
            Step::Title => {
                self.component.title = default_value(input, DEFAULT_TITLE).to_owned();
                Step::ComponentType
            }
            Step::ComponentType => {
                self.component.component_type = COMPONENT_TYPES[self.component_type].to_owned();
                Step::Description
            }
            Step::Description => {
                self.component.description = default_value(input, DEFAULT_DESCRIPTION).to_owned();
                Step::Purpose
            }
            Step::Purpose => {
                self.component.purpose = default_value(input, DEFAULT_PURPOSE).to_owned();
                Step::Controls
            }
            Step::Controls => {
                self.component.controls = self.control_selection.selected.iter().cloned().collect();
                Step::Tool
            }
            Step::Tool => Step::Rules,
            Step::Rules => {
                if let Err(error) = self.submit_rules() {
                    self.message = error.to_owned();
                    return None;
                }
                Step::AnotherComponent
            }
            Step::AnotherComponent => {
                let another: bool = match input.to_ascii_lowercase().as_str() {
                    "y" => true,
                    "" | "n" => false,
                    _ => {
                        self.message = "Enter y or n.".to_owned();
                        return None;
                    }
                };
                self.component.uuid = Uuid::new_v4().to_string();
                self.components.push(std::mem::take(&mut self.component));
                self.reset_component();
                if another {
                    Step::Title
                } else {
                    self.enter(Step::Output, "mappings.yaml".to_owned());
                    return None;
                }
            }
            Step::Output => {
                if input.is_empty() {
                    self.message = "Enter an output file path.".to_owned();
                    return None;
                }
                let Some(plan) = self.plan.as_ref() else {
                    self.message = "Load a System Security Plan before exporting.".to_owned();
                    return None;
                };
                let path: PathBuf = PathBuf::from(input);
                let document: ExportDocument = ExportDocument::new(plan, &self.components);
                self.busy = true;
                return Some(Box::new(move || {
                    let result = files::save(&path, &document)
                        .map_err(|error| format!("Could not save {}: {error}", path.display()));
                    return Some(Box::new(Saved { path, result }));
                }));
            }
            Step::Saved => return None,
        };
        self.enter(next, String::new());
        return None;
    }

    fn submit_rules(&mut self) -> Result<(), &'static str> {
        let tool: &str = SECURITY_TOOLS[self.tool];
        let mut rules: Vec<String> = self
            .input
            .split(',')
            .map(str::trim)
            .filter(|rule| !rule.is_empty())
            .map(str::to_owned)
            .collect();

        if tool == "ZAP" {
            if !rules
                .iter()
                .all(|rule| rule.bytes().all(|byte| byte.is_ascii_digit()))
            {
                return Err("ZAP rule IDs must contain digits only.");
            }
            rules.extend(self.zap_selection.selected.iter().cloned());
            rules.sort();
            rules.dedup();
        }
        if rules.is_empty() {
            return Err("Select or enter at least one rule ID.");
        }
        self.component.rules.push(ToolRules {
            tool: tool.to_owned(),
            rules,
        });
        self.reset_rule_selection();
        return Ok(());
    }

    fn is_zap_step(&self) -> bool {
        return self.step == Step::Rules && SECURITY_TOOLS[self.tool] == "ZAP";
    }

    fn accepts_text(&self) -> bool {
        return match self.step {
            Step::ComponentType | Step::Controls | Step::Tool | Step::Saved => false,
            Step::Rules => !self.is_zap_step() || self.zap_manual,
            _ => true,
        };
    }

    fn append_text(&mut self, text: &str) {
        self.clear_completions();
        self.input
            .extend(text.chars().filter(|character| !character.is_control()));
        return;
    }

    fn handle_key(&mut self, key: &Key) -> Cmd {
        let shortcut: String = key.keystroke();
        if shortcut == "ctrl+c" {
            return quit();
        }
        if self.step == Step::Saved && shortcut == "ctrl+r" {
            let (width, height): (usize, usize) = (self.width, self.height);
            *self = Self::new(None);
            self.width = width;
            self.height = height;
            return None;
        }
        if self.busy || self.step == Step::Saved {
            return None;
        }

        match shortcut.as_str() {
            "enter" => {
                self.clear_completions();
                return self.submit();
            }
            "esc" => self.back(),
            "ctrl+t" if self.step == Step::AnotherComponent => {
                self.tool = 0;
                self.enter(Step::Tool, String::new());
            }
            "ctrl+d" if self.step == Step::Tool => {
                self.enter(Step::AnotherComponent, String::new());
            }
            "tab" if self.step == Step::Plan => return self.complete_path(),
            "tab" if self.is_zap_step() => self.zap_manual = !self.zap_manual,
            "up" | "down" => self.move_cursor(shortcut == "down"),
            "space" if self.step == Step::Controls => {
                if let Some(control) = self.controls.get(self.control_selection.cursor) {
                    self.control_selection.toggle(control);
                }
            }
            "space" if self.is_zap_step() && !self.zap_manual => {
                self.zap_selection
                    .toggle(ZAP_RULES[self.zap_selection.cursor]);
            }
            "backspace" | "ctrl+h" if self.accepts_text() => {
                self.clear_completions();
                self.input.pop();
            }
            "space" if self.accepts_text() => self.append_text(" "),
            _ if self.accepts_text()
                && !key.mod_keys.contains(KeyMod::CTRL)
                && !key.mod_keys.contains(KeyMod::ALT)
                && !key.mod_keys.contains(KeyMod::META)
                && !key.text.is_empty() =>
            {
                self.append_text(&key.text)
            }
            _ => {}
        }
        return None;
    }

    fn move_cursor(&mut self, down: bool) {
        let (cursor, count): (&mut usize, usize) = match self.step {
            Step::ComponentType => (&mut self.component_type, COMPONENT_TYPES.len()),
            Step::Controls => (&mut self.control_selection.cursor, self.controls.len()),
            Step::Tool => (&mut self.tool, SECURITY_TOOLS.len()),
            Step::Rules if self.is_zap_step() && !self.zap_manual => {
                (&mut self.zap_selection.cursor, ZAP_RULES.len())
            }
            _ => return,
        };
        if count > 0 {
            *cursor = if down {
                (*cursor + 1) % count
            } else {
                (*cursor + count - 1) % count
            };
        }
        return;
    }
}

impl Model for TuiModel {
    fn init(&self) -> Cmd {
        return batch(vec![
            self.initial_path.clone().and_then(Self::load),
            check_resize(),
        ]);
    }

    fn update(&mut self, message: &dyn Msg) -> Cmd {
        let message = message.as_any();
        if let Some(key_press_msg) = message.downcast_ref::<KeyPressMsg>() {
            return self.handle_key(&key_press_msg.0);
        }
        // Bubble Tea queries the size at startup. Check for later changes to the terminal.
        if message.is::<ResizeCheck>() {
            if let Ok((width, height)) = terminal::size()
                && width > 0
                && height > 0
                && (usize::from(width), usize::from(height)) != (self.width, self.height)
            {
                return batch(vec![request_window_size(), check_resize()]);
            }
            return check_resize();
        }
        if let Some(window_size_msg) = message.downcast_ref::<WindowSizeMsg>() {
            if window_size_msg.width > 0 && window_size_msg.height > 0 {
                self.width = window_size_msg.width;
                self.height = window_size_msg.height;
            }
        } else if let Some(loaded) = message.downcast_ref::<Loaded>() {
            self.busy = false;
            match &loaded.result {
                Ok(plan) => {
                    if self.path.as_ref().is_some_and(|path| path != &loaded.path) {
                        self.reset_component();
                        self.components.clear();
                    }
                    self.controls = plan.controls();
                    self.control_selection
                        .selected
                        .retain(|control| self.controls.binary_search(control).is_ok());
                    self.control_selection.cursor = self
                        .control_selection
                        .cursor
                        .min(self.controls.len().saturating_sub(1));
                    self.plan = Some(plan.clone());
                    self.path = Some(loaded.path.clone());
                    self.enter(Step::Title, self.component.title.clone());
                }
                Err(error) => {
                    self.input = loaded.path.to_string_lossy().into_owned();
                    self.message.clone_from(error);
                }
            }
        } else if let Some(completed) = message.downcast_ref::<Completed>()
            && self.step == Step::Plan
            && !self.busy
            && self.completion.request == completed.request
        {
            match &completed.result {
                Ok(matches) if !matches.is_empty() => {
                    self.input.clone_from(&matches[0]);
                    self.completion.matches.clone_from(matches);
                    self.completion.index = 0;
                    self.message.clear();
                }
                Ok(_) => self.message = "No matching JSON files found.".to_owned(),
                Err(error) => self.message.clone_from(error),
            }
        } else if let Some(saved) = message.downcast_ref::<Saved>() {
            self.busy = false;
            match &saved.result {
                Ok(()) => {
                    self.enter(Step::Saved, String::new());
                    self.message = format!("Saved {}", saved.path.display());
                }
                Err(error) => self.message.clone_from(error),
            }
        } else if !self.busy
            && self.accepts_text()
            && let Some(paste_msg) = message.downcast_ref::<PasteMsg>()
        {
            self.append_text(&paste_msg.content);
        } else if self.step == Step::Controls {
            if let Some(mouse_wheel_msg) = message.downcast_ref::<MouseWheelMsg>() {
                match mouse_wheel_msg.0.button {
                    MouseButton::MouseWheelUp => self.move_cursor(false),
                    MouseButton::MouseWheelDown => self.move_cursor(true),
                    _ => {}
                }
            } else if let Some(mouse_click_msg) = message.downcast_ref::<MouseClickMsg>()
                && mouse_click_msg.0.button == MouseButton::MouseLeft
                && let Some(index) = self.control_at_row(mouse_click_msg.0.y)
            {
                self.control_selection.cursor = index;
                self.control_selection.toggle(&self.controls[index]);
            }
        }
        return None;
    }

    fn view(&self) -> View {
        return self.render();
    }
}

fn default_value<'a>(value: &'a str, default: &'a str) -> &'a str {
    if value.is_empty() {
        return default;
    }
    return value;
}

pub(super) fn run(path: Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("expected an interactive terminal for input and output".into());
    }

    Program::new(TuiModel::new(path)).run()?;
    return Ok(());
}
