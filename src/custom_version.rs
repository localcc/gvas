//! Custom version information

use crate::cursor_ext::{ReadExt, WriteExt};
use crate::engine_version::EngineVersion;
use crate::error::Error;
use crate::types::Guid;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_enum::IntoPrimitive;
use std::io::{Read, Seek, Write};

/// Stores CustomVersions serialized by UE4
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FCustomVersion {
    /// Key
    pub key: Guid,
    /// Value
    pub version: u32,
}

impl FCustomVersion {
    /// Creates a new instance of `FCustomVersion`
    #[inline]
    pub fn new(key: Guid, version: u32) -> Self {
        FCustomVersion { key, version }
    }

    /// Read FCustomVersion from a binary file
    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let key = cursor.read_guid()?;
        let version = cursor.read_u32::<LittleEndian>()?;

        Ok(FCustomVersion { key, version })
    }

    /// Write FCustomVersion to a binary file
    pub(crate) fn write<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_guid(&self.key)?;
        cursor.write_u32::<LittleEndian>(self.version)?;
        Ok(())
    }
}

/// Used for predefining custom versions for nicer checking when parsing
pub trait CustomVersionTrait {
    /// Mappings from engine version to version number of this custom version
    ///
    /// # Example
    /// UE4_27 -> 13
    /// UE4_23 -> 12
    const VERSION_MAPPINGS: &'static [(EngineVersion, i32)];
    /// Custom version friendly name
    const FRIENDLY_NAME: &'static str;
    /// Custom version guid
    const GUID: Guid;
}

macro_rules! impl_custom_version_trait {
    ($enum_name:ident, $friendly_name:expr, $guid:expr, $($ver_name:ident : $ver:ident),*) => {
        impl CustomVersionTrait for $enum_name {
            const VERSION_MAPPINGS: &'static [(EngineVersion, i32)] = &[
                $(
                    (EngineVersion::$ver_name, Self::$ver as i32),
                )*
            ];
            const FRIENDLY_NAME: &'static str = $friendly_name;
            const GUID: Guid = $guid;
        }
    }
}

/// Custom serialization version for changes made in Dev-Editor stream.
#[derive(IntoPrimitive)]
#[repr(i32)]
pub enum FEditorObjectVersion {
    /// Before any version changes were made
    /// Introduced: ObjectVersion.VER_UE4_OLDEST_LOADABLE_PACKAGE
    BeforeCustomVersionWasAdded = 0,

    /// Localizable text gathered and stored in packages is now flagged with a localizable text gathering process version
    /// Introduced: ObjectVersion.VER_UE4_STREAMABLE_TEXTURE_AABB
    GatheredTextProcessVersionFlagging,

    /// Fixed several issues with the gathered text cache stored in package headers
    /// Introduced: ObjectVersion.VER_UE4_NAME_HASHES_SERIALIZED
    GatheredTextPackageCacheFixesV1,

    /// Added support for "root" meta-data (meta-data not associated with a particular object in a package)
    /// Introduced: ObjectVersion.VER_UE4_INSTANCED_STEREO_UNIFORM_REFACTOR
    RootMetaDataSupport,

    /// Fixed issues with how Blueprint bytecode was cached
    /// Introduced: ObjectVersion.VER_UE4_INSTANCED_STEREO_UNIFORM_REFACTOR
    GatheredTextPackageCacheFixesV2,

    /// Updated FFormatArgumentData to allow variant data to be marshaled from a BP into C++
    /// Introduced: ObjectVersion.VER_UE4_INSTANCED_STEREO_UNIFORM_REFACTOR
    TextFormatArgumentDataIsVariant,

    /// Changes to SplineComponent
    /// Introduced: ObjectVersion.VER_UE4_INSTANCED_STEREO_UNIFORM_REFACTOR
    SplineComponentCurvesInStruct,

    /// Updated ComboBox to support toggling the menu open, better controller support
    /// Introduced: ObjectVersion.VER_UE4_COMPRESSED_SHADER_RESOURCES
    ComboBoxControllerSupportUpdate,

    /// Refactor mesh editor materials
    /// Introduced: ObjectVersion.VER_UE4_COMPRESSED_SHADER_RESOURCES
    RefactorMeshEditorMaterials,

    /// Added UFontFace assets
    /// Introduced: ObjectVersion.VER_UE4_TemplateIndex_IN_COOKED_EXPORTS
    AddedFontFaceAssets,

    /// Add UPROPERTY for TMap of Mesh section, so the serialize will be done normally (and export to text will work correctly)
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SEARCHABLE_NAMES
    UPropertryForMeshSection,

    /// Update the schema of all widget blueprints to use the WidgetGraphSchema
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SEARCHABLE_NAMES
    WidgetGraphSchema,

    /// Added a specialized content slot to the background blur widget
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SEARCHABLE_NAMES
    AddedBackgroundBlurContentSlot,

    /// Updated UserDefinedEnums to have stable keyed display names
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SEARCHABLE_NAMES
    StableUserDefinedEnumDisplayNames,

    /// Added "Inline" option to UFontFace assets
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SEARCHABLE_NAMES
    AddedInlineFontFaceAssets,

    /// Fix a serialization issue with static mesh FMeshSectionInfoMap FProperty
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SEARCHABLE_NAMES
    UPropertryForMeshSectionSerialize,

    /// Adding a version bump for the new fast widget construction in case of problems.
    /// Introduced: ObjectVersion.VER_UE4_64BIT_EXPORTMAP_SERIALSIZES
    FastWidgetTemplates,

    /// Update material thumbnails to be more intelligent on default primitive shape for certain material types
    /// Introduced: ObjectVersion.VER_UE4_64BIT_EXPORTMAP_SERIALSIZES
    MaterialThumbnailRenderingChanges,

    /// Introducing a new clipping system for Slate/UMG
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SWEEP_WHILE_WALKING_FLAG
    NewSlateClippingSystem,

    /// MovieScene Meta Data added as native Serialization
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SWEEP_WHILE_WALKING_FLAG
    MovieSceneMetaDataSerialization,

    /// Text gathered from properties now adds two variants: a version without the package localization ID (for use at runtime), and a version with it (which is editor-only)
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SWEEP_WHILE_WALKING_FLAG
    GatheredTextEditorOnlyPackageLocId,

    /// Added AlwaysSign to FNumberFormattingOptions
    /// Introduced: ObjectVersion.VER_UE4_ADDED_SOFT_OBJECT_PATH
    AddedAlwaysSignNumberFormattingOption,

    /// Added additional objects that must be serialized as part of this new material feature
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID
    AddedMaterialSharedInputs,

    /// Added morph target section indices
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID
    AddedMorphTargetSectionIndices,

    /// Serialize the instanced static mesh render data, to avoid building it at runtime
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID
    SerializeInstancedStaticMeshRenderData,

    /// Change to MeshDescription serialization (moved to release)
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID
    MeshDescriptionNewSerializationMovedToRelease,

    /// New format for mesh description attributes
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_SUMMARY_LOCALIZATION_ID
    MeshDescriptionNewAttributeFormat,

    /// Switch root component of SceneCapture actors from MeshComponent to SceneComponent
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    ChangeSceneCaptureRootComponent,

    /// StaticMesh serializes MeshDescription instead of RawMesh
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    StaticMeshDeprecatedRawMesh,

    /// MeshDescriptionBulkData contains a Guid used as a DDC key
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    MeshDescriptionBulkDataGuid,

    /// Change to MeshDescription serialization (removed FMeshPolygon::HoleContours)
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    MeshDescriptionRemovedHoles,

    /// Change to the WidgetCompoent WindowVisibilty default value
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    ChangedWidgetComponentWindowVisibilityDefault,

    /// Avoid keying culture invariant display strings during serialization to avoid non-deterministic cooking issues
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    CultureInvariantTextSerializationKeyStability,

    /// Change to UScrollBar and UScrollBox thickness property (removed implicit padding of 2, so thickness value must be incremented by 4).
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    ScrollBarThicknessChange,

    /// Deprecated LandscapeHoleMaterial
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    RemoveLandscapeHoleMaterial,

    /// MeshDescription defined by triangles instead of arbitrary polygons
    /// Introduced: ObjectVersion.VER_UE4_FIX_WIDE_STRING_CRC
    MeshDescriptionTriangles,

    /// Add weighted area and angle when computing the normals
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_OWNER
    ComputeWeightedNormals,

    /// SkeletalMesh now can be rebuild in editor, no more need to re-import
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_OWNER
    SkeletalMeshBuildRefactor,

    /// Move all SkeletalMesh source data into a private uasset in the same package has the skeletalmesh
    /// Introduced: ObjectVersion.VER_UE4_ADDED_PACKAGE_OWNER
    SkeletalMeshMoveEditorSourceDataToPrivateAsset,

    /// Parse text only if the number is inside the limits of its type
    /// Introduced: ObjectVersion.VER_UE4_NON_OUTER_PACKAGE_IMPORT
    NumberParsingOptionsNumberLimitsAndClamping,

    /// Make sure we can have more then 255 material in the skeletal mesh source data
    /// Introduced: ObjectVersion.VER_UE4_NON_OUTER_PACKAGE_IMPORT
    SkeletalMeshSourceDataSupport16bitOfMaterialNumber,

    /// Introduced: ObjectVersion.VER_UE4_AUTOMATIC_VERSION_PLUS_ONE
    VersionPlusOne,
    /// Introduced: ObjectVersion.VER_UE4_AUTOMATIC_VERSION
    LatestVersion = (FEditorObjectVersion::VersionPlusOne as i32) + 1,
}

impl_custom_version_trait!(
    FEditorObjectVersion,
    "FEditorObjectVersion",
    Guid::from_ints((0xE4B068ED, 0xF49442E9, 0xA231DA0B, 0x2E46BB41)),
    VER_UE4_AUTOMATIC_VERSION: LatestVersion,
    VER_UE4_AUTOMATIC_VERSION_PLUS_ONE: VersionPlusOne,
    VER_UE4_26: SkeletalMeshSourceDataSupport16bitOfMaterialNumber,
    VER_UE4_25: SkeletalMeshMoveEditorSourceDataToPrivateAsset,
    VER_UE4_24: SkeletalMeshBuildRefactor,
    VER_UE4_23: RemoveLandscapeHoleMaterial,
    VER_UE4_22: MeshDescriptionRemovedHoles,
    VER_UE4_21: MeshDescriptionNewAttributeFormat,
    VER_UE4_20: SerializeInstancedStaticMeshRenderData,
    VER_UE4_19: AddedMorphTargetSectionIndices,
    VER_UE4_17: GatheredTextEditorOnlyPackageLocId,
    VER_UE4_16: MaterialThumbnailRenderingChanges,
    VER_UE4_15: AddedInlineFontFaceAssets,
    VER_UE4_14: AddedFontFaceAssets,
    VER_UE4_13: SplineComponentCurvesInStruct,
    VER_UE4_12: GatheredTextPackageCacheFixesV1,
    VER_UE4_OLDEST_LOADABLE_PACKAGE: BeforeCustomVersionWasAdded
);
