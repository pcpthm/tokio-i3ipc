//! Contains structs for deserializing messages from i3
use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use std::collections::HashMap;

/// Generic success reply
#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct Success {
    pub success: bool,
    pub error: Option<String>,
}

/// Workspaces reply
pub type Workspaces = Vec<Workspace>;

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct Workspace {
    pub id: i64,
    pub num: i32,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub rect: Rect,
    pub output: String,
}

/// Outputs reply
pub type Outputs = Vec<Output>;

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct Output {
    pub name: String,
    pub active: bool,
    pub primary: bool,
    pub current_workspace: Option<String>,
    pub rect: Rect,
}

/// Tree/Node reply
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Node {
    pub id: i64,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub layout: NodeLayout,
    pub output: Option<String>,
    pub orientation: NodeOrientation,
    pub border: NodeBorder,
    pub percent: Option<f64>,
    pub rect: Rect,
    pub window_rect: Rect,
    pub deco_rect: Rect,
    pub geometry: Rect,
    pub window: Option<i64>,
    pub window_properties: Option<WindowProperties>,
    pub window_type: Option<WindowType>,
    pub current_border_width: i32,
    pub urgent: bool,
    pub marks: Option<Marks>,
    pub focused: bool,
    pub focus: Vec<i64>,
    pub sticky: bool,
    pub floating: Option<Floating>,
    pub floating_nodes: Vec<Node>,
    pub fullscreen_mode: FullscreenMode, // only containers and views
    pub nodes: Vec<Node>,

    pub representation: Option<String>,          // only workspaces
    pub app_id: Option<String>,                  // only views
    pub pid: Option<i32>,                        // only views
    pub visible: Option<bool>,                   // only views
    pub shell: Option<String>,                   // only views
    pub inhibit_idle: Option<bool>,              // only views
    pub idle_inhibitors: Option<IdleInhibitors>, // only views
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

#[derive(Eq, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct WindowProperties {
    pub title: Option<String>,
    pub instance: Option<String>,
    pub class: Option<String>,
    pub window_role: Option<String>,
    pub transient_for: Option<i64>,

    pub window_type: Option<String>,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Copy, Clone, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Floating {
    AutoOff,
    AutoOn,
    UserOff,
    UserOn,
}

#[derive(Deserialize_repr, Serialize_repr, Eq, PartialEq, Copy, Clone, Hash, Debug)]
#[repr(u8)]
pub enum FullscreenMode {
    None = 0,
    Output = 1,
    Global = 2,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Copy, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WindowProperty {
    Title,
    Instance,
    Class,
    WindowRole,
    TransientFor,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Copy, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WindowType {
    Normal,
    Dock,
    Dialog,
    Utility,
    Toolbar,
    Splash,
    Menu,
    DropdownMenu,
    PopupMenu,
    Tooltip,
    Notification,
    Unknown,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Hash, Debug)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Hash, Debug, Copy)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Root,
    Output,
    Con,
    FloatingCon,
    Workspace,
    Dockarea,
}
#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NodeBorder {
    Normal,
    None,
    Pixel,
    Csd,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum NodeLayout {
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
    Dockarea,
    Output,
    None,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum NodeOrientation {
    Horizontal,
    Vertical,
    None,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct IdleInhibitors {
    pub application: IdleInhibitorApplication,
    pub user: IdleInhibitorUser,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum IdleInhibitorApplication {
    Enabled,
    None,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum IdleInhibitorUser {
    Focus,
    FullScreen,
    Open,
    Visible,
    None,
}

/// Marks Reply
#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct Marks(Vec<String>);

/// BarIds
#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct BarIds(Vec<String>);

/// BarConfig Reply
#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct BarConfig {
    pub id: String,
    pub mode: String,
    pub position: String,
    pub status_command: String,
    pub font: String,
    pub workspace_buttons: bool,
    pub binding_mode_indicator: bool,
    pub verbose: bool,
    pub colors: HashMap<BarPart, String>,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BarPart {
    Background,
    Statusline,
    Separator,
    FocusedBackground,
    FocusedStatusline,
    FocusedSeparator,
    FocusedWorkspaceText,
    FocusedWorkspaceBg,
    FocusedWorkspaceBorder,
    ActiveWorkspaceText,
    ActiveWorkspaceBg,
    ActiveWorkspaceBorder,
    InactiveWorkspaceText,
    InactiveWorkspaceBg,
    InactiveWorkspaceBorder,
    UrgentWorkspaceText,
    UrgentWorkspaceBg,
    UrgentWorkspaceBorder,
    BindingModeText,
    BindingModeBg,
    BindingModeBorder,
}

/// Version reply
#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub human_readable: String,
    pub loaded_config_file_name: String,
}

/// Binding Modes Reply
#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct BindingModes(Vec<String>);

/// Config Reply
#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct Config {
    pub config: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let output = "{\"name\":\"xroot-0\",\"active\":false,\"primary\":false,\"rect\":{\"x\":0,\"y\":0,\"width\":5120,\"height\":1600},\"current_workspace\":null}";
        let o: Result<Output, serde_json::error::Error> = serde_json::from_str(output);
        assert!(o.is_ok());
    }

    #[test]
    fn test_success() {
        let output = "{\"success\":true}";
        let o: Result<Success, serde_json::error::Error> = serde_json::from_str(output);
        assert!(o.is_ok());
    }

    #[test]
    fn test_workspace() {
        let output = "{\"id\":1,\"num\":2,\"name\":\"2\",\"visible\":false,\"focused\":false,\"rect\":{\"x\":2560,\"y\":29,\"width\":2560,\"height\":1571},\"output\":\"DVI-I-3\",\"urgent\":false}";
        let o: Result<Workspace, serde_json::error::Error> = serde_json::from_str(output);
        assert!(o.is_ok());
    }
    #[test]
    fn test_binding_modes() {
        let output = "[\"resize\",\"default\"]";
        let o: Result<BindingModes, serde_json::error::Error> = serde_json::from_str(output);
        assert!(o.is_ok());
    }
    #[test]
    fn test_config() {
        let output = "{\"config\": \"some config data here\"}";
        let o: Result<Config, serde_json::error::Error> = serde_json::from_str(output);
        assert!(o.is_ok());
    }
    #[test]
    fn test_tree() {
        use std::fs;
        let output = fs::read_to_string("./test/tree.json").unwrap();
        let o: Result<Node, serde_json::error::Error> = serde_json::from_str(&output);
        assert!(o.is_ok());
    }
    #[test]
    fn test_other_tree() {
        use std::fs;
        let output = fs::read_to_string("./test/other_tree.json").unwrap();
        let o: Result<Node, serde_json::error::Error> = serde_json::from_str(&output);
        assert!(o.is_ok());
    }
    #[test]
    fn test_last_tree() {
        use std::fs;
        let output = fs::read_to_string("./test/last_tree.json").unwrap();
        let o: Result<Node, serde_json::error::Error> = serde_json::from_str(&output);
        assert!(o.is_ok());
    }
    #[test]
    fn test_version() {
        use std::fs;
        let output = fs::read_to_string("./test/version.json").unwrap();
        let o: Result<Version, serde_json::error::Error> = serde_json::from_str(&output);
        assert!(o.is_ok());
    }
}
