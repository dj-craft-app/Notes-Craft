//! Command Palette Module
//!
//! This module provides a command palette system for executing commands via keyboard shortcuts.
//! It allows users to quickly search and execute commands without using the mouse.

use std::cell::RefCell;
use std::collections::HashMap;

use slint::SharedString;

/// Represents a command that can be executed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    /// Unique identifier for the command
    pub id: String,
    /// Display name for the command
    pub name: String,
    /// Description of what the command does
    pub description: String,
    /// Category/group the command belongs to
    pub category: String,
    /// Keyboard shortcut (optional)
    pub shortcut: Option<String>,
    /// Whether the command is visible in the palette
    pub visible: bool,
}

impl Command {
    /// Creates a new command
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            category: "General".to_string(),
            shortcut: None,
            visible: true,
        }
    }

    /// Creates a new command with category
    pub fn with_category(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            category: category.into(),
            shortcut: None,
            visible: true,
        }
    }

    /// Sets the keyboard shortcut for this command
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Sets visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Checks if the command matches a search query
    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.name.to_lowercase().contains(&query)
            || self.description.to_lowercase().contains(&query)
            || self.id.to_lowercase().contains(&query)
            || self.category.to_lowercase().contains(&query)
    }
}

/// Result of executing a command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    /// Command executed successfully
    Success,
    /// Command failed with an error message
    Error(String),
    /// Command was cancelled
    Cancelled,
}

/// Action that a command can perform
pub type CommandAction = Box<dyn Fn() -> CommandResult + Send + Sync>;

/// Extended command with action
pub struct ExecutableCommand {
    pub command: Command,
    pub action: Option<CommandAction>,
}

impl ExecutableCommand {
    /// Creates a new executable command
    pub fn new(
        command: Command,
        action: impl Fn() -> CommandResult + Send + Sync + 'static,
    ) -> Self {
        Self {
            command,
            action: Some(Box::new(action)),
        }
    }

    /// Executes the command's action
    pub fn execute(&self) -> CommandResult {
        match &self.action {
            Some(action) => action(),
            None => CommandResult::Error("No action defined for command".to_string()),
        }
    }

    /// Checks if the command matches a search query
    pub fn matches(&self, query: &str) -> bool {
        self.command.matches(query)
    }
}

/// State of the command palette
#[derive(Debug, Clone)]
pub struct CommandPaletteState {
    /// Whether the palette is currently open
    pub is_open: bool,
    /// Current search query
    pub search_query: String,
    /// Index of the currently selected command
    pub selected_index: usize,
    /// Whether to show keyboard shortcuts in the palette
    pub show_shortcuts: bool,
}

impl Default for CommandPaletteState {
    fn default() -> Self {
        Self {
            is_open: false,
            search_query: String::new(),
            selected_index: 0,
            show_shortcuts: true,
        }
    }
}

impl CommandPaletteState {
    /// Creates a new state
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens the palette
    pub fn open(mut self) -> Self {
        self.is_open = true;
        self.search_query.clear();
        self.selected_index = 0;
        self
    }

    /// Closes the palette
    pub fn close(mut self) -> Self {
        self.is_open = false;
        self
    }

    /// Sets the search query
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.search_query = query.into();
        self.selected_index = 0;
        self
    }

    /// Updates the selected index
    pub fn select_next(mut self) -> Self {
        self.selected_index = self.selected_index.saturating_add(1);
        self
    }

    /// Updates the selected index
    pub fn select_previous(mut self) -> Self {
        self.selected_index = self.selected_index.saturating_sub(1);
        self
    }

    /// Resets the selection
    pub fn reset_selection(mut self) -> Self {
        self.selected_index = 0;
        self
    }
}

/// Event related to the command palette
#[derive(Debug, Clone, PartialEq)]
pub enum CommandPaletteEvent {
    /// Palette was opened
    Opened,
    /// Palette was closed
    Closed,
    /// Search query changed
    QueryChanged(String),
    /// Command was selected (by index)
    CommandSelected(usize),
    /// Command was executed
    CommandExecuted(String),
    /// Selection changed
    SelectionChanged(usize),
    /// Shortcut triggered
    ShortcutTriggered(String),
}

/// Manager for the command palette system
pub struct CommandPaletteManager {
    /// All registered commands
    commands: HashMap<String, ExecutableCommand>,
    /// Command categories for organization
    categories: Vec<String>,
    /// Current state
    state: CommandPaletteState,
    /// Event handler
    #[allow(clippy::type_complexity)]
    event_handler: Option<RefCell<Box<dyn FnMut(CommandPaletteEvent) + Send + Sync>>>,
}

impl CommandPaletteManager {
    /// Creates a new command palette manager
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            categories: Vec::new(),
            state: CommandPaletteState::default(),
            event_handler: None,
        }
    }

    /// Creates a manager with default commands
    pub fn with_defaults() -> Self {
        let mut manager = Self::new();
        manager.register_default_commands();
        manager
    }

    /// Registers a command
    pub fn register_command(&mut self, command: ExecutableCommand) -> &mut Self {
        if command.command.visible {
            // Add category if not exists
            if !self.categories.contains(&command.command.category) {
                self.categories.push(command.command.category.clone());
                self.categories.sort();
            }
        }
        self.commands.insert(command.command.id.clone(), command);
        self
    }

    /// Registers multiple commands
    pub fn register_commands(
        &mut self,
        commands: impl IntoIterator<Item = ExecutableCommand>,
    ) -> &mut Self {
        for command in commands {
            self.register_command(command);
        }
        self
    }

    /// Registers default commands
    pub fn register_default_commands(&mut self) -> &mut Self {
        // File commands
        self.register_command(ExecutableCommand::new(
            Command::with_category("file.new", "New File", "Create a new file", "File")
                .with_shortcut("Ctrl+N"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category("file.open", "Open File", "Open an existing file", "File")
                .with_shortcut("Ctrl+O"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category("file.save", "Save", "Save the current file", "File")
                .with_shortcut("Ctrl+S"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category("file.save_as", "Save As", "Save with a new name", "File"),
            || CommandResult::Success,
        ));

        // Edit commands
        self.register_command(ExecutableCommand::new(
            Command::with_category("edit.undo", "Undo", "Undo last action", "Edit")
                .with_shortcut("Ctrl+Z"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category("edit.redo", "Redo", "Redo last undone action", "Edit")
                .with_shortcut("Ctrl+Y"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category("edit.cut", "Cut", "Cut selected content", "Edit")
                .with_shortcut("Ctrl+X"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category("edit.copy", "Copy", "Copy selected content", "Edit")
                .with_shortcut("Ctrl+C"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category("edit.paste", "Paste", "Paste from clipboard", "Edit")
                .with_shortcut("Ctrl+V"),
            || CommandResult::Success,
        ));

        // View commands
        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "view.toggle_sidebar",
                "Toggle Sidebar",
                "Show/hide sidebar",
                "View",
            )
            .with_shortcut("Ctrl+B"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "view.toggle_command_palette",
                "Toggle Command Palette",
                "Show/hide command palette",
                "View",
            )
            .with_shortcut("Ctrl+P"),
            || CommandResult::Success,
        ));

        // Navigation commands
        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "nav.go_back",
                "Go Back",
                "Navigate to previous page",
                "Navigation",
            )
            .with_shortcut("Alt+Left"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "nav.go_forward",
                "Go Forward",
                "Navigate to next page",
                "Navigation",
            )
            .with_shortcut("Alt+Right"),
            || CommandResult::Success,
        ));

        // Block commands
        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "block.create_heading",
                "Create Heading",
                "Create a new heading block",
                "Blocks",
            )
            .with_shortcut("Ctrl+Alt+H"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "block.create_list",
                "Create List",
                "Create a new list block",
                "Blocks",
            )
            .with_shortcut("Ctrl+Alt+L"),
            || CommandResult::Success,
        ));

        // Page commands
        self.register_command(ExecutableCommand::new(
            Command::with_category("page.new", "New Page", "Create a new page", "Page")
                .with_shortcut("Ctrl+Shift+N"),
            || CommandResult::Success,
        ));

        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "page.delete",
                "Delete Page",
                "Delete the current page",
                "Page",
            ),
            || CommandResult::Success,
        ));

        // Settings commands
        self.register_command(ExecutableCommand::new(
            Command::with_category(
                "settings.open",
                "Open Settings",
                "Open application settings",
                "Settings",
            )
            .with_shortcut("Ctrl+, "),
            || CommandResult::Success,
        ));

        self
    }

    /// Gets all commands
    pub fn get_commands(&self) -> Vec<&ExecutableCommand> {
        self.commands.values().collect()
    }

    /// Gets commands matching a query
    pub fn search_commands(&self, query: &str) -> Vec<&ExecutableCommand> {
        let query = query.to_lowercase();
        self.commands
            .values()
            .filter(|cmd| cmd.command.visible && cmd.matches(&query))
            .collect()
    }

    /// Gets commands by category
    pub fn get_commands_by_category(&self, category: &str) -> Vec<&ExecutableCommand> {
        self.commands
            .values()
            .filter(|cmd| cmd.command.visible && cmd.command.category == category)
            .collect()
    }

    /// Gets all categories
    pub fn get_categories(&self) -> &[String] {
        &self.categories
    }

    /// Finds a command by ID
    pub fn get_command(&self, id: &str) -> Option<&ExecutableCommand> {
        self.commands.get(id)
    }

    /// Finds a command by shortcut
    pub fn get_command_by_shortcut(&self, shortcut: &str) -> Option<&ExecutableCommand> {
        self.commands
            .values()
            .find(|cmd| cmd.command.shortcut.as_deref() == Some(shortcut))
    }

    /// Executes a command by ID
    pub fn execute_command(&self, id: &str) -> CommandResult {
        match self.commands.get(id) {
            Some(cmd) => {
                let result = cmd.execute();
                if let Some(handler) = &self.event_handler {
                    let mut handler = handler.borrow_mut();
                    handler(CommandPaletteEvent::CommandExecuted(id.to_string()));
                }
                result
            }
            None => CommandResult::Error(format!("Command '{}' not found", id)),
        }
    }

    /// Executes a command by index from filtered list
    pub fn execute_command_at_index(&self, query: &str, index: usize) -> CommandResult {
        let filtered = self.search_commands(query);
        if let Some(cmd) = filtered.get(index) {
            self.execute_command(&cmd.command.id)
        } else {
            CommandResult::Error("Invalid command index".to_string())
        }
    }

    /// Opens the palette
    pub fn open(&mut self) {
        self.state = self.state.clone().open();
        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(CommandPaletteEvent::Opened);
        }
    }

    /// Closes the palette
    pub fn close(&mut self) {
        self.state = self.state.clone().close();
        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(CommandPaletteEvent::Closed);
        }
    }

    /// Sets the search query
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.state.search_query = query.into();
        self.state.selected_index = 0;
        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(CommandPaletteEvent::QueryChanged(
                self.state.search_query.clone(),
            ));
        }
    }

    /// Updates selection
    pub fn select_next(&mut self) {
        let filtered = self.search_commands(&self.state.search_query);
        if !filtered.is_empty() {
            self.state.selected_index = (self.state.selected_index + 1) % filtered.len();
            if let Some(handler) = &self.event_handler {
                let mut handler = handler.borrow_mut();
                handler(CommandPaletteEvent::SelectionChanged(
                    self.state.selected_index,
                ));
            }
        }
    }

    /// Updates selection
    pub fn select_previous(&mut self) {
        let filtered = self.search_commands(&self.state.search_query);
        if !filtered.is_empty() {
            self.state.selected_index = if self.state.selected_index == 0 {
                filtered.len() - 1
            } else {
                self.state.selected_index - 1
            };
            if let Some(handler) = &self.event_handler {
                let mut handler = handler.borrow_mut();
                handler(CommandPaletteEvent::SelectionChanged(
                    self.state.selected_index,
                ));
            }
        }
    }

    /// Gets the current state
    pub fn state(&self) -> &CommandPaletteState {
        &self.state
    }

    /// Gets mutable state
    pub fn state_mut(&mut self) -> &mut CommandPaletteState {
        &mut self.state
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: FnMut(CommandPaletteEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(RefCell::new(Box::new(handler)));
        self
    }

    /// Handles an event
    pub fn handle_event(&mut self, event: CommandPaletteEvent) {
        match &event {
            CommandPaletteEvent::Opened => self.open(),
            CommandPaletteEvent::Closed => self.close(),
            CommandPaletteEvent::QueryChanged(query) => self.set_query(query.clone()),
            CommandPaletteEvent::CommandSelected(index) => {
                let _ = self.execute_command_at_index(&self.state.search_query, *index);
            }
            CommandPaletteEvent::CommandExecuted(_) => {}
            CommandPaletteEvent::SelectionChanged(_) => {}
            CommandPaletteEvent::ShortcutTriggered(shortcut) => {
                if let Some(cmd) = self.get_command_by_shortcut(shortcut) {
                    let _ = cmd.execute();
                }
            }
        }

        if let Some(handler) = &self.event_handler {
            let mut handler = handler.borrow_mut();
            handler(event);
        }
    }

    /// Gets filtered commands based on current query
    pub fn get_filtered_commands(&self) -> Vec<&ExecutableCommand> {
        self.search_commands(&self.state.search_query)
    }

    /// Gets the currently selected command
    pub fn get_selected_command(&self) -> Option<&ExecutableCommand> {
        let filtered = self.get_filtered_commands();
        filtered.get(self.state.selected_index).copied()
    }
}

impl Default for CommandPaletteManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating command palette managers
#[derive(Default)]
pub struct CommandPaletteBuilder {
    commands: Vec<ExecutableCommand>,
}

impl CommandPaletteBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a command
    pub fn add_command(mut self, command: ExecutableCommand) -> Self {
        self.commands.push(command);
        self
    }

    /// Adds multiple commands
    pub fn add_commands(mut self, commands: impl IntoIterator<Item = ExecutableCommand>) -> Self {
        self.commands.extend(commands);
        self
    }

    /// Builds the manager
    pub fn build(self) -> CommandPaletteManager {
        let mut manager = CommandPaletteManager::new();
        manager.register_commands(self.commands);
        manager
    }
}

// ============================================================================
// Slint Integration
// ============================================================================

/// Creates a Slint-compatible model for command palette
pub fn create_command_palette_model(
    manager: &CommandPaletteManager,
    query: &str,
) -> Vec<HashMap<String, SharedString>> {
    let commands = manager.search_commands(query);
    commands
        .into_iter()
        .map(|cmd| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), SharedString::from(&cmd.command.id));
            map.insert("name".to_string(), SharedString::from(&cmd.command.name));
            map.insert(
                "description".to_string(),
                SharedString::from(&cmd.command.description),
            );
            map.insert(
                "category".to_string(),
                SharedString::from(&cmd.command.category),
            );
            if let Some(shortcut) = &cmd.command.shortcut {
                map.insert("shortcut".to_string(), SharedString::from(shortcut));
            }
            map
        })
        .collect()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("test.id", "Test Command", "Test description");
        assert_eq!(cmd.id, "test.id");
        assert_eq!(cmd.name, "Test Command");
        assert_eq!(cmd.description, "Test description");
        assert_eq!(cmd.category, "General");
        assert!(cmd.visible);
        assert!(cmd.shortcut.is_none());
    }

    #[test]
    fn test_command_with_category() {
        let cmd = Command::with_category("test.id", "Test", "Desc", "Custom");
        assert_eq!(cmd.category, "Custom");
    }

    #[test]
    fn test_command_with_shortcut() {
        let cmd = Command::new("test.id", "Test", "Desc").with_shortcut("Ctrl+T");
        assert_eq!(cmd.shortcut, Some("Ctrl+T".to_string()));
    }

    #[test]
    fn test_command_matches() {
        let cmd = Command::new("file.save", "Save File", "Saves the current file");

        assert!(cmd.matches("save"));
        assert!(cmd.matches("SAVE"));
        assert!(cmd.matches("file"));
        assert!(cmd.matches("File"));
        assert!(cmd.matches("current"));
        assert!(!cmd.matches("open"));
    }

    #[test]
    fn test_executable_command_execute() {
        let cmd = ExecutableCommand::new(Command::new("test", "Test", "Desc"), || {
            CommandResult::Success
        });

        assert!(matches!(cmd.execute(), CommandResult::Success));
    }

    #[test]
    fn test_command_palette_state() {
        let state = CommandPaletteState::new();
        assert!(!state.is_open);
        assert!(state.search_query.is_empty());
        assert_eq!(state.selected_index, 0);

        let state = state.open();
        assert!(state.is_open);

        let state = state.with_query("test");
        assert_eq!(state.search_query, "test");
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_command_palette_manager_new() {
        let manager = CommandPaletteManager::new();
        assert!(manager.get_commands().is_empty());
        assert!(manager.get_categories().is_empty());
    }

    #[test]
    fn test_command_palette_manager_with_defaults() {
        let manager = CommandPaletteManager::with_defaults();
        assert!(!manager.get_commands().is_empty());
        assert!(!manager.get_categories().is_empty());
    }

    #[test]
    fn test_command_palette_register_command() {
        let mut manager = CommandPaletteManager::new();
        let cmd = ExecutableCommand::new(Command::new("test", "Test", "Desc"), || {
            CommandResult::Success
        });

        manager.register_command(cmd);
        assert_eq!(manager.get_commands().len(), 1);
    }

    #[test]
    fn test_command_palette_search() {
        let manager = CommandPaletteManager::with_defaults();
        let results = manager.search_commands("save");
        assert!(!results.is_empty());

        for cmd in results {
            assert!(
                cmd.command.name.to_lowercase().contains("save")
                    || cmd.command.description.to_lowercase().contains("save")
            );
        }
    }

    #[test]
    fn test_command_palette_get_by_category() {
        let manager = CommandPaletteManager::with_defaults();
        let file_cmds = manager.get_commands_by_category("File");
        assert!(!file_cmds.is_empty());

        for cmd in file_cmds {
            assert_eq!(cmd.command.category, "File");
        }
    }

    #[test]
    fn test_command_palette_execute() {
        let mut manager = CommandPaletteManager::new();
        manager.register_command(ExecutableCommand::new(
            Command::new("test", "Test", "Desc"),
            || CommandResult::Success,
        ));

        assert!(matches!(
            manager.execute_command("test"),
            CommandResult::Success
        ));
        assert!(matches!(
            manager.execute_command("nonexistent"),
            CommandResult::Error(_)
        ));
    }

    #[test]
    fn test_command_palette_state_management() {
        let mut manager = CommandPaletteManager::new();

        manager.open();
        assert!(manager.state().is_open);

        manager.close();
        assert!(!manager.state().is_open);

        manager.set_query("test");
        assert_eq!(manager.state().search_query, "test");
    }

    #[test]
    fn test_command_palette_selection() {
        let manager = CommandPaletteManager::with_defaults();
        let filtered = manager.search_commands("");

        assert!(!filtered.is_empty());

        let mut manager = CommandPaletteManager::with_defaults();
        manager.set_query("");

        manager.select_next();
        assert!(manager.state().selected_index > 0);

        manager.select_previous();
        assert_eq!(manager.state().selected_index, 0);
    }

    #[test]
    fn test_command_palette_get_selected() {
        let mut manager = CommandPaletteManager::with_defaults();
        manager.set_query("");

        let selected = manager.get_selected_command();
        assert!(selected.is_some());
    }

    #[test]
    fn test_command_palette_builder() {
        let builder = CommandPaletteBuilder::new()
            .add_command(ExecutableCommand::new(
                Command::new("test1", "Test 1", "Desc 1"),
                || CommandResult::Success,
            ))
            .add_command(ExecutableCommand::new(
                Command::new("test2", "Test 2", "Desc 2"),
                || CommandResult::Success,
            ));

        let manager = builder.build();
        assert_eq!(manager.get_commands().len(), 2);
    }

    #[test]
    fn test_command_palette_event_handling() {
        use std::sync::{Arc, Mutex};

        let mut manager = CommandPaletteManager::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        manager.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        manager.open();
        assert!(!events.lock().unwrap().is_empty());
        assert!(matches!(
            events.lock().unwrap().last().unwrap(),
            CommandPaletteEvent::Opened
        ));

        manager.close();
        assert!(matches!(
            events.lock().unwrap().last().unwrap(),
            CommandPaletteEvent::Closed
        ));
    }

    #[test]
    fn test_default_commands_registered() {
        let manager = CommandPaletteManager::with_defaults();

        // Check file commands
        assert!(manager.get_command("file.new").is_some());
        assert!(manager.get_command("file.open").is_some());
        assert!(manager.get_command("file.save").is_some());

        // Check edit commands
        assert!(manager.get_command("edit.undo").is_some());
        assert!(manager.get_command("edit.redo").is_some());
        assert!(manager.get_command("edit.cut").is_some());
        assert!(manager.get_command("edit.copy").is_some());
        assert!(manager.get_command("edit.paste").is_some());

        // Check view commands
        assert!(manager.get_command("view.toggle_sidebar").is_some());
        assert!(manager.get_command("view.toggle_command_palette").is_some());

        // Check navigation commands
        assert!(manager.get_command("nav.go_back").is_some());
        assert!(manager.get_command("nav.go_forward").is_some());
    }

    #[test]
    fn test_command_by_shortcut() {
        let manager = CommandPaletteManager::with_defaults();

        // These shortcuts should exist in default commands
        let save_cmd = manager.get_command_by_shortcut("Ctrl+S");
        assert!(save_cmd.is_some());
        assert_eq!(save_cmd.unwrap().command.id, "file.save");

        let undo_cmd = manager.get_command_by_shortcut("Ctrl+Z");
        assert!(undo_cmd.is_some());
        assert_eq!(undo_cmd.unwrap().command.id, "edit.undo");
    }
}
