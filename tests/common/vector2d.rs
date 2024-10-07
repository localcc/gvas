use gvas::{
    engine_version::FEngineVersion,
    game_version::DeserializedGameVersion,
    properties::{
        delegate_property::{Delegate, MulticastInlineDelegateProperty, MulticastScriptDelegate},
        int_property::{BoolProperty, FloatProperty, IntProperty},
        str_property::StrProperty,
        struct_property::{StructProperty, StructPropertyValue},
        struct_types::Vector2D,
        Property,
    },
    types::{map::HashableIndexMap, Guid},
    GvasFile, GvasHeader,
};
use ordered_float::OrderedFloat;
use std::str::FromStr;

const DELEGATE_PREFIX: &str = "/Game/DefaultMap.DefaultMap:PersistentLevel.";

pub(crate) fn expected() -> GvasFile {
    GvasFile {
        deserialized_game_version: DeserializedGameVersion::Default,
        header: GvasHeader::Version3 {
            package_file_version: 522,
            package_file_version_ue5: 1009,
            engine_version: FEngineVersion {
                major: 5,
                minor: 3,
                patch: 2,
                change_list: 29314046,
                branch: String::from("++UE5+Release-5.3"),
            },
            custom_version_format: 3,
            custom_versions: HashableIndexMap::from([
                (
                    Guid::from_str("22D5549C-BE4F-26A8-4607-2194D082B461").unwrap(),
                    44,
                ),
                (
                    Guid::from_str("A35C9162-F74B-8E1C-C712-0EA3F79D21C8").unwrap(),
                    32,
                ),
                (
                    Guid::from_str("240D40CC-7B4E-E9E0-83A2-F99B27C0C0DC").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("E432D8B0-0D4F-891F-B77E-CFACA24AFD36").unwrap(),
                    10,
                ),
                (
                    Guid::from_str("2843C6E1-534D-2CA2-868E-6CA38CBD1764").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("3CC15E37-FB48-E406-F084-00B57E712A26").unwrap(),
                    4,
                ),
                (
                    Guid::from_str("ED68B0E4-E942-94F4-0BDA-31A241BB462E").unwrap(),
                    40,
                ),
                (
                    Guid::from_str("3F74FCCF-8044-B043-DF14-919373201D17").unwrap(),
                    37,
                ),
                (
                    Guid::from_str("B5492BB0-E944-20BB-B732-04A36003E452").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("5C10E4A4-B549-A159-C440-C5A7EEDF7E54").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("C931C839-DC47-E65A-179C-449A7C8E1C3E").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("331BF078-984F-EAEB-EA84-B4B9A25AB9CC").unwrap(),
                    20,
                ),
                (
                    Guid::from_str("0F383166-E043-4D2D-27CF-09805AA95669").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("9F8BF812-FC4A-7588-0CD9-7CA629BD3A38").unwrap(),
                    47,
                ),
                (
                    Guid::from_str("4CE75A7B-104C-70D2-9857-58A95A2A210B").unwrap(),
                    13,
                ),
                (
                    Guid::from_str("186929D7-DD4B-D61D-A864-E29D8438C13C").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("7852A1C2-FE4A-E7BF-FF90-176C55F71D53").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("DD75E529-2746-A3E0-76D2-109DEADC2C23").unwrap(),
                    17,
                ),
                (
                    Guid::from_str("5DA643AF-4749-D37F-8E3E-739805BBC1D9").unwrap(),
                    15,
                ),
                (
                    Guid::from_str("EC6C266B-8F4B-C71E-D9E4-0BA307FC4209").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("613DF70D-EA47-3FA2-E989-27B79A49410C").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("86181D60-844F-64AC-DED3-16AAD6C7EA0D").unwrap(),
                    111,
                ),
                (
                    Guid::from_str("5B2CBC8D-E043-A754-BBFC-68A76090A27D").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("B7064C5B-F84A-6324-70BF-5B80DDD0F5CD").unwrap(),
                    10,
                ),
                (
                    Guid::from_str("686308E7-584C-236B-701B-3984915E2616").unwrap(),
                    11,
                ),
                (
                    Guid::from_str("D6BCFF9D-5801-4F49-8212-21E288A8923C").unwrap(),
                    10,
                ),
                (
                    Guid::from_str("ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0").unwrap(),
                    10,
                ),
                (
                    Guid::from_str("834AF935-6C40-58E2-F509-18A37C241096").unwrap(),
                    41,
                ),
                (
                    Guid::from_str("6EC18FB6-E242-1B8B-5C21-53B4FE448805").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("0685E1B2-C2CF-7342-BBF4-4EA507BA8B75").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("3689F564-BA42-1BFD-8972-96BA4EFAD0D5").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("81D57D69-AB41-4FE6-EC51-4AAA28B6B7BE").unwrap(),
                    118,
                ),
                (
                    Guid::from_str("425E9BD8-464D-BD24-A8AC-1284791764DF").unwrap(),
                    47,
                ),
                (
                    Guid::from_str("525DDA59-4849-3212-7859-78B88BE9B870").unwrap(),
                    8,
                ),
                (
                    Guid::from_str("325A0726-0847-0F73-328C-E988059D59F1").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("27D80E6F-9548-09A6-8D99-919CA40E1890").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("E38BD530-8242-EA95-59B1-E3A66AB0EBD8").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("E79E7F71-3A49-B0E9-3291-B3880781381B").unwrap(),
                    17,
                ),
                (
                    Guid::from_str("FC09C468-8649-9570-D2AC-6389835186C4").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("194D0C43-7049-5471-699B-6987E5B090DF").unwrap(),
                    15,
                ),
                (
                    Guid::from_str("BD32FEAA-144C-9553-255E-6AB6DDD13210").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("8EE1AF23-584E-E14C-52C2-618DB7BE53B9").unwrap(),
                    11,
                ),
                (
                    Guid::from_str("EAB762A4-3A4E-99F4-1FEC-C199B2E12482").unwrap(),
                    4,
                ),
                (
                    Guid::from_str("BDFDB52E-104D-AC01-8FF3-3681DAA59333").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("4F359D50-2F49-E6F6-B285-49A71C633C07").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("3EF0A495-E449-0B7E-56D3-43BAD987FF94").unwrap(),
                    7,
                ),
                (
                    Guid::from_str("1C1BE3B6-EC11-9FD2-859F-7E85E270996F").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("40EB564A-DC11-F510-7E34-D392E76AC9B2").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("8A991784-EC43-C0BB-19D1-B38122272D07").unwrap(),
                    19,
                ),
                (
                    Guid::from_str("004A8AD7-9746-58E8-B519-A8BAB4467D48").unwrap(),
                    18,
                ),
                (
                    Guid::from_str("86F87955-1F4C-3A93-7B08-BA832FB96163").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("52BE2F61-0B40-53DA-914F-0D917C85B19F").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("367A23A4-C941-EACA-F818-A28FF31B6858").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("753F4E80-494B-8870-068C-D6A4DCB67E3C").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("F448D01E-684C-2E2F-A453-D0892D108FF1").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("F20A68FB-A34B-EF59-B519-A8BA3D44C873").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("0EB75099-174E-1AB4-0DFA-CCBBD67F8157").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("CD14175E-5129-4E48-A789-7A7078AB0293").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("7B472509-0140-3D76-73D6-919D11B4750B").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("1B218842-C616-4845-B267-761A002A7A50").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("9B9549DC-E74D-C053-88EA-5691395D7C5E").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("FB0C82A7-5943-A720-142C-548C50CF2396").unwrap(),
                    27,
                ),
                (
                    Guid::from_str("4E7CE782-A543-2333-C513-6BB4F30D3197").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("AA1C1EE2-5E42-47AF-D46A-BF89BBA8444C").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("7E154A13-A349-E2D5-3C84-4E8D319EFE98").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("FA7AF5FC-8342-7650-58E6-A9B9322DA0FF").unwrap(),
                    79,
                ),
                (
                    Guid::from_str("ED0A3111-614D-552E-A39A-67AF2C08A1C5").unwrap(),
                    17,
                ),
                (
                    Guid::from_str("78BBDFF6-E4A0-50BB-4DB8-184023AFCB60").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("F37ABB24-834F-4656-C22D-2F1FFF96AD49").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("2923A576-B545-2309-41D8-AE98D86A2FCF").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("0769BC5F-AE40-C855-84F1-678E3FF1FF5E").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("438C7392-9C4D-8829-BE9B-3D9AC09FFF6E").unwrap(),
                    1,
                ),
            ]),
            save_game_class_name: String::from(
                "/Game/_Blueprints/BP_SettingsSave.BP_SettingsSave_C",
            ),
        },
        properties: HashableIndexMap::from([
            (
                String::from("SettingsChanged"),
                Property::from(MulticastInlineDelegateProperty {
                    value: MulticastScriptDelegate {
                        delegates: vec![
                            Delegate::new(
                                format!("{}BP_ActionTool_WaterGauge_C_2147482315", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plow_C_2147482312", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Plow_Row_Single_C_2147482309",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plow_Row_3_C_2147482305", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plow_5Row_C_2147482301", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plow_Row_5_C_2147482297", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plant_C_2147482293", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plant_Row_C_2147482286", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plant_Row3_C_2147482280", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Plant_Row5_C_2147482274", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Cultivate_C_2147482268", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Cultivate_Row_C_2147482265",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Cultivate_Row3_C_2147482261",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Cultivate_Row5_C_2147482257",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_PlasticRow_C_2147482253", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Purchase_C_2147482249", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Purchase_1x10_C_2147482242",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Purchase_3Row_C_2147482235",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Purchase_5Row_C_2147482228",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Purchase_10x10_C_2147482221",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Modify_C_2147482214", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Row_C_2147482198", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Row3_C_2147482181", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Harvest_C_2147482164", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Harvest_Row_C_2147482161",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Harvest_Row_3_C_2147482157",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Harvest_Row_5_C_2147482153",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_Harvest_Row_C_2147482149",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_AutomatedActionControl_C_2147482145",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_RemovePlaceable_C_2147482142",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_SeedSilo_C_2147482139", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_TractorBarn_C_2147482132",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Sell_C_2147482125", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_FuelStorageTank_C_2147482118",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_ChickenRun_C_2147482115", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_MovePlaceable_C_2147482112",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Beehive_C_2147482109", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_SetPHTool_Row_C_2147482106", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_BiodieselRefinery_C_2147482089",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_OilPress_C_2147482086", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_FlourMill_C_2147482083", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_LargeChickenCoop_C_2147482080",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_CropSign_C_2147482077", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Mulch_C_2147482070", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Mulch_Row_C_2147482054", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Mulch_Row3_C_2147482037", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Warehouse_C_2147482020", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_HarvestSilo_C_2147482013",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_ActionTool_Stockpile_C_2147482008", DELEGATE_PREFIX),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!(
                                    "{}BP_ActionTool_CompostStation_C_2147482001",
                                    DELEGATE_PREFIX
                                ),
                                String::from("SettingsChanged_Event"),
                            ),
                            Delegate::new(
                                format!("{}BP_Renders_C_1", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_PlayerPawn_C_2147482331", DELEGATE_PREFIX),
                                String::from("UpdatedSavedSettings"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478921", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478905", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478890", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478875", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478860", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478303", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478288", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478273", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478258", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478243", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478228", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478141", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478126", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478111", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147478096", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477750", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477735", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477720", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477705", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477690", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477675", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477660", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477645", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477189", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                            Delegate::new(
                                format!("{}BP_AutomatedTool_C_2147477162", DELEGATE_PREFIX),
                                String::from("SettingsChanged"),
                            ),
                        ],
                    },
                }),
            ),
            (
                String::from("AudioSettings"),
                Property::StructProperty(StructProperty {
                    guid: Guid::default(),
                    value: StructPropertyValue::CustomStruct {
                        type_name: String::from("GameAudioSettings"),
                        properties: HashableIndexMap::from([
                            (
                                String::from("MasterLevel"),
                                vec![Property::FloatProperty(FloatProperty {
                                    value: OrderedFloat::from(0.20348908),
                                })],
                            ),
                            (
                                String::from("MusicLevel"),
                                vec![Property::FloatProperty(FloatProperty {
                                    value: OrderedFloat::from(0.1511635),
                                })],
                            ),
                            (
                                String::from("SFXLevel"),
                                vec![Property::FloatProperty(FloatProperty {
                                    value: OrderedFloat::from(0.5436054),
                                })],
                            ),
                        ]),
                    },
                }),
            ),
            (
                String::from("GameSettings"),
                Property::StructProperty(StructProperty {
                    guid: Guid::default(),
                    value: StructPropertyValue::CustomStruct {
                        type_name: String::from("GameSettings"),
                        properties: HashableIndexMap::from([
                            (
                                String::from("CurrentSaveSlot"),
                                vec![Property::StrProperty(StrProperty::from("SAVE2"))],
                            ),
                            (
                                String::from("LoadTutorial"),
                                vec![Property::BoolProperty(BoolProperty::new(false))],
                            ),
                            (
                                String::from("DisplayNewOrders"),
                                vec![Property::BoolProperty(BoolProperty::new(false))],
                            ),
                            (
                                String::from("EscapeExitsTool"),
                                vec![Property::BoolProperty(BoolProperty::new(false))],
                            ),
                            (
                                String::from("UseDarkMode"),
                                vec![Property::BoolProperty(BoolProperty::new(true))],
                            ),
                            (
                                String::from("AnimateDayCycle"),
                                vec![Property::BoolProperty(BoolProperty::new(false))],
                            ),
                            (
                                String::from("EnableTractorCollision"),
                                vec![Property::BoolProperty(BoolProperty::new(false))],
                            ),
                            (
                                String::from("ShowInventory"),
                                vec![Property::BoolProperty(BoolProperty::new(true))],
                            ),
                            (
                                String::from("CameraAngle"),
                                vec![Property::StructProperty(StructProperty {
                                    guid: Guid::default(),
                                    value: StructPropertyValue::Vector2D(Vector2D {
                                        x: OrderedFloat::from(30.574748247861862),
                                        y: OrderedFloat::from(60.42525175213814),
                                    }),
                                })],
                            ),
                        ]),
                    },
                }),
            ),
            (
                String::from("HighScore"),
                Property::IntProperty(IntProperty { value: 2649 }),
            ),
        ]),
    }
}

pub const VECTOR2D_JSON: &str = r#"{
  "header": {
    "type": "Version3",
    "package_file_version": 522,
    "package_file_version_ue5": 1009,
    "engine_version": {
      "major": 5,
      "minor": 3,
      "patch": 2,
      "change_list": 29314046,
      "branch": "++UE5+Release-5.3"
    },
    "custom_version_format": 3,
    "custom_versions": {
      "22D5549C-BE4F-26A8-4607-2194D082B461": 44,
      "A35C9162-F74B-8E1C-C712-0EA3F79D21C8": 32,
      "240D40CC-7B4E-E9E0-83A2-F99B27C0C0DC": 0,
      "E432D8B0-0D4F-891F-B77E-CFACA24AFD36": 10,
      "2843C6E1-534D-2CA2-868E-6CA38CBD1764": 0,
      "3CC15E37-FB48-E406-F084-00B57E712A26": 4,
      "ED68B0E4-E942-94F4-0BDA-31A241BB462E": 40,
      "3F74FCCF-8044-B043-DF14-919373201D17": 37,
      "B5492BB0-E944-20BB-B732-04A36003E452": 3,
      "5C10E4A4-B549-A159-C440-C5A7EEDF7E54": 0,
      "C931C839-DC47-E65A-179C-449A7C8E1C3E": 0,
      "331BF078-984F-EAEB-EA84-B4B9A25AB9CC": 20,
      "0F383166-E043-4D2D-27CF-09805AA95669": 0,
      "9F8BF812-FC4A-7588-0CD9-7CA629BD3A38": 47,
      "4CE75A7B-104C-70D2-9857-58A95A2A210B": 13,
      "186929D7-DD4B-D61D-A864-E29D8438C13C": 3,
      "7852A1C2-FE4A-E7BF-FF90-176C55F71D53": 1,
      "D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209": 3,
      "DD75E529-2746-A3E0-76D2-109DEADC2C23": 17,
      "5DA643AF-4749-D37F-8E3E-739805BBC1D9": 15,
      "EC6C266B-8F4B-C71E-D9E4-0BA307FC4209": 1,
      "613DF70D-EA47-3FA2-E989-27B79A49410C": 1,
      "86181D60-844F-64AC-DED3-16AAD6C7EA0D": 111,
      "5B2CBC8D-E043-A754-BBFC-68A76090A27D": 2,
      "B7064C5B-F84A-6324-70BF-5B80DDD0F5CD": 10,
      "686308E7-584C-236B-701B-3984915E2616": 11,
      "D6BCFF9D-5801-4F49-8212-21E288A8923C": 10,
      "ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA": 1,
      "0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0": 10,
      "834AF935-6C40-58E2-F509-18A37C241096": 41,
      "6EC18FB6-E242-1B8B-5C21-53B4FE448805": 1,
      "0685E1B2-C2CF-7342-BBF4-4EA507BA8B75": 1,
      "3689F564-BA42-1BFD-8972-96BA4EFAD0D5": 1,
      "81D57D69-AB41-4FE6-EC51-4AAA28B6B7BE": 118,
      "425E9BD8-464D-BD24-A8AC-1284791764DF": 47,
      "525DDA59-4849-3212-7859-78B88BE9B870": 8,
      "325A0726-0847-0F73-328C-E988059D59F1": 0,
      "27D80E6F-9548-09A6-8D99-919CA40E1890": 2,
      "E38BD530-8242-EA95-59B1-E3A66AB0EBD8": 1,
      "E79E7F71-3A49-B0E9-3291-B3880781381B": 17,
      "FC09C468-8649-9570-D2AC-6389835186C4": 3,
      "194D0C43-7049-5471-699B-6987E5B090DF": 15,
      "BD32FEAA-144C-9553-255E-6AB6DDD13210": 1,
      "8EE1AF23-584E-E14C-52C2-618DB7BE53B9": 11,
      "EAB762A4-3A4E-99F4-1FEC-C199B2E12482": 4,
      "BDFDB52E-104D-AC01-8FF3-3681DAA59333": 5,
      "4F359D50-2F49-E6F6-B285-49A71C633C07": 0,
      "3EF0A495-E449-0B7E-56D3-43BAD987FF94": 7,
      "1C1BE3B6-EC11-9FD2-859F-7E85E270996F": 1,
      "40EB564A-DC11-F510-7E34-D392E76AC9B2": 3,
      "8A991784-EC43-C0BB-19D1-B38122272D07": 19,
      "004A8AD7-9746-58E8-B519-A8BAB4467D48": 18,
      "86F87955-1F4C-3A93-7B08-BA832FB96163": 2,
      "52BE2F61-0B40-53DA-914F-0D917C85B19F": 1,
      "367A23A4-C941-EACA-F818-A28FF31B6858": 5,
      "753F4E80-494B-8870-068C-D6A4DCB67E3C": 5,
      "F448D01E-684C-2E2F-A453-D0892D108FF1": 1,
      "F20A68FB-A34B-EF59-B519-A8BA3D44C873": 2,
      "0EB75099-174E-1AB4-0DFA-CCBBD67F8157": 1,
      "CD14175E-5129-4E48-A789-7A7078AB0293": 3,
      "7B472509-0140-3D76-73D6-919D11B4750B": 1,
      "1B218842-C616-4845-B267-761A002A7A50": 1,
      "9B9549DC-E74D-C053-88EA-5691395D7C5E": 2,
      "FB0C82A7-5943-A720-142C-548C50CF2396": 27,
      "4E7CE782-A543-2333-C513-6BB4F30D3197": 0,
      "AA1C1EE2-5E42-47AF-D46A-BF89BBA8444C": 0,
      "7E154A13-A349-E2D5-3C84-4E8D319EFE98": 2,
      "FA7AF5FC-8342-7650-58E6-A9B9322DA0FF": 79,
      "ED0A3111-614D-552E-A39A-67AF2C08A1C5": 17,
      "78BBDFF6-E4A0-50BB-4DB8-184023AFCB60": 2,
      "F37ABB24-834F-4656-C22D-2F1FFF96AD49": 5,
      "2923A576-B545-2309-41D8-AE98D86A2FCF": 5,
      "0769BC5F-AE40-C855-84F1-678E3FF1FF5E": 1,
      "438C7392-9C4D-8829-BE9B-3D9AC09FFF6E": 1
    },
    "save_game_class_name": "/Game/_Blueprints/BP_SettingsSave.BP_SettingsSave_C"
  },
  "properties": {
    "SettingsChanged": {
      "type": "MulticastInlineDelegateProperty",
      "value": {
        "delegates": [
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_WaterGauge_C_2147482315",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_C_2147482312",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_Row_Single_C_2147482309",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_Row_3_C_2147482305",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_5Row_C_2147482301",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_Row_5_C_2147482297",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_C_2147482293",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_Row_C_2147482286",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_Row3_C_2147482280",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_Row5_C_2147482274",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_C_2147482268",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_Row_C_2147482265",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_Row3_C_2147482261",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_Row5_C_2147482257",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_PlasticRow_C_2147482253",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_C_2147482249",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_1x10_C_2147482242",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_3Row_C_2147482235",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_5Row_C_2147482228",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_10x10_C_2147482221",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Modify_C_2147482214",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Row_C_2147482198",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Row3_C_2147482181",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_C_2147482164",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_C_2147482161",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_3_C_2147482157",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_5_C_2147482153",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_C_2147482149",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_AutomatedActionControl_C_2147482145",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_RemovePlaceable_C_2147482142",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_SeedSilo_C_2147482139",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_TractorBarn_C_2147482132",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Sell_C_2147482125",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_FuelStorageTank_C_2147482118",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_ChickenRun_C_2147482115",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_MovePlaceable_C_2147482112",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Beehive_C_2147482109",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_SetPHTool_Row_C_2147482106",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_BiodieselRefinery_C_2147482089",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_OilPress_C_2147482086",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_FlourMill_C_2147482083",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_LargeChickenCoop_C_2147482080",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_CropSign_C_2147482077",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Mulch_C_2147482070",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Mulch_Row_C_2147482054",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Mulch_Row3_C_2147482037",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Warehouse_C_2147482020",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_HarvestSilo_C_2147482013",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Stockpile_C_2147482008",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_CompostStation_C_2147482001",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_Renders_C_1",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_PlayerPawn_C_2147482331",
            "function_name": "UpdatedSavedSettings"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478921",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478905",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478890",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478875",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478860",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478303",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478288",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478273",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478258",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478243",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478228",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478141",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478126",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478111",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478096",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477750",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477735",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477720",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477705",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477690",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477675",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477660",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477645",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477189",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477162",
            "function_name": "SettingsChanged"
          }
        ]
      }
    },
    "AudioSettings": {
      "type": "StructProperty",
      "CustomStruct": {
        "type_name": "GameAudioSettings",
        "properties": {
          "MasterLevel": [
            {
              "type": "FloatProperty",
              "value": 0.20348908
            }
          ],
          "MusicLevel": [
            {
              "type": "FloatProperty",
              "value": 0.1511635
            }
          ],
          "SFXLevel": [
            {
              "type": "FloatProperty",
              "value": 0.5436054
            }
          ]
        }
      }
    },
    "GameSettings": {
      "type": "StructProperty",
      "CustomStruct": {
        "type_name": "GameSettings",
        "properties": {
          "CurrentSaveSlot": [
            {
              "type": "StrProperty",
              "value": "SAVE2"
            }
          ],
          "LoadTutorial": [
            {
              "type": "BoolProperty",
              "value": false
            }
          ],
          "DisplayNewOrders": [
            {
              "type": "BoolProperty",
              "value": false
            }
          ],
          "EscapeExitsTool": [
            {
              "type": "BoolProperty",
              "value": false
            }
          ],
          "UseDarkMode": [
            {
              "type": "BoolProperty",
              "value": true
            }
          ],
          "AnimateDayCycle": [
            {
              "type": "BoolProperty",
              "value": false
            }
          ],
          "EnableTractorCollision": [
            {
              "type": "BoolProperty",
              "value": false
            }
          ],
          "ShowInventory": [
            {
              "type": "BoolProperty",
              "value": true
            }
          ],
          "CameraAngle": [
            {
              "type": "StructProperty",
              "Vector2D": {
                "x": 30.574748247861862,
                "y": 60.42525175213814
              }
            }
          ]
        }
      }
    },
    "HighScore": {
      "type": "IntProperty",
      "value": 2649
    }
  }
}"#;
