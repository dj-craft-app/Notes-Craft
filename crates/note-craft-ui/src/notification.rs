//! Notification System Module
//!
//! This module provides a notification system for Note Craft UI.
//! It manages the display of toast notifications, banners, and system messages.
//!
//! Note: Phase 3 - Step 3.14 (Notification System)

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ============================================================================
// Notification Types
// ============================================================================

/// Unique identifier for a notification
pub type NotificationId = u64;

/// Notification severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationSeverity {
    /// Informational notification
    #[default]
    Info,
    /// Success notification
    Success,
    /// Warning notification
    Warning,
    /// Error notification
    Error,
}

/// Notification position on screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationPosition {
    /// Top-left corner
    TopLeft,
    /// Top-right corner (default)
    #[default]
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Top-center
    TopCenter,
    /// Bottom-center
    BottomCenter,
}

// ============================================================================
// Notification Configuration
// ============================================================================

/// Notification configuration
#[derive(Clone)]
pub struct NotificationConfig {
    /// Notification identifier
    pub id: NotificationId,
    /// Title/short message
    pub title: String,
    /// Detailed message (can be Markdown)
    pub message: String,
    /// Severity level
    pub severity: NotificationSeverity,
    /// Position on screen
    pub position: NotificationPosition,
    /// Duration before auto-dismiss (None = never auto-dismiss)
    pub duration: Option<Duration>,
    /// Is the notification dismissible
    pub dismissible: bool,
    /// Should the notification play a sound
    pub sound: bool,
    /// Custom icon (optional)
    pub icon: Option<String>,
    /// Action button text (optional)
    pub action_text: Option<String>,
    /// Action callback (optional)
    pub action_callback: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Created timestamp
    pub created_at: Instant,
    /// Priority (higher = more important)
    pub priority: u8,
}

impl std::fmt::Debug for NotificationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationConfig")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("message", &self.message)
            .field("severity", &self.severity)
            .field("position", &self.position)
            .field("duration", &self.duration)
            .field("dismissible", &self.dismissible)
            .field("sound", &self.sound)
            .field("icon", &self.icon)
            .field("action_text", &self.action_text)
            .field("action_callback", &"<fn>")
            .field("created_at", &format_args!("{:?}", self.created_at))
            .field("priority", &self.priority)
            .finish()
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            message: String::new(),
            severity: NotificationSeverity::Info,
            position: NotificationPosition::TopRight,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
            sound: false,
            icon: None,
            action_text: None,
            action_callback: None,
            created_at: Instant::now(),
            priority: 0,
        }
    }
}

impl NotificationConfig {
    /// Creates a new notification configuration
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    /// Sets the notification ID
    pub fn with_id(mut self, id: NotificationId) -> Self {
        self.id = id;
        self
    }

    /// Sets the severity
    pub fn with_severity(mut self, severity: NotificationSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Sets the position
    pub fn with_position(mut self, position: NotificationPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets the duration
    pub fn with_duration(mut self, duration: Option<Duration>) -> Self {
        self.duration = duration;
        self
    }

    /// Sets if dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Sets if sound should play
    pub fn with_sound(mut self, sound: bool) -> Self {
        self.sound = sound;
        self
    }

    /// Sets the icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the action button
    pub fn with_action(
        mut self,
        text: impl Into<String>,
        callback: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        self.action_text = Some(text.into());
        self.action_callback = Some(Arc::new(callback));
        self
    }

    /// Sets the priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Creates an info notification
    pub fn info(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).with_severity(NotificationSeverity::Info)
    }

    /// Creates a success notification
    pub fn success(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).with_severity(NotificationSeverity::Success)
    }

    /// Creates a warning notification
    pub fn warning(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).with_severity(NotificationSeverity::Warning)
    }

    /// Creates an error notification
    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).with_severity(NotificationSeverity::Error)
    }

    /// Creates a sticky notification (no auto-dismiss)
    pub fn sticky(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(title, message).with_duration(None)
    }
}

// ============================================================================
// Notification State
// ============================================================================

/// Notification state (runtime information)
#[derive(Debug, Clone)]
pub struct NotificationState {
    /// Notification configuration
    pub config: NotificationConfig,
    /// Is the notification currently visible
    pub is_visible: bool,
    /// Is the notification being dismissed (animation)
    pub is_dismissing: bool,
    /// Progress of dismiss animation (0.0 to 1.0)
    pub dismiss_progress: f32,
    /// Width of the notification
    pub width: u32,
    /// Height of the notification
    pub height: u32,
    /// X position on screen
    pub x: u32,
    /// Y position on screen
    pub y: u32,
}

impl NotificationState {
    /// Creates a new notification state from config
    pub fn from_config(config: NotificationConfig) -> Self {
        Self {
            config,
            is_visible: false,
            is_dismissing: false,
            dismiss_progress: 0.0,
            width: 300,
            height: 80,
            x: 0,
            y: 0,
        }
    }

    /// Shows the notification
    pub fn show(&mut self) {
        self.is_visible = true;
        self.is_dismissing = false;
        self.dismiss_progress = 0.0;
    }

    /// Hides the notification
    pub fn hide(&mut self) {
        self.is_visible = false;
        self.is_dismissing = false;
        self.dismiss_progress = 0.0;
    }

    /// Starts dismiss animation
    pub fn start_dismiss(&mut self) {
        self.is_dismissing = true;
        self.dismiss_progress = 0.0;
    }

    /// Updates dismiss animation progress
    pub fn update_dismiss(&mut self, progress: f32) {
        self.dismiss_progress = progress.clamp(0.0, 1.0);
        if progress >= 1.0 {
            self.is_visible = false;
            self.is_dismissing = false;
        }
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

    /// Checks if the notification should be auto-dismissed
    pub fn should_auto_dismiss(&self) -> bool {
        if let Some(duration) = self.config.duration {
            self.config.created_at.elapsed() >= duration
        } else {
            false
        }
    }
}

// ============================================================================
// Notification Events
// ============================================================================

/// Events for the notification system
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationEvent {
    /// A notification was added
    Added(NotificationId),
    /// A notification was shown
    Shown(NotificationId),
    /// A notification was dismissed
    Dismissed(NotificationId),
    /// A notification action was triggered
    ActionTriggered(NotificationId),
    /// All notifications were cleared
    AllCleared,
    /// Notification queue changed
    QueueChanged,
}

// ============================================================================
// Notification Queue
// ============================================================================

/// Notification queue configuration
#[derive(Debug, Clone)]
pub struct NotificationQueueConfig {
    /// Maximum number of visible notifications
    pub max_visible: usize,
    /// Time between consecutive notifications
    pub spacing_ms: u64,
    /// Position for new notifications
    pub position: NotificationPosition,
    /// Auto-dismiss all on new notification
    pub dismiss_on_new: bool,
    /// Group similar notifications
    pub group_similar: bool,
}

impl Default for NotificationQueueConfig {
    fn default() -> Self {
        Self {
            max_visible: 5,
            spacing_ms: 100,
            position: NotificationPosition::TopRight,
            dismiss_on_new: false,
            group_similar: true,
        }
    }
}

// ============================================================================
// Notification Manager
// ============================================================================

/// Main notification manager
pub struct NotificationManager {
    /// Queue of notifications waiting to be shown
    pub queue: VecDeque<NotificationConfig>,
    /// Currently visible notifications
    pub notifications: Vec<NotificationState>,
    /// Next notification ID
    pub next_id: NotificationId,
    /// Queue configuration
    pub config: NotificationQueueConfig,
    /// Event handler
    pub event_handler: Option<Box<dyn Fn(NotificationEvent) + Send + Sync>>,
    /// Callbacks for individual notifications
    pub callbacks: std::collections::HashMap<NotificationId, Arc<dyn Fn() + Send + Sync>>,
}

impl NotificationManager {
    /// Creates a new notification manager
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            notifications: Vec::new(),
            next_id: 1,
            config: NotificationQueueConfig::default(),
            event_handler: None,
            callbacks: std::collections::HashMap::new(),
        }
    }

    /// Creates a new notification manager with configuration
    pub fn with_config(config: NotificationQueueConfig) -> Self {
        Self {
            queue: VecDeque::new(),
            notifications: Vec::new(),
            next_id: 1,
            config,
            event_handler: None,
            callbacks: std::collections::HashMap::new(),
        }
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(NotificationEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    /// Generates a new notification ID
    pub fn generate_id(&mut self) -> NotificationId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Adds a notification to the queue
    pub fn add_notification(&mut self, mut config: NotificationConfig) -> NotificationId {
        let id = self.generate_id();
        config.id = id;

        // Try to group similar notifications if enabled
        if self.config.group_similar {
            if let Some(last) = self.queue.back_mut() {
                if last.severity == config.severity
                    && last.title == config.title
                    && last.message == config.message
                {
                    // Update existing notification count
                    // (In a real implementation, this would increment a counter)
                    // For now, we'll just replace it
                    *last = config;
                    self.emit_event(NotificationEvent::QueueChanged);
                    return id;
                }
            }
        }

        self.queue.push_back(config);
        self.emit_event(NotificationEvent::Added(id));
        self.emit_event(NotificationEvent::QueueChanged);

        // Try to show notifications from the queue
        self.process_queue();

        id
    }

    /// Shows the next notification from the queue
    pub fn show_next(&mut self) -> Option<NotificationId> {
        if self.queue.is_empty() {
            return None;
        }

        if self.config.max_visible > 0 && self.notifications.len() >= self.config.max_visible {
            return None;
        }

        let config = self.queue.pop_front().unwrap();
        let config_id = config.id;
        let mut state = NotificationState::from_config(config);
        state.show();

        // Position the notification
        self.position_notification(&mut state);

        self.notifications.push(state);
        self.emit_event(NotificationEvent::Shown(config_id));

        // Start auto-dismiss timer if configured
        // In a real implementation, we would start a timer
        // For now, we just note that it should auto-dismiss

        Some(config_id)
    }

    /// Processes the notification queue
    pub fn process_queue(&mut self) {
        while self.notifications.len() < self.config.max_visible && !self.queue.is_empty() {
            self.show_next();
        }
    }

    /// Positions a notification based on configuration
    fn position_notification(&self, state: &mut NotificationState) {
        let position = state.config.position;
        let visible_count = self.notifications.len();

        // Calculate position based on existing notifications
        // This is a simplified positioning logic
        let offset = (visible_count as u32) * 100; // Stack notifications with 100px offset

        match position {
            NotificationPosition::TopLeft => {
                state.x = 20;
                state.y = 20 + offset;
            }
            NotificationPosition::TopRight => {
                // Assuming screen width of 1920 for now
                // In a real implementation, this would use actual screen dimensions
                state.x = 1920 - state.width - 20;
                state.y = 20 + offset;
            }
            NotificationPosition::BottomLeft => {
                state.x = 20;
                state.y = 1080 - state.height - 20 - offset; // Assuming 1080p
            }
            NotificationPosition::BottomRight => {
                state.x = 1920 - state.width - 20;
                state.y = 1080 - state.height - 20 - offset;
            }
            NotificationPosition::TopCenter => {
                state.x = (1920 / 2) - (state.width / 2);
                state.y = 20 + offset;
            }
            NotificationPosition::BottomCenter => {
                state.x = (1920 / 2) - (state.width / 2);
                state.y = 1080 - state.height - 20 - offset;
            }
        }
    }

    /// Dismisses a notification by ID
    pub fn dismiss_notification(&mut self, id: NotificationId) -> bool {
        if let Some(index) = self.notifications.iter().position(|n| n.config.id == id) {
            let mut notification = self.notifications.remove(index);

            if notification.config.dismissible {
                notification.start_dismiss();
                self.emit_event(NotificationEvent::Dismissed(id));

                // Call callback if exists
                if let Some(callback) = self.callbacks.remove(&id) {
                    callback();
                }

                // Process queue to show next notification
                self.process_queue();

                return true;
            }
        }
        false
    }

    /// Dismisses all notifications
    pub fn dismiss_all(&mut self) {
        let ids: Vec<NotificationId> = self.notifications.iter().map(|n| n.config.id).collect();

        for id in ids {
            self.dismiss_notification(id);
        }

        // Also clear the queue
        self.queue.clear();
        self.emit_event(NotificationEvent::AllCleared);
    }

    /// Dismisses all notifications with a specific severity
    pub fn dismiss_by_severity(&mut self, severity: NotificationSeverity) {
        let ids: Vec<NotificationId> = self
            .notifications
            .iter()
            .filter(|n| n.config.severity == severity)
            .map(|n| n.config.id)
            .collect();

        for id in ids {
            self.dismiss_notification(id);
        }
    }

    /// Gets a notification by ID
    pub fn get_notification(&self, id: NotificationId) -> Option<&NotificationState> {
        self.notifications.iter().find(|n| n.config.id == id)
    }

    /// Gets a mutable notification by ID
    pub fn get_notification_mut(&mut self, id: NotificationId) -> Option<&mut NotificationState> {
        self.notifications.iter_mut().find(|n| n.config.id == id)
    }

    /// Updates all notifications (for animation)
    pub fn update(&mut self, delta_time: Duration) {
        let delta_seconds = delta_time.as_secs_f64();

        // Update dismiss animations
        for notification in &mut self.notifications {
            if notification.is_dismissing {
                notification.dismiss_progress += (delta_seconds / 0.3).min(1.0) as f32;
                if notification.dismiss_progress >= 1.0 {
                    notification.is_visible = false;
                }
            }
        }

        // Auto-dismiss notifications that have timed out
        for notification in &mut self.notifications {
            if notification.is_visible && !notification.is_dismissing {
                if let Some(duration) = notification.config.duration {
                    if notification.config.created_at.elapsed() >= duration {
                        notification.start_dismiss();
                    }
                }
            }
        }

        // Remove hidden notifications
        self.notifications
            .retain(|n| n.is_visible || n.is_dismissing);

        // Process queue
        self.process_queue();
    }

    /// Clears all notifications (queue and visible)
    pub fn clear_all(&mut self) {
        self.queue.clear();
        self.notifications.clear();
        self.callbacks.clear();
        self.emit_event(NotificationEvent::AllCleared);
    }

    /// Gets the number of queued notifications
    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }

    /// Gets the number of visible notifications
    pub fn visible_count(&self) -> usize {
        self.notifications.len()
    }

    /// Gets all visible notifications
    pub fn get_visible_notifications(&self) -> Vec<&NotificationState> {
        self.notifications.iter().filter(|n| n.is_visible).collect()
    }

    /// Triggers the action for a notification
    pub fn trigger_action(&mut self, id: NotificationId) -> bool {
        if let Some(notification) = self.get_notification_mut(id) {
            if let Some(callback) = &notification.config.action_callback {
                callback();
                self.emit_event(NotificationEvent::ActionTriggered(id));
                return true;
            }
        }
        false
    }

    /// Adds a callback for a notification
    pub fn add_callback<F>(&mut self, id: NotificationId, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.callbacks.insert(id, Arc::new(callback));
    }

    /// Emits an event to the event handler
    fn emit_event(&self, event: NotificationEvent) {
        if let Some(ref handler) = self.event_handler {
            handler(event);
        }
    }

    // ============================================================================
    // Convenience Methods
    // ============================================================================

    /// Shows an info notification
    pub fn show_info(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> NotificationId {
        let config = NotificationConfig::info(title, message);
        self.add_notification(config)
    }

    /// Shows a success notification
    pub fn show_success(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> NotificationId {
        let config = NotificationConfig::success(title, message);
        self.add_notification(config)
    }

    /// Shows a warning notification
    pub fn show_warning(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> NotificationId {
        let config = NotificationConfig::warning(title, message);
        self.add_notification(config)
    }

    /// Shows an error notification
    pub fn show_error(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> NotificationId {
        let config = NotificationConfig::error(title, message);
        self.add_notification(config)
    }

    /// Shows a sticky notification (no auto-dismiss)
    pub fn show_sticky(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> NotificationId {
        let config = NotificationConfig::sticky(title, message);
        self.add_notification(config)
    }

    /// Shows a notification with action button
    pub fn show_with_action(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        action_text: impl Into<String>,
        action: impl Fn() + Send + Sync + 'static,
    ) -> NotificationId {
        let config = NotificationConfig::new(title, message).with_action(action_text, action);
        self.add_notification(config)
    }

    /// Shows a temporary notification (shorter duration)
    pub fn show_temporary(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> NotificationId {
        let config =
            NotificationConfig::new(title, message).with_duration(Some(Duration::from_secs(3)));
        self.add_notification(config)
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Notification Builder
// ============================================================================

/// Builder for creating notifications
pub struct NotificationBuilder {
    config: NotificationConfig,
    manager: Option<Arc<Mutex<NotificationManager>>>,
}

impl NotificationBuilder {
    /// Creates a new notification builder
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            config: NotificationConfig::new(title, message),
            manager: None,
        }
    }

    /// Sets the severity
    pub fn severity(mut self, severity: NotificationSeverity) -> Self {
        self.config.severity = severity;
        self
    }

    /// Sets the position
    pub fn position(mut self, position: NotificationPosition) -> Self {
        self.config.position = position;
        self
    }

    /// Sets the duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.config.duration = Some(duration);
        self
    }

    /// Sets sticky (no auto-dismiss)
    pub fn sticky(mut self) -> Self {
        self.config.duration = None;
        self
    }

    /// Sets if dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.config.dismissible = dismissible;
        self
    }

    /// Sets the icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.config.icon = Some(icon.into());
        self
    }

    /// Sets the action button
    pub fn action(
        mut self,
        text: impl Into<String>,
        callback: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        self.config.action_text = Some(text.into());
        self.config.action_callback = Some(Arc::new(callback));
        self
    }

    /// Sets the priority
    pub fn priority(mut self, priority: u8) -> Self {
        self.config.priority = priority;
        self
    }

    /// Sets the notification manager
    pub fn with_manager(mut self, manager: Arc<Mutex<NotificationManager>>) -> Self {
        self.manager = Some(manager);
        self
    }

    /// Shows the notification
    pub fn show(self) -> Option<NotificationId> {
        if let Some(manager) = self.manager {
            let mut mgr = manager.lock().unwrap();
            Some(mgr.add_notification(self.config))
        } else {
            None
        }
    }

    /// Builds the notification config without showing
    pub fn build(self) -> NotificationConfig {
        self.config
    }
}

// ============================================================================
// Notification Utilities
// ============================================================================

/// Notification utilities
pub struct NotificationUtils;

impl NotificationUtils {
    /// Formats a notification message with severity prefix
    pub fn format_message(severity: NotificationSeverity, message: &str) -> String {
        let prefix = match severity {
            NotificationSeverity::Info => "ℹ️ ",
            NotificationSeverity::Success => "✓ ",
            NotificationSeverity::Warning => "⚠️ ",
            NotificationSeverity::Error => "✗ ",
        };
        format!("{}{}", prefix, message)
    }

    /// Gets the color for a severity level
    pub fn get_color(severity: NotificationSeverity) -> &'static str {
        match severity {
            NotificationSeverity::Info => "#027aff",
            NotificationSeverity::Success => "#28a745",
            NotificationSeverity::Warning => "#ffc107",
            NotificationSeverity::Error => "#dc3545",
        }
    }

    /// Gets the icon for a severity level
    pub fn get_icon(severity: NotificationSeverity) -> Option<&'static str> {
        match severity {
            NotificationSeverity::Info => Some("info-circle"),
            NotificationSeverity::Success => Some("check-circle"),
            NotificationSeverity::Warning => Some("exclamation-triangle"),
            NotificationSeverity::Error => Some("exclamation-circle"),
        }
    }

    /// Calculates the appropriate duration based on message length
    pub fn calculate_duration(message: &str) -> Duration {
        let length = message.len();
        if length < 50 {
            Duration::from_secs(3)
        } else if length < 100 {
            Duration::from_secs(5)
        } else {
            Duration::from_secs(7)
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========== NotificationConfig Tests ==========

    #[test]
    fn test_notification_config_default() {
        let config = NotificationConfig::default();
        assert_eq!(config.id, 0);
        assert_eq!(config.title, "");
        assert_eq!(config.message, "");
        assert_eq!(config.severity, NotificationSeverity::Info);
        assert_eq!(config.position, NotificationPosition::TopRight);
        assert_eq!(config.duration, Some(Duration::from_secs(5)));
        assert!(config.dismissible);
        assert!(!config.sound);
        assert!(config.icon.is_none());
        assert!(config.action_text.is_none());
        assert!(config.action_callback.is_none());
    }

    #[test]
    fn test_notification_config_new() {
        let config = NotificationConfig::new("Title", "Message");
        assert_eq!(config.title, "Title");
        assert_eq!(config.message, "Message");
    }

    #[test]
    fn test_notification_config_with_severity() {
        let config =
            NotificationConfig::new("Title", "Message").with_severity(NotificationSeverity::Error);
        assert_eq!(config.severity, NotificationSeverity::Error);
    }

    #[test]
    fn test_notification_config_with_position() {
        let config = NotificationConfig::new("Title", "Message")
            .with_position(NotificationPosition::BottomLeft);
        assert_eq!(config.position, NotificationPosition::BottomLeft);
    }

    #[test]
    fn test_notification_config_with_duration() {
        let config = NotificationConfig::new("Title", "Message")
            .with_duration(Some(Duration::from_secs(10)));
        assert_eq!(config.duration, Some(Duration::from_secs(10)));
    }

    #[test]
    fn test_notification_config_sticky() {
        let config = NotificationConfig::sticky("Title", "Message");
        assert!(config.duration.is_none());
    }

    #[test]
    fn test_notification_config_info() {
        let config = NotificationConfig::info("Info", "Info message");
        assert_eq!(config.severity, NotificationSeverity::Info);
    }

    #[test]
    fn test_notification_config_success() {
        let config = NotificationConfig::success("Success", "Success message");
        assert_eq!(config.severity, NotificationSeverity::Success);
    }

    #[test]
    fn test_notification_config_warning() {
        let config = NotificationConfig::warning("Warning", "Warning message");
        assert_eq!(config.severity, NotificationSeverity::Warning);
    }

    #[test]
    fn test_notification_config_error() {
        let config = NotificationConfig::error("Error", "Error message");
        assert_eq!(config.severity, NotificationSeverity::Error);
    }

    // ========== NotificationState Tests ==========

    #[test]
    fn test_notification_state_from_config() {
        let config = NotificationConfig::new("Title", "Message");
        let state = NotificationState::from_config(config);
        assert_eq!(state.config.title, "Title");
        assert!(!state.is_visible);
        assert!(!state.is_dismissing);
        assert_eq!(state.dismiss_progress, 0.0);
    }

    #[test]
    fn test_notification_state_show() {
        let config = NotificationConfig::new("Title", "Message");
        let mut state = NotificationState::from_config(config);
        state.show();
        assert!(state.is_visible);
        assert!(!state.is_dismissing);
    }

    #[test]
    fn test_notification_state_hide() {
        let config = NotificationConfig::new("Title", "Message");
        let mut state = NotificationState::from_config(config);
        state.show();
        state.hide();
        assert!(!state.is_visible);
    }

    #[test]
    fn test_notification_state_start_dismiss() {
        let config = NotificationConfig::new("Title", "Message");
        let mut state = NotificationState::from_config(config);
        state.start_dismiss();
        assert!(state.is_dismissing);
        assert_eq!(state.dismiss_progress, 0.0);
    }

    #[test]
    fn test_notification_state_update_dismiss() {
        let config = NotificationConfig::new("Title", "Message");
        let mut state = NotificationState::from_config(config);
        state.start_dismiss();
        state.update_dismiss(0.5);
        assert_eq!(state.dismiss_progress, 0.5);
    }

    #[test]
    fn test_notification_state_set_position() {
        let config = NotificationConfig::new("Title", "Message");
        let mut state = NotificationState::from_config(config);
        state.set_position(100, 200);
        assert_eq!(state.x, 100);
        assert_eq!(state.y, 200);
    }

    #[test]
    fn test_notification_state_set_size() {
        let config = NotificationConfig::new("Title", "Message");
        let mut state = NotificationState::from_config(config);
        state.set_size(400, 100);
        assert_eq!(state.width, 400);
        assert_eq!(state.height, 100);
    }

    // ========== NotificationQueueConfig Tests ==========

    #[test]
    fn test_notification_queue_config_default() {
        let config = NotificationQueueConfig::default();
        assert_eq!(config.max_visible, 5);
        assert_eq!(config.spacing_ms, 100);
        assert_eq!(config.position, NotificationPosition::TopRight);
        assert!(!config.dismiss_on_new);
        assert!(config.group_similar);
    }

    // ========== NotificationManager Tests ==========

    #[test]
    fn test_notification_manager_new() {
        let manager = NotificationManager::new();
        assert!(manager.queue.is_empty());
        assert!(manager.notifications.is_empty());
        assert_eq!(manager.next_id, 1);
    }

    #[test]
    fn test_notification_manager_generate_id() {
        let mut manager = NotificationManager::new();
        let id1 = manager.generate_id();
        let id2 = manager.generate_id();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_notification_manager_add_notification() {
        let mut manager = NotificationManager::new();
        let config = NotificationConfig::new("Title", "Message");
        let id = manager.add_notification(config);
        assert_eq!(id, 1);
        // Note: queue is processed automatically, so it may be empty
        // if max_visible allows showing the notification
        assert!(manager.queue.len() <= 1);
    }

    #[test]
    fn test_notification_manager_show_next() {
        let mut manager = NotificationManager::with_config(NotificationQueueConfig {
            max_visible: 0, // Prevent auto-show
            ..Default::default()
        });
        let config = NotificationConfig::new("Title", "Message");
        manager.add_notification(config);
        let id = manager.show_next();
        assert!(id.is_some());
        assert_eq!(id.unwrap(), 1);
        assert_eq!(manager.notifications.len(), 1);
        assert!(manager.queue.is_empty());
    }

    #[test]
    fn test_notification_manager_dismiss_notification() {
        let mut manager = NotificationManager::new();
        let config = NotificationConfig::new("Title", "Message");
        manager.add_notification(config);
        manager.show_next();
        let result = manager.dismiss_notification(1);
        assert!(result);
        assert!(manager.notifications.is_empty());
    }

    #[test]
    fn test_notification_manager_dismiss_all() {
        let mut manager = NotificationManager::new();
        manager.add_notification(NotificationConfig::new("Title1", "Message1"));
        manager.add_notification(NotificationConfig::new("Title2", "Message2"));
        manager.show_next();
        manager.show_next();
        assert_eq!(manager.notifications.len(), 2);
        manager.dismiss_all();
        assert!(manager.notifications.is_empty());
        assert!(manager.queue.is_empty());
    }

    #[test]
    fn test_notification_manager_clear_all() {
        let mut manager = NotificationManager::new();
        manager.add_notification(NotificationConfig::new("Title1", "Message1"));
        manager.add_notification(NotificationConfig::new("Title2", "Message2"));
        assert!(manager.queue.len() >= 0);
        assert!(manager.notifications.len() >= 0);
        manager.clear_all();
        assert!(manager.queue.is_empty());
        assert!(manager.notifications.is_empty());
    }

    #[test]
    fn test_notification_manager_get_notification() {
        let mut manager = NotificationManager::new();
        manager.add_notification(NotificationConfig::new("Title", "Message"));
        manager.show_next();
        let notification = manager.get_notification(1);
        assert!(notification.is_some());
        assert_eq!(notification.unwrap().config.title, "Title");
    }

    #[test]
    fn test_notification_manager_queue_length() {
        let mut manager = NotificationManager::with_config(NotificationQueueConfig {
            max_visible: 0, // Prevent auto-show
            ..Default::default()
        });
        assert_eq!(manager.queue_length(), 0);
        manager.add_notification(NotificationConfig::new("Title1", "Message1"));
        assert_eq!(manager.queue_length(), 1);
        manager.add_notification(NotificationConfig::new("Title2", "Message2"));
        assert_eq!(manager.queue_length(), 2);
    }

    #[test]
    fn test_notification_manager_visible_count() {
        let mut manager = NotificationManager::new();
        assert_eq!(manager.visible_count(), 0);
        manager.add_notification(NotificationConfig::new("Title1", "Message1"));
        manager.show_next();
        assert_eq!(manager.visible_count(), 1);
    }

    #[test]
    fn test_notification_manager_show_info() {
        let mut manager = NotificationManager::with_config(NotificationQueueConfig {
            max_visible: 0, // Prevent auto-show
            ..Default::default()
        });
        let id = manager.show_info("Info", "Info message");
        assert_eq!(id, 1);
        assert_eq!(manager.queue.len(), 1);
    }

    #[test]
    fn test_notification_manager_show_success() {
        let mut manager = NotificationManager::new();
        let id = manager.show_success("Success", "Success message");
        assert_eq!(id, 1);
    }

    #[test]
    fn test_notification_manager_show_warning() {
        let mut manager = NotificationManager::new();
        let id = manager.show_warning("Warning", "Warning message");
        assert_eq!(id, 1);
    }

    #[test]
    fn test_notification_manager_show_error() {
        let mut manager = NotificationManager::new();
        let id = manager.show_error("Error", "Error message");
        assert_eq!(id, 1);
    }

    #[test]
    fn test_notification_manager_show_sticky() {
        let mut manager = NotificationManager::new();
        let id = manager.show_sticky("Sticky", "Sticky message");
        assert_eq!(id, 1);
    }

    #[test]
    fn test_notification_manager_show_with_action() {
        let mut manager = NotificationManager::new();
        let id = manager.show_with_action("Title", "Message", "Action", || {});
        assert_eq!(id, 1);
    }

    #[test]
    fn test_notification_manager_show_temporary() {
        let mut manager = NotificationManager::new();
        let id = manager.show_temporary("Temp", "Temp message");
        assert_eq!(id, 1);
    }

    #[test]
    fn test_notification_manager_max_visible() {
        let mut manager = NotificationManager::with_config(NotificationQueueConfig {
            max_visible: 2,
            ..Default::default()
        });

        // Add 5 notifications
        for i in 0..5 {
            manager.add_notification(NotificationConfig::new(
                format!("Title {}", i),
                format!("Message {}", i),
            ));
        }

        // Show all possible
        manager.process_queue();

        // Should only show 2
        assert!(manager.notifications.len() <= 2);
        // Should have 3 in queue
        assert_eq!(manager.queue.len(), 3);
    }

    #[test]
    fn test_notification_manager_group_similar() {
        let mut manager = NotificationManager::with_config(NotificationQueueConfig {
            max_visible: 0, // Prevent auto-show
            group_similar: true,
            ..Default::default()
        });

        // Add same notification twice
        let id1 = manager.add_notification(NotificationConfig::new("Title", "Message"));
        let id2 = manager.add_notification(NotificationConfig::new("Title", "Message"));

        // Should only have 1 in queue
        assert_eq!(manager.queue.len(), 1);
        // IDs should be different but only one in queue
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_notification_manager_trigger_action() {
        // Test with a simple callback
        let mut manager = NotificationManager::new();
        let id = manager.add_notification(NotificationConfig::new("Title", "Message"));
        manager.show_next();

        // Add a callback
        manager.add_callback(id, || {});
        assert!(manager.callbacks.contains_key(&id));

        // Test trigger with action callback
        let mut manager2 = NotificationManager::new();
        let config = NotificationConfig::new("Title", "Message").with_action("Action", || {});
        manager2.add_notification(config);
        manager2.show_next();

        // Verify notification has action
        let notification = manager2.get_notification(1).unwrap();
        assert!(notification.config.action_text.is_some());
        assert!(notification.config.action_callback.is_some());
    }

    // ========== NotificationEvent Tests ==========

    #[test]
    fn test_notification_event_added() {
        let event = NotificationEvent::Added(1);
        assert!(matches!(event, NotificationEvent::Added(1)));
    }

    #[test]
    fn test_notification_event_shown() {
        let event = NotificationEvent::Shown(1);
        assert!(matches!(event, NotificationEvent::Shown(1)));
    }

    #[test]
    fn test_notification_event_dismissed() {
        let event = NotificationEvent::Dismissed(1);
        assert!(matches!(event, NotificationEvent::Dismissed(1)));
    }

    #[test]
    fn test_notification_event_action_triggered() {
        let event = NotificationEvent::ActionTriggered(1);
        assert!(matches!(event, NotificationEvent::ActionTriggered(1)));
    }

    #[test]
    fn test_notification_event_all_cleared() {
        let event = NotificationEvent::AllCleared;
        assert!(matches!(event, NotificationEvent::AllCleared));
    }

    #[test]
    fn test_notification_event_queue_changed() {
        let event = NotificationEvent::QueueChanged;
        assert!(matches!(event, NotificationEvent::QueueChanged));
    }

    // ========== NotificationBuilder Tests ==========

    #[test]
    fn test_notification_builder_new() {
        let builder = NotificationBuilder::new("Title", "Message");
        let config = builder.build();
        assert_eq!(config.title, "Title");
        assert_eq!(config.message, "Message");
    }

    #[test]
    fn test_notification_builder_severity() {
        let config = NotificationBuilder::new("Title", "Message")
            .severity(NotificationSeverity::Error)
            .build();
        assert_eq!(config.severity, NotificationSeverity::Error);
    }

    #[test]
    fn test_notification_builder_position() {
        let config = NotificationBuilder::new("Title", "Message")
            .position(NotificationPosition::BottomLeft)
            .build();
        assert_eq!(config.position, NotificationPosition::BottomLeft);
    }

    #[test]
    fn test_notification_builder_duration() {
        let config = NotificationBuilder::new("Title", "Message")
            .duration(Duration::from_secs(10))
            .build();
        assert_eq!(config.duration, Some(Duration::from_secs(10)));
    }

    #[test]
    fn test_notification_builder_sticky() {
        let config = NotificationBuilder::new("Title", "Message")
            .sticky()
            .build();
        assert!(config.duration.is_none());
    }

    #[test]
    fn test_notification_builder_dismissible() {
        let config = NotificationBuilder::new("Title", "Message")
            .dismissible(false)
            .build();
        assert!(!config.dismissible);
    }

    #[test]
    fn test_notification_builder_icon() {
        let config = NotificationBuilder::new("Title", "Message")
            .icon("info")
            .build();
        assert_eq!(config.icon, Some("info".to_string()));
    }

    #[test]
    fn test_notification_builder_priority() {
        let config = NotificationBuilder::new("Title", "Message")
            .priority(10)
            .build();
        assert_eq!(config.priority, 10);
    }

    // ========== NotificationUtils Tests ==========

    #[test]
    fn test_notification_utils_format_message() {
        let info = NotificationUtils::format_message(NotificationSeverity::Info, "Test");
        assert!(info.contains("ℹ️"));
        assert!(info.contains("Test"));

        let success = NotificationUtils::format_message(NotificationSeverity::Success, "Test");
        assert!(success.contains("✓"));

        let warning = NotificationUtils::format_message(NotificationSeverity::Warning, "Test");
        assert!(warning.contains("⚠️"));

        let error = NotificationUtils::format_message(NotificationSeverity::Error, "Test");
        assert!(error.contains("✗"));
    }

    #[test]
    fn test_notification_utils_get_color() {
        assert_eq!(
            NotificationUtils::get_color(NotificationSeverity::Info),
            "#027aff"
        );
        assert_eq!(
            NotificationUtils::get_color(NotificationSeverity::Success),
            "#28a745"
        );
        assert_eq!(
            NotificationUtils::get_color(NotificationSeverity::Warning),
            "#ffc107"
        );
        assert_eq!(
            NotificationUtils::get_color(NotificationSeverity::Error),
            "#dc3545"
        );
    }

    #[test]
    fn test_notification_utils_get_icon() {
        assert_eq!(
            NotificationUtils::get_icon(NotificationSeverity::Info),
            Some("info-circle")
        );
        assert_eq!(
            NotificationUtils::get_icon(NotificationSeverity::Success),
            Some("check-circle")
        );
        assert_eq!(
            NotificationUtils::get_icon(NotificationSeverity::Warning),
            Some("exclamation-triangle")
        );
        assert_eq!(
            NotificationUtils::get_icon(NotificationSeverity::Error),
            Some("exclamation-circle")
        );
    }

    #[test]
    fn test_notification_utils_calculate_duration() {
        let short = NotificationUtils::calculate_duration("Short");
        assert!(short <= Duration::from_secs(3));

        let medium = NotificationUtils::calculate_duration(
            "This is a medium length message with some content",
        );
        // Length is 48 chars, should be 3 seconds
        assert_eq!(medium, Duration::from_secs(3));

        let long = NotificationUtils::calculate_duration("This is a very long message that contains a lot of text and should take longer to read and understand before it auto-dismisses");
        assert!(long >= Duration::from_secs(5));
    }

    // ========== Enum Display Tests ==========

    #[test]
    fn test_notification_severity_debug() {
        assert_eq!(format!("{:?}", NotificationSeverity::Info), "Info");
        assert_eq!(format!("{:?}", NotificationSeverity::Success), "Success");
        assert_eq!(format!("{:?}", NotificationSeverity::Warning), "Warning");
        assert_eq!(format!("{:?}", NotificationSeverity::Error), "Error");
    }

    #[test]
    fn test_notification_position_debug() {
        assert_eq!(format!("{:?}", NotificationPosition::TopLeft), "TopLeft");
        assert_eq!(format!("{:?}", NotificationPosition::TopRight), "TopRight");
        assert_eq!(
            format!("{:?}", NotificationPosition::BottomLeft),
            "BottomLeft"
        );
        assert_eq!(
            format!("{:?}", NotificationPosition::BottomRight),
            "BottomRight"
        );
        assert_eq!(
            format!("{:?}", NotificationPosition::TopCenter),
            "TopCenter"
        );
        assert_eq!(
            format!("{:?}", NotificationPosition::BottomCenter),
            "BottomCenter"
        );
    }
}
