use hotkey;
use pulsectl::controllers::SinkController;
use pulsectl::controllers::AppControl;
use x11_dl::keysym;

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

pub fn mut_app_vol(app_name: &str, vol: f64) {
    let mut audiohandler = SinkController::create().unwrap();
    let apps = audiohandler.list_applications()
    .expect("Failed to list applications");

    for app in apps.clone() {
        if app.name.as_ref().unwrap() == app_name {
            if vol > 0.0 {
                audiohandler.increase_app_volume_by_percent(app.index, vol);
            } else {
                audiohandler.decrease_app_volume_by_percent(app.index, vol * -1.0);
            } //TODO: fix infinite volume glitch real 2022 working 100% free virus free
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
