//! egui-multiselect

// With thanks to ItsEthra for inspiration https://github.com/ItsEthra/egui-dropdown

//#![warn(missing_docs)]

use eframe::egui::style::StyleModifier;
use eframe::egui::{Align, Button, Id, Layout, Popup, Response, Ui, Vec2, Widget};
use std::hash::Hash;

/// MultiSelect widget
pub struct MultiSelect<'a, F: FnMut(&mut Ui, &str) -> Response> {
    popup_id: Id,
    answers: &'a mut Vec<String>,
    options: &'a Vec<String>,
    display: F,
    max_opt: &'a u8,
    toasted: &'a mut bool,
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response> MultiSelect<'a, F> {
    /// Creates new MultiSelect box.
    pub fn new(
        id_source: impl Hash,
        answers: &'a mut Vec<String>,
        options: &'a Vec<String>,
        display: F,
        max_opt: &'a u8,
        toasted: &'a mut bool,
    ) -> Self {
        Self {
            popup_id: Id::new(id_source),
            answers,
            options,
            display,
            max_opt,
            toasted,
        }
    }
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response> Widget for MultiSelect<'a, F> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            popup_id,
            answers,
            options,
            mut display,
            max_opt,
            toasted,
        } = self;

        let mut items = options.clone();
        for item in answers.clone() {
            items.retain(|x| *x != item);
        }

        let mut r = if answers.is_empty() {
            ui.add(
                Button::new(format!("Choose max {} options", max_opt))
                    .min_size(Vec2 { x: 200.0, y: 22.0 }),
            )
        } else {
            ui.horizontal(|ui| {
                ui.set_width(320.0);
                ui.horizontal_wrapped(|ui| {
                    ui.set_max_width(220.0);
                    for (i, item) in answers.clone().iter().enumerate() {
                        if ui.selectable_label(true, format!("{item} ｘ")).clicked() {
                            answers.remove(i);
                            answers.sort_by_key(|s| {
                                options.iter().position(|x| x == s).unwrap_or_default()
                            });
                            items.push(item.clone());
                            items.sort_by_key(|s| {
                                options.iter().position(|x| x == s).unwrap_or_default()
                            });

                            Popup::open_id(ui.ctx(), popup_id);
                        };
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    let icon_trash = egui_phosphor::regular::TRASH.to_owned();
                    if ui.button(icon_trash).clicked() {
                        answers.clear();
                        items = options.clone();
                        Popup::open_id(ui.ctx(), popup_id);
                    };
                    let icon_open = egui_phosphor::regular::FOLDER_OPEN.to_owned();
                    if ui.button(icon_open).clicked() && !items.is_empty() {
                        Popup::open_id(ui.ctx(), popup_id);
                    }
                });
            })
            .response
        };
        let mut changed = false;
        Popup::menu(&r)
            .id(popup_id)
            .style(StyleModifier::default())
            .close_behavior(eframe::egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, var) in items.clone().iter().enumerate() {
                        let text = var.clone();
                        if display(ui, &text).clicked() {
                            if answers.len() < *max_opt as usize {
                                answers.push(text.clone());
                                answers.sort_by_key(|s| {
                                    options.iter().position(|x| x == s).unwrap_or_default()
                                });
                                items.remove(i);
                                items.sort_by_key(|s| {
                                    options.iter().position(|x| x == s).unwrap_or_default()
                                });
                                changed = true;
                            } else {
                                *toasted = true;
                            }
                        }
                    }
                });
            });

        if changed {
            r.mark_changed();
            // close when max reached:
            if answers.len() == *max_opt as usize {
                Popup::close_id(ui.ctx(), popup_id);
            }
        }
        r
    }
}
