use std::{collections::HashMap, fs::File, path::Path};

use gvas::GvasFile;
use serde_json::json;

fn get_hints() -> HashMap<String, String> {
    let mut hints = HashMap::new();

    hints.insert(
        "MinersManualKnownObjects.SetProperty.StructProperty".to_string(),
        "Struct".to_string(),
    );
    hints.insert(
        "GameplayDatabase.MapProperty.Value.StructProperty".to_string(),
        "Struct".to_string(),
    );
    hints.insert(
        "PlayerAttributes.MapProperty.Key.StructProperty".to_string(),
        "Struct".to_string(),
    );

    hints
}

#[test]
fn deserialize() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/SaveSlot_03.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    let file =
        GvasFile::read_with_hints(&mut file, &get_hints()).expect("Failed to parse gvas file");
    let json = json!({"LastSaveTime":{"type":"StructProperty","value":{"DateTime":{"ticks":638160761644140000u64}}},"PlayerClass":{"type":"ObjectProperty","value":"/Game/Character/Player/Blueprints/BP_Soldier.BP_Soldier_C"},"Version":{"type":"IntProperty","value":3},"GameplayDatabase":{"type":"MapProperty","key_type":"NameProperty","value_type":"StructProperty","allocation_flags":0,"value":[[{"type":"NameProperty","value":"unlock.welcomescreen.seen"},{"type":"StructProperty","value":{"CustomStruct":["Struct",[["AsFloat",{"type":"FloatProperty","value":0.0}],["AsString",{"type":"StrProperty","value":null}]]]}}],[{"type":"NameProperty","value":"game.tutorial.finished"},{"type":"StructProperty","value":{"CustomStruct":["Struct",[["AsFloat",{"type":"FloatProperty","value":1.0}],["AsString",{"type":"StrProperty","value":null}]]]}}],[{"type":"NameProperty","value":"game.tutorial.skipped"},{"type":"StructProperty","value":{"CustomStruct":["Struct",[["AsFloat",{"type":"FloatProperty","value":1.0}],["AsString",{"type":"StrProperty","value":null}]]]}}],[{"type":"NameProperty","value":"dialogs.messages.seen.Rumiko.0.50"},{"type":"StructProperty","value":{"CustomStruct":["Struct",[["AsFloat",{"type":"FloatProperty","value":1.0}],["AsString",{"type":"StrProperty","value":null}]]]}}],[{"type":"NameProperty","value":"codex.Rumiko"},{"type":"StructProperty","value":{"CustomStruct":["Struct",[["AsFloat",{"type":"FloatProperty","value":1.0}],["AsString",{"type":"StrProperty","value":null}]]]}}]]},"PlayerAttributes":{"type":"MapProperty","key_type":"StructProperty","value_type":"FloatProperty","allocation_flags":0,"value":[[{"type":"StructProperty","value":{"CustomStruct":["Struct",[["AttributeName",{"type":"StrProperty","value":"Currency_Blueprints"}],["Attribute",{"type":"FieldPathProperty","value":{"path":["Currency_Blueprints"],"resolved_owner":"/Script/CD.CDPlayerAttributeSet"}}],["AttributeOwner",{"type":"ObjectProperty","value":"None"}]]]}},{"type":"FloatProperty","value":0.0}],[{"type":"StructProperty","value":{"CustomStruct":["Struct",[["AttributeName",{"type":"StrProperty","value":"Currency_Electrum"}],["Attribute",{"type":"FieldPathProperty","value":{"path":["Currency_Electrum"],"resolved_owner":"/Script/CD.CDPlayerAttributeSet"}}],["AttributeOwner",{"type":"ObjectProperty","value":"None"}]]]}},{"type":"FloatProperty","value":0.0}]]},"SecondaryWeaponClass":{"type":"ObjectProperty","value":"/Game/Weapons/RocketLauncher/Blueprints/BP_RocketLauncher.BP_RocketLauncher_C"}});
    assert_eq!(json, serde_json::to_value(&file.properties).unwrap())
}
