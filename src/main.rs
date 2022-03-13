use hotkey;
use pulsectl::controllers::SinkController;
use pulsectl::controllers::AppControl;
use x11_dl::keysym;
use substring::Substring;
use mpris::PlayerFinder;

// Customization options
static DELTAVOL: f64 = 0.02;
static CKEY: u32 = keysym::XK_Pause;
static CCKEY: u32 = hotkey::keys::INSERT;

pub fn register_hotkeys() {
    let mut hotkeylistener: hotkey::Listener = hotkey::Listener::new();

    hotkeylistener.register_hotkey(
        0,
        CCKEY,
        //hotkey::keys::ARROW_UP,
        || handle_vol(true)
    ).unwrap();

    hotkeylistener.register_hotkey(
        0,
        CKEY,
        //hotkey::keys::ARROW_DOWN,
        || handle_vol(false)
    ).unwrap();

    hotkeylistener.register_hotkey(
        hotkey::modifiers::CONTROL,
        CCKEY,
        || handle_ctrl_vol(true)
    ).unwrap();

    hotkeylistener.register_hotkey(
        hotkey::modifiers::CONTROL,
        CKEY,
        || handle_ctrl_vol(false)
    ).unwrap();

    hotkeylistener.listen();
}

pub fn mut_app_vol(app_name: &str, mut vol: f64) {
    let mut audiohandler = SinkController::create().unwrap();
    let apps = audiohandler.list_applications()
    .expect("Failed to list applications");

    for app in apps.clone() {
        if app.name.as_ref().unwrap() == app_name {

            // Clip max vol to 100%
            let currentvol = app.volume.print().substring(3,6).replace(" ", "").parse::<i32>().unwrap();
            if currentvol + (vol*100.0) as i32 > 100 {
                vol = (currentvol - 100) as f64 / 100.0;
            }
            
            if vol > 0.0 {
                audiohandler.increase_app_volume_by_percent(app.index, vol);
            } else {
                audiohandler.decrease_app_volume_by_percent(app.index, vol * -1.0);
            }
        }
    }
}

fn find_media_name() -> String {
    let player = PlayerFinder::new().unwrap().find_active().unwrap();
    let identity = player.identity().to_owned();

    if identity == "Firefox" {
        let metadata = player.get_metadata();
        if metadata.as_ref().unwrap().url().unwrap().contains("youtube.com") {
            let title = metadata.unwrap().title().unwrap().to_string().clone();
            return title + " - YouTube";
        }

        return String::new();
    }

    return identity;
}

fn handle_vol(up: bool) {
    let app = find_media_name();
    mut_app_vol(&app, DELTAVOL * (if up {1.0} else {-1.0}));
}

fn handle_ctrl_vol(up: bool) {
    mut_app_vol("playStream", DELTAVOL * (if up {1.0} else {-1.0}));
}

fn main() {    
    register_hotkeys();
}
