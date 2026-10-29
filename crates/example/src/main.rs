use noobwerkz_client::app::run;
use noobwerkz_client::callbacks::*;

use noobwerkz_example::gui::*;
use noobwerkz_example::setup::*;
use noobwerkz_example::update::*;
use noobwerkz_example::user_data::*;

fn main() {
    { // Unsure if necessary. Test and get rid of the braces.
        let mut data_guard = USER_DATA
            .lock()
            .expect("User data mutex poisoned at startup.");
        let success = data_guard.init();

        if !success {
            println!("[Could not initialize user data structures. Exiting.");
            return;
        }
    }

    init_user_setup_callback(user_setup);
    init_user_update_callback(user_update);
    init_user_gui_callback(user_gui);
    run().unwrap();
}
