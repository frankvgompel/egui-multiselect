//! egui-multiselect

// With thanks to ItsEthra for inspiration https://github.com/ItsEthra/egui-dropdown

//#![warn(missing_docs)]

use eframe::egui::{Id, Response, Ui, Widget, Button, Vec2, Layout, Align};
use std::hash::Hash;


/// MultiSelect widget

pub struct MultiSelect<
    'a,
    F: FnMut(&mut Ui, &str) -> Response,
> {
    popup_id: Id,
    items: &'a mut Vec<String>,
    answers: &'a mut Vec<String>,
    options: &'a Vec<String>,
    display: F,
    max_opt: &'a u8,
    toasted: &'a mut bool,
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response>
    MultiSelect<'a, F>
{
    /// Creates new MultiSelect box.
    pub fn new(
        id_source: impl Hash,
        items: &'a mut Vec<String>,
        answers: &'a mut Vec<String>,
        options: &'a Vec<String>,
        display: F,
        max_opt: &'a u8,
        toasted: &'a mut bool,
    ) -> Self {
        Self {
            popup_id: Id::new(id_source),
            items,
            answers,
            options,
            display,
            max_opt,
            toasted,
        }
    }
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response> Widget
    for MultiSelect<'a, F>
{
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            popup_id,
            items,
            answers,
            options,
            mut display,
            max_opt,
            toasted,
        } = self;

        if items.is_empty() && answers.is_empty() {
            for item in options.clone() {
                items.push(item)
            }
        }
        let mut r = 
            if answers.is_empty() {
                ui.add(Button::new(format!("Choose max {} options", max_opt)).min_size(Vec2 { x: 200.0, y: 22.0 }))
            }
            else {
                ui.horizontal(|ui| {
                    ui.set_width(320.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.set_max_width(220.0);
                        for (i, item) in answers.clone().iter().enumerate() {
                            if ui.selectable_label(true, format!("{item} ｘ")).clicked() {
                                answers.remove(i);
                                items.push(item.clone());
                                ui.memory_mut(|m| m.open_popup(popup_id))
                            };
                        }
                    });
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        let icon_trash = egui_phosphor::regular::TRASH.to_owned();
                        if ui.button(icon_trash).clicked() {
                            answers.clear();
                            items.clear();
                            for item in options.clone() {
                                items.push(item)
                            }
                            ui.memory_mut(|m| m.open_popup(popup_id))
                        };
                        let icon_open = egui_phosphor::regular::FOLDER_OPEN.to_owned();
                        if ui.button(icon_open).clicked() && !items.is_empty() {
                            ui.memory_mut(|m| m.open_popup(popup_id))
                        }
                    });
                }).response
            };
        if r.clicked() {
            ui.memory_mut(|m| m.open_popup(popup_id));
        }
        let mut changed = false;
        eframe::egui::popup_below_widget(ui, popup_id, &r, |ui| {
            eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, var) in items.clone().iter().enumerate() {
                    let text = var.clone();

                    if display(ui, &text).clicked() {
                        if answers.clone().len() != *max_opt as usize {
                            answers.push(text.clone());
                            items.remove(i);
                            changed = true;
                        }
                        else {
                            *toasted = true;                  
                        }
                    }
                }
            });
        });
        if changed {
            r.mark_changed();
            if !items.is_empty() {
                ui.memory_mut(|m| m.open_popup(popup_id))
            }
            if answers.len() == *max_opt as usize {
                ui.memory_mut(|m| m.close_popup())
            }
        }
        r
    }
}

