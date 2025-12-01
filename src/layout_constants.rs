//! Layout and dimension constants for consistent sizing across the application
//!
//! Constants are grouped by category:
//! - **Layout dimensions**: Sidebar, mini calendar, calendar views
//! - **Spacing & Padding**: Consistent spacing values
//! - **Border styling**: Radius and width values
//! - **Color picker**: Button and grid dimensions
//! - **Shadow properties**: Drop shadow configuration
//! - **Icons**: System icon names
//! - **Font sizes**: Typography scale
//! - **Event display**: Event chip dimensions and thresholds
//! - **Day cell layout**: Cell structure dimensions

// =============================================================================
// Layout Dimensions
// =============================================================================

/// Width of the sidebar panel
pub const SIDEBAR_WIDTH: f32 = 280.0;

/// Size of day buttons in the mini calendar
pub const MINI_CALENDAR_DAY_BUTTON_SIZE: f32 = 32.0;

/// Fixed height for mini calendar (5 weeks + header row)
pub const MINI_CALENDAR_GRID_HEIGHT: f32 = 210.0;

// =============================================================================
// Calendar View Dimensions
// =============================================================================

/// Height of each hour slot in week/day views
pub const HOUR_ROW_HEIGHT: f32 = 60.0;

/// Width of the time labels column
pub const TIME_LABEL_WIDTH: f32 = 60.0;

/// Height of all-day events section
pub const ALL_DAY_HEADER_HEIGHT: f32 = 40.0;

/// Width of week number column (slim)
pub const WEEK_NUMBER_WIDTH: f32 = 32.0;

// =============================================================================
// Menu Bar Dimensions (COSMIC Standard)
// =============================================================================

/// Standard height for menu items
pub const MENU_ITEM_HEIGHT: u16 = 40;

/// Standard width for menu items
pub const MENU_ITEM_WIDTH: u16 = 240;

/// Spacing between menu items
pub const MENU_SPACING: f32 = 4.0;

// =============================================================================
// Spacing Values
// =============================================================================

/// Tiny spacing (1px) - minimal separation
pub const SPACING_TINY: u16 = 1;

/// Extra extra small spacing (2px)
pub const SPACING_XXS: u16 = 2;

/// Small spacing (4px) - tight layouts
pub const SPACING_SMALL: u16 = 4;

/// Medium spacing (8px) - standard element separation
pub const SPACING_MEDIUM: u16 = 8;

/// Mini calendar specific spacing (12px)
pub const SPACING_MINI_CALENDAR: u16 = 12;

/// Large spacing (20px) - section separation
pub const SPACING_LARGE: u16 = 20;

// =============================================================================
// Padding Values
// =============================================================================

/// Tiny padding (4px) - minimal internal space
pub const PADDING_TINY: u16 = 4;

/// Small padding (8px) - compact elements
pub const PADDING_SMALL: u16 = 8;

/// Medium padding (12px) - standard elements
pub const PADDING_MEDIUM: u16 = 12;

/// Standard padding (16px) - containers and dialogs
pub const PADDING_STANDARD: u16 = 16;

/// Month grid padding (20px)
pub const PADDING_MONTH_GRID: u16 = 20;

/// Large padding (40px) - large dialogs/panels
#[allow(dead_code)] // Reserved for future larger dialogs/panels
pub const PADDING_LARGE: u16 = 40;

/// Day cell padding [top, right, bottom, left]
pub const PADDING_DAY_CELL: [u16; 4] = [4, 8, 4, 8];

/// Indented color picker padding [top, right, bottom, left]
pub const PADDING_COLOR_PICKER_NESTED: [u16; 4] = [4, 0, 4, 36];

// =============================================================================
// Border Styling
// =============================================================================

/// Standard border radius for containers and chips (uniform corners)
pub const BORDER_RADIUS: [f32; 4] = [4.0, 4.0, 4.0, 4.0];

/// Single border radius value for use with span_border_radius functions
pub const BORDER_RADIUS_VALUE: f32 = 4.0;

/// Smaller border radius for compact event indicators
pub const BORDER_RADIUS_SMALL: f32 = 2.0;

/// Thin border width for cell grid borders
pub const BORDER_WIDTH_THIN: f32 = 0.5;

/// Normal border width for standard day cell borders
pub const BORDER_WIDTH_NORMAL: f32 = 1.0;

/// Highlight border width for today indicator and selected items
pub const BORDER_WIDTH_HIGHLIGHT: f32 = 2.0;

/// Selected border width for color picker selected state
pub const BORDER_WIDTH_SELECTED: f32 = 3.0;

/// Light border opacity for subtle borders
#[allow(dead_code)] // Reserved for future border styling
pub const LIGHT_BORDER_OPACITY: f32 = 0.2;

// =============================================================================
// Color Picker Dimensions
// =============================================================================

/// Small color button size (28px)
pub const COLOR_BUTTON_SIZE_SMALL: f32 = 28.0;

/// Medium color button size (32px)
pub const COLOR_BUTTON_SIZE_MEDIUM: f32 = 32.0;

/// Large color button size (36px)
pub const COLOR_BUTTON_SIZE_LARGE: f32 = 36.0;

/// Size of color indicator circle
pub const COLOR_INDICATOR_SIZE: f32 = 24.0;

/// Number of colors per row in palette grid
#[allow(dead_code)] // Reserved for future color palette grid
pub const COLOR_PALETTE_COLUMNS: usize = 6;

/// Spacing within color grid
pub const SPACING_COLOR_GRID: u16 = 6;

/// Spacing around color container
pub const SPACING_COLOR_CONTAINER: u16 = 8;

// =============================================================================
// Shadow Properties
// =============================================================================

/// Shadow opacity for drop shadows
pub const SHADOW_OPACITY: f32 = 0.3;

/// Shadow horizontal offset
pub const SHADOW_OFFSET_X: f32 = 2.0;

/// Shadow vertical offset
pub const SHADOW_OFFSET_Y: f32 = 0.0;

/// Shadow blur radius
pub const SHADOW_BLUR_RADIUS: f32 = 10.0;

// =============================================================================
// Icon Names (System Symbolic Icons)
// =============================================================================

/// Previous/back navigation icon
pub const ICON_PREVIOUS: &str = "go-previous-symbolic";

/// Next/forward navigation icon
pub const ICON_NEXT: &str = "go-next-symbolic";

/// Search icon
pub const ICON_SEARCH: &str = "system-search-symbolic";

/// Today/calendar icon
pub const ICON_TODAY: &str = "x-office-calendar-symbolic";

/// Add/plus icon
pub const ICON_ADD: &str = "list-add-symbolic";

// =============================================================================
// Font Sizes
// =============================================================================

/// Small font size (11px) - compact text
#[allow(dead_code)] // For future use
pub const FONT_SIZE_SMALL: u16 = 11;

/// Medium font size (12px) - standard UI text
pub const FONT_SIZE_MEDIUM: u16 = 12;

/// Body font size (14px) - readable content
#[allow(dead_code)] // For future use
pub const FONT_SIZE_BODY: u16 = 14;

/// Large font size (24px) - headers
pub const FONT_SIZE_LARGE: u16 = 24;

// =============================================================================
// Event Display Dimensions
// =============================================================================

/// Height of a date event chip in full mode (overlay and day cells must match)
pub const DATE_EVENT_HEIGHT: f32 = 19.0;

/// Height of a compact date event indicator (thin colored line)
pub const COMPACT_EVENT_HEIGHT: f32 = 6.0;

/// Spacing between date event rows
pub const DATE_EVENT_SPACING: f32 = 2.0;

/// Height of the "+N more" overflow indicator (full mode)
pub const OVERFLOW_INDICATOR_HEIGHT: f32 = 14.0;

/// Height of the "+N" overflow indicator (compact mode)
pub const COMPACT_OVERFLOW_HEIGHT: f32 = 10.0;

// =============================================================================
// Event Display Mode Thresholds
// =============================================================================

/// Minimum cell height to show full event chips (below this, use compact mode)
pub const MIN_CELL_HEIGHT_FOR_FULL_EVENTS: f32 = 80.0;

/// Minimum cell width to show full event chips (below this, use compact mode)
pub const MIN_CELL_WIDTH_FOR_FULL_EVENTS: f32 = 80.0;

/// Minimum cell height to show overflow indicator (below this, hide it)
pub const MIN_CELL_HEIGHT_FOR_OVERFLOW: f32 = 50.0;

// =============================================================================
// Day Cell Layout
// =============================================================================

/// Height reserved for day number header in day cells
pub const DAY_HEADER_HEIGHT: f32 = 28.0;

/// Offset from top of cell to where events start (header + spacing)
pub const DAY_CELL_HEADER_OFFSET: f32 = 32.0;

/// Top padding of day cells
pub const DAY_CELL_TOP_PADDING: f32 = 4.0;
