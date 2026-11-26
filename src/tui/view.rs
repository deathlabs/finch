use rusty_bubbletea::view::{MouseMode, View};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use super::{
    COMPONENT_TYPES, DEFAULT_DESCRIPTION, DEFAULT_PURPOSE, DEFAULT_SSP_PATH, DEFAULT_TITLE,
    SECURITY_TOOLS, Selection, Step, TuiModel, ZAP_RULES,
};

const STAGES: &[&str] = &[
    "Load System Security Plan (SSP)",
    "Create a Component",
    "Map Controls to Component",
    "Map Rules to Controls",
    "Export Mappings",
];

#[derive(Clone, Copy)]
enum Style {
    Plain,
    Accent,
    AccentText,
    Step(usize),
    Detail,
    Error,
    Success,
}

#[derive(Default)]
struct Screen {
    lines: Vec<String>,
    control_rows: Vec<(usize, usize)>,
}

impl Screen {
    fn line(&mut self, model: &TuiModel, text: &str, mut style: Style) {
        let mut line: String = String::new();
        let mut width: usize = 0;
        for character in text.chars().map(|character| {
            if character.is_control() {
                return '�';
            }
            return character;
        }) {
            let character_width: usize = character.width().unwrap_or(0);
            if width + character_width > model.width && !line.is_empty() {
                self.lines.push(paint(&line, style));
                if let Style::Step(prefix) = style {
                    style = Style::Step(prefix.saturating_sub(line.len()));
                }
                line.clear();
                width = 0;
            }
            line.push(character);
            width += character_width;
        }
        self.lines.push(paint(&line, style));
        return;
    }

    fn choices(&mut self, model: &TuiModel, choices: &[&str], cursor: usize) {
        for (index, choice) in choices.iter().enumerate() {
            let prefix: &str = if cursor == index { "> " } else { "  " };
            self.line(model, &format!("{prefix}{choice}"), Style::Plain);
        }
        return;
    }

    fn checklist(&mut self, model: &TuiModel) {
        let is_controls: bool = model.step == Step::Controls;
        let (count, selection, label, reserved): (usize, &Selection, &str, usize) = if is_controls {
            (
                model.controls.len(),
                &model.control_selection,
                "Controls",
                15,
            )
        } else {
            (ZAP_RULES.len(), &model.zap_selection, "Rules", 13)
        };
        if count == 0 {
            self.line(model, "No controls found in the SSP.", Style::Detail);
            return;
        }
        let visible: std::ops::Range<usize> = selection.visible(count, model.height, reserved);
        for index in visible.clone() {
            let item: &str = if is_controls {
                &model.controls[index]
            } else {
                ZAP_RULES[index]
            };
            let prefix: &str = if selection.cursor == index && !model.is_zap_manual() {
                "> "
            } else {
                "  "
            };
            let marker: &str = if selection.selected.contains(item) {
                "[x]"
            } else {
                "[ ]"
            };
            let row: usize = self.lines.len();
            self.line(model, &format!("{prefix}{marker} {item}"), Style::Plain);
            if is_controls {
                self.control_rows
                    .extend((row..self.lines.len()).map(|row| (row, index)));
            }
        }
        if visible.start > 0 || visible.end < count {
            self.line(
                model,
                &format!("{label} {}-{} of {}", visible.start + 1, visible.end, count),
                Style::Detail,
            );
        }
        return;
    }
}

impl TuiModel {
    pub(super) fn render(&self) -> View {
        let screen: Screen = self.screen();
        let mut view: View = View::new(&screen.lines.join("\n"));
        view.alt_screen = true;
        view.mouse_mode = MouseMode::MouseModeCellMotion;
        return view;
    }

    pub(super) fn control_at_row(&self, row: usize) -> Option<usize> {
        return self
            .screen()
            .control_rows
            .into_iter()
            .find_map(|(line, index)| {
                return (line == row).then_some(index);
            });
    }

    fn screen(&self) -> Screen {
        let mut screen: Screen = Screen::default();
        screen.line(self, "Finch", Style::Accent);
        screen.line(self, "", Style::Plain);
        self.progress(&mut screen);
        screen.line(self, "", Style::Plain);

        if let Some((prompt, style)) = self.prompt() {
            screen.line(self, &prompt, style);
            self.render_input(&mut screen);
            if !self.message.is_empty() {
                screen.line(self, "", Style::Plain);
                screen.line(self, &self.message, Style::Error);
            }
        } else {
            screen.line(self, &self.message, Style::Success);
        }

        let title: &str = self
            .plan
            .as_ref()
            .map_or("None", |plan| plan.metadata.title.as_str());
        let status: String = format!(
            "SSP: {title} | Components Created: {}",
            self.components.len()
        );
        screen.line(self, "", Style::Plain);
        screen.line(self, &status, Style::Detail);
        screen.line(self, "", Style::Plain);
        screen.line(self, self.help(), Style::Detail);
        return screen;
    }

    fn progress(&self, screen: &mut Screen) {
        let stage: usize = match self.step {
            Step::Plan => 0,
            Step::Title | Step::ComponentType | Step::Description | Step::Purpose => 1,
            Step::Controls => 2,
            Step::Tool | Step::Rules | Step::AnotherComponent => 3,
            Step::Output | Step::Saved => 4,
        };

        let prefix: String = format!("Part {}.", stage + 1);
        let active_label: String = format!("{prefix} {}", STAGES[stage]);

        let labels: Vec<String> = STAGES
            .iter()
            .enumerate()
            .map(|(index, label)| {
                let marker: &str = if index <= stage { "●" } else { "○" };
                let label: &str = if index == stage { &active_label } else { label };
                return format!("{marker} {label}");
            })
            .collect();
        if labels.join("  ").width() > self.width {
            screen.line(self, &active_label, Style::Step(prefix.len()));
            return;
        }

        let labels: Vec<String> = labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                if index == stage {
                    let (marker, step_label): (&str, &str) = label.split_at("● ".len());
                    return paint(marker, Style::AccentText)
                        + &paint(step_label, Style::Step(prefix.len()));
                }
                let style: Style = if index <= stage {
                    Style::Accent
                } else {
                    Style::Detail
                };
                return paint(label, style);
            })
            .collect();
        screen.lines.push(labels.join("  "));
        return;
    }

    fn prompt(&self) -> Option<(String, Style)> {
        let rules_prompt: String;
        let (number, title): (usize, &str) = match self.step {
            Step::Plan => (1, "Enter the file path to your SSP."),
            Step::Title => (1, "Enter a component title."),
            Step::ComponentType => (2, "Select a component type."),
            Step::Description => (3, "Enter a component description."),
            Step::Purpose => (4, "Enter a component purpose."),
            Step::Controls => (1, "Select the controls implemented by the component."),
            Step::Tool => (1, "Select a scanner."),
            Step::Rules if self.is_zap_step() => (2, "Select one or more ZAP rules."),
            Step::Rules => {
                rules_prompt = format!(
                    "Enter rules for {} (comma separated).",
                    SECURITY_TOOLS[self.tool]
                );
                (2, &rules_prompt)
            }
            Step::AnotherComponent => (3, "Add another component (y/N)?"),
            Step::Output => (1, "Enter the mappings file path."),
            Step::Saved => return None,
        };
        let prefix: String = format!("Step {number}.");
        return Some((format!("{prefix} {title}"), Style::Step(prefix.len())));
    }

    fn render_input(&self, screen: &mut Screen) {
        if self.busy {
            let message: &str = if self.step == Step::Plan {
                "Loading SSP..."
            } else {
                "Working..."
            };
            screen.line(self, message, Style::Detail);
            return;
        }
        match self.step {
            Step::ComponentType => screen.choices(self, COMPONENT_TYPES, self.component_type),
            Step::Tool => screen.choices(self, SECURITY_TOOLS, self.tool),
            Step::Controls => screen.checklist(self),
            Step::Rules if self.is_zap_step() => {
                screen.checklist(self);
                let prefix: &str = if self.zap_manual { "> " } else { "  " };
                screen.line(
                    self,
                    &format!("{prefix}Additional IDs (comma separated): {}", self.input),
                    Style::Detail,
                );
            }
            _ => {
                let placeholder: &str = match self.step {
                    Step::Plan => DEFAULT_SSP_PATH,
                    Step::Title => DEFAULT_TITLE,
                    Step::Description => DEFAULT_DESCRIPTION,
                    Step::Purpose => DEFAULT_PURPOSE,
                    _ => "",
                };
                if self.input.is_empty() && !placeholder.is_empty() {
                    screen.line(self, &format!("> {placeholder}"), Style::Detail);
                } else {
                    screen.line(self, &format!("> {}_", self.input), Style::Plain);
                }
            }
        }
        return;
    }

    fn is_zap_manual(&self) -> bool {
        return self.is_zap_step() && self.zap_manual;
    }

    fn help(&self) -> &'static str {
        if self.busy {
            return "ctrl+c: quit";
        }
        return match self.step {
            Step::Plan => "enter: continue | tab: cycle JSON files | ctrl+c: quit",
            Step::ComponentType => "enter: continue | ↑/↓: select | esc: back | ctrl+c: quit",
            Step::Controls => {
                "enter: continue | ↑/↓: move | space / click: select | esc: back | ctrl+c: quit"
            }
            Step::Tool => {
                "enter: continue | ↑/↓: select | ctrl+d: finish rules | esc: back | ctrl+c: quit"
            }
            Step::Rules if self.is_zap_step() => {
                "enter: continue | ↑/↓: move | space: select | tab: additional IDs | esc: back | ctrl+c: quit"
            }
            Step::AnotherComponent => {
                "enter: continue | y / n: add another component | ctrl+t: add another tool | esc: back | ctrl+c: quit"
            }
            Step::Output => "enter: save | ctrl+c: quit",
            Step::Saved => "ctrl+r: start over | ctrl+c: quit",
            _ => "enter: continue | esc: back | ctrl+c: quit",
        };
    }
}

fn paint(text: &str, style: Style) -> String {
    if text.is_empty() || std::env::var_os("NO_COLOR").is_some() {
        return text.to_owned();
    }
    let code: &str = match style {
        Style::Plain => return text.to_owned(),
        Style::Accent => "1;38;5;177",
        Style::AccentText => "38;5;177",
        Style::Step(prefix) => {
            let (prefix, title): (&str, &str) = text.split_at(prefix.min(text.len()));
            return paint(prefix, Style::Accent) + &paint(title, Style::AccentText);
        }
        Style::Detail => "38;5;245",
        Style::Error => "1;31",
        Style::Success => "1;32",
    };
    return format!("\x1b[{code}m{text}\x1b[0m");
}
