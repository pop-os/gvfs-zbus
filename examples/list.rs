use std::ffi::CStr;

fn main() {
    let conn = zbus::blocking::Connection::session().unwrap();
    let proxy = gvfs_zbus::MountTracker::MountTrackerProxyBlocking::builder(&conn)
        .destination("org.gtk.vfs.Daemon")
        .unwrap()
        .path("/org/gtk/vfs/mounttracker")
        .unwrap()
        .build()
        .unwrap();
    let mounts = proxy.list_mounts().unwrap();
    for mount in mounts {
        //println!("{:X?}", mount);

        let (
            dbus_id,
            obj_path,
            display_name,
            stable_name,
            x_content_types,
            icon_str,
            symbolic_icon_str,
            prefered_filename_encoding,
            user_visible,
            fuse_mountpoint,
            iter_mount_spec,
            default_location,
        ) = mount;

        if !user_visible {
            //TODO: flag to toggle this
            continue;
        }

        println!("{}:{}", dbus_id, obj_path);
        println!("  display_name {display_name}");
        println!("  stable_name {stable_name}");
        println!("  user_visible {user_visible}");
        println!(
            "  fuse_mountpoint {:?}",
            CStr::from_bytes_until_nul(&fuse_mountpoint)
        );
        println!(
            "  iter_mount_spec {:?}",
            CStr::from_bytes_until_nul(&iter_mount_spec.0)
        );
        for (string, value) in iter_mount_spec.1.iter() {
            let array: &zbus::zvariant::Array = value.try_into().unwrap();
            let mut bytes = Vec::with_capacity(array.len());
            for value in array.iter() {
                let byte: &u8 = value.try_into().unwrap();
                bytes.push(*byte);
            }
            println!("    {}: {:?}", string, CStr::from_bytes_until_nul(&bytes));
        }
        println!(
            "  default_location {:?}",
            CStr::from_bytes_until_nul(&default_location)
        );
    }
}
