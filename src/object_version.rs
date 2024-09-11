use num_enum::IntoPrimitive;

/// UE5 object versions.
#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum EUnrealEngineObjectUE5Version {
    /// The original UE5 version, at the time this was added the UE4 version was 522, so UE5 will start from 1000 to show a clear difference
    InitialVersion = 1000,

    /// Support stripping names that are not referenced from export data
    NamesReferencedFromExportData,

    /// Added a payload table of contents to the package summary
    PayloadToc,

    /// Added data to identify references from and to optional package
    OptionalResources,

    /// Large world coordinates converts a number of core types to double components by default.
    LargeWorldCoordinates,

    /// Remove package GUID from FObjectExport
    RemoveObjectExportPackageGuid,

    /// Add IsInherited to the FObjectExport entry
    TrackObjectExportIsInherited,

    /// Replace FName asset path in FSoftObjectPath with (package name, asset name) pair FTopLevelAssetPath
    FsoftobjectpathRemoveAssetPathFnames,

    /// Add a soft object path list to the package summary for fast remap
    AddSoftobjectpathList,

    /// Added bulk/data resource table
    DataResources,

    /// Added script property serialization offset to export table entries for saved, versioned packages
    ScriptSerializationOffset,

    /// Adding property tag extension,
    /// Support for overridable serialization on UObject,
    /// Support for overridable logic in containers
    PropertyTagExtensionAndOverridableSerialization,

    /// Added property tag complete type name and serialization type
    PropertyTagCompleteTypeName,
}
