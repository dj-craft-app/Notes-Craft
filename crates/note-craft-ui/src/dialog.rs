//! Dialog System Module
//!
//! This module provides a dialog system for Note Craft UI.
//! It manages the display of modal dialogs, message boxes, and input prompts.
//!
//! Note: Phase 3 - Step 3.13 (Dialog System)

use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// Dialog Types
// ============================================================================

/// Unique identifier for a dialog
pub type DialogId = String;

/// Dialog button type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogButton {
    /// OK button (accepts/confirms)
    #[default]
    Ok,
    /// Cancel button (rejects/cancels)
    Cancel,
    /// Yes button
    Yes,
    /// No button
    No,
    /// Close button (for non-modal dialogs)
    Close,
    /// Custom button with text
    Custom,
}

/// Dialog result (which button was clicked)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogResult {
    /// OK was clicked
    Ok,
    /// Cancel was clicked
    Cancel,
    /// Yes was clicked
    Yes,
    /// No was clicked
    No,
    /// Close was clicked
    Close,
    /// Custom button was clicked with its index
    Custom(usize),
    /// Dialog was closed without a button click
    Closed,
}

/// Dialog icon type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogIcon {
    /// No icon
    #[default]
    None,
    /// Information icon
    Info,
    /// Warning icon
    Warning,
    /// Error icon
    Error,
    /// Question icon
    Question,
    /// Success icon
    Success,
}

/// Dialog style/severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogStyle {
    /// Standard dialog
    #[default]
    Standard,
    /// Primary dialog (for important actions)
    Primary,
    /// Danger dialog (for destructive actions)
    Danger,
}

// ============================================================================
// Dialog Configuration
// ============================================================================

/// Button configuration for a dialog
#[derive(Debug, Clone)]
pub struct DialogButtonConfig {
    /// Button type
    pub button_type: DialogButton,
    /// Display text
    pub text: String,
    /// Is this the default button (triggered by Enter)
    pub is_default: bool,
    /// Is this the cancel button (triggered by Escape)
    pub is_cancel: bool,
    /// Optional style override
    pub style: Option<DialogStyle>,
}

impl DialogButtonConfig {
    /// Creates a new button configuration
    pub fn new(button_type: DialogButton, text: impl Into<String>) -> Self {
        Self {
            button_type,
            text: text.into(),
            is_default: false,
            is_cancel: false,
            style: None,
        }
    }

    /// Sets if this is the default button
    pub fn default(mut self, is_default: bool) -> Self {
        self.is_default = is_default;
        self
    }

    /// Sets if this is the cancel button
    pub fn cancel(mut self, is_cancel: bool) -> Self {
        self.is_cancel = is_cancel;
        self
    }

    /// Sets the style
    pub fn with_style(mut self, style: DialogStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// Dialog configuration
#[derive(Debug, Clone)]
pub struct DialogConfig {
    /// Dialog identifier
    pub id: DialogId,
    /// Dialog title
    pub title: String,
    /// Dialog message (can be Markdown)
    pub message: String,
    /// Icon to display
    pub icon: DialogIcon,
    /// Style/severity
    pub style: DialogStyle,
    /// List of buttons
    pub buttons: Vec<DialogButtonConfig>,
    /// Is the dialog modal (blocks interaction with parent)
    pub modal: bool,
    /// Can the dialog be closed by clicking outside
    pub close_on_outside_click: bool,
    /// Can the dialog be closed with Escape key
    pub close_on_escape: bool,
    /// Width of the dialog (None for auto)
    pub width: Option<u32>,
    /// Height of the dialog (None for auto)
    pub height: Option<u32>,
    /// Minimum width
    pub min_width: u32,
    /// Minimum height
    pub min_height: u32,
}

impl Default for DialogConfig {
    fn default() -> Self {
        Self {
            id: "dialog".to_string(),
            title: "Dialog".to_string(),
            message: String::new(),
            icon: DialogIcon::None,
            style: DialogStyle::Standard,
            buttons: vec![DialogButtonConfig::new(DialogButton::Ok, "OK").default(true)],
            modal: true,
            close_on_outside_click: true,
            close_on_escape: true,
            width: None,
            height: None,
            min_width: 300,
            min_height: 200,
        }
    }
}

impl DialogConfig {
    /// Creates a new dialog configuration
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            ..Default::default()
        }
    }

    /// Sets the message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Sets the icon
    pub fn with_icon(mut self, icon: DialogIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Sets the style
    pub fn with_style(mut self, style: DialogStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the buttons
    pub fn with_buttons(mut self, buttons: Vec<DialogButtonConfig>) -> Self {
        self.buttons = buttons;
        self
    }

    /// Adds a button
    pub fn add_button(mut self, button: DialogButtonConfig) -> Self {
        self.buttons.push(button);
        self
    }

    /// Sets if the dialog is modal
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Sets if the dialog can be closed by clicking outside
    pub fn close_on_outside_click(mut self, close: bool) -> Self {
        self.close_on_outside_click = close;
        self
    }

    /// Sets if the dialog can be closed with Escape
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Sets the width
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets minimum width
    pub fn with_min_width(mut self, min_width: u32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Sets minimum height
    pub fn with_min_height(mut self, min_height: u32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Creates a simple OK dialog
    pub fn ok_only(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(id, title)
            .with_message(message)
            .with_buttons(vec![
                DialogButtonConfig::new(DialogButton::Ok, "OK").default(true)
            ])
    }

    /// Creates a simple OK/Cancel dialog
    pub fn ok_cancel(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(id, title)
            .with_message(message)
            .with_buttons(vec![
                DialogButtonConfig::new(DialogButton::Cancel, "Cancel").cancel(true),
                DialogButtonConfig::new(DialogButton::Ok, "OK").default(true),
            ])
    }

    /// Creates a simple Yes/No dialog
    pub fn yes_no(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(id, title)
            .with_message(message)
            .with_buttons(vec![
                DialogButtonConfig::new(DialogButton::No, "No").cancel(true),
                DialogButtonConfig::new(DialogButton::Yes, "Yes").default(true),
            ])
    }
}

// ============================================================================
// Dialog State
// ============================================================================

/// Dialog state (runtime information)
#[derive(Debug, Clone)]
pub struct DialogState {
    /// Dialog configuration
    pub config: DialogConfig,
    /// Is the dialog currently open
    pub is_open: bool,
    /// Has the dialog been initialized
    pub initialized: bool,
    /// X position (for non-modal dialogs)
    pub x: u32,
    /// Y position (for non-modal dialogs)
    pub y: u32,
    /// Current width
    pub width: u32,
    /// Current height
    pub height: u32,
}

impl DialogState {
    /// Creates a new dialog state from config
    pub fn from_config(config: DialogConfig) -> Self {
        let width = config.width.unwrap_or(400);
        let height = config.height.unwrap_or(300);
        Self {
            config,
            is_open: false,
            initialized: false,
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    /// Opens the dialog
    pub fn open(&mut self) {
        self.is_open = true;
    }

    /// Closes the dialog
    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Toggles the dialog
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    /// Sets the position
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    /// Sets the size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    /// Updates the geometry
    pub fn update_geometry(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }
}

// ============================================================================
// Input Dialog Types
// ============================================================================

/// Input type for dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputType {
    /// Single line text input
    #[default]
    Text,
    /// Multi-line text input
    TextArea,
    /// Number input
    Number,
    /// Password input
    Password,
    /// File path input
    File,
    /// Directory path input
    Directory,
}

/// Input field configuration
#[derive(Debug, Clone)]
pub struct InputConfig {
    /// Field identifier
    pub id: String,
    /// Label
    pub label: String,
    /// Placeholder text
    pub placeholder: Option<String>,
    /// Input type
    pub input_type: InputType,
    /// Default value
    pub value: String,
    /// Is required
    pub required: bool,
    /// Validation pattern (regex)
    pub pattern: Option<String>,
    /// Min length
    pub min_length: Option<usize>,
    /// Max length
    pub max_length: Option<usize>,
    /// Min value (for numbers)
    pub min_value: Option<f64>,
    /// Max value (for numbers)
    pub max_value: Option<f64>,
}

impl InputConfig {
    /// Creates a new input configuration
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            placeholder: None,
            input_type: InputType::Text,
            value: String::new(),
            required: false,
            pattern: None,
            min_length: None,
            max_length: None,
            min_value: None,
            max_value: None,
        }
    }

    /// Sets the placeholder
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the input type
    pub fn with_input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    /// Sets the default value
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets if the field is required
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets a validation pattern
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Sets minimum length
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Sets maximum length
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Sets minimum value
    pub fn with_min_value(mut self, min_value: f64) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets maximum value
    pub fn with_max_value(mut self, max_value: f64) -> Self {
        self.max_value = Some(max_value);
        self
    }
}

// ============================================================================
// Input Dialog Configuration
// ============================================================================

/// Input dialog configuration
#[derive(Debug, Clone)]
pub struct InputDialogConfig {
    /// Base dialog configuration
    pub base: DialogConfig,
    /// List of input fields
    pub inputs: Vec<InputConfig>,
    /// Focus the first input on open
    pub focus_first_input: bool,
    /// Submit on Enter
    pub submit_on_enter: bool,
}

impl InputDialogConfig {
    /// Creates a new input dialog configuration
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            base: DialogConfig::new(id, title),
            inputs: Vec::new(),
            focus_first_input: true,
            submit_on_enter: true,
        }
    }

    /// Sets the message
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.base.message = message.into();
        self
    }

    /// Adds an input field
    pub fn add_input(mut self, input: InputConfig) -> Self {
        self.inputs.push(input);
        self
    }

    /// Sets the buttons
    pub fn with_buttons(mut self, buttons: Vec<DialogButtonConfig>) -> Self {
        self.base.buttons = buttons;
        self
    }

    /// Sets if the first input should be focused
    pub fn focus_first_input(mut self, focus: bool) -> Self {
        self.focus_first_input = focus;
        self
    }

    /// Sets if the dialog should submit on Enter
    pub fn submit_on_enter(mut self, submit: bool) -> Self {
        self.submit_on_enter = submit;
        self
    }
}

/// Input dialog result (includes button result and input values)
#[derive(Debug, Clone, PartialEq)]
pub struct InputDialogResult {
    /// Which button was clicked
    pub button_result: DialogResult,
    /// Input values (field_id -> value)
    pub input_values: HashMap<String, String>,
}

impl InputDialogResult {
    /// Creates a new input dialog result
    pub fn new(button_result: DialogResult) -> Self {
        Self {
            button_result,
            input_values: HashMap::new(),
        }
    }

    /// Sets an input value
    pub fn set_value(&mut self, field_id: impl Into<String>, value: impl Into<String>) {
        self.input_values.insert(field_id.into(), value.into());
    }

    /// Gets an input value
    pub fn get_value(&self, field_id: &str) -> Option<&String> {
        self.input_values.get(field_id)
    }
}

// ============================================================================
// Dialog Events
// ============================================================================

/// Events for the dialog system
#[derive(Debug, Clone, PartialEq)]
pub enum DialogEvent {
    /// A dialog was opened
    Opened(DialogId),
    /// A dialog was closed
    Closed(DialogId, DialogResult),
    /// Input value changed
    InputChanged(DialogId, String, String),
    /// Dialog was submitted (all inputs valid)
    Submitted(DialogId, InputDialogResult),
    /// All dialogs were closed
    AllClosed,
}

// ============================================================================
// Dialog Manager
// ============================================================================

/// Main dialog manager
pub struct DialogManager {
    /// Map of dialogs by ID
    pub dialogs: HashMap<DialogId, DialogState>,
    /// Currently open dialogs
    pub open_dialogs: Vec<DialogId>,
    /// Next dialog ID counter
    pub next_dialog_id: u32,
    /// Event handler
    pub event_handler: Option<Box<dyn Fn(DialogEvent) + Send + Sync>>,
    /// Callback for dialog results
    pub result_callbacks: HashMap<DialogId, Arc<dyn Fn(DialogResult) + Send + Sync>>,
    /// Callback for input dialog results
    pub input_callbacks: HashMap<DialogId, Arc<dyn Fn(InputDialogResult) + Send + Sync>>,
}

impl DialogManager {
    /// Creates a new dialog manager
    pub fn new() -> Self {
        Self {
            dialogs: HashMap::new(),
            open_dialogs: Vec::new(),
            next_dialog_id: 1,
            event_handler: None,
            result_callbacks: HashMap::new(),
            input_callbacks: HashMap::new(),
        }
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(DialogEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    /// Generates a unique dialog ID
    pub fn generate_dialog_id(&mut self) -> String {
        let id = format!("dialog_{}", self.next_dialog_id);
        self.next_dialog_id += 1;
        id
    }

    /// Adds a dialog
    pub fn add_dialog(&mut self, config: DialogConfig) -> DialogId {
        let id = config.id.clone();
        let state = DialogState::from_config(config);
        self.dialogs.insert(id.clone(), state);
        id
    }

    /// Opens a dialog by ID
    pub fn open_dialog<T: AsRef<str>>(&mut self, dialog_id: T) -> Option<DialogState> {
        let dialog_id = dialog_id.as_ref().to_string();
        // Check if dialog exists first
        if !self.dialogs.contains_key(&dialog_id) {
            return None;
        }

        // Get mutable access to update state
        let state_clone = {
            let state = self.dialogs.get_mut(&dialog_id).unwrap();
            let was_open = state.is_open;
            state.open();
            if !was_open && !self.open_dialogs.contains(&dialog_id) {
                self.open_dialogs.push(dialog_id.clone());
            }
            state.clone()
        };

        // Now emit event without holding mutable borrow
        self.emit_event(DialogEvent::Opened(dialog_id));
        Some(state_clone)
    }

    /// Closes a dialog by ID with a result
    pub fn close_dialog<T: AsRef<str>>(
        &mut self,
        dialog_id: T,
        result: DialogResult,
    ) -> Option<DialogState> {
        let dialog_id = dialog_id.as_ref().to_string();
        if let Some(mut state) = self.dialogs.remove(&dialog_id) {
            state.close();
            self.open_dialogs.retain(|id| id != &dialog_id);

            self.emit_event(DialogEvent::Closed(dialog_id.clone(), result));

            // Call result callback if exists
            if let Some(callback) = self.result_callbacks.remove(&dialog_id) {
                callback(result);
            }

            // Check if all dialogs are closed
            if self.open_dialogs.is_empty() {
                self.emit_event(DialogEvent::AllClosed);
            }

            Some(state)
        } else {
            None
        }
    }

    /// Closes all open dialogs
    pub fn close_all_dialogs(&mut self) {
        let dialog_ids: Vec<DialogId> = self.open_dialogs.drain(..).collect();
        for dialog_id in dialog_ids {
            if let Some(mut state) = self.dialogs.remove(&dialog_id) {
                state.close();
                self.emit_event(DialogEvent::Closed(dialog_id, DialogResult::Closed));
            }
        }
        self.emit_event(DialogEvent::AllClosed);
    }

    /// Gets a dialog by ID
    pub fn get_dialog<T: AsRef<str>>(&self, dialog_id: T) -> Option<&DialogState> {
        self.dialogs.get(dialog_id.as_ref())
    }

    /// Gets a mutable dialog by ID
    pub fn get_dialog_mut<T: AsRef<str>>(&mut self, dialog_id: T) -> Option<&mut DialogState> {
        self.dialogs.get_mut(dialog_id.as_ref())
    }

    /// Checks if a dialog is open
    pub fn is_dialog_open<T: AsRef<str>>(&self, dialog_id: T) -> bool {
        self.dialogs
            .get(dialog_id.as_ref())
            .map(|s| s.is_open)
            .unwrap_or(false)
    }

    /// Gets the list of open dialogs
    pub fn get_open_dialogs(&self) -> Vec<&DialogState> {
        self.open_dialogs
            .iter()
            .filter_map(|id| self.dialogs.get(id))
            .collect()
    }

    /// Removes a dialog by ID
    pub fn remove_dialog<T: AsRef<str>>(&mut self, dialog_id: T) -> Option<DialogState> {
        self.dialogs.remove(dialog_id.as_ref())
    }

    /// Clears all dialogs
    pub fn clear_dialogs(&mut self) {
        self.dialogs.clear();
        self.open_dialogs.clear();
        self.result_callbacks.clear();
        self.input_callbacks.clear();
    }

    /// Shows a simple message dialog (OK only)
    pub fn show_message(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Option<DialogId> {
        let dialog_id = self.generate_dialog_id();
        let config = DialogConfig::ok_only(&dialog_id, title, message);
        self.add_dialog(config);
        self.open_dialog(&dialog_id);
        Some(dialog_id)
    }

    /// Shows a simple message dialog with icon
    pub fn show_message_with_icon(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        icon: DialogIcon,
    ) -> Option<DialogId> {
        let dialog_id = self.generate_dialog_id();
        let config = DialogConfig::ok_only(&dialog_id, title, message).with_icon(icon);
        self.add_dialog(config);
        self.open_dialog(&dialog_id);
        Some(dialog_id)
    }

    /// Shows a confirmation dialog (OK/Cancel)
    pub fn show_confirmation(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        callback: impl Fn(DialogResult) + Send + Sync + 'static,
    ) -> Option<DialogId> {
        let dialog_id = self.generate_dialog_id();
        let config = DialogConfig::ok_cancel(&dialog_id, title, message);
        self.add_dialog(config.clone());

        // Store callback
        self.result_callbacks
            .insert(dialog_id.clone(), Arc::new(callback));

        self.open_dialog(&dialog_id);
        Some(dialog_id)
    }

    /// Shows a yes/no dialog
    pub fn show_yes_no(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        callback: impl Fn(DialogResult) + Send + Sync + 'static,
    ) -> Option<DialogId> {
        let dialog_id = self.generate_dialog_id();
        let config = DialogConfig::yes_no(&dialog_id, title, message);
        self.add_dialog(config.clone());

        // Store callback
        self.result_callbacks
            .insert(dialog_id.clone(), Arc::new(callback));

        self.open_dialog(&dialog_id);
        Some(dialog_id)
    }

    /// Shows an input dialog
    pub fn show_input_dialog(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        inputs: Vec<InputConfig>,
        callback: impl Fn(InputDialogResult) + Send + Sync + 'static,
    ) -> Option<DialogId> {
        let dialog_id = self.generate_dialog_id();
        let mut config = InputDialogConfig::new(&dialog_id, title);
        config = config.with_message(message);
        config.inputs = inputs;

        // Add OK/Cancel buttons
        config = config.with_buttons(vec![
            DialogButtonConfig::new(DialogButton::Cancel, "Cancel").cancel(true),
            DialogButtonConfig::new(DialogButton::Ok, "OK").default(true),
        ]);

        // Create base dialog config
        let base_config = config.base;
        self.add_dialog(base_config);

        // Store input callback
        self.input_callbacks
            .insert(dialog_id.clone(), Arc::new(callback));

        self.open_dialog(&dialog_id);
        Some(dialog_id)
    }

    /// Shows a prompt dialog (single text input)
    pub fn show_prompt(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        label: impl Into<String>,
        default_value: Option<&str>,
        callback: impl Fn(InputDialogResult) + Send + Sync + 'static,
    ) -> Option<DialogId> {
        let mut input = InputConfig::new("input", label);
        if let Some(value) = default_value {
            input = input.with_value(value);
        }
        self.show_input_dialog(title, message, vec![input], callback)
    }

    /// Handles a button click for a dialog
    pub fn handle_button_click<T: AsRef<str>>(
        &mut self,
        dialog_id: T,
        button_index: usize,
    ) -> Option<DialogResult> {
        let dialog_id = dialog_id.as_ref().to_string();
        if let Some(state) = self.get_dialog(&dialog_id) {
            if button_index < state.config.buttons.len() {
                let button = &state.config.buttons[button_index];
                let result = match button.button_type {
                    DialogButton::Ok => DialogResult::Ok,
                    DialogButton::Cancel => DialogResult::Cancel,
                    DialogButton::Yes => DialogResult::Yes,
                    DialogButton::No => DialogResult::No,
                    DialogButton::Close => DialogResult::Close,
                    DialogButton::Custom => DialogResult::Custom(button_index),
                };

                // Close the dialog with the result
                self.close_dialog(&dialog_id, result);
                return Some(result);
            }
        }
        None
    }

    /// Handles input change
    pub fn handle_input_change<T: AsRef<str>>(
        &mut self,
        dialog_id: T,
        field_id: String,
        value: String,
    ) -> bool {
        let dialog_id = dialog_id.as_ref().to_string();
        if let Some(_state) = self.get_dialog_mut(&dialog_id) {
            // Find the input field and update it
            // (In a real implementation, this would update the UI)
            self.emit_event(DialogEvent::InputChanged(dialog_id, field_id, value));
            true
        } else {
            false
        }
    }

    /// Submits a dialog (for input dialogs)
    pub fn submit_dialog<T: AsRef<str>>(&mut self, dialog_id: T) -> Option<InputDialogResult> {
        let dialog_id = dialog_id.as_ref().to_string();
        // This would validate all inputs and collect their values
        // For now, we'll just create a basic result
        if let Some(_state) = self.get_dialog(&dialog_id) {
            let result = InputDialogResult::new(DialogResult::Ok);
            // In a real implementation, we would collect actual input values
            // For now, we'll just use empty values
            self.emit_event(DialogEvent::Submitted(dialog_id.clone(), result.clone()));

            // Close the dialog
            self.close_dialog(&dialog_id, DialogResult::Ok);

            // Call input callback if exists
            if let Some(callback) = self.input_callbacks.remove(&dialog_id) {
                callback(result.clone());
            }

            Some(result)
        } else {
            None
        }
    }

    /// Emits an event to the event handler
    fn emit_event(&self, event: DialogEvent) {
        if let Some(ref handler) = self.event_handler {
            handler(event);
        }
    }
}

impl Default for DialogManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Predefined Dialog Configurations
// ============================================================================

/// Predefined dialog configurations
pub struct PredefinedDialogs;

impl PredefinedDialogs {
    /// Creates an info dialog configuration
    pub fn info(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> DialogConfig {
        DialogConfig::ok_only(id, title, message).with_icon(DialogIcon::Info)
    }

    /// Creates a warning dialog configuration
    pub fn warning(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> DialogConfig {
        DialogConfig::ok_only(id, title, message).with_icon(DialogIcon::Warning)
    }

    /// Creates an error dialog configuration
    pub fn error(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> DialogConfig {
        DialogConfig::ok_only(id, title, message).with_icon(DialogIcon::Error)
    }

    /// Creates a success dialog configuration
    pub fn success(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> DialogConfig {
        DialogConfig::ok_only(id, title, message).with_icon(DialogIcon::Success)
    }

    /// Creates a confirmation dialog configuration
    pub fn confirm(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
        confirm_text: impl Into<String>,
        cancel_text: impl Into<String>,
    ) -> DialogConfig {
        DialogConfig::new(id, title)
            .with_message(message)
            .with_icon(DialogIcon::Question)
            .with_buttons(vec![
                DialogButtonConfig::new(DialogButton::Cancel, cancel_text).cancel(true),
                DialogButtonConfig::new(DialogButton::Ok, confirm_text).default(true),
            ])
    }

    /// Creates a yes/no dialog configuration
    pub fn yes_no(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> DialogConfig {
        DialogConfig::new(id, title)
            .with_message(message)
            .with_icon(DialogIcon::Question)
            .with_buttons(vec![
                DialogButtonConfig::new(DialogButton::No, "No").cancel(true),
                DialogButtonConfig::new(DialogButton::Yes, "Yes").default(true),
            ])
    }

    /// Creates a dialog with custom buttons
    pub fn with_buttons(
        id: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
        buttons: Vec<(DialogButton, &str)>,
    ) -> DialogConfig {
        let button_configs = buttons
            .into_iter()
            .enumerate()
            .map(|(i, (button_type, text))| {
                DialogButtonConfig::new(button_type, text)
                    .default(i == 0) // First button is default
                    .cancel(button_type == DialogButton::Cancel || button_type == DialogButton::No)
            })
            .collect();

        DialogConfig::new(id, title)
            .with_message(message)
            .with_buttons(button_configs)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== DialogConfig Tests ==========

    #[test]
    fn test_dialog_config_default() {
        let config = DialogConfig::default();
        assert_eq!(config.id, "dialog");
        assert_eq!(config.title, "Dialog");
        assert_eq!(config.message, "");
        assert_eq!(config.icon, DialogIcon::None);
        assert_eq!(config.style, DialogStyle::Standard);
        assert_eq!(config.buttons.len(), 1);
        assert_eq!(config.buttons[0].button_type, DialogButton::Ok);
        assert!(config.modal);
        assert!(config.close_on_outside_click);
        assert!(config.close_on_escape);
        assert_eq!(config.min_width, 300);
        assert_eq!(config.min_height, 200);
    }

    #[test]
    fn test_dialog_config_new() {
        let config = DialogConfig::new("test", "Test Dialog");
        assert_eq!(config.id, "test");
        assert_eq!(config.title, "Test Dialog");
    }

    #[test]
    fn test_dialog_config_with_message() {
        let config = DialogConfig::new("test", "Title").with_message("Test message");
        assert_eq!(config.message, "Test message");
    }

    #[test]
    fn test_dialog_config_with_icon() {
        let config = DialogConfig::new("test", "Title").with_icon(DialogIcon::Error);
        assert_eq!(config.icon, DialogIcon::Error);
    }

    #[test]
    fn test_dialog_config_ok_only() {
        let config = DialogConfig::ok_only("test", "Title", "Message");
        assert_eq!(config.id, "test");
        assert_eq!(config.title, "Title");
        assert_eq!(config.message, "Message");
        assert_eq!(config.buttons.len(), 1);
        assert_eq!(config.buttons[0].button_type, DialogButton::Ok);
    }

    #[test]
    fn test_dialog_config_ok_cancel() {
        let config = DialogConfig::ok_cancel("test", "Title", "Message");
        assert_eq!(config.id, "test");
        assert_eq!(config.message, "Message");
        assert_eq!(config.buttons.len(), 2);
        assert_eq!(config.buttons[0].button_type, DialogButton::Cancel);
        assert_eq!(config.buttons[1].button_type, DialogButton::Ok);
    }

    #[test]
    fn test_dialog_config_yes_no() {
        let config = DialogConfig::yes_no("test", "Title", "Message");
        assert_eq!(config.buttons.len(), 2);
        assert_eq!(config.buttons[0].button_type, DialogButton::No);
        assert_eq!(config.buttons[1].button_type, DialogButton::Yes);
    }

    // ========== DialogButtonConfig Tests ==========

    #[test]
    fn test_dialog_button_config_new() {
        let button = DialogButtonConfig::new(DialogButton::Ok, "OK");
        assert_eq!(button.button_type, DialogButton::Ok);
        assert_eq!(button.text, "OK");
        assert!(!button.is_default);
        assert!(!button.is_cancel);
    }

    #[test]
    fn test_dialog_button_config_default() {
        let button = DialogButtonConfig::new(DialogButton::Ok, "OK").default(true);
        assert!(button.is_default);
    }

    #[test]
    fn test_dialog_button_config_cancel() {
        let button = DialogButtonConfig::new(DialogButton::Cancel, "Cancel").cancel(true);
        assert!(button.is_cancel);
    }

    // ========== DialogState Tests ==========

    #[test]
    fn test_dialog_state_from_config() {
        let config = DialogConfig::new("test", "Title");
        let state = DialogState::from_config(config);
        assert_eq!(state.config.id, "test");
        assert!(!state.is_open);
        assert!(!state.initialized);
    }

    #[test]
    fn test_dialog_state_open() {
        let config = DialogConfig::new("test", "Title");
        let mut state = DialogState::from_config(config);
        state.open();
        assert!(state.is_open);
    }

    #[test]
    fn test_dialog_state_close() {
        let config = DialogConfig::new("test", "Title");
        let mut state = DialogState::from_config(config);
        state.open();
        state.close();
        assert!(!state.is_open);
    }

    #[test]
    fn test_dialog_state_toggle() {
        let config = DialogConfig::new("test", "Title");
        let mut state = DialogState::from_config(config);
        assert!(!state.is_open);
        state.toggle();
        assert!(state.is_open);
        state.toggle();
        assert!(!state.is_open);
    }

    #[test]
    fn test_dialog_state_set_position() {
        let config = DialogConfig::new("test", "Title");
        let mut state = DialogState::from_config(config);
        state.set_position(100, 200);
        assert_eq!(state.x, 100);
        assert_eq!(state.y, 200);
    }

    #[test]
    fn test_dialog_state_set_size() {
        let config = DialogConfig::new("test", "Title");
        let mut state = DialogState::from_config(config);
        state.set_size(400, 300);
        assert_eq!(state.width, 400);
        assert_eq!(state.height, 300);
    }

    // ========== InputConfig Tests ==========

    #[test]
    fn test_input_config_new() {
        let input = InputConfig::new("username", "Username");
        assert_eq!(input.id, "username");
        assert_eq!(input.label, "Username");
        assert_eq!(input.input_type, InputType::Text);
        assert_eq!(input.value, "");
        assert!(!input.required);
    }

    #[test]
    fn test_input_config_with_placeholder() {
        let input = InputConfig::new("email", "Email").with_placeholder("Enter your email");
        assert_eq!(input.placeholder, Some("Enter your email".to_string()));
    }

    #[test]
    fn test_input_config_with_value() {
        let input = InputConfig::new("name", "Name").with_value("John Doe");
        assert_eq!(input.value, "John Doe");
    }

    #[test]
    fn test_input_config_required() {
        let input = InputConfig::new("password", "Password").required(true);
        assert!(input.required);
    }

    #[test]
    fn test_input_config_with_pattern() {
        let input = InputConfig::new("email", "Email").with_pattern(r"^[^@]+@[^@]+\.[^@]+$");
        assert!(input.pattern.is_some());
    }

    // ========== InputDialogConfig Tests ==========

    #[test]
    fn test_input_dialog_config_new() {
        let config = InputDialogConfig::new("form", "Form Dialog");
        assert_eq!(config.base.id, "form");
        assert_eq!(config.base.title, "Form Dialog");
        assert!(config.inputs.is_empty());
        assert!(config.focus_first_input);
        assert!(config.submit_on_enter);
    }

    #[test]
    fn test_input_dialog_config_add_input() {
        let input = InputConfig::new("name", "Name");
        let config = InputDialogConfig::new("form", "Form").add_input(input);
        assert_eq!(config.inputs.len(), 1);
        assert_eq!(config.inputs[0].id, "name");
    }

    // ========== InputDialogResult Tests ==========

    #[test]
    fn test_input_dialog_result_new() {
        let result = InputDialogResult::new(DialogResult::Ok);
        assert_eq!(result.button_result, DialogResult::Ok);
        assert!(result.input_values.is_empty());
    }

    #[test]
    fn test_input_dialog_result_set_value() {
        let mut result = InputDialogResult::new(DialogResult::Ok);
        result.set_value("username", "john");
        assert_eq!(result.get_value("username"), Some(&"john".to_string()));
    }

    // ========== DialogManager Tests ==========

    #[test]
    fn test_dialog_manager_new() {
        let manager = DialogManager::new();
        assert!(manager.dialogs.is_empty());
        assert!(manager.open_dialogs.is_empty());
    }

    #[test]
    fn test_dialog_manager_generate_dialog_id() {
        let mut manager = DialogManager::new();
        let id1 = manager.generate_dialog_id();
        let id2 = manager.generate_dialog_id();
        assert_eq!(id1, "dialog_1");
        assert_eq!(id2, "dialog_2");
    }

    #[test]
    fn test_dialog_manager_add_dialog() {
        let mut manager = DialogManager::new();
        let config = DialogConfig::new("test", "Test");
        let dialog_id = manager.add_dialog(config);
        assert_eq!(dialog_id, "test");
        assert!(manager.dialogs.contains_key("test"));
    }

    #[test]
    fn test_dialog_manager_open_dialog() {
        let mut manager = DialogManager::new();
        let config = DialogConfig::new("test", "Test");
        manager.add_dialog(config);
        let state = manager.open_dialog("test");
        assert!(state.is_some());
        assert!(state.unwrap().is_open);
        assert!(manager.open_dialogs.contains(&"test".to_string()));
    }

    #[test]
    fn test_dialog_manager_close_dialog() {
        let mut manager = DialogManager::new();
        let config = DialogConfig::new("test", "Test");
        manager.add_dialog(config);
        manager.open_dialog("test");
        let state = manager.close_dialog("test", DialogResult::Ok);
        assert!(state.is_some());
        assert!(!state.unwrap().is_open);
        assert!(!manager.open_dialogs.contains(&"test".to_string()));
    }

    #[test]
    fn test_dialog_manager_get_dialog() {
        let mut manager = DialogManager::new();
        let config = DialogConfig::new("test", "Test");
        manager.add_dialog(config);
        let state = manager.get_dialog("test");
        assert!(state.is_some());
        assert_eq!(state.unwrap().config.id, "test");
    }

    #[test]
    fn test_dialog_manager_is_dialog_open() {
        let mut manager = DialogManager::new();
        let config = DialogConfig::new("test", "Test");
        manager.add_dialog(config);
        assert!(!manager.is_dialog_open("test"));
        manager.open_dialog("test");
        assert!(manager.is_dialog_open("test"));
    }

    #[test]
    fn test_dialog_manager_get_open_dialogs() {
        let mut manager = DialogManager::new();
        let config1 = DialogConfig::new("dialog1", "Dialog 1");
        let config2 = DialogConfig::new("dialog2", "Dialog 2");
        manager.add_dialog(config1);
        manager.add_dialog(config2);
        manager.open_dialog("dialog1");
        manager.open_dialog("dialog2");
        let open_dialogs = manager.get_open_dialogs();
        assert_eq!(open_dialogs.len(), 2);
    }

    #[test]
    fn test_dialog_manager_remove_dialog() {
        let mut manager = DialogManager::new();
        let config = DialogConfig::new("test", "Test");
        manager.add_dialog(config);
        let state = manager.remove_dialog("test");
        assert!(state.is_some());
        assert!(!manager.dialogs.contains_key("test"));
    }

    #[test]
    fn test_dialog_manager_clear_dialogs() {
        let mut manager = DialogManager::new();
        let config1 = DialogConfig::new("dialog1", "Dialog 1");
        let config2 = DialogConfig::new("dialog2", "Dialog 2");
        manager.add_dialog(config1);
        manager.add_dialog(config2);
        manager.clear_dialogs();
        assert!(manager.dialogs.is_empty());
        assert!(manager.open_dialogs.is_empty());
    }

    #[test]
    fn test_dialog_manager_show_message() {
        let mut manager = DialogManager::new();
        let dialog_id = manager.show_message("Title", "Message");
        assert!(dialog_id.is_some());
        assert!(manager.is_dialog_open(&dialog_id.unwrap()));
    }

    #[test]
    fn test_dialog_manager_show_confirmation() {
        let mut manager = DialogManager::new();
        let dialog_id = manager.show_confirmation("Title", "Message", |result| {
            assert_eq!(result, DialogResult::Ok);
        });
        assert!(dialog_id.is_some());
        // We can't easily test the callback without triggering it
    }

    #[test]
    fn test_dialog_manager_show_prompt() {
        let mut manager = DialogManager::new();
        let dialog_id =
            manager.show_prompt("Title", "Message", "Name", Some("Default"), |result| {
                assert_eq!(result.button_result, DialogResult::Ok);
            });
        assert!(dialog_id.is_some());
    }

    #[test]
    fn test_dialog_manager_handle_button_click() {
        let mut manager = DialogManager::new();
        let config = DialogConfig::ok_cancel("test", "Title", "Message");
        manager.add_dialog(config);
        manager.open_dialog("test");

        let result = manager.handle_button_click("test", 0); // Cancel button
        assert!(result.is_some());
        assert_eq!(result.unwrap(), DialogResult::Cancel);
        assert!(!manager.is_dialog_open("test"));
    }

    #[test]
    fn test_dialog_manager_close_all_dialogs() {
        let mut manager = DialogManager::new();
        let config1 = DialogConfig::new("dialog1", "Dialog 1");
        let config2 = DialogConfig::new("dialog2", "Dialog 2");
        manager.add_dialog(config1);
        manager.add_dialog(config2);
        manager.open_dialog("dialog1");
        manager.open_dialog("dialog2");

        manager.close_all_dialogs();
        assert!(manager.open_dialogs.is_empty());
    }

    // ========== Dialog Events Tests ==========

    #[test]
    fn test_dialog_event_opened() {
        let event = DialogEvent::Opened("test".to_string());
        assert!(matches!(event, DialogEvent::Opened(_)));
    }

    #[test]
    fn test_dialog_event_closed() {
        let event = DialogEvent::Closed("test".to_string(), DialogResult::Ok);
        assert!(matches!(event, DialogEvent::Closed(_, _)));
    }

    #[test]
    fn test_dialog_event_input_changed() {
        let event =
            DialogEvent::InputChanged("test".to_string(), "field".to_string(), "value".to_string());
        assert!(matches!(event, DialogEvent::InputChanged(_, _, _)));
    }

    #[test]
    fn test_dialog_event_submitted() {
        let event =
            DialogEvent::Submitted("test".to_string(), InputDialogResult::new(DialogResult::Ok));
        assert!(matches!(event, DialogEvent::Submitted(_, _)));
    }

    #[test]
    fn test_dialog_event_all_closed() {
        let event = DialogEvent::AllClosed;
        assert!(matches!(event, DialogEvent::AllClosed));
    }

    // ========== Dialog Enums Tests ==========

    #[test]
    fn test_dialog_button_debug() {
        assert_eq!(format!("{:?}", DialogButton::Ok), "Ok");
        assert_eq!(format!("{:?}", DialogButton::Cancel), "Cancel");
        assert_eq!(format!("{:?}", DialogButton::Yes), "Yes");
        assert_eq!(format!("{:?}", DialogButton::No), "No");
        assert_eq!(format!("{:?}", DialogButton::Close), "Close");
        assert_eq!(format!("{:?}", DialogButton::Custom), "Custom");
    }

    #[test]
    fn test_dialog_result_debug() {
        assert_eq!(format!("{:?}", DialogResult::Ok), "Ok");
        assert_eq!(format!("{:?}", DialogResult::Cancel), "Cancel");
        assert_eq!(format!("{:?}", DialogResult::Yes), "Yes");
        assert_eq!(format!("{:?}", DialogResult::No), "No");
        assert_eq!(format!("{:?}", DialogResult::Closed), "Closed");
    }

    #[test]
    fn test_dialog_icon_debug() {
        assert_eq!(format!("{:?}", DialogIcon::None), "None");
        assert_eq!(format!("{:?}", DialogIcon::Info), "Info");
        assert_eq!(format!("{:?}", DialogIcon::Warning), "Warning");
        assert_eq!(format!("{:?}", DialogIcon::Error), "Error");
        assert_eq!(format!("{:?}", DialogIcon::Question), "Question");
        assert_eq!(format!("{:?}", DialogIcon::Success), "Success");
    }

    #[test]
    fn test_dialog_style_debug() {
        assert_eq!(format!("{:?}", DialogStyle::Standard), "Standard");
        assert_eq!(format!("{:?}", DialogStyle::Primary), "Primary");
        assert_eq!(format!("{:?}", DialogStyle::Danger), "Danger");
    }

    #[test]
    fn test_input_type_debug() {
        assert_eq!(format!("{:?}", InputType::Text), "Text");
        assert_eq!(format!("{:?}", InputType::TextArea), "TextArea");
        assert_eq!(format!("{:?}", InputType::Number), "Number");
        assert_eq!(format!("{:?}", InputType::Password), "Password");
        assert_eq!(format!("{:?}", InputType::File), "File");
        assert_eq!(format!("{:?}", InputType::Directory), "Directory");
    }

    // ========== Predefined Dialogs Tests ==========

    #[test]
    fn test_predefined_dialogs_info() {
        let config = PredefinedDialogs::info("test", "Info", "This is an info message");
        assert_eq!(config.title, "Info");
        assert_eq!(config.message, "This is an info message");
        assert_eq!(config.icon, DialogIcon::Info);
    }

    #[test]
    fn test_predefined_dialogs_warning() {
        let config = PredefinedDialogs::warning("test", "Warning", "This is a warning");
        assert_eq!(config.icon, DialogIcon::Warning);
    }

    #[test]
    fn test_predefined_dialogs_error() {
        let config = PredefinedDialogs::error("test", "Error", "This is an error");
        assert_eq!(config.icon, DialogIcon::Error);
    }

    #[test]
    fn test_predefined_dialogs_success() {
        let config = PredefinedDialogs::success("test", "Success", "Operation succeeded");
        assert_eq!(config.icon, DialogIcon::Success);
    }

    #[test]
    fn test_predefined_dialogs_confirm() {
        let config = PredefinedDialogs::confirm("test", "Confirm", "Are you sure?", "Yes", "No");
        assert_eq!(config.buttons.len(), 2);
        assert_eq!(config.buttons[0].text, "No");
        assert_eq!(config.buttons[1].text, "Yes");
    }

    #[test]
    fn test_predefined_dialogs_yes_no() {
        let config = PredefinedDialogs::yes_no("test", "Question", "Do you want to continue?");
        assert_eq!(config.icon, DialogIcon::Question);
    }
}
