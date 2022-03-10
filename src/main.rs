use hotkey;
use pulsectl::controllers::SinkController;
use pulsectl::controllers::AppControl;
use x11_dl::keysym;
use substring::Substring;

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
        || handle_vol_up()
    ).unwrap();

    hotkeylistener.register_hotkey(
        0,
        CKEY,
        //hotkey::keys::ARROW_DOWN,
        || handle_vol_down()
    ).unwrap();

    hotkeylistener.register_hotkey(
        hotkey::modifiers::CONTROL,
        CCKEY,
        || handle_ctrl_vol_up()
    ).unwrap();

    hotkeylistener.register_hotkey(
        hotkey::modifiers::CONTROL,
        CKEY,
        || handle_ctrl_vol_down()
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
            if (currentvol + (vol*100.0) as i32 > 100) {
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

fn handle_vol_up() {
    mut_app_vol("Spotify", DELTAVOL);
}

fn handle_vol_down() {
    mut_app_vol("Spotify", DELTAVOL * -1.0);
}

fn handle_ctrl_vol_up() {

}

fn handle_ctrl_vol_down() {

}

fn main() {    
    register_hotkeys();
}
