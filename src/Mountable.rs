//! # DBus interface proxy for: `org.gtk.vfs.Mountable`
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

#[dbus_proxy(interface = "org.gtk.vfs.Mountable", assume_defaults = true)]
trait Mountable {
    /// Mount method
    fn mount(
        &self,
        mount_spec: &(
            &[u8],
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ),
        automount: bool,
        mount_source: &(&str, zbus::zvariant::ObjectPath<'_>),
    ) -> zbus::Result<()>;
}
