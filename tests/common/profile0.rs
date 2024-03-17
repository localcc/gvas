use std::collections::HashMap;

pub(crate) fn hints() -> HashMap<String, String> {
    HashMap::from([(
        "achievementHistoryScope.StructProperty.metrics.MapProperty.Value.StructProperty"
            .to_string(),
        "this type hint is unused".to_string(),
    )])
}

pub(crate) const PROFILE_0_JSON: &str = r#"{
  "header": {
    "type": "Version2",
    "package_file_version": 522,
    "engine_version": {
      "major": 4,
      "minor": 27,
      "patch": 2,
      "change_list": 0,
      "branch": "++UE4+Release-4.27"
    },
    "custom_version_format": 3,
    "custom_versions": {
      "12E426FB-4D4B-151F-0A55-7293702F1D96": 3,
      "F37ABB24-834F-4656-C22D-2F1FFF96AD49": 5,
      "FA7AF5FC-8342-7650-58E6-A9B9322DA0FF": 68,
      "ED0A3111-614D-552E-A39A-67AF2C08A1C5": 17,
      "2923A576-B545-2309-41D8-AE98D86A2FCF": 5,
      "0769BC5F-AE40-C855-84F1-678E3FF1FF5E": 1,
      "4E7CE782-A543-2333-C513-6BB4F30D3197": 0,
      "22D5549C-BE4F-26A8-4607-2194D082B461": 43,
      "E432D8B0-0D4F-891F-B77E-CFACA24AFD36": 10,
      "2843C6E1-534D-2CA2-868E-6CA38CBD1764": 0,
      "3CC15E37-FB48-E406-F084-00B57E712A26": 4,
      "ED68B0E4-E942-94F4-0BDA-31A241BB462E": 40,
      "3F74FCCF-8044-B043-DF14-919373201D17": 37,
      "B5492BB0-E944-20BB-B732-04A36003E452": 3,
      "5C10E4A4-B549-A159-C440-C5A7EEDF7E54": 0,
      "C931C839-DC47-E65A-179C-449A7C8E1C3E": 0,
      "331BF078-984F-EAEB-EA84-B4B9A25AB9CC": 14,
      "0F383166-E043-4D2D-27CF-09805AA95669": 0,
      "9F8BF812-FC4A-7588-0CD9-7CA629BD3A38": 45,
      "4CE75A7B-104C-70D2-9857-58A95A2A210B": 13,
      "186929D7-DD4B-D61D-A864-E29D8438C13C": 3,
      "7852A1C2-FE4A-E7BF-FF90-176C55F71D53": 1,
      "D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209": 3,
      "DD75E529-2746-A3E0-76D2-109DEADC2C23": 17,
      "5DA643AF-4749-D37F-8E3E-739805BBC1D9": 15,
      "EC6C266B-8F4B-C71E-D9E4-0BA307FC4209": 1,
      "613DF70D-EA47-3FA2-E989-27B79A49410C": 1,
      "86181D60-844F-64AC-DED3-16AAD6C7EA0D": 47,
      "686308E7-584C-236B-701B-3984915E2616": 1,
      "D6BCFF9D-5801-4F49-8212-21E288A8923C": 10,
      "ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA": 1,
      "0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0": 10,
      "834AF935-6C40-58E2-F509-18A37C241096": 41,
      "6EC18FB6-E242-1B8B-5C21-53B4FE448805": 1,
      "0685E1B2-C2CF-7342-BBF4-4EA507BA8B75": 1,
      "3689F564-BA42-1BFD-8972-96BA4EFAD0D5": 1,
      "27D80E6F-9548-09A6-8D99-919CA40E1890": 2,
      "E79E7F71-3A49-B0E9-3291-B3880781381B": 8,
      "50326854-AF48-9980-9698-C88BB7F9ADFB": 0,
      "194D0C43-7049-5471-699B-6987E5B090DF": 15,
      "BD32FEAA-144C-9553-255E-6AB6DDD13210": 1,
      "8EE1AF23-584E-E14C-52C2-618DB7BE53B9": 11,
      "EAB762A4-3A4E-99F4-1FEC-C199B2E12482": 4,
      "BDFDB52E-104D-AC01-8FF3-3681DAA59333": 5,
      "4F359D50-2F49-E6F6-B285-49A71C633C07": 0,
      "40EB564A-DC11-F510-7E34-D392E76AC9B2": 2,
      "004A8AD7-9746-58E8-B519-A8BAB4467D48": 18,
      "86F87955-1F4C-3A93-7B08-BA832FB96163": 2,
      "52BE2F61-0B40-53DA-914F-0D917C85B19F": 1,
      "367A23A4-C941-EACA-F818-A28FF31B6858": 4,
      "753F4E80-494B-8870-068C-D6A4DCB67E3C": 5,
      "F20A68FB-A34B-EF59-B519-A8BA3D44C873": 2,
      "0EB75099-174E-1AB4-0DFA-CCBBD67F8157": 1
    },
    "save_game_class_name": "/Script/EconCore.GenericSaveGame"
  },
  "properties": {
    "dbStrings": {
      "type": "MapProperty",
      "str_strs": {
        "Story/Campaign/index": "c1.m19a.s2",
        "music": "music.lasthope",
        "preferedEngine": "DieselEngine",
        "Option_settings.a.master": "50.0%",
        "Option_settings.a.music": "50.0%",
        "Option_settings.g.terrainres": "tfactor.x2",
        "Option_settings.dpiscale": "120%",
        "Option_settings.daytime": "toggle.enabled",
        "preferedTrainColor": "Crimson"
      }
    },
    "dbNumbers": {
      "type": "MapProperty",
      "str_ints": {
        "SeenMovie_Welcome": 1,
        "meta.attemptingload": 0,
        "appearin_lastsfx_time_intro": 8734,
        "appearin_lastsfx_time_outro": 8738,
        "playtime_418AD60A45FCE310E9D3DEB5B9894EF2": 38,
        "bestrank_418AD60A45FCE310E9D3DEB5B9894EF2": 4,
        "money": 14,
        "roamPlayerPosition_x": -1366,
        "roamPlayerPosition_y": -934,
        "playtime_2AB6B5764521494B8DE77891EFD0A8AE": 68,
        "bestrank_2AB6B5764521494B8DE77891EFD0A8AE": 4,
        "playtime_9696568941466D44AE95DD9F1F170004": 67,
        "bestrank_9696568941466D44AE95DD9F1F170004": 4,
        "playstyle.modifier.noghosting": 0,
        "playstyle.modifier.notimer": 0,
        "playstyle.modifier.infinitemoney": 0,
        "playstyle.modifier.endless": 0,
        "playtime_197C3AD14915E04F243623981F89CA32": 496,
        "bestrank_197C3AD14915E04F243623981F89CA32": 4,
        "didNotify_unlock.placeable.oilwell": 1,
        "didNotify_unlock.upgrade.oilpower": 1,
        "playtime_275848C64EC0DFAC00DF819482F5E31D": 802,
        "bestrank_275848C64EC0DFAC00DF819482F5E31D": 4,
        "didNotify_unlock.BasicEngine": 1,
        "playtime_82A9B46F42BF6C5555D608A6619B35E6": 1374,
        "bestrank_82A9B46F42BF6C5555D608A6619B35E6": 4,
        "didNotify_unlock.CustomEngine": 1,
        "didNotify_unlock.placeable.geothermal": 1,
        "playtime_976B945848D92B929BC9318144D2AFD9": 1615,
        "bestrank_976B945848D92B929BC9318144D2AFD9": 2,
        "didNotify_unlock.placeable.oilpower": 1,
        "didNotify_unlock.placeable.limestonemine": 1,
        "didNotify_unlock.placeable.coppermine": 1,
        "didNotify_unlock.placeable.concreteyard": 1,
        "didNotify_unlock.upgrade.concreteyard": 1,
        "didNotify_unlock.upgrade.coiler": 1,
        "playtime_23565CB1440C2F8CF29A20BE0F344E14": 1761,
        "bestrank_23565CB1440C2F8CF29A20BE0F344E14": 3,
        "didNotify_unlock.autoBranching": 1,
        "playtime_C40ABA944C06215E9BA1F3808798BD78": 866,
        "bestrank_C40ABA944C06215E9BA1F3808798BD78": 4,
        "playtime_ABD6676B4D2F295C2C56F7BC7A80CF1B": 1049,
        "bestrank_ABD6676B4D2F295C2C56F7BC7A80CF1B": 4,
        "didNotify_unlock.heightIncrease1": 1,
        "playtime_0A465B724263583BBBAC469FE7D852F8": 2255,
        "bestrank_0A465B724263583BBBAC469FE7D852F8": 1,
        "didNotify_unlock.placeable.saltsifter": 1,
        "didNotify_unlock.placeable.coalmine": 1,
        "didNotify_unlock.upgrade.saltsifter": 1,
        "playtime_7CA6B81C4B27D3DD5D9110A91BD3B333": 3046,
        "bestrank_7CA6B81C4B27D3DD5D9110A91BD3B333": 1,
        "didNotify_unlock.placeable.coalpower": 1,
        "didNotify_unlock.ClimberEngine": 1,
        "didNotify_unlock.placeable.ironmine": 1,
        "playtime_E2AB4F1B4783E580DF19F5BB1048C40C": 1258,
        "bestrank_E2AB4F1B4783E580DF19F5BB1048C40C": 4,
        "didNotify_unlock.placeable.steelmill": 1,
        "playtime_E54ECA4B4EBE680952C1E1A108EBFED6": 1402,
        "bestrank_E54ECA4B4EBE680952C1E1A108EBFED6": 4,
        "didNotify_unlock.placeable.plasticizer": 1,
        "playtime_3D2D8F324CA52B457413909B8FA06DBD": 3128,
        "bestrank_3D2D8F324CA52B457413909B8FA06DBD": 2,
        "didNotify_unlock.placeable.furnitureAssembler": 1,
        "didNotify_unlock.upgrade.plasticizer": 1,
        "didNotify_unlock.upgrade.furnitureassembler": 1,
        "didNotify_unlock.upgrade.coalpower": 1,
        "didNotify_unlock.upgrade.steelmill": 1,
        "playtime_79C004794149BAEF9D01D0812D468C73": 1699,
        "bestrank_79C004794149BAEF9D01D0812D468C73": 3,
        "playtime_C351F593458BA314057C44964D17796E": 1908,
        "bestrank_C351F593458BA314057C44964D17796E": 4,
        "didNotify_unlock.placeable.goodsfactory": 1,
        "didNotify_unlock.placeable.oilrefinery": 1,
        "didNotify_unlock.placeable.hardmold": 1,
        "didNotify_unlock.placeable.neonrefinery": 1,
        "playtime_CE7047994E27F2501309BFBF10504939": 2497,
        "bestrank_CE7047994E27F2501309BFBF10504939": 3,
        "playtime_6F26D84E49EF7EE051658D88685804A3": 2789,
        "bestrank_6F26D84E49EF7EE051658D88685804A3": 2,
        "didNotify_unlock.placeable.incinerator": 1,
        "didNotify_unlock.placeable.oregassifier": 1,
        "didNotify_unlock.placeable.electronicsfab": 1,
        "didNotify_unlock.upgrade.goodsfactory": 1,
        "playtime_7792A16A471FD6ABABB19C909A879023": 4257,
        "bestrank_7792A16A471FD6ABABB19C909A879023": 2,
        "playtime_A8C916CA44A8F236048AC88B3B9D9D1B": 279,
        "bestrank_A8C916CA44A8F236048AC88B3B9D9D1B": 4,
        "playtime_8017CDB54D7D1D368B309198831A15F1": 2814,
        "bestrank_8017CDB54D7D1D368B309198831A15F1": 4,
        "didNotify_unlock.placeable.tooldie": 1,
        "didNotify_unlock.upgrade.tooldie": 1,
        "playtime_2E33E5724251952E750D6FA29B6A6242": 3373,
        "bestrank_2E33E5724251952E750D6FA29B6A6242": 3,
        "playtime_BDE017E34B75C14E86D9AA973EFE3385": 1571,
        "bestrank_BDE017E34B75C14E86D9AA973EFE3385": 4,
        "didNotify_unlock.upgrade.oilrefinery": 1,
        "playtime_FDE6EA5B41E9DF7229789A93AE4E8170": 2556,
        "bestrank_FDE6EA5B41E9DF7229789A93AE4E8170": 3,
        "playtime_E6D6A92C43B137E2C8B402A0F43E5F51": 2366,
        "bestrank_E6D6A92C43B137E2C8B402A0F43E5F51": 4,
        "didNotify_unlock.placeable.hullyard": 1,
        "didNotify_unlock.heightIncrease2": 1,
        "didNotify_unlock.HybridEngine": 1,
        "didNotify_unlock.upgrade.neonrefinery": 1,
        "didNotify_unlock.upgrade.electronicsfab": 1,
        "didNotify_unlock.upgrade.hullyard": 1,
        "didNotify_unlock.upgrade.incinerator": 1,
        "didNotify_unlock.upgrade.oregassifier": 1,
        "didNotify_unlock.upgrade.hardmold": 1,
        "playtime_49106FD1406C726B44788684DD19013B": 3678,
        "bestrank_49106FD1406C726B44788684DD19013B": 3,
        "didNotify_unlock.placeable.luxuryassembler": 1,
        "playtime_19FC4F6F4F082AEF91001DB636DD3CB0": 2258,
        "bestrank_19FC4F6F4F082AEF91001DB636DD3CB0": 4,
        "didNotify_unlock.placeable.glasssmelter": 1,
        "playtime_B9C9338A41EB24590F4F3D8F77E1DD59": 5376,
        "bestrank_B9C9338A41EB24590F4F3D8F77E1DD59": 4,
        "didNotify_unlock.placeable.signworks": 1,
        "didNotify_unlock.placeable.chipfab": 1,
        "playtime_1F2DF7A54B9EF8A02928BFAF03675B92": 1187,
        "bestrank_1F2DF7A54B9EF8A02928BFAF03675B92": 4,
        "didNotify_unlock.upgrade.glasssmelter": 1,
        "didNotify_unlock.stationSpeed": 1,
        "playtime_F65B1395431DE131107D1BA87B09A133": 6844,
        "bestrank_F65B1395431DE131107D1BA87B09A133": 2,
        "didNotify_unlock.placeable.tubeplant": 1,
        "didNotify_unlock.placeable.motorAssembly": 1,
        "didNotify_unlock.upgrade.motorAssembly": 1,
        "playtime_AB6DD6AB4678ACAC44968D8ABDD312E3": 6083,
        "bestrank_AB6DD6AB4678ACAC44968D8ABDD312E3": 3,
        "didNotify_unlock.placeable.heavyworks": 1,
        "didNotify_unlock.placeable.framer": 1,
        "playtime_6071814D4CF0425177A7F39AC189358B": 1383,
        "bestrank_6071814D4CF0425177A7F39AC189358B": 4,
        "playtime_85EB724D43C96BAA7832A3BAA3AEDEC2": 712,
        "bestrank_85EB724D43C96BAA7832A3BAA3AEDEC2": 4,
        "playtime_7CA27AEE46CC848BE165FEA4F8081A9C": 1167,
        "bestrank_7CA27AEE46CC848BE165FEA4F8081A9C": 4,
        "playtime_02700E05478FA5C176BA6F9F97675886": 1310,
        "bestrank_02700E05478FA5C176BA6F9F97675886": 4,
        "playtime_428959F042082EEB444E56AC0D976004": 1366,
        "bestrank_428959F042082EEB444E56AC0D976004": 4,
        "playtime_8633BE2044A6EC9315E5F6BAA128ACE8": 1273,
        "bestrank_8633BE2044A6EC9315E5F6BAA128ACE8": 4,
        "didNotify_unlock.DieselEngine": 1,
        "playtime_573832E84B1534B065BF6693C7523FC4": 1832,
        "bestrank_573832E84B1534B065BF6693C7523FC4": 4,
        "playtime_977D0299439D9BF70DFC5BB4CF27591C": 1648,
        "bestrank_977D0299439D9BF70DFC5BB4CF27591C": 4,
        "playtime_CCEE486545CECC02A0810280F994B0D8": 3575,
        "bestrank_CCEE486545CECC02A0810280F994B0D8": 2,
        "playtime_030D7E264B5C212715FEFE80F6EB97E2": 3677,
        "bestrank_030D7E264B5C212715FEFE80F6EB97E2": 3,
        "didNotify_unlock.upgrade.signworks": 1,
        "playtime_A92DE3C648A91DFDF02B598AE39A3DCA": 4637,
        "bestrank_A92DE3C648A91DFDF02B598AE39A3DCA": 4,
        "didNotify_unlock.upgrade.chipfab": 1,
        "playtime_11EC647042ECCC315E1B909C4D4A9666": 2706,
        "bestrank_11EC647042ECCC315E1B909C4D4A9666": 4,
        "didNotify_unlock.RescueEngine": 1,
        "didNotify_unlock.upgrade.tubeplant": 1,
        "didNotify_unlock.upgrade.luxuryassembler": 1,
        "didNotify_unlock.upgrade.framer": 1,
        "didNotify_unlock.upgrade.heavyworks": 1,
        "playtime_173683C94F06A5F7B0D24F80BA2CF073": 4455,
        "bestrank_173683C94F06A5F7B0D24F80BA2CF073": 3,
        "playtime_E312638F4BED14780F466B95B370E0DD": 2011,
        "bestrank_E312638F4BED14780F466B95B370E0DD": 4,
        "playtime_E23150FF41457A230C7FC4B84226C0C5": 5053,
        "bestrank_E23150FF41457A230C7FC4B84226C0C5": 4,
        "playtime_4D163CA741A2EDAA7ED54C83D77539FF": 5906,
        "bestrank_4D163CA741A2EDAA7ED54C83D77539FF": 3,
        "playtime_A698A0C242D5A317F8DB6D97EF0AFF8E": 4176,
        "bestrank_A698A0C242D5A317F8DB6D97EF0AFF8E": 4,
        "didNotify_unlock.placeable.rocketfactory": 1,
        "didNotify_unlock.placeable.boosterplant": 1,
        "didNotify_unlock.placeable.armory": 1,
        "didNotify_unlock.BulletEngine": 1,
        "didNotify_unlock.upgrade.rocketfactory": 1,
        "playtime_70B1F2D8471C128390408C94C7EB161E": 1914,
        "bestrank_70B1F2D8471C128390408C94C7EB161E": 4
      }
    },
    "UnlockLayer": {
      "type": "StructProperty",
      "value": {
        "CustomStruct": [
          "UnlockLayer",
          [
            [
              "ownCounts",
              {
                "type": "MapProperty",
                "name_ints": {
                  "unlock.c1.t2": 1,
                  "unlock.c1.t3": 1,
                  "unlock.c1.m1": 1,
                  "unlock.c1.m2": 1,
                  "unlock.milestone.oilwell": 1,
                  "unlock.upgrade.oilpower": 1,
                  "unlock.milestone.Workhorse": 1,
                  "unlock.c1.m2bonus": 1,
                  "unlock.c1.m3": 1,
                  "unlock.CustomEngine": 1,
                  "unlock.placeable.geothermal": 1,
                  "unlock.upgrade.geothermal": 1,
                  "unlock.milestone.oilPower": 1,
                  "unlock.c1.m4": 1,
                  "unlock.milestone.coppermine": 1,
                  "unlock.milestone.coiler": 1,
                  "unlock.milestone.limestonemine": 1,
                  "unlock.milestone.concreteYard": 1,
                  "unlock.upgrade.concreteyard": 1,
                  "unlock.upgrade.coiler": 1,
                  "unlock.c1.mspecial1": 1,
                  "unlock.milestone.autoBranching": 1,
                  "unlock.c1.m6": 1,
                  "unlock.milestone.MediumHeight": 1,
                  "unlock.c1.m7": 1,
                  "unlock.milestone.saltsifter": 1,
                  "unlock.milestone.coalMine": 1,
                  "unlock.upgrade.saltsifter": 1,
                  "unlock.c1.m8": 1,
                  "unlock.milestone.coalpower": 1,
                  "unlock.milestone.Industrial": 1,
                  "unlock.milestone.ironmine": 1,
                  "unlock.c1.m9": 1,
                  "unlock.c1.m8bonus1": 1,
                  "unlock.c1.m8bonus2": 1,
                  "unlock.milestone.steelMill": 1,
                  "unlock.milestone.plasticizer": 1,
                  "unlock.c1.mspecial2": 1,
                  "unlock.c1.m9bonus": 1,
                  "unlock.milestone.furnitureAssembler": 1,
                  "unlock.upgrade.plasticizer": 1,
                  "unlock.upgrade.furnitureassembler": 1,
                  "unlock.upgrade.coalpower": 1,
                  "unlock.upgrade.steelmill": 1,
                  "unlock.c1.m10a1": 1,
                  "unlock.c1.m10b1": 1,
                  "unlock.milestone.oilrefinery": 1,
                  "unlock.milestone.neonrefinery": 1,
                  "unlock.milestone.goodsFactory": 1,
                  "unlock.milestone.hardmold": 1,
                  "unlock.milestone.electronicsFab": 1,
                  "unlock.c1.m11": 1,
                  "unlock.c1.m10bonus": 1,
                  "unlock.milestone.incinerator": 1,
                  "unlock.milestone.oreGassifier": 1,
                  "unlock.c1.mspecial3": 1,
                  "unlock.c1.m11bonus1": 1,
                  "unlock.c1.m11bonus2": 1,
                  "unlock.placeable.electronicsfab": 1,
                  "unlock.upgrade.goodsfactory": 1,
                  "unlock.c1.m12": 1,
                  "unlock.milestone.tooldie": 1,
                  "unlock.c1.m12bonus": 1,
                  "unlock.c1.m13": 1,
                  "unlock.upgrade.tooldie": 1,
                  "unlock.upgrade.oilrefinery": 1,
                  "unlock.c1.m10a2": 1,
                  "unlock.milestone.Electric": 1,
                  "unlock.c1.m13bonus1": 1,
                  "unlock.c1.m13bonus2": 1,
                  "unlock.c1.m14": 1,
                  "unlock.milestone.hullyard": 1,
                  "unlock.milestone.MaxHeight": 1,
                  "unlock.HybridEngine": 1,
                  "unlock.upgrade.neonrefinery": 1,
                  "unlock.upgrade.electronicsfab": 1,
                  "unlock.upgrade.hullyard": 1,
                  "unlock.upgrade.incinerator": 1,
                  "unlock.upgrade.oregassifier": 1,
                  "unlock.upgrade.hardmold": 1,
                  "unlock.milestone.luxuryAssembler": 1,
                  "unlock.c1.mspecial4": 1,
                  "unlock.c1.m14bonus": 1,
                  "unlock.milestone.rescue": 1,
                  "unlock.c1.m15": 1,
                  "unlock.milestone.glassSmelter": 1,
                  "unlock.milestone.signworks": 1,
                  "unlock.c1.m16": 1,
                  "unlock.c1.m15bonus1": 1,
                  "unlock.c1.m15bonus2": 1,
                  "unlock.milestone.chipfab": 1,
                  "unlock.upgrade.glasssmelter": 1,
                  "unlock.stationSpeed": 1,
                  "unlock.c1.m17a": 1,
                  "unlock.c1.m17b": 1,
                  "unlock.c1.m16bonus": 1,
                  "unlock.milestone.tubePlant": 1,
                  "unlock.milestone.diesel": 1,
                  "unlock.milestone.motor": 1,
                  "unlock.upgrade.motorAssembly": 1,
                  "unlock.milestone.heavyworks": 1,
                  "unlock.milestone.framer": 1,
                  "unlock.c1.m18a": 1,
                  "unlock.c1.m18b": 1,
                  "unlock.c1.m17bonus": 1,
                  "placeholder": 1,
                  "unlock.c1.m4bonus": 1,
                  "unlock.c1.m5": 1,
                  "unlock.c1.m4bonus2": 1,
                  "unlock.DieselEngine": 1,
                  "unlock.upgrade.signworks": 1,
                  "unlock.upgrade.chipfab": 1,
                  "unlock.RescueEngine": 1,
                  "unlock.upgrade.tubeplant": 1,
                  "unlock.upgrade.luxuryassembler": 1,
                  "unlock.upgrade.framer": 1,
                  "unlock.upgrade.heavyworks": 1,
                  "unlock.milestone.armory": 1,
                  "unlock.milestone.boosterplant": 1,
                  "unlock.milestone.rocketfactory": 1,
                  "unlock.milestone.Bullet": 1,
                  "unlock.c1.m19a": 1,
                  "unlock.c1.m19b": 1,
                  "unlock.c1.m18bonus": 1,
                  "unlock.BulletEngine": 1,
                  "unlock.upgrade.rocketfactory": 1
                }
              }
            ],
            [
              "historicCounts",
              {
                "type": "MapProperty",
                "name_ints": {
                  "unlock.c1.t2": 1,
                  "unlock.c1.t3": 1,
                  "unlock.c1.m1": 1,
                  "unlock.c1.m2": 1,
                  "unlock.milestone.oilwell": 1,
                  "unlock.upgrade.oilpower": 1,
                  "unlock.milestone.Workhorse": 1,
                  "unlock.c1.m2bonus": 1,
                  "unlock.c1.m3": 1,
                  "unlock.CustomEngine": 1,
                  "unlock.placeable.geothermal": 1,
                  "unlock.upgrade.geothermal": 1,
                  "unlock.milestone.oilPower": 1,
                  "unlock.c1.m4": 1,
                  "unlock.milestone.coppermine": 1,
                  "unlock.milestone.coiler": 1,
                  "unlock.milestone.limestonemine": 1,
                  "unlock.milestone.concreteYard": 1,
                  "unlock.upgrade.concreteyard": 1,
                  "unlock.upgrade.coiler": 1,
                  "unlock.c1.mspecial1": 1,
                  "unlock.milestone.autoBranching": 1,
                  "unlock.c1.m6": 1,
                  "unlock.milestone.MediumHeight": 1,
                  "unlock.c1.m7": 1,
                  "unlock.milestone.saltsifter": 1,
                  "unlock.milestone.coalMine": 1,
                  "unlock.upgrade.saltsifter": 1,
                  "unlock.c1.m8": 1,
                  "unlock.milestone.coalpower": 1,
                  "unlock.milestone.Industrial": 1,
                  "unlock.milestone.ironmine": 1,
                  "unlock.c1.m9": 1,
                  "unlock.c1.m8bonus1": 1,
                  "unlock.c1.m8bonus2": 1,
                  "unlock.milestone.steelMill": 1,
                  "unlock.milestone.plasticizer": 1,
                  "unlock.c1.mspecial2": 1,
                  "unlock.c1.m9bonus": 1,
                  "unlock.milestone.furnitureAssembler": 1,
                  "unlock.upgrade.plasticizer": 1,
                  "unlock.upgrade.furnitureassembler": 1,
                  "unlock.upgrade.coalpower": 1,
                  "unlock.upgrade.steelmill": 1,
                  "unlock.c1.m10a1": 1,
                  "unlock.c1.m10b1": 1,
                  "unlock.milestone.oilrefinery": 1,
                  "unlock.milestone.neonrefinery": 1,
                  "unlock.milestone.goodsFactory": 1,
                  "unlock.milestone.hardmold": 1,
                  "unlock.milestone.electronicsFab": 1,
                  "unlock.c1.m11": 1,
                  "unlock.c1.m10bonus": 1,
                  "unlock.milestone.incinerator": 1,
                  "unlock.milestone.oreGassifier": 1,
                  "unlock.c1.mspecial3": 1,
                  "unlock.c1.m11bonus1": 1,
                  "unlock.c1.m11bonus2": 1,
                  "unlock.placeable.electronicsfab": 1,
                  "unlock.upgrade.goodsfactory": 1,
                  "unlock.c1.m12": 1,
                  "unlock.milestone.tooldie": 1,
                  "unlock.c1.m12bonus": 1,
                  "unlock.c1.m13": 1,
                  "unlock.upgrade.tooldie": 1,
                  "unlock.upgrade.oilrefinery": 1,
                  "unlock.c1.m10a2": 1,
                  "unlock.milestone.Electric": 1,
                  "unlock.c1.m13bonus1": 1,
                  "unlock.c1.m13bonus2": 1,
                  "unlock.c1.m14": 1,
                  "unlock.milestone.hullyard": 1,
                  "unlock.milestone.MaxHeight": 1,
                  "unlock.HybridEngine": 1,
                  "unlock.upgrade.neonrefinery": 1,
                  "unlock.upgrade.electronicsfab": 1,
                  "unlock.upgrade.hullyard": 1,
                  "unlock.upgrade.incinerator": 1,
                  "unlock.upgrade.oregassifier": 1,
                  "unlock.upgrade.hardmold": 1,
                  "unlock.milestone.luxuryAssembler": 1,
                  "unlock.c1.mspecial4": 1,
                  "unlock.c1.m14bonus": 1,
                  "unlock.milestone.rescue": 1,
                  "unlock.c1.m15": 1,
                  "unlock.milestone.glassSmelter": 1,
                  "unlock.milestone.signworks": 1,
                  "unlock.c1.m16": 1,
                  "unlock.c1.m15bonus1": 1,
                  "unlock.c1.m15bonus2": 1,
                  "unlock.milestone.chipfab": 1,
                  "unlock.upgrade.glasssmelter": 1,
                  "unlock.stationSpeed": 1,
                  "unlock.c1.m17a": 1,
                  "unlock.c1.m17b": 1,
                  "unlock.c1.m16bonus": 1,
                  "unlock.milestone.tubePlant": 1,
                  "unlock.milestone.diesel": 1,
                  "unlock.milestone.motor": 1,
                  "unlock.upgrade.motorAssembly": 1,
                  "unlock.milestone.heavyworks": 1,
                  "unlock.milestone.framer": 1,
                  "unlock.c1.m18a": 1,
                  "unlock.c1.m18b": 1,
                  "unlock.c1.m17bonus": 1,
                  "placeholder": 1,
                  "unlock.c1.m4bonus": 1,
                  "unlock.c1.m5": 1,
                  "unlock.c1.m4bonus2": 1,
                  "unlock.DieselEngine": 1,
                  "unlock.upgrade.signworks": 1,
                  "unlock.upgrade.chipfab": 1,
                  "unlock.RescueEngine": 1,
                  "unlock.upgrade.tubeplant": 1,
                  "unlock.upgrade.luxuryassembler": 1,
                  "unlock.upgrade.framer": 1,
                  "unlock.upgrade.heavyworks": 1,
                  "unlock.milestone.armory": 1,
                  "unlock.milestone.boosterplant": 1,
                  "unlock.milestone.rocketfactory": 1,
                  "unlock.milestone.Bullet": 1,
                  "unlock.c1.m19a": 1,
                  "unlock.c1.m19b": 1,
                  "unlock.c1.m18bonus": 1,
                  "unlock.BulletEngine": 1,
                  "unlock.upgrade.rocketfactory": 1
                }
              }
            ]
          ]
        ]
      }
    },
    "achievementHistoryScope": {
      "type": "StructProperty",
      "value": {
        "CustomStruct": [
          "MetaMetricStorageScope",
          [
            [
              "metrics",
              {
                "type": "MapProperty",
                "value_type": "StructProperty",
                "name_props": {
                  "Profit": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 737
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "export.rate": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "Energy": 0,
                                "Mainframes": 0
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "produce": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "Energy": 1043,
                                "CrudeOil": 1047
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "cycle": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "placeable.geothermal": 70
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "build.track": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 8341
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "build.branch": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 1848
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "build.train": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 1120
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "arrive.length": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 13
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "arrive.freight": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "Water": 2067,
                                "Steel": 2059
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "salvage.track": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 1128
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "salvage.branch": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 59
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "upgrade": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 268,
                                "placeable.waterpump": 32
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "Path": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "None": 61
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "mission": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "82A9B46F42BF6C5555D608A6619B35E6": 2
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  },
                  "maxupgrade": {
                    "type": "StructProperty",
                    "value": {
                      "CustomStruct": [
                        "this type hint is unused",
                        [
                          [
                            "valueByFilter",
                            {
                              "type": "MapProperty",
                              "name_ints": {
                                "city": 2
                              }
                            }
                          ]
                        ]
                      ]
                    }
                  }
                }
              }
            ]
          ]
        ]
      }
    }
  }
}"#;
