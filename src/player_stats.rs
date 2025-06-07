use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Stat {
    DamageResisted(i64),
    FishCaught(i64),
    AnimalsBred(i64),
    RaidWin(i64),
    InteractWithBrewingstand(i64),
    PlayRecord(i64),
    InteractWithLectern(i64),
    DamageAbsorbed(i64),
    InteractWithBlastFurnace(i64),
    TriggerTrappedChest(i64),
    WalkOneCm(i64),
    InteractWithCartographyTable(i64),
    OpenBarrel(i64),
    PotFlower(i64),
    BoatOneCm(i64),
    TimeSinceRest(i64),
    PigOneCm(i64),
    ClimbOneCm(i64),
    StriderOneCm(i64),
    CleanShulkerBox(i64),
    MinecartOneCm(i64),
    InspectDropper(i64),
    TargetHit(i64),
    PlayTime(i64),
    DamageBlockedByShield(i64),
    TotalWorldTime(i64),
    InspectHopper(i64),
    Jump(i64),
    InteractWithCraftingTable(i64),
    InteractWithSmithingTable(i64),
    InteractWithAnvil(i64),
    Deaths(i64),
    CrouchOneCm(i64),
    LeaveGame(i64),
    PlayNoteblock(i64),
    SleepInBed(i64),
    AviateOneCm(i64),
    OpenChest(i64),
    SneakTime(i64),
    InteractWithGrindstone(i64),
    OpenShulkerBox(i64),
    CleanArmor(i64),
    MobKills(i64),
    FillCauldron(i64),
    DamageDealt(i64),
    WalkUnderWaterOneCm(i64),
    EnchantItem(i64),
    PlayerKills(i64),
    OpenEnderchest(i64),
    FlyOneCm(i64),
    InteractWithStonecutter(i64),
    InteractWithSmoker(i64),
    InteractWithFurnace(i64),
    TuneNoteblock(i64),
    FallOneCm(i64),
    RaidTrigger(i64),
    SwimOneCm(i64),
    InspectDispenser(i64),
    DropCount(i64),
    DamageDealtAbsorbed(i64),
    EatCakeSlice(i64),
    TimeSinceDeath(i64),
    HorseOneCm(i64),
    TradedWithVillager(i64),
    DamageDealtResisted(i64),
    UseCauldron(i64),
    TalkedToVillager(i64),
    SprintOneCm(i64),
    InteractWithLoom(i64),
    InteractWithCampfire(i64),
    DamageTaken(i64),
    CleanBanner(i64),
    BellRing(i64),
    InteractWithBeacon(i64),
    WalkOnWaterOneCm(i64),
    Other(String, i64),
}

/// Wrapper for deserializing a map of player stats
#[derive(Debug, Deserialize)]
pub struct StatMap(
    #[serde(deserialize_with = "deserialize_stat_map")] pub Vec<Stat>,
);

macro_rules! stat_match {
    (
        $($key:literal => $variant:ident),* $(,)?
    ) => {
        fn deserialize_stat_map<'de, D>(deserializer: D) -> Result<Vec<Stat>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let map: BTreeMap<String, i64> = BTreeMap::deserialize(deserializer)?;
            Ok(map
                .into_iter()
                .map(|(k, v)| match k.as_str() {
                    $(
                        $key => Stat::$variant(v),
                    )*
                    _ => Stat::Other(k, v),
                })
                .collect())
        }

        impl From<StatMap> for BTreeMap<String, i64> {
            fn from(stat_map: StatMap) -> Self {
                let mut map = BTreeMap::new();
                for stat in stat_map.0 {
                    match stat {
                        $(
                            Stat::$variant(v) => {
                                map.insert($key.to_string(), v);
                            }
                        )*
                        Stat::Other(k, v) => {
                            map.insert(k, v);
                        }
                    }
                }
                map
            }
        }
    };
}

stat_match! {
    "damage_resisted" => DamageResisted,
    "fish_caught" => FishCaught,
    "animals_bred" => AnimalsBred,
    "raid_win" => RaidWin,
    "interact_with_brewingstand" => InteractWithBrewingstand,
    "play_record" => PlayRecord,
    "interact_with_lectern" => InteractWithLectern,
    "damage_absorbed" => DamageAbsorbed,
    "interact_with_blast_furnace" => InteractWithBlastFurnace,
    "trigger_trapped_chest" => TriggerTrappedChest,
    "walk_one_cm" => WalkOneCm,
    "interact_with_cartography_table" => InteractWithCartographyTable,
    "open_barrel" => OpenBarrel,
    "pot_flower" => PotFlower,
    "boat_one_cm" => BoatOneCm,
    "time_since_rest" => TimeSinceRest,
    "pig_one_cm" => PigOneCm,
    "climb_one_cm" => ClimbOneCm,
    "strider_one_cm" => StriderOneCm,
    "clean_shulker_box" => CleanShulkerBox,
    "minecart_one_cm" => MinecartOneCm,
    "inspect_dropper" => InspectDropper,
    "target_hit" => TargetHit,
    "play_time" => PlayTime,
    "damage_blocked_by_shield" => DamageBlockedByShield,
    "total_world_time" => TotalWorldTime,
    "inspect_hopper" => InspectHopper,
    "jump" => Jump,
    "interact_with_crafting_table" => InteractWithCraftingTable,
    "interact_with_smithing_table" => InteractWithSmithingTable,
    "interact_with_anvil" => InteractWithAnvil,
    "deaths" => Deaths,
    "crouch_one_cm" => CrouchOneCm,
    "leave_game" => LeaveGame,
    "play_noteblock" => PlayNoteblock,
    "sleep_in_bed" => SleepInBed,
    "aviate_one_cm" => AviateOneCm,
    "open_chest" => OpenChest,
    "sneak_time" => SneakTime,
    "interact_with_grindstone" => InteractWithGrindstone,
    "open_shulker_box" => OpenShulkerBox,
    "clean_armor" => CleanArmor,
    "mob_kills" => MobKills,
    "fill_cauldron" => FillCauldron,
    "damage_dealt" => DamageDealt,
    "walk_under_water_one_cm" => WalkUnderWaterOneCm,
    "enchant_item" => EnchantItem,
    "player_kills" => PlayerKills,
    "open_enderchest" => OpenEnderchest,
    "fly_one_cm" => FlyOneCm,
    "interact_with_stonecutter" => InteractWithStonecutter,
    "interact_with_smoker" => InteractWithSmoker,
    "interact_with_furnace" => InteractWithFurnace,
    "tune_noteblock" => TuneNoteblock,
    "fall_one_cm" => FallOneCm,
    "raid_trigger" => RaidTrigger,
    "swim_one_cm" => SwimOneCm,
    "inspect_dispenser" => InspectDispenser,
    "drop_count" => DropCount,
    "damage_dealt_absorbed" => DamageDealtAbsorbed,
    "eat_cake_slice" => EatCakeSlice,
    "time_since_death" => TimeSinceDeath,
    "horse_one_cm" => HorseOneCm,
    "traded_with_villager" => TradedWithVillager,
    "damage_dealt_resisted" => DamageDealtResisted,
    "use_cauldron" => UseCauldron,
    "talked_to_villager" => TalkedToVillager,
    "sprint_one_cm" => SprintOneCm,
    "interact_with_loom" => InteractWithLoom,
    "interact_with_campfire" => InteractWithCampfire,
    "damage_taken" => DamageTaken,
    "clean_banner" => CleanBanner,
    "bell_ring" => BellRing,
    "interact_with_beacon" => InteractWithBeacon,
    "walk_on_water_one_cm" => WalkOnWaterOneCm,
}
