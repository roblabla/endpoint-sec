//! [`EventIoKitOpen`]

use std::ffi::OsStr;

use endpoint_sec_sys::es_event_iokit_open_t;

/// Open a connection to an I/O Kit IOService event.
#[doc(alias = "es_event_iokit_open_t")]
pub struct EventIoKitOpen<'a> {
    /// Raw event
    pub(crate) raw: &'a es_event_iokit_open_t,

    /// The version of the message.
    pub(crate) version: u32,
}

impl<'a> EventIoKitOpen<'a> {
    /// A constant specifying the type of connection to be created, interpreted only by the IOService's family.
    ///
    /// **Note**: This corresponds to the type argument to `IOServiceOpen()`.
    #[inline(always)]
    pub fn user_client_type(&self) -> u32 {
        self.raw.user_client_type
    }

    /// The name of the new object linked to the source file.
    #[inline(always)]
    pub fn user_client_class(&self) -> &'a OsStr {
        // Safety: 'a tied to self, object obtained through ES
        unsafe { self.raw.user_client_class.as_os_str() }
    }

    /// The IOKit registry ID of the parent of the user class. Conceptually this
    /// is what the user class is connecting to. It can be resolved to a an
    /// `io_service_t` with by calling `IORegistryEntryIDMatching` then
    /// `IOServiceGetMatchingService`
    ///
    /// Field available only if message version >= 10.
    #[cfg(feature = "macos_26_0_0")]
    pub fn parent_registry_id(&self) -> Option<u64> {
        if self.version >= 10 {
            Some(self.raw.parent_registry_id)
        } else {
            None
        }
    }

    /// The path in the IOKit device tree to the class being opened. It can be
    /// resolved to an `io_registry_entry_t` by calling `IORegistryEntryFromPath`.
    ///
    /// Field available only if message version >= 10.
    #[cfg(feature = "macos_26_0_0")]
    pub fn parent_path(&self) -> Option<&'a OsStr> {
        if self.version >= 10 {
            // Safety: 'a tied to self, object obtained through ES
            let s = unsafe { self.raw.parent_path.as_os_str() };
            Some(s)
        } else {
            None
        }
    }
}

// Safety: safe to send across threads: does not contain any interior mutability nor depend on current thread state
unsafe impl Send for EventIoKitOpen<'_> {}
// Safety: safe to share across threads: does not contain any interior mutability nor depend on current thread state
unsafe impl Sync for EventIoKitOpen<'_> {}

impl_debug_eq_hash_with_functions!(EventIoKitOpen<'a>;
    user_client_type,
    user_client_class,
    #[cfg(feature = "macos_26_0_0")] parent_registry_id,
    #[cfg(feature = "macos_26_0_0")] parent_path,
);
