#[cfg(target_os = "linux")]
use crate::ipc::start_pa;
use crate::ui_cm_interface::{start_ipc, ConnectionManager, InvokeUiCM};

#[cfg(windows)]
use clipboard::{
    create_cliprdr_context, empty_clipboard, get_rx_clip_client, server_clip_file, set_conn_enabled,
};

use hbb_common::{allow_err, log};
use sciter::{make_args, Element, Value, HELEMENT};
use std::sync::Mutex;
use std::{ops::Deref, sync::Arc};

#[derive(Clone, Default)]
pub struct SciterHandler {
    pub element: Arc<Mutex<Option<Element>>>,
}

impl InvokeUiCM for SciterHandler {
    fn add_connection(&self, client: &crate::ui_cm_interface::Client) {
        self.call(
            "addConnection",
            &make_args!(
                client.id,
                client.is_file_transfer,
                client.port_forward.clone(),
                client.peer_id.clone(),
                client.name.clone(),
                client.authorized,
                client.keyboard,
                client.clipboard,
                client.audio,
                client.file,
                client.restart,
                client.recording
            ),
        );
    }

    fn remove_connection(&self, id: i32) {
        self.call("removeConnection", &make_args!(id));
        if crate::ui_cm_interface::get_clients_length().eq(&0) {
            crate::platform::quit_gui();
        }
    }

    fn new_message(&self, id: i32, text: String) {
        self.call("newMessage", &make_args!(id, text));
    }

    fn change_theme(&self, _dark: String) {
        // TODO
    }

    fn change_language(&self) {
        // TODO
    }
}

impl SciterHandler {
    #[inline]
    fn call(&self, func: &str, args: &[Value]) {
        if let Some(e) = self.element.lock().unwrap().as_ref() {
            allow_err!(e.call_method(func, &super::value_crash_workaround(args)[..]));
        }
    }
}

pub struct SciterConnectionManager(ConnectionManager<SciterHandler>);

impl Deref for SciterConnectionManager {
    type Target = ConnectionManager<SciterHandler>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SciterConnectionManager {
    pub fn new() -> Self {
        #[cfg(target_os = "linux")]
        std::thread::spawn(start_pa);
        let cm = ConnectionManager {
            ui_handler: SciterHandler::default(),
        };
        let cloned = cm.clone();
        std::thread::spawn(move || start_ipc(cloned));
        SciterConnectionManager(cm)
    }

    fn get_icon(&mut self) -> String {
        crate::get_icon()
    }

    fn check_click_time(&mut self, id: i32) {
        crate::ui_cm_interface::check_click_time(id);
    }

    fn get_click_time(&self) -> f64 {
        crate::ui_cm_interface::get_click_time() as _
    }

    fn switch_permission(&self, id: i32, name: String, enabled: bool) {
        crate::ui_cm_interface::switch_permission(id, name, enabled);
    }

    fn close(&self, id: i32) {
        crate::ui_cm_interface::close(id);
    }

    fn authorize(&self, id: i32) {
        crate::ui_cm_interface::authorize(id);
    }

    fn send_msg(&self, id: i32, text: String) {
        crate::ui_cm_interface::send_chat(id, text);
    }

    fn t(&self, name: String) -> String {
        crate::client::translate(name)
    }
}

impl sciter::EventHandler for SciterConnectionManager {
    fn attached(&mut self, root: HELEMENT) {
        *self.ui_handler.element.lock().unwrap() = Some(Element::from(root));
    }

    sciter::dispatch_script_call! {
        fn t(String);
        fn check_click_time(i32);
        fn get_click_time();
        fn get_icon();
        fn close(i32);
        fn authorize(i32);
        fn switch_permission(i32, String, bool);
        fn send_msg(i32, String);
    }
}
