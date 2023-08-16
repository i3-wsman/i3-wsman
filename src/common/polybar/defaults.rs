// Default
pub const FG: &str = "77fdfefe";
pub const BG: &str = "77000000";
pub const WS_LABEL: &str = "";

// Focused
pub const FOCUSED_FG: &str = "ff8080f0";
pub const FOCUSED_BG: &str = "b9010202";
pub const FOCUSED_WS_LABEL: &str = "";

// Urgent
pub const URGENT_FG: &str = "ffc2bd60";
pub const URGENT_BG: &str = BG;
// pub const URGENT_WS_LABEL: &str = "";
pub const URGENT_WS_LABEL: &str = "";

// Visible
pub const VISIBLE_FG: &str = "ccfdfefe";
pub const VISIBLE_BG: &str = "99010202";
pub const VISIBLE_WS_LABEL: &str = WS_LABEL;

// Hidden
pub const HIDDEN_FG: &str = "11fdfefe";
pub const HIDDEN_BG: &str = BG;
pub const HIDDEN_WS_LABEL: &str = "";

// Hidden + Focused
pub const HIDDEN_FOCUSED_FG: &str = "cc8080f0";
pub const HIDDEN_FOCUSED_BG: &str = VISIBLE_BG;
pub const HIDDEN_FOCUSED_WS_LABEL: &str = HIDDEN_WS_LABEL;

// Hidden + Urgent
pub const HIDDEN_URGENT_FG: &str = "88c2bd60";
pub const HIDDEN_URGENT_BG: &str = BG;
pub const HIDDEN_URGENT_WS_LABEL: &str = URGENT_WS_LABEL;

// Hidden + Visible
pub const HIDDEN_VISIBLE_FG: &str = "aafdfefe";
pub const HIDDEN_VISIBLE_BG: &str = VISIBLE_BG;
pub const HIDDEN_VISIBLE_WS_LABEL: &str = HIDDEN_WS_LABEL;

// Toggle Hidden Button
pub const GROUP_ALL_LABEL: &str = "";
pub const TOGGLE_HIDDEN_LABEL: &str = "";
pub const TOGGLE_HIDDEN_ALL_FG: &str = "00000000";
pub const TOGGLE_HIDDEN_ON_FG: &str = VISIBLE_FG;
pub const TOGGLE_HIDDEN_OFF_FG: &str = "33fdfefe";



// Group
pub const GROUP_FG: &str = "ccfdfefe";
pub const GROUP_BG: &str = "82010202";

// Group: Focused
pub const GROUP_FOCUSED_FG: &str = "ccfdfefe";
pub const GROUP_FOCUSED_BG: &str = VISIBLE_BG;

// Group: Active
pub const GROUP_ACTIVE_FG: &str = VISIBLE_FG;
pub const GROUP_ACTIVE_BG: &str = "b9010202";

// Group: Hidden
pub const GROUP_HIDDEN_FG: &str = "55fdfefe";
pub const GROUP_HIDDEN_BG: &str = HIDDEN_BG;

// Group: Hidden + Focused
pub const GROUP_HIDDEN_FOCUSED_FG: &str = FG;
pub const GROUP_HIDDEN_FOCUSED_BG: &str = VISIBLE_BG;
