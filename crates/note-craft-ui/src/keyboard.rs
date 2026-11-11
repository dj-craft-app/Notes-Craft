//! Keyboard Shortcuts Module
//!
//! This module provides a keyboard shortcut system for Note Craft.
//! It allows binding actions to key combinations and handling keyboard input globally.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

/// Represents a physical key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Character keys (A-Z, a-z, 0-9, etc.)
    Character(char),
    /// Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    /// Arrow keys
    Up,
    Down,
    Left,
    Right,
    /// Modifier keys
    Shift,
    Control,
    Alt,
    Meta,
    /// Navigation keys
    Backspace,
    Delete,
    Tab,
    Enter,
    Escape,
    /// Special keys
    Space,
    Home,
    End,
    PageUp,
    PageDown,
    /// Number pad keys
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Character(c) => write!(f, "{}", c),
            Key::F1 => write!(f, "F1"),
            Key::F2 => write!(f, "F2"),
            Key::F3 => write!(f, "F3"),
            Key::F4 => write!(f, "F4"),
            Key::F5 => write!(f, "F5"),
            Key::F6 => write!(f, "F6"),
            Key::F7 => write!(f, "F7"),
            Key::F8 => write!(f, "F8"),
            Key::F9 => write!(f, "F9"),
            Key::F10 => write!(f, "F10"),
            Key::F11 => write!(f, "F11"),
            Key::F12 => write!(f, "F12"),
            Key::Up => write!(f, "↑"),
            Key::Down => write!(f, "↓"),
            Key::Left => write!(f, "←"),
            Key::Right => write!(f, "→"),
            Key::Shift => write!(f, "Shift"),
            Key::Control => write!(f, "Ctrl"),
            Key::Alt => write!(f, "Alt"),
            Key::Meta => write!(f, "Meta"),
            Key::Backspace => write!(f, "Backspace"),
            Key::Delete => write!(f, "Delete"),
            Key::Tab => write!(f, "Tab"),
            Key::Enter => write!(f, "Enter"),
            Key::Escape => write!(f, "Esc"),
            Key::Space => write!(f, "Space"),
            Key::Home => write!(f, "Home"),
            Key::End => write!(f, "End"),
            Key::PageUp => write!(f, "PageUp"),
            Key::PageDown => write!(f, "PageDown"),
            _ => write!(f, "?"),
        }
    }
}

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" | "↑" => Ok(Self::Up),
            "down" | "↓" => Ok(Self::Down),
            "left" | "←" => Ok(Self::Left),
            "right" | "→" => Ok(Self::Right),
            "shift" => Ok(Self::Shift),
            "ctrl" | "control" => Ok(Self::Control),
            "alt" => Ok(Self::Alt),
            "meta" | "cmd" | "command" => Ok(Self::Meta),
            "backspace" => Ok(Self::Backspace),
            "delete" | "del" => Ok(Self::Delete),
            "tab" => Ok(Self::Tab),
            "enter" | "return" => Ok(Self::Enter),
            "escape" | "esc" => Ok(Self::Escape),
            "space" => Ok(Self::Space),
            "home" => Ok(Self::Home),
            "end" => Ok(Self::End),
            "pageup" => Ok(Self::PageUp),
            "pagedown" => Ok(Self::PageDown),
            "f1" => Ok(Self::F1),
            "f2" => Ok(Self::F2),
            "f3" => Ok(Self::F3),
            "f4" => Ok(Self::F4),
            "f5" => Ok(Self::F5),
            "f6" => Ok(Self::F6),
            "f7" => Ok(Self::F7),
            "f8" => Ok(Self::F8),
            "f9" => Ok(Self::F9),
            "f10" => Ok(Self::F10),
            "f11" => Ok(Self::F11),
            "f12" => Ok(Self::F12),
            c if c.len() == 1 => Ok(Self::Character(c.chars().next().unwrap())),
            _ => Err(format!("Unknown key: {}", s)),
        }
    }
}

impl Key {
    /// Parses a key from a string (fallible)
    pub fn parse(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}

/// Keyboard modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.meta {
            parts.push("Meta");
        }
        write!(f, "{}", parts.join("+"))
    }
}

impl Modifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            meta: false,
        }
    }

    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: false,
            meta: false,
        }
    }

    pub fn shift() -> Self {
        Self {
            ctrl: false,
            shift: true,
            alt: false,
            meta: false,
        }
    }

    pub fn alt() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: true,
            meta: false,
        }
    }

    pub fn meta() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            meta: true,
        }
    }

    pub fn ctrl_shift() -> Self {
        Self {
            ctrl: true,
            shift: true,
            alt: false,
            meta: false,
        }
    }

    pub fn ctrl_alt() -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: true,
            meta: false,
        }
    }

    pub fn alt_shift() -> Self {
        Self {
            ctrl: false,
            shift: true,
            alt: true,
            meta: false,
        }
    }

    pub fn with_ctrl(mut self, ctrl: bool) -> Self {
        self.ctrl = ctrl;
        self
    }

    pub fn with_shift(mut self, shift: bool) -> Self {
        self.shift = shift;
        self
    }

    pub fn with_alt(mut self, alt: bool) -> Self {
        self.alt = alt;
        self
    }

    pub fn with_meta(mut self, meta: bool) -> Self {
        self.meta = meta;
        self
    }
}

/// Keyboard shortcut (key + modifiers)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shortcut {
    pub key: Key,
    pub modifiers: Modifiers,
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers == Modifiers::empty() {
            write!(f, "{}", self.key)
        } else {
            write!(f, "{} {}", self.modifiers, self.key)
        }
    }
}

impl FromStr for Shortcut {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('+').map(|s| s.trim()).collect();

        let mut modifiers = Modifiers::empty();
        let mut key_str = String::new();

        for (i, part) in parts.iter().enumerate() {
            if i < parts.len() - 1 {
                // This should be a modifier
                match part.to_lowercase().as_str() {
                    "ctrl" | "control" => modifiers.ctrl = true,
                    "shift" => modifiers.shift = true,
                    "alt" => modifiers.alt = true,
                    "meta" | "cmd" | "command" => modifiers.meta = true,
                    _ => {
                        // Not a modifier, treat as part of key
                        if !key_str.is_empty() {
                            key_str.push('+');
                        }
                        key_str.push_str(part);
                    }
                }
            } else {
                // This is the key
                if !key_str.is_empty() {
                    key_str.push('+');
                }
                key_str.push_str(part);
            }
        }

        Ok(Self {
            key: Key::parse(&key_str).ok_or_else(|| format!("Invalid key: {}", key_str))?,
            modifiers,
        })
    }
}

impl Shortcut {
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { key, modifiers }
    }

    /// Parses a shortcut from a string (fallible)
    pub fn parse(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }

    pub fn with_key(mut self, key: Key) -> Self {
        self.key = key;
        self
    }

    pub fn with_modifiers(mut self, modifiers: Modifiers) -> Self {
        self.modifiers = modifiers;
        self
    }
}

/// Action to execute when shortcut is triggered
pub type ShortcutAction = Box<dyn Fn() + Send + Sync>;

/// Shortcut binding with action
pub struct ShortcutBinding {
    pub shortcut: Shortcut,
    pub action: Option<ShortcutAction>,
    pub description: String,
    pub category: String,
    pub enabled: bool,
}

impl std::fmt::Debug for ShortcutBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShortcutBinding")
            .field("shortcut", &self.shortcut)
            .field("description", &self.description)
            .field("category", &self.category)
            .field("enabled", &self.enabled)
            .field(
                "action",
                &if self.action.is_some() {
                    "Some(...)"
                } else {
                    "None"
                },
            )
            .finish()
    }
}

impl Clone for ShortcutBinding {
    fn clone(&self) -> Self {
        Self {
            shortcut: self.shortcut.clone(),
            action: None, // Actions cannot be cloned
            description: self.description.clone(),
            category: self.category.clone(),
            enabled: self.enabled,
        }
    }
}

impl ShortcutBinding {
    pub fn new(shortcut: Shortcut, description: impl Into<String>) -> Self {
        Self {
            shortcut,
            action: None,
            description: description.into(),
            category: "General".to_string(),
            enabled: true,
        }
    }

    pub fn with_action(mut self, action: impl Fn() + Send + Sync + 'static) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn execute(&self) {
        if self.enabled {
            if let Some(action) = &self.action {
                action();
            }
        }
    }

    pub fn matches(&self, shortcut: &Shortcut) -> bool {
        self.enabled && self.shortcut == *shortcut
    }
}

/// Shortcut context (global, editor, sidebar, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShortcutContext {
    /// Global shortcuts (always active)
    Global,
    /// Editor shortcuts (active when editor has focus)
    Editor,
    /// Sidebar shortcuts (active when sidebar has focus)
    Sidebar,
    /// Dialog shortcuts (active when dialog is open)
    Dialog,
    /// Command palette shortcuts (active when command palette is open)
    CommandPalette,
    /// Custom context
    Custom(String),
}

impl fmt::Display for ShortcutContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShortcutContext::Global => write!(f, "Global"),
            ShortcutContext::Editor => write!(f, "Editor"),
            ShortcutContext::Sidebar => write!(f, "Sidebar"),
            ShortcutContext::Dialog => write!(f, "Dialog"),
            ShortcutContext::CommandPalette => write!(f, "CommandPalette"),
            ShortcutContext::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Event related to keyboard shortcuts
#[derive(Debug, Clone, PartialEq)]
pub enum ShortcutEvent {
    /// Shortcut was pressed
    Pressed(Shortcut),
    /// Shortcut was executed
    Executed(Shortcut),
    /// Shortcut context changed
    ContextChanged(ShortcutContext),
    /// Shortcut was registered
    Registered(Shortcut),
    /// Shortcut was unregistered
    Unregistered(Shortcut),
}

/// Manager for keyboard shortcuts
pub struct ShortcutManager {
    /// All registered bindings by context
    bindings: HashMap<ShortcutContext, Vec<ShortcutBinding>>,
    /// Active context
    active_context: ShortcutContext,
    /// Context stack (for nested contexts)
    context_stack: Vec<ShortcutContext>,
    /// Global shortcuts (always active)
    global_bindings: Vec<ShortcutBinding>,
    /// Pressed keys (for tracking multi-key shortcuts)
    pressed_keys: HashSet<Key>,
    /// Event handler
    event_handler: Option<Box<dyn Fn(ShortcutEvent) + Send + Sync>>,
}

impl ShortcutManager {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            active_context: ShortcutContext::Global,
            context_stack: Vec::new(),
            global_bindings: Vec::new(),
            pressed_keys: HashSet::new(),
            event_handler: None,
        }
    }

    /// Registers a binding in a specific context
    pub fn register(&mut self, context: ShortcutContext, binding: ShortcutBinding) -> &mut Self {
        let shortcut_clone = binding.shortcut.clone();

        if context == ShortcutContext::Global {
            self.global_bindings.push(binding);
        } else {
            self.bindings
                .entry(context.clone())
                .or_default()
                .push(binding);
        }

        if let Some(handler) = &self.event_handler {
            handler(ShortcutEvent::Registered(shortcut_clone));
        }

        self
    }

    /// Registers a global binding
    pub fn register_global(&mut self, binding: ShortcutBinding) -> &mut Self {
        self.register(ShortcutContext::Global, binding)
    }

    /// Registers a binding in the editor context
    pub fn register_editor(&mut self, binding: ShortcutBinding) -> &mut Self {
        self.register(ShortcutContext::Editor, binding)
    }

    /// Registers a binding in the sidebar context
    pub fn register_sidebar(&mut self, binding: ShortcutBinding) -> &mut Self {
        self.register(ShortcutContext::Sidebar, binding)
    }

    /// Registers multiple bindings
    pub fn register_all(
        &mut self,
        context: ShortcutContext,
        bindings: impl IntoIterator<Item = ShortcutBinding>,
    ) -> &mut Self {
        for binding in bindings {
            self.register(context.clone(), binding);
        }
        self
    }

    /// Unregisters a binding
    pub fn unregister(
        &mut self,
        context: ShortcutContext,
        shortcut: &Shortcut,
    ) -> Option<ShortcutBinding> {
        if context == ShortcutContext::Global {
            self.global_bindings
                .iter()
                .position(|b| &b.shortcut == shortcut)
                .map(|i| self.global_bindings.remove(i))
        } else if let Some(bindings) = self.bindings.get_mut(&context) {
            bindings
                .iter()
                .position(|b| &b.shortcut == shortcut)
                .map(|i| bindings.remove(i))
        } else {
            None
        }
    }

    /// Sets the active context
    pub fn set_context(&mut self, context: ShortcutContext) -> &mut Self {
        if self.active_context != context {
            self.context_stack.push(self.active_context.clone());
            self.active_context = context.clone();

            if let Some(handler) = &self.event_handler {
                handler(ShortcutEvent::ContextChanged(context));
            }
        }
        self
    }

    /// Pushes a context onto the stack
    pub fn push_context(&mut self, context: ShortcutContext) -> &mut Self {
        self.context_stack.push(self.active_context.clone());
        self.active_context = context.clone();

        if let Some(handler) = &self.event_handler {
            handler(ShortcutEvent::ContextChanged(context));
        }

        self
    }

    /// Pops the top context from the stack
    pub fn pop_context(&mut self) -> Option<ShortcutContext> {
        let old_context = self.active_context.clone();
        self.active_context = self.context_stack.pop().unwrap_or(ShortcutContext::Global);

        if old_context != self.active_context {
            if let Some(handler) = &self.event_handler {
                handler(ShortcutEvent::ContextChanged(self.active_context.clone()));
            }
        }

        Some(old_context)
    }

    /// Gets the active context
    pub fn active_context(&self) -> &ShortcutContext {
        &self.active_context
    }

    /// Handles a key press
    pub fn handle_key_press(&mut self, key: Key, modifiers: Modifiers) -> bool {
        let shortcut = Shortcut { key, modifiers };
        self.pressed_keys.insert(key);

        // Check global bindings first
        for binding in &self.global_bindings {
            if binding.matches(&shortcut) {
                binding.execute();
                if let Some(handler) = &self.event_handler {
                    handler(ShortcutEvent::Executed(shortcut.clone()));
                }
                return true;
            }
        }

        // Check active context bindings
        if let Some(bindings) = self.bindings.get(&self.active_context) {
            for binding in bindings {
                if binding.matches(&shortcut) {
                    binding.execute();
                    if let Some(handler) = &self.event_handler {
                        handler(ShortcutEvent::Executed(shortcut.clone()));
                    }
                    return true;
                }
            }
        }

        // Check all contexts in stack
        for context in self.context_stack.iter().rev() {
            if let Some(bindings) = self.bindings.get(context) {
                for binding in bindings {
                    if binding.matches(&shortcut) {
                        binding.execute();
                        if let Some(handler) = &self.event_handler {
                            handler(ShortcutEvent::Executed(shortcut.clone()));
                        }
                        return true;
                    }
                }
            }
        }

        // No shortcut matched
        if let Some(handler) = &self.event_handler {
            handler(ShortcutEvent::Pressed(shortcut));
        }

        false
    }

    /// Handles a key release
    pub fn handle_key_release(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    /// Clears all pressed keys
    pub fn clear_pressed_keys(&mut self) {
        self.pressed_keys.clear();
    }

    /// Gets all bindings for a context
    pub fn get_bindings(&self, context: &ShortcutContext) -> Vec<&ShortcutBinding> {
        if *context == ShortcutContext::Global {
            self.global_bindings.iter().collect()
        } else {
            self.bindings
                .get(context)
                .map(|v| v.iter().collect())
                .unwrap_or_default()
        }
    }

    /// Gets all bindings
    pub fn get_all_bindings(&self) -> Vec<&ShortcutBinding> {
        let mut result = Vec::new();
        result.extend(self.global_bindings.iter());
        for bindings in self.bindings.values() {
            result.extend(bindings.iter());
        }
        result
    }

    /// Finds a binding by shortcut
    pub fn get_binding(&self, shortcut: &Shortcut) -> Option<&ShortcutBinding> {
        // Check global
        for binding in &self.global_bindings {
            if &binding.shortcut == shortcut {
                return Some(binding);
            }
        }

        // Check all contexts
        for bindings in self.bindings.values() {
            for binding in bindings {
                if &binding.shortcut == shortcut {
                    return Some(binding);
                }
            }
        }

        None
    }

    /// Checks if a shortcut is registered
    pub fn is_registered(&self, shortcut: &Shortcut) -> bool {
        self.get_binding(shortcut).is_some()
    }

    /// Clears all bindings
    pub fn clear(&mut self) -> &mut Self {
        self.bindings.clear();
        self.global_bindings.clear();
        self.pressed_keys.clear();
        self.context_stack.clear();
        self.active_context = ShortcutContext::Global;
        self
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(ShortcutEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    /// Gets a list of all registered shortcuts with their descriptions
    pub fn get_shortcut_help(&self) -> Vec<(String, Shortcut, String, String)> {
        let mut result = Vec::new();

        // Global
        for binding in &self.global_bindings {
            result.push((
                "Global".to_string(),
                binding.shortcut.clone(),
                binding.description.clone(),
                binding.category.clone(),
            ));
        }

        // Other contexts
        for (context, bindings) in &self.bindings {
            let context_name = context.to_string();
            for binding in bindings {
                result.push((
                    context_name.clone(),
                    binding.shortcut.clone(),
                    binding.description.clone(),
                    binding.category.clone(),
                ));
            }
        }

        result
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating shortcut managers
pub struct ShortcutManagerBuilder {
    bindings: Vec<(ShortcutContext, ShortcutBinding)>,
    initial_context: ShortcutContext,
}

impl Default for ShortcutManagerBuilder {
    fn default() -> Self {
        Self {
            bindings: Vec::new(),
            initial_context: ShortcutContext::Global,
        }
    }
}

impl ShortcutManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_binding(mut self, context: ShortcutContext, binding: ShortcutBinding) -> Self {
        self.bindings.push((context, binding));
        self
    }

    pub fn with_initial_context(mut self, context: ShortcutContext) -> Self {
        self.initial_context = context;
        self
    }

    pub fn build(self) -> ShortcutManager {
        let mut manager = ShortcutManager::new();
        manager.active_context = self.initial_context.clone();
        for (context, binding) in self.bindings {
            manager.register(context, binding);
        }
        manager
    }
}

// ============================================================================
// Default Shortcuts
// ============================================================================

/// Creates default global shortcuts
pub fn create_default_global_shortcuts() -> Vec<ShortcutBinding> {
    vec![
        ShortcutBinding::new(Shortcut::parse("Ctrl+Q").unwrap(), "Quit application")
            .with_category("File"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+N").unwrap(), "New file").with_category("File"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+O").unwrap(), "Open file").with_category("File"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save file").with_category("File"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+P").unwrap(), "Open command palette")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Shift+P").unwrap(), "Open search")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+B").unwrap(), "Toggle sidebar")
            .with_category("View"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+,").unwrap(), "Open settings")
            .with_category("View"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Shift+T").unwrap(), "Toggle theme")
            .with_category("View"),
    ]
}

/// Creates default editor shortcuts
pub fn create_default_editor_shortcuts() -> Vec<ShortcutBinding> {
    vec![
        ShortcutBinding::new(Shortcut::parse("Ctrl+Z").unwrap(), "Undo").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Y").unwrap(), "Redo").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+X").unwrap(), "Cut").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+C").unwrap(), "Copy").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+V").unwrap(), "Paste").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+A").unwrap(), "Select all")
            .with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+F").unwrap(), "Find").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+H").unwrap(), "Replace").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+D").unwrap(), "Duplicate line")
            .with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+L").unwrap(), "Select line")
            .with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+/").unwrap(), "Toggle comment")
            .with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Tab").unwrap(), "Indent").with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Shift+Tab").unwrap(), "Unindent")
            .with_category("Edit"),
        ShortcutBinding::new(Shortcut::parse("Up").unwrap(), "Move cursor up")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Down").unwrap(), "Move cursor down")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Left").unwrap(), "Move cursor left")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Right").unwrap(), "Move cursor right")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Home").unwrap(), "Go to start")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+End").unwrap(), "Go to end")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Up").unwrap(), "Scroll up")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Down").unwrap(), "Scroll down")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Shift+Up").unwrap(), "Select to start")
            .with_category("Selection"),
        ShortcutBinding::new(Shortcut::parse("Ctrl+Shift+Down").unwrap(), "Select to end")
            .with_category("Selection"),
    ]
}

/// Creates default sidebar shortcuts
pub fn create_default_sidebar_shortcuts() -> Vec<ShortcutBinding> {
    vec![
        ShortcutBinding::new(Shortcut::parse("Up").unwrap(), "Select item above")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Down").unwrap(), "Select item below")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Right").unwrap(), "Expand item")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Left").unwrap(), "Collapse item")
            .with_category("Navigation"),
        ShortcutBinding::new(Shortcut::parse("Enter").unwrap(), "Open selected item")
            .with_category("Action"),
        ShortcutBinding::new(Shortcut::parse("F2").unwrap(), "Rename item").with_category("Action"),
        ShortcutBinding::new(Shortcut::parse("Delete").unwrap(), "Delete item")
            .with_category("Action"),
    ]
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_display() {
        assert_eq!(format!("{}", Key::Character('A')), "A");
        assert_eq!(format!("{}", Key::Up), "↑");
        assert_eq!(format!("{}", Key::Down), "↓");
        assert_eq!(format!("{}", Key::Control), "Ctrl");
    }

    #[test]
    fn test_key_from_str() {
        assert_eq!(Key::parse("a"), Some(Key::Character('a')));
        assert_eq!(Key::parse("Up"), Some(Key::Up));
        assert_eq!(Key::parse("up"), Some(Key::Up));
        assert_eq!(Key::parse("Ctrl"), Some(Key::Control));
        assert_eq!(Key::parse("unknown"), None);
    }

    #[test]
    fn test_modifiers_display() {
        let empty = Modifiers::empty();
        assert_eq!(format!("{}", empty), "");

        let ctrl = Modifiers::ctrl();
        assert_eq!(format!("{}", ctrl), "Ctrl");

        let ctrl_shift = Modifiers::ctrl_shift();
        assert_eq!(format!("{}", ctrl_shift), "Ctrl+Shift");
    }

    #[test]
    fn test_modifiers_constructors() {
        let ctrl = Modifiers::ctrl();
        assert!(ctrl.ctrl);
        assert!(!ctrl.shift);
        assert!(!ctrl.alt);
        assert!(!ctrl.meta);

        let ctrl_shift = Modifiers::ctrl_shift();
        assert!(ctrl_shift.ctrl);
        assert!(ctrl_shift.shift);
    }

    #[test]
    fn test_shortcut_from_str() {
        let ctrl_s = Shortcut::parse("Ctrl+S");
        assert!(ctrl_s.is_some());
        let ctrl_s = ctrl_s.unwrap();
        assert_eq!(ctrl_s.key, Key::Character('s'));
        assert!(ctrl_s.modifiers.ctrl);
        assert!(!ctrl_s.modifiers.shift);

        let shift_tab = Shortcut::parse("Shift+Tab");
        assert!(shift_tab.is_some());
        let shift_tab = shift_tab.unwrap();
        assert_eq!(shift_tab.key, Key::Tab);
        assert!(shift_tab.modifiers.shift);
    }

    #[test]
    fn test_shortcut_display() {
        let shortcut = Shortcut::new(Key::Character('s'), Modifiers::ctrl());
        assert_eq!(format!("{}", shortcut), "Ctrl s");

        let shortcut = Shortcut::new(Key::Up, Modifiers::empty());
        assert_eq!(format!("{}", shortcut), "↑");
    }

    #[test]
    fn test_shortcut_binding_new() {
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save file");
        assert_eq!(binding.description, "Save file");
        assert_eq!(binding.category, "General");
        assert!(binding.enabled);
    }

    #[test]
    fn test_shortcut_binding_with_category() {
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save file")
            .with_category("File");
        assert_eq!(binding.category, "File");
    }

    #[test]
    fn test_shortcut_context_display() {
        assert_eq!(format!("{}", ShortcutContext::Global), "Global");
        assert_eq!(format!("{}", ShortcutContext::Editor), "Editor");
        assert_eq!(format!("{}", ShortcutContext::Sidebar), "Sidebar");
    }

    #[test]
    fn test_shortcut_manager_new() {
        let manager = ShortcutManager::new();
        assert_eq!(manager.active_context(), &ShortcutContext::Global);
        assert!(manager.get_all_bindings().is_empty());
    }

    #[test]
    fn test_shortcut_manager_register() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");

        manager.register(ShortcutContext::Global, binding);
        assert_eq!(manager.get_all_bindings().len(), 1);
    }

    #[test]
    fn test_shortcut_manager_register_global() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");

        manager.register_global(binding);
        assert_eq!(manager.get_bindings(&ShortcutContext::Global).len(), 1);
    }

    #[test]
    fn test_shortcut_manager_register_editor() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+Z").unwrap(), "Undo");

        manager.register_editor(binding);
        assert_eq!(manager.get_bindings(&ShortcutContext::Editor).len(), 1);
    }

    #[test]
    fn test_shortcut_manager_set_context() {
        let mut manager = ShortcutManager::new();
        manager.set_context(ShortcutContext::Editor);
        assert_eq!(manager.active_context(), &ShortcutContext::Editor);
    }

    #[test]
    fn test_shortcut_manager_context_stack() {
        let mut manager = ShortcutManager::new();
        manager.push_context(ShortcutContext::Editor);
        assert_eq!(manager.active_context(), &ShortcutContext::Editor);

        manager.push_context(ShortcutContext::Dialog);
        assert_eq!(manager.active_context(), &ShortcutContext::Dialog);

        manager.pop_context();
        assert_eq!(manager.active_context(), &ShortcutContext::Editor);
    }

    #[test]
    fn test_shortcut_manager_handle_key_press() {
        let mut manager = ShortcutManager::new();
        let executed = std::sync::Arc::new(std::sync::Mutex::new(false));
        let executed_clone = executed.clone();

        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save").with_action(
            move || {
                *executed_clone.lock().unwrap() = true;
            },
        );

        manager.register_global(binding);

        // Simulate Ctrl+S
        let result = manager.handle_key_press(Key::Character('s'), Modifiers::ctrl());
        assert!(result);
        assert!(*executed.lock().unwrap());
    }

    #[test]
    fn test_shortcut_manager_get_binding() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");

        manager.register_global(binding);

        let shortcut = Shortcut::parse("Ctrl+S").unwrap();
        let found = manager.get_binding(&shortcut);
        assert!(found.is_some());
    }

    #[test]
    fn test_shortcut_manager_is_registered() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");

        manager.register_global(binding);

        let shortcut = Shortcut::parse("Ctrl+S").unwrap();
        assert!(manager.is_registered(&shortcut));
    }

    #[test]
    fn test_shortcut_manager_clear() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");

        manager.register_global(binding);
        manager.clear();

        assert!(manager.get_all_bindings().is_empty());
    }

    #[test]
    fn test_shortcut_manager_event_handler() {
        use std::sync::{Arc, Mutex};

        let mut manager = ShortcutManager::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        manager.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");
        manager.register_global(binding);

        let events = events.lock().unwrap();
        assert!(!events.is_empty());
        assert!(matches!(events[0], ShortcutEvent::Registered(_)));
    }

    #[test]
    fn test_shortcut_manager_get_shortcut_help() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save file")
            .with_category("File");

        manager.register_global(binding);

        let help = manager.get_shortcut_help();
        assert!(!help.is_empty());
        assert_eq!(help[0].0, "Global");
        assert_eq!(help[0].2, "Save file");
        assert_eq!(help[0].3, "File");
    }

    #[test]
    fn test_default_global_shortcuts() {
        let shortcuts = create_default_global_shortcuts();
        assert!(!shortcuts.is_empty());

        // Check that Ctrl+S is in there
        let ctrl_s = Shortcut::parse("Ctrl+S").unwrap();
        assert!(shortcuts.iter().any(|b| b.shortcut == ctrl_s));
    }

    #[test]
    fn test_default_editor_shortcuts() {
        let shortcuts = create_default_editor_shortcuts();
        assert!(!shortcuts.is_empty());

        // Check that Ctrl+Z is in there
        let ctrl_z = Shortcut::parse("Ctrl+Z").unwrap();
        assert!(shortcuts.iter().any(|b| b.shortcut == ctrl_z));
    }

    #[test]
    fn test_default_sidebar_shortcuts() {
        let shortcuts = create_default_sidebar_shortcuts();
        assert!(!shortcuts.is_empty());

        // Check that Up is in there
        let up = Shortcut::parse("Up").unwrap();
        assert!(shortcuts.iter().any(|b| b.shortcut == up));
    }

    #[test]
    fn test_shortcut_manager_unregister() {
        let mut manager = ShortcutManager::new();
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");

        manager.register_global(binding);
        assert_eq!(manager.get_all_bindings().len(), 1);

        let ctrl_s = Shortcut::parse("Ctrl+S").unwrap();
        manager.unregister(ShortcutContext::Global, &ctrl_s);
        assert_eq!(manager.get_all_bindings().len(), 0);
    }

    #[test]
    fn test_shortcut_manager_builder() {
        let binding = ShortcutBinding::new(Shortcut::parse("Ctrl+S").unwrap(), "Save");

        let manager = ShortcutManagerBuilder::new()
            .add_binding(ShortcutContext::Global, binding)
            .build();

        assert_eq!(manager.get_all_bindings().len(), 1);
    }

    #[test]
    fn test_shortcut_manager_handle_key_release() {
        let mut manager = ShortcutManager::new();
        manager.handle_key_press(Key::Character('s'), Modifiers::ctrl());
        assert!(manager.pressed_keys.contains(&Key::Character('s')));

        manager.handle_key_release(Key::Character('s'));
        assert!(!manager.pressed_keys.contains(&Key::Character('s')));
    }

    #[test]
    fn test_shortcut_manager_clear_pressed_keys() {
        let mut manager = ShortcutManager::new();
        manager.handle_key_press(Key::Character('a'), Modifiers::empty());
        manager.handle_key_press(Key::Character('b'), Modifiers::empty());

        assert_eq!(manager.pressed_keys.len(), 2);

        manager.clear_pressed_keys();
        assert!(manager.pressed_keys.is_empty());
    }
}
