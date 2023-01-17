#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use discord_rpc_client::Client;
use discord_rpc_client::models::rich_presence::Activity;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use tauri::{
    State,
    Manager
};
struct App {
    client: Mutex<Client>,
    client_win: Mutex<DiscordIpcClient>,
}

fn main() {
    tauri::Builder::default()
    .manage(App {
        client: Mutex::new(init_discord_client()),
        client_win: Mutex::new(init_discord_client_win()),
    })
    .invoke_handler(tauri::generate_handler![update_presence, clear_presence])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn init_discord_client() -> Client {
    let mut client = Client::new(876674264500154408);
    client.start();
    return client;
}

fn init_discord_client_win() -> DiscordIpcClient {
    let mut client = DiscordIpcClient::new("876674264500154408").unwrap();
    client.connect().unwrap();
    return client;
}



#[tauri::command(async)]

fn update_presence(game: String, state: State<App>) -> bool {
    if cfg!(target_os = "windows") {
        let mut client_win = state.client_win.lock().unwrap();
        (*client_win).set_activity(
            activity::Activity::new()
                .state("Server cheap")
                .assets(
                    activity::Assets::new()
                        .large_image("ms-icon-310x310")
                        .large_text("Making Minecraft ")
                        .small_image("ms-icon-310x310")
                        .small_text("made easy")
                )
                .timestamps(
                    activity::Timestamps::new()
                        .start(SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap().as_secs().try_into().unwrap())
                )
        ).unwrap();
        return true;
    }

    let activity = Activity::new()
    .state("Server cheap")
        .assets(|assets| {
            assets
            .large_image("ms-icon-310x310")
            .large_text("Making Minecraft ")
            .small_image("ms-icon-310x310")
            .small_text("made easy")
        })
        .timestamps(|ts| {
            let time = SystemTime::now();
            let since_the_epoch = time
                .duration_since(UNIX_EPOCH)
                .unwrap().as_secs();
            ts.start(since_the_epoch)
        });
    let mut client = state.client.lock().unwrap();
    let act = (*client).set_activity(|_| activity);
    return act.is_ok();
}
#[tauri::command(async)]
fn clear_presence(state: State<App>) -> bool {
    if cfg!(target_os = "windows") {
        let mut client_win = state.client_win.lock().unwrap();
        return (*client_win).reconnect().is_ok();
    }
    let mut client = state.client.lock().unwrap();
    return (*client).clear_activity().is_ok();
}