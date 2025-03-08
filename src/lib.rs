//! egui-dropdown

#![warn(missing_docs)]

use egui::{text_edit::TextEditOutput, Id, Response, ScrollArea, TextEdit, Ui, Widget, WidgetText};
use egui_chip::{ChipEdit, ChipEditOutput};
use std::hash::Hash;

/// Trait for editable responses
pub trait EditableResponse {
    /// Get the response of the widget
    fn get_response(&self) -> &Response;

    /// Check if the widget should show a popup
    fn show_popup(&self) -> bool;
}

impl EditableResponse for TextEditOutput {
    fn get_response(&self) -> &Response {
        &self.response
    }

    fn show_popup(&self) -> bool {
        self.response.gained_focus()
    }
}

impl EditableResponse for ChipEditOutput {
    fn get_response(&self) -> &Response {
        &self.response
    }

    fn show_popup(&self) -> bool {
        self.gained_focus
    }
}

/// Trait for editable widgets
pub trait Editable<R: EditableResponse> {
    /// Render the widget
    fn show_uneditable(&mut self, ui: &mut Ui) -> Response;

    /// Render the widget
    fn show_editable(&mut self, ui: &mut Ui, hint_text: &str) -> R;

    /// Get the text of the widget
    fn text(&self) -> String;
}

impl Editable<TextEditOutput> for String {
    fn show_uneditable(&mut self, ui: &mut Ui) -> Response {
        ui.label(self.as_str())
    }

    fn show_editable(&mut self, ui: &mut Ui, hint_text: &str) -> TextEditOutput {
        let edit = TextEdit::singleline(self).hint_text(hint_text);
        edit.show(ui)
    }

    fn text(&self) -> String {
        self.to_string()
    }
}

impl Editable<ChipEditOutput> for ChipEdit {
    fn show_uneditable(&mut self, ui: &mut Ui) -> Response {
        self.ui(ui)
    }

    fn show_editable(&mut self, ui: &mut Ui, _hint_text: &str) -> ChipEditOutput {
        self.show(ui)
    }

    fn text(&self) -> String {
        self.values().join(", ").to_string()
    }
}

/// Dropdown widget
pub struct DropDownBox<'a, Resp: EditableResponse, V: Editable<Resp> + Clone> {
    current: &'a mut V,
    popup_id: Id,
    items: &'a mut [V],
    hint_text: WidgetText,
    filter_by_input: bool,
    max_height: Option<f32>,
    _marker: std::marker::PhantomData<Resp>,
}

impl<'a, R: EditableResponse, V: Editable<R> + Clone> DropDownBox<'a, R, V> {
    /// Creates new dropdown box.
    pub fn from_iter(items: &'a mut [V], id_source: impl Hash, current: &'a mut V) -> Self {
        Self {
            popup_id: Id::new(id_source),
            items,
            current,
            hint_text: WidgetText::default(),
            filter_by_input: true,
            max_height: None,
            _marker: std::marker::PhantomData,
        }
    }

    /// Add a hint text to the Text Edit
    pub fn hint_text(mut self, hint_text: impl Into<WidgetText>) -> Self {
        self.hint_text = hint_text.into();
        self
    }

    /// Determine whether to filter box items based on what is in the Text Edit already
    pub fn filter_by_input(mut self, filter_by_input: bool) -> Self {
        self.filter_by_input = filter_by_input;
        self
    }

    /// Set a maximum height limit for the opened popup
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height.into();
        self
    }
}

impl<R: EditableResponse, V: Editable<R> + Clone> Widget for DropDownBox<'_, R, V> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            popup_id,
            current,
            items,
            hint_text,
            filter_by_input,
            max_height,
            _marker: std::marker::PhantomData,
        } = self;

        let edit_output = current.show_editable(ui, hint_text.text());
        let mut response = edit_output.get_response().to_owned();
        if edit_output.show_popup() {
            ui.memory_mut(|m| m.open_popup(popup_id));
        }

        let mut changed = false;
        egui::popup_below_widget(
            ui,
            popup_id,
            &response,
            egui::PopupCloseBehavior::CloseOnClick,
            |ui| {
                if let Some(max) = max_height {
                    ui.set_max_height(max);
                }

                ScrollArea::vertical()
                    .max_height(f32::INFINITY)
                    .show(ui, |ui| {
                        for item in items {
                            if filter_by_input
                                && !current.text().is_empty()
                                && !item
                                    .text()
                                    .to_lowercase()
                                    .contains(&current.text().to_lowercase())
                            {
                                continue;
                            }

                            if item.show_uneditable(ui).clicked() {
                                *current = item.clone();
                                changed = true;

                                ui.memory_mut(|m| m.close_popup());
                            }
                        }
                    });
            },
        );

        if changed {
            response.mark_changed();
        }

        response.to_owned()
    }
}
