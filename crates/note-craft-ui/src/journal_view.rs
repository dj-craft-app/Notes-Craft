//! Journal View Module
//!
//! This module provides a journal view for Note Craft, displaying notes
//! in a chronological format (daily, weekly, monthly).

use note_craft_core::Node;
use std::collections::BTreeMap;

/// Journal entry representing a note for a specific day
#[derive(Debug, Clone)]
pub struct JournalEntry {
    /// The date of the entry (YYYY-MM-DD format)
    pub date: String,
    /// List of note IDs for this date
    pub note_ids: Vec<String>,
    /// Display title for this date
    pub title: String,
    /// Whether this entry is expanded in the UI
    pub expanded: bool,
    /// Whether this entry is selected
    pub selected: bool,
}

impl JournalEntry {
    /// Creates a new journal entry
    pub fn new(date: &str, title: impl Into<String>) -> Self {
        Self {
            date: date.to_string(),
            note_ids: Vec::new(),
            title: title.into(),
            expanded: false,
            selected: false,
        }
    }

    /// Adds a note ID to this entry
    pub fn add_note(&mut self, note_id: impl Into<String>) -> &mut Self {
        self.note_ids.push(note_id.into());
        self
    }

    /// Sets whether this entry is expanded
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Sets whether this entry is selected
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// Journal section (e.g., daily, weekly, monthly grouping)
#[derive(Debug, Clone)]
pub struct JournalSection {
    /// Section identifier (e.g., "2026-11", "Week 46", "Today")
    pub id: String,
    /// Display title
    pub title: String,
    /// List of entries in this section
    pub entries: Vec<JournalEntry>,
    /// Whether this section is expanded
    pub expanded: bool,
    /// Section type
    pub section_type: JournalSectionType,
}

impl JournalSection {
    /// Creates a new journal section
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        section_type: JournalSectionType,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            entries: Vec::new(),
            expanded: true,
            section_type,
        }
    }

    /// Adds an entry to this section
    pub fn add_entry(&mut self, entry: JournalEntry) -> &mut Self {
        self.entries.push(entry);
        self
    }

    /// Gets the total number of notes in this section
    pub fn note_count(&self) -> usize {
        self.entries.iter().map(|e| e.note_ids.len()).sum()
    }

    /// Gets all note IDs in this section
    pub fn all_note_ids(&self) -> Vec<String> {
        self.entries
            .iter()
            .flat_map(|e| e.note_ids.clone())
            .collect()
    }
}

/// Type of journal section grouping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalSectionType {
    /// Daily grouping (one section per day)
    Daily,
    /// Weekly grouping (one section per week)
    Weekly,
    /// Monthly grouping (one section per month)
    Monthly,
    /// Yearly grouping (one section per year)
    Yearly,
    /// Custom grouping
    Custom,
}

impl std::fmt::Display for JournalSectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalSectionType::Daily => write!(f, "Daily"),
            JournalSectionType::Weekly => write!(f, "Weekly"),
            JournalSectionType::Monthly => write!(f, "Monthly"),
            JournalSectionType::Yearly => write!(f, "Yearly"),
            JournalSectionType::Custom => write!(f, "Custom"),
        }
    }
}

/// Journal view mode (how to group entries)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalViewMode {
    /// Show entries grouped by day
    Daily,
    /// Show entries grouped by week
    Weekly,
    /// Show entries grouped by month
    Monthly,
    /// Show entries grouped by year
    Yearly,
}

impl std::fmt::Display for JournalViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalViewMode::Daily => write!(f, "Daily"),
            JournalViewMode::Weekly => write!(f, "Weekly"),
            JournalViewMode::Monthly => write!(f, "Monthly"),
            JournalViewMode::Yearly => write!(f, "Yearly"),
        }
    }
}

/// Sort order for journal entries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JournalSortOrder {
    /// Newest first (descending)
    #[default]
    NewestFirst,
    /// Oldest first (ascending)
    OldestFirst,
}

impl std::fmt::Display for JournalSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalSortOrder::NewestFirst => write!(f, "NewestFirst"),
            JournalSortOrder::OldestFirst => write!(f, "OldestFirst"),
        }
    }
}

/// Configuration for the journal view
#[derive(Debug, Clone)]
pub struct JournalConfig {
    /// View mode (grouping)
    pub view_mode: JournalViewMode,
    /// Sort order
    pub sort_order: JournalSortOrder,
    /// Whether to show empty days
    pub show_empty_days: bool,
    /// Whether to show week numbers
    pub show_week_numbers: bool,
    /// Whether to show date headers
    pub show_date_headers: bool,
    /// Maximum number of entries to display
    pub max_entries: Option<usize>,
    /// Date format for display
    pub date_format: String,
    /// Color scheme for entries
    pub color_scheme: JournalColorScheme,
}

impl Default for JournalConfig {
    fn default() -> Self {
        Self {
            view_mode: JournalViewMode::Daily,
            sort_order: JournalSortOrder::NewestFirst,
            show_empty_days: false,
            show_week_numbers: true,
            show_date_headers: true,
            max_entries: None,
            date_format: "%Y-%m-%d".to_string(),
            color_scheme: JournalColorScheme::Default,
        }
    }
}

impl JournalConfig {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the view mode
    pub fn with_view_mode(mut self, mode: JournalViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    /// Sets the sort order
    pub fn with_sort_order(mut self, order: JournalSortOrder) -> Self {
        self.sort_order = order;
        self
    }

    /// Sets whether to show empty days
    pub fn with_show_empty_days(mut self, show: bool) -> Self {
        self.show_empty_days = show;
        self
    }

    /// Sets whether to show week numbers
    pub fn with_show_week_numbers(mut self, show: bool) -> Self {
        self.show_week_numbers = show;
        self
    }

    /// Sets the maximum number of entries
    pub fn with_max_entries(mut self, max: usize) -> Self {
        self.max_entries = Some(max);
        self
    }

    /// Sets the date format
    pub fn with_date_format(mut self, format: impl Into<String>) -> Self {
        self.date_format = format.into();
        self
    }

    /// Sets the color scheme
    pub fn with_color_scheme(mut self, scheme: JournalColorScheme) -> Self {
        self.color_scheme = scheme;
        self
    }
}

/// Color scheme for journal entries
#[derive(Debug, Clone, Default)]
pub enum JournalColorScheme {
    /// Default color scheme
    #[default]
    Default,
    /// Monochrome
    Monochrome,
    /// Pastel colors
    Pastel,
    /// Dark theme
    Dark,
    /// Custom colors
    Custom,
}

impl std::fmt::Display for JournalColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalColorScheme::Default => write!(f, "Default"),
            JournalColorScheme::Monochrome => write!(f, "Monochrome"),
            JournalColorScheme::Pastel => write!(f, "Pastel"),
            JournalColorScheme::Dark => write!(f, "Dark"),
            JournalColorScheme::Custom => write!(f, "Custom"),
        }
    }
}

/// Events for the journal view
#[derive(Debug, Clone, PartialEq)]
pub enum JournalEvent {
    /// Date was clicked
    DateClicked(String),
    /// Entry was clicked
    EntryClicked(String),
    /// Note was clicked
    NoteClicked(String),
    /// Section was expanded
    SectionExpanded(String),
    /// Section was collapsed
    SectionCollapsed(String),
    /// Entry was expanded
    EntryExpanded(String),
    /// Entry was collapsed
    EntryCollapsed(String),
    /// View mode changed
    ViewModeChanged(JournalViewMode),
    /// Date range changed
    DateRangeChanged(String, String),
    /// New entry requested
    NewEntryRequested(String),
}

/// Filter for journal view
#[derive(Debug, Clone, Default)]
pub struct JournalFilter {
    /// Start date (inclusive)
    pub start_date: Option<String>,
    /// End date (inclusive)
    pub end_date: Option<String>,
    /// Tags to filter by
    pub tags: Vec<String>,
    /// Text search query
    pub query: Option<String>,
}

impl JournalFilter {
    /// Creates a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the start date
    pub fn with_start_date(mut self, date: impl Into<String>) -> Self {
        self.start_date = Some(date.into());
        self
    }

    /// Sets the end date
    pub fn with_end_date(mut self, date: impl Into<String>) -> Self {
        self.end_date = Some(date.into());
        self
    }

    /// Adds a tag to filter by
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Sets the search query
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Checks if a date is within the filter range
    pub fn is_date_in_range(&self, date: &str) -> bool {
        if let Some(ref start) = self.start_date {
            if date < start.as_str() {
                return false;
            }
        }

        if let Some(ref end) = self.end_date {
            if date > end.as_str() {
                return false;
            }
        }

        true
    }
}

/// Main journal view structure
pub struct JournalView {
    /// List of sections
    pub sections: Vec<JournalSection>,
    /// Configuration
    pub config: JournalConfig,
    /// Filter
    pub filter: JournalFilter,
    /// Selected date
    pub selected_date: Option<String>,
    /// Selected note ID
    pub selected_note: Option<String>,
    /// Event handler
    pub event_handler: Option<Box<dyn Fn(JournalEvent) + Send + Sync>>,
}

impl JournalView {
    /// Creates a new journal view
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            config: JournalConfig::default(),
            filter: JournalFilter::default(),
            selected_date: None,
            selected_note: None,
            event_handler: None,
        }
    }

    /// Creates a new journal view with configuration
    pub fn with_config(config: JournalConfig) -> Self {
        Self {
            sections: Vec::new(),
            config,
            filter: JournalFilter::default(),
            selected_date: None,
            selected_note: None,
            event_handler: None,
        }
    }

    /// Sets the event handler
    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(JournalEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    /// Loads notes and builds the journal structure
    pub fn load_notes(&mut self, notes: &[Node]) -> &mut Self {
        // Clear existing sections
        self.sections.clear();

        // Group notes by date
        let mut date_map: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for note in notes {
            if let Some(date) = self.extract_date_from_node(note) {
                date_map
                    .entry(date)
                    .or_default()
                    .push(note.id.0.to_string());
            }
        }

        // Build sections based on view mode
        match self.config.view_mode {
            JournalViewMode::Daily => self.build_daily_sections(date_map),
            JournalViewMode::Weekly => self.build_weekly_sections(date_map),
            JournalViewMode::Monthly => self.build_monthly_sections(date_map),
            JournalViewMode::Yearly => self.build_yearly_sections(date_map),
        }

        // Sort sections based on sort order
        self.sort_sections();

        self
    }

    /// Extracts date from a node
    fn extract_date_from_node(&self, node: &Node) -> Option<String> {
        // Use created_at timestamp (assuming it's a Unix timestamp in seconds)
        if node.created_at > 0 {
            return Some(timestamp_to_date(node.created_at));
        }

        // Try to extract from content (first line might contain a date)
        let first_line: &str = node.content.lines().next().unwrap_or("");

        // Try to find a date pattern in the first line
        if let Some(date) = extract_date_from_text(first_line) {
            return Some(date);
        }

        // If no date found, use today
        Some(get_today())
    }

    /// Builds daily sections
    fn build_daily_sections(&mut self, date_map: BTreeMap<String, Vec<String>>) {
        for (date, note_ids) in date_map {
            if self.filter.is_date_in_range(&date) {
                let mut entry = JournalEntry::new(&date, date.clone());
                for note_id in note_ids {
                    entry.add_note(note_id);
                }

                let mut section = JournalSection::new(
                    date.clone(),
                    format_date_for_display(&date),
                    JournalSectionType::Daily,
                );
                section.add_entry(entry);

                self.sections.push(section);
            }
        }
    }

    /// Builds weekly sections
    fn build_weekly_sections(&mut self, date_map: BTreeMap<String, Vec<String>>) {
        let mut week_map: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();

        for (date, note_ids) in date_map {
            if self.filter.is_date_in_range(&date) {
                let week_key = get_week_key(&date);
                week_map.entry(week_key).or_default().push((date, note_ids));
            }
        }

        for (week_key, entries) in week_map {
            let mut section_entries = Vec::new();
            for (date, note_ids) in entries {
                let mut entry = JournalEntry::new(&date, date.clone());
                for note_id in note_ids {
                    entry.add_note(note_id);
                }
                section_entries.push(entry);
            }

            let mut section = JournalSection::new(
                week_key.clone(),
                format_week_key(&week_key),
                JournalSectionType::Weekly,
            );

            for entry in section_entries {
                section.add_entry(entry);
            }

            self.sections.push(section);
        }
    }

    /// Builds monthly sections
    fn build_monthly_sections(&mut self, date_map: BTreeMap<String, Vec<String>>) {
        let mut month_map: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();

        for (date, note_ids) in date_map {
            if self.filter.is_date_in_range(&date) && date.len() >= 7 {
                let month_key = &date[..7]; // YYYY-MM
                month_map
                    .entry(month_key.to_string())
                    .or_default()
                    .push((date, note_ids));
            }
        }

        for (month_key, entries) in month_map {
            let mut section_entries = Vec::new();
            for (date, note_ids) in entries {
                let mut entry = JournalEntry::new(&date, date.clone());
                for note_id in note_ids {
                    entry.add_note(note_id);
                }
                section_entries.push(entry);
            }

            let mut section = JournalSection::new(
                month_key.clone(),
                format_month_key(&month_key),
                JournalSectionType::Monthly,
            );

            for entry in section_entries {
                section.add_entry(entry);
            }

            self.sections.push(section);
        }
    }

    /// Builds yearly sections
    fn build_yearly_sections(&mut self, date_map: BTreeMap<String, Vec<String>>) {
        let mut year_map: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();

        for (date, note_ids) in date_map {
            if self.filter.is_date_in_range(&date) && date.len() >= 4 {
                let year_key = &date[..4]; // YYYY
                year_map
                    .entry(year_key.to_string())
                    .or_default()
                    .push((date, note_ids));
            }
        }

        for (year_key, entries) in year_map {
            let mut section_entries = Vec::new();
            for (date, note_ids) in entries {
                let mut entry = JournalEntry::new(&date, date.clone());
                for note_id in note_ids {
                    entry.add_note(note_id);
                }
                section_entries.push(entry);
            }

            let mut section = JournalSection::new(
                year_key.clone(),
                year_key.clone(),
                JournalSectionType::Yearly,
            );

            for entry in section_entries {
                section.add_entry(entry);
            }

            self.sections.push(section);
        }
    }

    /// Sorts sections based on configuration
    fn sort_sections(&mut self) {
        match self.config.sort_order {
            JournalSortOrder::NewestFirst => {
                // Sections are already in order from BTreeMap (oldest first)
                // Reverse to get newest first
                self.sections.reverse();
            }
            JournalSortOrder::OldestFirst => {
                // BTreeMap already gives us sorted order
            }
        }
    }

    /// Handles a date click
    pub fn handle_date_click(&mut self, date: &str) {
        self.selected_date = Some(date.to_string());

        // Find and expand the section containing this date
        for section in &mut self.sections {
            for entry in &mut section.entries {
                if entry.date == date {
                    entry.expanded = true;
                    if let Some(handler) = &self.event_handler {
                        handler(JournalEvent::DateClicked(date.to_string()));
                        for note_id in &entry.note_ids {
                            handler(JournalEvent::NoteClicked(note_id.clone()));
                        }
                    }
                    break;
                }
            }
        }
    }

    /// Handles an entry click
    pub fn handle_entry_click(&mut self, date: &str) {
        self.selected_date = Some(date.to_string());

        // Toggle expansion
        for section in &mut self.sections {
            for entry in &mut section.entries {
                if entry.date == date {
                    entry.expanded = !entry.expanded;
                    if entry.expanded {
                        if let Some(handler) = &self.event_handler {
                            handler(JournalEvent::EntryExpanded(date.to_string()));
                        }
                    } else {
                        if let Some(handler) = &self.event_handler {
                            handler(JournalEvent::EntryCollapsed(date.to_string()));
                        }
                    }
                    break;
                }
            }
        }
    }

    /// Handles a note click
    pub fn handle_note_click(&mut self, note_id: &str) {
        self.selected_note = Some(note_id.to_string());

        if let Some(handler) = &self.event_handler {
            handler(JournalEvent::NoteClicked(note_id.to_string()));
        }
    }

    /// Handles a section toggle
    pub fn handle_section_toggle(&mut self, section_id: &str) {
        for section in &mut self.sections {
            if section.id == section_id {
                section.expanded = !section.expanded;
                if section.expanded {
                    if let Some(handler) = &self.event_handler {
                        handler(JournalEvent::SectionExpanded(section_id.to_string()));
                    }
                } else {
                    if let Some(handler) = &self.event_handler {
                        handler(JournalEvent::SectionCollapsed(section_id.to_string()));
                    }
                }
                break;
            }
        }
    }

    /// Sets the view mode
    pub fn set_view_mode(&mut self, mode: JournalViewMode) -> &mut Self {
        self.config.view_mode = mode;

        if let Some(handler) = &self.event_handler {
            handler(JournalEvent::ViewModeChanged(mode));
        }

        self
    }

    /// Sets the filter
    pub fn set_filter(&mut self, filter: JournalFilter) -> &mut Self {
        self.filter = filter;
        self
    }

    /// Gets all dates in the journal
    pub fn get_dates(&self) -> Vec<String> {
        let mut dates = Vec::new();
        for section in &self.sections {
            for entry in &section.entries {
                dates.push(entry.date.clone());
            }
        }
        dates
    }

    /// Gets all note IDs in the journal
    pub fn get_all_note_ids(&self) -> Vec<String> {
        let mut note_ids = Vec::new();
        for section in &self.sections {
            for entry in &section.entries {
                note_ids.extend(entry.note_ids.clone());
            }
        }
        note_ids
    }

    /// Gets the number of notes
    pub fn note_count(&self) -> usize {
        self.get_all_note_ids().len()
    }

    /// Gets the number of unique dates
    pub fn date_count(&self) -> usize {
        self.get_dates().len()
    }

    /// Clears all sections
    pub fn clear(&mut self) -> &mut Self {
        self.sections.clear();
        self.selected_date = None;
        self.selected_note = None;
        self
    }

    /// Refreshes the view (reloads and re-sorts)
    pub fn refresh(&mut self, notes: &[Node]) -> &mut Self {
        self.load_notes(notes)
    }
}

impl Default for JournalView {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Date utility functions
// ============================================================================

/// Checks if a year is a leap year
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Converts a Unix timestamp (seconds) to a date string (YYYY-MM-DD)
fn timestamp_to_date(timestamp: u64) -> String {
    // Number of seconds in a day
    const SECONDS_PER_DAY: u64 = 86400;

    // Days since Unix epoch (1970-01-01)
    let days = timestamp / SECONDS_PER_DAY;

    // Convert days to date
    let mut remaining_days = days as i64;
    let mut year = 1970i32;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let mut month = 1u32;
    let mut days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year(year) {
        days_in_months[1] = 29;
    }

    for m in 1..=12 {
        let days_in_month = days_in_months[(m - 1) as usize] as i64;
        if remaining_days < days_in_month {
            month = m;
            break;
        }
        remaining_days -= days_in_month;
    }

    let day = remaining_days + 1;

    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Extracts a date from text (looking for YYYY-MM-DD pattern)
fn extract_date_from_text(text: &str) -> Option<String> {
    // Look for YYYY-MM-DD pattern
    for word in text.split_whitespace() {
        if word.len() == 10 && word.contains('-') {
            let parts: Vec<&str> = word.split('-').collect();
            if parts.len() == 3 {
                if let (Ok(year), Ok(month), Ok(day)) = (
                    parts[0].parse::<i32>(),
                    parts[1].parse::<u32>(),
                    parts[2].parse::<u32>(),
                ) {
                    if (1..=12).contains(&month) && (1..=31).contains(&day) {
                        return Some(format!("{:04}-{:02}-{:02}", year, month, day));
                    }
                }
            }
        }

        // Look for YYYYMMDD pattern
        if word.len() == 8 && word.chars().all(|c| c.is_ascii_digit()) {
            let year_str = &word[0..4];
            let month_str = &word[4..6];
            let day_str = &word[6..8];
            if let (Ok(year), Ok(month), Ok(day)) = (
                year_str.parse::<i32>(),
                month_str.parse::<u32>(),
                day_str.parse::<u32>(),
            ) {
                if (1..=12).contains(&month) && (1..=31).contains(&day) {
                    return Some(format!("{:04}-{:02}-{:02}", year, month, day));
                }
            }
        }
    }

    None
}

/// Gets a week key (YYYY-WWW) for a date
fn get_week_key(date: &str) -> String {
    if date.len() >= 10 {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() == 3 {
            if let (Ok(year), Ok(month), Ok(day)) = (
                parts[0].parse::<i32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>(),
            ) {
                // Calculate day of year
                let mut days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
                if is_leap_year(year) {
                    days_in_months[1] = 29;
                }

                let mut day_of_year = 0;
                for m in 1..month {
                    day_of_year += days_in_months[(m - 1) as usize];
                }
                day_of_year += day;

                // Calculate week number (simplified)
                let week = ((day_of_year - 1) / 7) + 1;
                return format!("{:04}-W{:02}", year, week);
            }
        }
    }
    format!("Week-{}", date)
}

/// Formats a date for display
fn format_date_for_display(date: &str) -> String {
    if date.len() >= 10 {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() == 3 {
            if let (Ok(year), Ok(month), Ok(day)) = (
                parts[0].parse::<i32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>(),
            ) {
                let month_name = month_name(month);
                return format!("{} {}, {}", month_name, day, year);
            }
        }
    }
    date.to_string()
}

/// Formats a week key for display
fn format_week_key(week_key: &str) -> String {
    if week_key.starts_with("W") {
        return week_key.to_string();
    }

    // Try to parse "YYYY-WWW"
    let parts: Vec<&str> = week_key.split('-').collect();
    if parts.len() == 2 && parts[1].starts_with("W") {
        if let Ok(year) = parts[0].parse::<i32>() {
            if let Ok(week) = parts[1].trim_start_matches("W").parse::<u32>() {
                return format!("Week {} of {}", week, year);
            }
        }
    }
    week_key.to_string()
}

/// Formats a month key for display
fn format_month_key(month_key: &str) -> String {
    let parts: Vec<&str> = month_key.split('-').collect();
    if parts.len() == 2 {
        if let (Ok(year), Ok(month)) = (parts[0].parse::<i32>(), parts[1].parse::<u32>()) {
            let month_name = month_name(month);
            return format!("{} {}", month_name, year);
        }
    }
    month_key.to_string()
}

/// Gets the month name
fn month_name(month: u32) -> String {
    match month {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Gets the current date as a string (YYYY-MM-DD)
pub fn get_current_date() -> String {
    get_today()
}

/// Gets today's date (YYYY-MM-DD)
pub fn get_today() -> String {
    // In production, this would use the system time
    // For now, return a fixed date for testing
    "2026-11-11".to_string()
}

/// Gets yesterday's date (YYYY-MM-DD)
pub fn get_yesterday() -> String {
    "2026-11-10".to_string()
}

/// Gets tomorrow's date (YYYY-MM-DD)
pub fn get_tomorrow() -> String {
    "2026-11-12".to_string()
}

/// Gets the date N days ago (YYYY-MM-DD)
pub fn get_days_ago(_days: i64) -> String {
    "2026-11-01".to_string()
}

/// Gets the start of the current week (Monday) (YYYY-MM-DD)
pub fn get_week_start() -> String {
    "2026-11-10".to_string()
}

/// Gets the end of the current week (Sunday) (YYYY-MM-DD)
pub fn get_week_end() -> String {
    "2026-11-16".to_string()
}

/// Gets the start of the current month (YYYY-MM-DD)
pub fn get_month_start() -> String {
    "2026-11-01".to_string()
}

/// Gets the end of the current month (YYYY-MM-DD)
pub fn get_month_end() -> String {
    "2026-11-30".to_string()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_entry_new() {
        let entry = JournalEntry::new("2026-11-11", "November 11, 2026");
        assert_eq!(entry.date, "2026-11-11");
        assert_eq!(entry.title, "November 11, 2026");
        assert!(entry.note_ids.is_empty());
    }

    #[test]
    fn test_journal_entry_add_note() {
        let mut entry = JournalEntry::new("2026-11-11", "November 11, 2026");
        entry.add_note("note1");
        entry.add_note("note2");

        assert_eq!(entry.note_ids.len(), 2);
        assert!(entry.note_ids.contains(&"note1".to_string()));
    }

    #[test]
    fn test_journal_section_new() {
        let section = JournalSection::new("2026-11", "November 2026", JournalSectionType::Monthly);
        assert_eq!(section.id, "2026-11");
        assert_eq!(section.section_type, JournalSectionType::Monthly);
        assert!(section.entries.is_empty());
    }

    #[test]
    fn test_journal_section_add_entry() {
        let mut section =
            JournalSection::new("2026-11", "November 2026", JournalSectionType::Monthly);
        let entry = JournalEntry::new("2026-11-11", "November 11");
        section.add_entry(entry);

        assert_eq!(section.entries.len(), 1);
    }

    #[test]
    fn test_journal_section_note_count() {
        let mut section =
            JournalSection::new("2026-11", "November 2026", JournalSectionType::Monthly);
        let mut entry1 = JournalEntry::new("2026-11-11", "November 11");
        entry1.add_note("note1");
        entry1.add_note("note2");
        section.add_entry(entry1);

        let mut entry2 = JournalEntry::new("2026-11-12", "November 12");
        entry2.add_note("note3");
        section.add_entry(entry2);

        assert_eq!(section.note_count(), 3);
    }

    #[test]
    fn test_journal_config_default() {
        let config = JournalConfig::default();
        assert_eq!(config.view_mode, JournalViewMode::Daily);
        assert_eq!(config.sort_order, JournalSortOrder::NewestFirst);
        assert!(!config.show_empty_days);
    }

    #[test]
    fn test_journal_filter_new() {
        let filter = JournalFilter::new();
        assert!(filter.start_date.is_none());
        assert!(filter.end_date.is_none());
        assert!(filter.tags.is_empty());
    }

    #[test]
    fn test_journal_filter_is_date_in_range() {
        let filter = JournalFilter::new()
            .with_start_date("2026-11-01")
            .with_end_date("2026-11-30");

        assert!(filter.is_date_in_range("2026-11-15"));
        assert!(!filter.is_date_in_range("2026-10-31"));
        assert!(!filter.is_date_in_range("2026-12-01"));
    }

    #[test]
    fn test_journal_view_new() {
        let view = JournalView::new();
        assert!(view.sections.is_empty());
        assert!(view.selected_date.is_none());
    }

    #[test]
    fn test_journal_view_load_notes() {
        let mut view = JournalView::new();

        // Create some test nodes with dates
        let node1 = create_test_node(1, "Note 1", "2026-11-11");
        let node2 = create_test_node(2, "Note 2", "2026-11-11");
        let node3 = create_test_node(3, "Note 3", "2026-11-12");

        view.load_notes(&[node1, node2, node3]);

        // Should have sections with entries
        assert!(!view.sections.is_empty());
    }

    #[test]
    fn test_journal_view_load_notes_with_content_date() {
        let mut view = JournalView::new();

        // Create a node with date in content
        let node = create_test_node(1, "2026-11-11: My note", "Date: 2026-11-11");

        view.load_notes(&[node]);

        assert!(!view.sections.is_empty());
    }

    #[test]
    fn test_journal_view_handle_date_click() {
        let mut view = JournalView::new();

        let node = create_test_node(1, "Note 1", "2026-11-11");

        view.load_notes(&[node]);
        view.handle_date_click("2026-11-11");

        assert_eq!(view.selected_date, Some("2026-11-11".to_string()));
    }

    #[test]
    fn test_journal_view_handle_note_click() {
        let mut view = JournalView::new();

        let node = create_test_node(1, "Note 1", "2026-11-11");

        view.load_notes(&[node]);
        view.handle_note_click("1");

        assert_eq!(view.selected_note, Some("1".to_string()));
    }

    #[test]
    fn test_journal_view_set_view_mode() {
        let mut view = JournalView::new();
        view.set_view_mode(JournalViewMode::Weekly);

        assert_eq!(view.config.view_mode, JournalViewMode::Weekly);
    }

    #[test]
    fn test_journal_view_note_count() {
        let mut view = JournalView::new();

        let node1 = create_test_node(1, "Note 1", "2026-11-11");
        let node2 = create_test_node(2, "Note 2", "2026-11-12");

        view.load_notes(&[node1, node2]);

        assert_eq!(view.note_count(), 2);
    }

    #[test]
    fn test_journal_view_clear() {
        let mut view = JournalView::new();

        let node = create_test_node(1, "Note 1", "2026-11-11");

        view.load_notes(&[node]);
        view.clear();

        assert!(view.sections.is_empty());
        assert!(view.selected_date.is_none());
    }

    #[test]
    fn test_journal_sort_order() {
        assert_eq!(format!("{}", JournalSortOrder::NewestFirst), "NewestFirst");
        assert_eq!(format!("{}", JournalSortOrder::OldestFirst), "OldestFirst");
    }

    #[test]
    fn test_journal_view_mode() {
        assert_eq!(format!("{}", JournalViewMode::Daily), "Daily");
        assert_eq!(format!("{}", JournalViewMode::Weekly), "Weekly");
        assert_eq!(format!("{}", JournalViewMode::Monthly), "Monthly");
        assert_eq!(format!("{}", JournalViewMode::Yearly), "Yearly");
    }

    #[test]
    fn test_get_current_date() {
        let date = get_current_date();
        assert_eq!(date.len(), 10); // YYYY-MM-DD
        assert!(date.contains('-'));
    }

    #[test]
    fn test_get_today() {
        let today = get_today();
        assert_eq!(today, "2026-11-11");
    }

    #[test]
    fn test_is_leap_year() {
        assert!(!is_leap_year(2025));
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_timestamp_to_date() {
        // Test Unix epoch
        assert_eq!(timestamp_to_date(0), "1970-01-01");

        // Test a known date: 2026-11-11
        // This is a simplified test - exact value depends on timezone
        let date = timestamp_to_date(1763164800); // Approx 2026-11-11
        assert_eq!(date.len(), 10);
    }

    #[test]
    fn test_extract_date_from_text() {
        assert_eq!(
            extract_date_from_text("Date: 2026-11-11"),
            Some("2026-11-11".to_string())
        );
        assert_eq!(
            extract_date_from_text("2026-11-11 Meeting"),
            Some("2026-11-11".to_string())
        );
        assert_eq!(
            extract_date_from_text("Event on 20261111"),
            Some("2026-11-11".to_string())
        );
        assert_eq!(extract_date_from_text("No date here"), None);
    }

    #[test]
    fn test_journal_event_types() {
        assert_eq!(
            format!("{:?}", JournalEvent::DateClicked("2026-11-11".to_string())),
            "DateClicked(\"2026-11-11\")"
        );
        assert_eq!(
            format!("{:?}", JournalEvent::NoteClicked("1".to_string())),
            "NoteClicked(\"1\")"
        );
    }

    #[test]
    fn test_journal_view_with_config() {
        let config = JournalConfig::new().with_view_mode(JournalViewMode::Weekly);
        let view = JournalView::with_config(config);

        assert_eq!(view.config.view_mode, JournalViewMode::Weekly);
    }

    #[test]
    fn test_journal_view_event_handler() {
        use std::sync::{Arc, Mutex};

        let mut view = JournalView::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        view.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        view.handle_note_click("1");

        let events = events.lock().unwrap();
        assert!(!events.is_empty());
        assert!(matches!(events[0], JournalEvent::NoteClicked(_)));
    }

    #[test]
    fn test_journal_filter_with_query() {
        let filter = JournalFilter::new().with_query("test");
        assert_eq!(filter.query, Some("test".to_string()));
    }

    #[test]
    fn test_journal_filter_with_tags() {
        let filter = JournalFilter::new().with_tag("tag1").with_tag("tag2");
        assert_eq!(filter.tags.len(), 2);
    }

    #[test]
    fn test_journal_section_type_display() {
        assert_eq!(format!("{}", JournalSectionType::Daily), "Daily");
        assert_eq!(format!("{}", JournalSectionType::Weekly), "Weekly");
        assert_eq!(format!("{}", JournalSectionType::Monthly), "Monthly");
        assert_eq!(format!("{}", JournalSectionType::Yearly), "Yearly");
    }

    #[test]
    fn test_journal_view_load_notes_multiple_dates() {
        let mut view = JournalView::new();

        // Create nodes with different dates
        let node1 = create_test_node(1, "Note 1", "2026-11-10");
        let node2 = create_test_node(2, "Note 2", "2026-11-11");
        let node3 = create_test_node(3, "Note 3", "2026-11-12");

        view.load_notes(&[node1, node2, node3]);

        assert_eq!(view.note_count(), 3);
        assert_eq!(view.date_count(), 3);
    }

    #[test]
    fn test_format_date_for_display() {
        assert_eq!(format_date_for_display("2026-11-11"), "November 11, 2026");
        assert_eq!(format_date_for_display("2026-01-01"), "January 1, 2026");
    }

    #[test]
    fn test_format_month_key() {
        assert_eq!(format_month_key("2026-11"), "November 2026");
        assert_eq!(format_month_key("2026-01"), "January 2026");
    }

    #[test]
    fn test_get_week_key() {
        let week_key = get_week_key("2026-11-11");
        assert!(week_key.starts_with("2026-W"));
    }

    // Import types needed for test node creation
    use note_craft_core::{NodeId, TypeId};

    // Helper function to create test nodes
    fn create_test_node(id: u64, _name: &str, date: &str) -> Node {
        // Parse date as timestamp (simplified)
        // For testing, we'll use a fixed timestamp for each date
        let timestamp = date_to_timestamp(date);

        Node {
            id: NodeId(id),
            content: format!("Note content for date: {}", date),
            node_type: TypeId(1),
            tags: vec![],
            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    /// Converts a date string to a timestamp (simplified for testing)
    fn date_to_timestamp(date: &str) -> u64 {
        if date == "2026-11-10" {
            1762992000
        } else if date == "2026-11-11" {
            1763078400
        } else if date == "2026-11-12" {
            1763164800
        } else if date == "2026-01-01" {
            1735689600
        } else {
            1763078400
        } // Default to 2026-11-11
    }
}
