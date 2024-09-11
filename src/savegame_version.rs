use num_enum::IntoPrimitive;

/// Save Game File Version from FSaveGameFileVersion::Type
#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum SaveGameVersion {
    /// Initial version.
    InitialVersion = 1,
    /// serializing custom versions into the savegame data to handle that type of versioning
    AddedCustomVersions = 2,
    /// added a new UE5 version number to FPackageFileSummary
    PackageFileSummaryVersionChange = 3,
}
