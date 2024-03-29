use std::collections::HashMap;

pub fn hints() -> HashMap<String, String> {
    HashMap::from([
        (
            "SeasonSave.StructProperty.Seasons.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty".to_string(),
            "Unk".to_string(),
        ),
        (
            "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty.CompletedSpecialChallenges.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string()
        ),
        (
            "UnLockedMissionParameters.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "UnLockedMissionParameters.MapProperty.Value.StructProperty".to_string(),
            "Unk".to_string(),
        ),
        (
            "ItemUpgradeSelections.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "ItemUpgradeSelections.MapProperty.Value.StructProperty".to_string(),
            "Unk".to_string(),
        ),
        (
            "ItemUpgradeLoadouts.ArrayProperty.Loadout.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "ItemUpgradeLoadouts.ArrayProperty.Loadout.MapProperty.Value.StructProperty"
                .to_string(),
            "Unk".to_string(),
        ),
        (
            "EnemiesKilled.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "UnlockedItemSkins.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "UnlockedItemSkins.MapProperty.Value.StructProperty".to_string(),
            "Unk".to_string(),
        ),
        (
            "Resources.StructProperty.OwnedResources.MapProperty.Key.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "FSDEventRewardsSave.StructProperty.EventsSeen.SetProperty.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "GameDLCSave.StructProperty.AnnouncedIDs.SetProperty.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "Drinks.StructProperty.UnlockedDrinks.SetProperty.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "UnlockedItemSkins.MapProperty.Value.StructProperty.Skins.SetProperty.StructProperty"
                .to_string(),
            "Guid".to_string(),
        ),
        (
            "UnlockedPickaxeParts.SetProperty.StructProperty".to_string(),
            "Guid".to_string(),
        ),
        (
            "MinersManualKnownObjects.SetProperty.StructProperty".to_string(),
            "Guid".to_string(),
        ),
    ])
}
