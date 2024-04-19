//! # DBus interface proxy for: `org.gtk.vfs.Mount`
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

#[dbus_proxy(interface = "org.gtk.vfs.Mount", assume_defaults = true)]
trait Mount {
    /// Copy method
    fn copy(
        &self,
        path1_data: &[u8],
        path2_data: &[u8],
        flags: u32,
        progress_obj_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// CreateDirectoryMonitor method
    fn create_directory_monitor(&self, path_data: &[u8], flags: u32) -> zbus::Result<String>;

    /// CreateFileMonitor method
    fn create_file_monitor(&self, path_data: &[u8], flags: u32) -> zbus::Result<String>;

    /// Delete method
    fn delete(&self, path_data: &[u8]) -> zbus::Result<()>;

    /// EjectMountable method
    fn eject_mountable(
        &self,
        path_data: &[u8],
        flags: u32,
        dbus_id: &str,
        obj_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// Enumerate method
    fn enumerate(
        &self,
        path_data: &[u8],
        obj_path: &str,
        attributes: &str,
        flags: u32,
        uri: &str,
    ) -> zbus::Result<()>;

    /// MakeDirectory method
    fn make_directory(&self, path_data: &[u8]) -> zbus::Result<()>;

    /// MakeSymbolicLink method
    fn make_symbolic_link(&self, path_data: &[u8], symlink_value: &[u8]) -> zbus::Result<()>;

    /// MountMountable method
    #[allow(clippy::too_many_arguments)]
    fn mount_mountable(
        &self,
        path_data: &[u8],
        dbus_id: &str,
        obj_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<(
        bool,
        Vec<u8>,
        bool,
        (
            Vec<u8>,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        ),
    )>;

    /// Move method
    fn move_(
        &self,
        path1_data: &[u8],
        path2_data: &[u8],
        flags: u32,
        progress_obj_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// OpenForRead method
    fn open_for_read(
        &self,
        path_data: &[u8],
        pid: u32,
    ) -> zbus::Result<(zbus::zvariant::OwnedFd, bool)>;

    /// OpenForWrite method
    #[allow(clippy::too_many_arguments)]
    fn open_for_write(
        &self,
        path_data: &[u8],
        mode: u16,
        etag: &str,
        make_backup: bool,
        flags: u32,
        pid: u32,
    ) -> zbus::Result<(zbus::zvariant::OwnedFd, bool, u64)>;

    /// OpenForWriteFlags method
    #[allow(clippy::too_many_arguments)]
    fn open_for_write_flags(
        &self,
        path_data: &[u8],
        mode: u16,
        etag: &str,
        make_backup: bool,
        flags: u32,
        pid: u32,
    ) -> zbus::Result<(zbus::zvariant::OwnedFd, u32, u64)>;

    /// OpenIconForRead method
    fn open_icon_for_read(&self, path_data: &[u8])
        -> zbus::Result<(zbus::zvariant::OwnedFd, bool)>;

    /// PollMountable method
    fn poll_mountable(&self, path_data: &[u8]) -> zbus::Result<()>;

    /// Pull method
    fn pull(
        &self,
        path_data: &[u8],
        local_path: &[u8],
        send_progress: bool,
        flags: u32,
        progress_obj_path: &zbus::zvariant::ObjectPath<'_>,
        remove_source: bool,
    ) -> zbus::Result<()>;

    /// Push method
    fn push(
        &self,
        path_data: &[u8],
        local_path: &[u8],
        send_progress: bool,
        flags: u32,
        progress_obj_path: &zbus::zvariant::ObjectPath<'_>,
        remove_source: bool,
    ) -> zbus::Result<()>;

    /// QueryFilesystemInfo method
    fn query_filesystem_info(
        &self,
        path_data: &[u8],
        attributes: &str,
    ) -> zbus::Result<Vec<(String, u32, zbus::zvariant::OwnedValue)>>;

    /// QueryInfo method
    fn query_info(
        &self,
        path_data: &[u8],
        attributes: &str,
        flags: u32,
        uri: &str,
    ) -> zbus::Result<Vec<(String, u32, zbus::zvariant::OwnedValue)>>;

    /// QuerySettableAttributes method
    fn query_settable_attributes(&self, path_data: &[u8]) -> zbus::Result<Vec<(String, u32, u32)>>;

    /// QueryWritableNamespaces method
    fn query_writable_namespaces(&self, path_data: &[u8]) -> zbus::Result<Vec<(String, u32, u32)>>;

    /// SetAttribute method
    fn set_attribute(
        &self,
        path_data: &[u8],
        flags: u32,
        attribute: &(&str, u32, zbus::zvariant::Value<'_>),
    ) -> zbus::Result<()>;

    /// SetDisplayName method
    fn set_display_name(&self, path_data: &[u8], display_name: &str) -> zbus::Result<Vec<u8>>;

    /// StartMountable method
    fn start_mountable(
        &self,
        path_data: &[u8],
        dbus_id: &str,
        obj_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// StopMountable method
    fn stop_mountable(
        &self,
        path_data: &[u8],
        flags: u32,
        dbus_id: &str,
        obj_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// Trash method
    fn trash(&self, path_data: &[u8]) -> zbus::Result<()>;

    /// Unmount method
    fn unmount(
        &self,
        dbus_id: &str,
        obj_path: &zbus::zvariant::ObjectPath<'_>,
        flags: u32,
    ) -> zbus::Result<()>;

    /// UnmountMountable method
    fn unmount_mountable(
        &self,
        path_data: &[u8],
        flags: u32,
        dbus_id: &str,
        obj_path: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;
}
