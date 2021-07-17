use std::collections::HashMap;
use std::error::Error;
use zbus::dbus_proxy;
use zvariant::Value;

#[dbus_proxy]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

pub fn send_notification(title: &str, body: &str) -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;

    let proxy = NotificationsProxy::new(&connection)?;
    proxy.notify(
        "calendar-notification",
        1337,
        "",
        title,
        body,
        &[],
        HashMap::new(),
        0,
    )?;
    Ok(())
}
