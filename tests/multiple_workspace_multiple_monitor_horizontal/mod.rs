use std::collections::HashMap;
use std::time::Instant;

use hyprland::shared::WorkspaceId;

use window_switcher::{MonitorData, WorkspaceData};
use window_switcher::sort::{sort_clients, update_clients, update_monitors};

use crate::common::{create_svg_from_client_tests, is_sorted, MockClient};

/// ```
///                   Monitor 1                                   Monitor 2
///       Workspace 0           Workspace 1           Workspace 10          Workspace 11
/// 1  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 2  |  1   |  |  2   | | |  5   |  |  6   |  |  |  9   |  |  10  | | |  13  |  |  14  |
/// 3  |      |  |      | | |      |  +------+  |  |      |  |      | | |      |  +------+
/// 4  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 5  +------+  +------+ | +------+  |  7   |  |  +---------+  +---+ | +------+  |  15  |
/// 6  |  3   |  |  4   | | |  8   |  |      |  |  |   11    |  |12 | | |  16  |  |      |
/// 7  +------+  +------+ | +------+  +------+  |  +---------+  +---+ | +------+  +------+
///    1      2  3      4   1      2  3      4     5      6  7  8   9   5      6  7   8  9
/// ```
#[test]
fn default() {
    let clients = vec![
        MockClient(1, 1, 1, 3, 0, 0, "1".to_string()),
        MockClient(3, 1, 1, 3, 0, 0, "2".to_string()),
        MockClient(1, 5, 1, 2, 0, 0, "3".to_string()),
        MockClient(3, 5, 1, 2, 0, 0, "4".to_string()),

        MockClient(1, 1, 1, 3, 1, 0, "5".to_string()),
        MockClient(3, 1, 1, 2, 1, 0, "6".to_string()),
        MockClient(3, 4, 1, 3, 1, 0, "7".to_string()),
        MockClient(1, 5, 1, 2, 1, 0, "8".to_string()),

        MockClient(5, 1, 1, 3, 10, 1, "9".to_string()),
        MockClient(7, 1, 2, 3, 10, 1, "10".to_string()),
        MockClient(5, 5, 2, 2, 10, 1, "11".to_string()),
        MockClient(8, 5, 1, 2, 10, 1, "12".to_string()),

        MockClient(5, 1, 1, 3, 11, 1, "13".to_string()),
        MockClient(7, 1, 2, 2, 11, 1, "14".to_string()),
        MockClient(7, 4, 2, 3, 11, 1, "15".to_string()),
        MockClient(5, 5, 1, 2, 11, 1, "16".to_string()),
    ];

    let mut monitor_data: HashMap<i64, MonitorData> = HashMap::new();
    monitor_data.insert(0, MonitorData { x: 0, y: 0, width: 4, height: 7, combined_width: 8, combined_height: 7, workspaces_on_monitor: 2 });
    monitor_data.insert(1, MonitorData { x: 5, y: 0, width: 5, height: 7, combined_width: 10, combined_height: 7, workspaces_on_monitor: 2 });

    let mut workspace_data: HashMap<WorkspaceId, WorkspaceData> = HashMap::new();
    workspace_data.insert(0, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(1, WorkspaceData { x: 5, y: 0 });
    workspace_data.insert(10, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(11, WorkspaceData { x: 5, y: 0 });

    let monitor_data = update_monitors(monitor_data);
    println!("updated monitor_data: {:?}", monitor_data);
    let clients = update_clients(clients, &workspace_data, &monitor_data);
    println!("updated clients: {:?}", clients);

    let start = Instant::now();
    let ve = sort_clients(clients, false, false);
    println!("{ve:?} ({:?})", start.elapsed());
    create_svg_from_client_tests(&ve, "multiple_workspace_multiple_monitor_horizontal", monitor_data);

    assert!(is_sorted(&ve));
}


/// ```
///                   Monitor 1                                   Monitor 2
///       Workspace 0           Workspace 1           Workspace 10         Workspace 11
/// 1  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 2  |  1   |  |  2   | | |  3   |  |  4   |  |  |  9   |  |  10  | | |  11  |  |  12  |
/// 3  |      |  |      | | |      |  +------+  |  |      |  |      | | |      |  +------+
/// 4  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 5  +------+  +------+ | +------+  |  5   |  |  +---------+  +---+ | +------+  |  13  |
/// 6  |  6   |  |  7   | | |  8   |  |      |  |  |   14    |  |15 | | |  16  |  |      |
/// 7  +------+  +------+ | +------+  +------+  |  +---------+  +---+ | +------+  +------+
///    1      2  3      4   1      2  3      4     5      6  7  8   9   5      6  7   8  9
/// ```
#[test]
fn ignore_workspaces() {
    let ve = vec![
        MockClient(1, 1, 1, 3, 0, 0, "1".to_string()),
        MockClient(3, 1, 1, 3, 0, 0, "2".to_string()),
        MockClient(1, 5, 1, 2, 0, 0, "6".to_string()),
        MockClient(3, 5, 1, 2, 0, 0, "7".to_string()),

        MockClient(1, 1, 1, 3, 1, 0, "3".to_string()),
        MockClient(3, 1, 1, 2, 1, 0, "4".to_string()),
        MockClient(3, 4, 1, 3, 1, 0, "5".to_string()),
        MockClient(1, 5, 1, 2, 1, 0, "8".to_string()),

        MockClient(5, 1, 1, 3, 10, 1, "9".to_string()),
        MockClient(7, 1, 2, 3, 10, 1, "10".to_string()),
        MockClient(5, 5, 2, 2, 10, 1, "14".to_string()),
        MockClient(8, 5, 1, 2, 10, 1, "15".to_string()),

        MockClient(5, 1, 1, 3, 11, 1, "11".to_string()),
        MockClient(7, 1, 2, 2, 11, 1, "12".to_string()),
        MockClient(7, 4, 2, 3, 11, 1, "13".to_string()),
        MockClient(5, 5, 1, 2, 11, 1, "16".to_string()),
    ];

    let mut monitor_data: HashMap<i64, MonitorData> = HashMap::new();
    monitor_data.insert(0, MonitorData { x: 0, y: 0, width: 4, height: 7, combined_width: 8, combined_height: 7, workspaces_on_monitor: 2 });
    monitor_data.insert(1, MonitorData { x: 5, y: 0, width: 5, height: 7, combined_width: 10, combined_height: 7, workspaces_on_monitor: 2 });

    let mut workspace_data: HashMap<WorkspaceId, WorkspaceData> = HashMap::new();
    workspace_data.insert(0, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(1, WorkspaceData { x: 5, y: 0 });
    workspace_data.insert(10, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(11, WorkspaceData { x: 5, y: 0 });

    let ve = update_clients(ve, &workspace_data, &monitor_data);


    let start = Instant::now();
    let ve = sort_clients(ve, true, false);
    println!("{ve:?} ({:?})", start.elapsed());
    create_svg_from_client_tests(&ve, "multiple_workspace_multiple_monitor_horizontal_ignore_workspaces", monitor_data);

    assert!(is_sorted(&ve));
}



/// ```
///                   Monitor 1                                   Monitor 2
///       Workspace 0           Workspace 1           Workspace 10          Workspace 11
/// 1  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 2  |  1   |  |  2   | | |  9   |  |  10  |  |  |  3   |  |  4   | | |  12  |  |  13  |
/// 3  |      |  |      | | |      |  +------+  |  |      |  |      | | |      |  +------+
/// 4  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 5  +------+  +------+ | +------+  |  11  |  |  +---------+  +---+ | +------+  |  14  |
/// 6  |  5   |  |  6   | | |  15  |  |      |  |  |   7     |  | 8 | | |  16  |  |      |
/// 7  +------+  +------+ | +------+  +------+  |  +---------+  +---+ | +------+  +------+
///    1      2  3      4   1      2  3      4     5      6  7  8   9   5      6  7  8   9
/// ```
#[test]
fn ignore_monitor() {
    let ve = vec![
        MockClient(1, 1, 1, 3, 0, 0, "1".to_string()),
        MockClient(3, 1, 1, 3, 0, 0, "2".to_string()),
        MockClient(1, 5, 1, 2, 0, 0, "5".to_string()),
        MockClient(3, 5, 1, 2, 0, 0, "6".to_string()),

        MockClient(1, 1, 1, 3, 1, 0, "9".to_string()),
        MockClient(3, 1, 1, 2, 1, 0, "10".to_string()),
        MockClient(3, 4, 1, 3, 1, 0, "11".to_string()),
        MockClient(1, 5, 1, 2, 1, 0, "15".to_string()),

        MockClient(5, 1, 1, 3, 10, 1, "3".to_string()),
        MockClient(7, 1, 2, 3, 10, 1, "4".to_string()),
        MockClient(5, 5, 2, 2, 10, 1, "7".to_string()),
        MockClient(8, 5, 1, 2, 10, 1, "8".to_string()),

        MockClient(5, 1, 1, 3, 11, 1, "12".to_string()),
        MockClient(7, 1, 2, 2, 11, 1, "13".to_string()),
        MockClient(7, 4, 2, 3, 11, 1, "14".to_string()),
        MockClient(5, 5, 1, 2, 11, 1, "16".to_string()),
    ];

    let mut monitor_data: HashMap<i64, MonitorData> = HashMap::new();
    monitor_data.insert(0, MonitorData { x: 0, y: 0, width: 4, height: 7, combined_width: 8, combined_height: 7, workspaces_on_monitor: 2 });
    monitor_data.insert(1, MonitorData { x: 5, y: 0, width: 5, height: 7, combined_width: 10, combined_height: 7, workspaces_on_monitor: 2 });

    let mut workspace_data: HashMap<WorkspaceId, WorkspaceData> = HashMap::new();
    workspace_data.insert(0, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(1, WorkspaceData { x: 5, y: 0 });
    workspace_data.insert(10, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(11, WorkspaceData { x: 5, y: 0 });

    let ve = update_clients(ve, &workspace_data, &monitor_data);

    let start = Instant::now();
    let ve = sort_clients(ve, false, true);
    println!("{ve:?} ({:?})", start.elapsed());
    create_svg_from_client_tests(&ve, "multiple_workspace_multiple_monitor_horizontal_ignore_monitor", monitor_data);

    assert!(is_sorted(&ve));
}


/// ```
///                   Monitor 1                                   Monitor 2
///       Workspace 1           Workspace 2           Workspace 3           Workspace 4
/// 1  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 2  |  1   |  |  2   | | |  3   |  |  4   |  |  |  6   |  |  7   | | |  8   |  |  9   |
/// 3  |      |  |      | | |      |  +------+  |  |      |  |      | | |      |  +------+
/// 4  +------+  +------+ | +------+  +------+  |  +------+  +------+ | +------+  +------+
/// 5  +------+  +------+ | +------+  |  5   |  |  +---------+  +---+ | +------+  |  10  |
/// 6  |  11  |  |  12  | | |  13  |  |      |  |  |   14    |  |15 | | |  16  |  |      |
/// 7  +------+  +------+ | +------+  +------+  |  +---------+  +---+ | +------+  +------+
///    1      2  3      4   1      2  3      4     5      6  7  8   9   5      6  7  8   9
/// ```
#[test]
fn ignore_monitor_ignore_workspace() {
    let ve = vec![
        MockClient(1, 1, 1, 3, 0, 0, "1".to_string()),
        MockClient(3, 1, 1, 3, 0, 0, "2".to_string()),
        MockClient(1, 5, 1, 2, 0, 0, "11".to_string()),
        MockClient(3, 5, 1, 2, 0, 0, "12".to_string()),

        MockClient(1, 1, 1, 3, 1, 0, "3".to_string()),
        MockClient(3, 1, 1, 2, 1, 0, "4".to_string()),
        MockClient(3, 4, 1, 3, 1, 0, "5".to_string()),
        MockClient(1, 5, 1, 2, 1, 0, "13".to_string()),

        MockClient(5, 1, 1, 3, 10, 1, "6".to_string()),
        MockClient(7, 1, 2, 3, 10, 1, "7".to_string()),
        MockClient(5, 5, 2, 2, 10, 1, "14".to_string()),
        MockClient(8, 5, 1, 2, 10, 1, "15".to_string()),

        MockClient(5, 1, 1, 3, 11, 1, "8".to_string()),
        MockClient(7, 1, 2, 2, 11, 1, "9".to_string()),
        MockClient(7, 4, 2, 3, 11, 1, "10".to_string()),
        MockClient(5, 5, 1, 2, 11, 1, "16".to_string()),
    ];

    let mut monitor_data: HashMap<i64, MonitorData> = HashMap::new();
    monitor_data.insert(0, MonitorData { x: 0, y: 0, width: 4, height: 7, combined_width: 8, combined_height: 7, workspaces_on_monitor: 2 });
    monitor_data.insert(1, MonitorData { x: 5, y: 0, width: 5, height: 7, combined_width: 10, combined_height: 7, workspaces_on_monitor: 2 });

    let mut workspace_data: HashMap<WorkspaceId, WorkspaceData> = HashMap::new();
    workspace_data.insert(0, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(1, WorkspaceData { x: 5, y: 0 });
    workspace_data.insert(10, WorkspaceData { x: 0, y: 0 });
    workspace_data.insert(11, WorkspaceData { x: 5, y: 0 });

    let ve = update_clients(ve, &workspace_data, &monitor_data);

    let start = Instant::now();
    let ve = sort_clients(ve, true, true);
    println!("{ve:?} ({:?})", start.elapsed());
    create_svg_from_client_tests(&ve, "multiple_workspace_multiple_monitor_horizontal_ignore_monitor_ignore_workspace", monitor_data);

    assert!(is_sorted(&ve));
}