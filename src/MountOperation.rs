//! # DBus interface proxy for: `org.gtk.vfs.MountOperation`
//!
//! This code was generated by `zbus-xmlgen` `4.0.0` from DBus introspection data.
//! Source: `org.gtk.vfs.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus2.github.io/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.gtk.vfs.MountOperation", assume_defaults = true)]
trait MountOperation {
    /// Aborted method
    fn aborted(&self) -> zbus::Result<()>;

    /// AskPassword method
    #[allow(clippy::too_many_arguments)]
    fn ask_password(
        &self,
        message_string: &str,
        default_user: &str,
        default_domain: &str,
        flags_as_int: u32,
    ) -> zbus::Result<(bool, bool, String, String, String, bool, u32)>;

    /// AskQuestion method
    fn ask_question(
        &self,
        message_string: &str,
        choices: &[&str],
    ) -> zbus::Result<(bool, bool, u32)>;

    /// ShowProcesses method
    fn show_processes(
        &self,
        message_string: &str,
        choices: &[&str],
        processes: &[i32],
    ) -> zbus::Result<(bool, bool, u32)>;

    /// ShowUnmountProgress method
    fn show_unmount_progress(
        &self,
        message_string: &str,
        time_left: i64,
        bytes_left: i64,
    ) -> zbus::Result<()>;
}
