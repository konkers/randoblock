use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize, Serialize)]
pub struct Level {
    #[serde(rename = "Data")]
    pub data: LevelData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LevelData {
    #[serde(rename = "LastPlayed")]
    pub last_played: i64,

    #[serde(rename = "SpawnX")]
    pub spawn_x: i32,

    #[serde(rename = "SpawnY")]
    pub spawn_y: i32,

    #[serde(rename = "BorderWarningBlocks")]
    pub border_warning_blocks: f64,

    #[serde(rename = "WorldGenSettings")]
    pub world_gen_settings: WorldGenSettings,

    #[serde(rename = "CustomBossEvents")]
    pub custom_boss_events: HashMap<String, nbt::Value>,

    #[serde(rename = "BorderDamagePerBlock")]
    pub border_damage_per_block: f64,

    #[serde(rename = "DataVersion")]
    pub data_version: i64,

    #[serde(rename = "ScheduledEvents")]
    pub scheduled_events: Vec<nbt::Value>,

    #[serde(rename = "Difficulty")]
    pub difficulty: Difficulty,

    pub hardcore: bool,

    #[serde(rename = "WasModded")]
    pub was_modded: bool,

    #[serde(rename = "rainTime")]
    pub rain_time: i32,

    #[serde(rename = "GameType")]
    pub game_type: GameType,

    #[serde(rename = "WanderingTraderSpawnChance")]
    pub wandering_trader_spawn_chance: i32,

    pub raining: bool,

    #[serde(rename = "BorderSizeLerpTime")]
    pub border_size_lerp_time: i32,

    pub initialized: bool,

    #[serde(rename = "BorderSizeLerpTarget")]
    pub border_size_lerp_target: f64,

    #[serde(rename = "DragonFight")]
    pub dragon_fight: DragonFight,

    #[serde(rename = "allowCommands")]
    pub allow_commands: bool,

    #[serde(rename = "GameRules")]
    pub game_rules: HashMap<String, String>,

    #[serde(rename = "BorderSafeZone")]
    pub border_safe_zone: f64,

    #[serde(rename = "BorderCenterX")]
    pub border_center_x: f64,

    #[serde(rename = "DifficultyLocked")]
    pub difficulty_locked: bool,

    #[serde(rename = "LevelName")]
    pub level_name: String,

    #[serde(rename = "clearWeatherTime")]
    pub clear_weather_time: i32,

    #[serde(rename = "thunderTime")]
    pub thunder_time: i32,

    pub version: i32,

    #[serde(rename = "DataPacks")]
    pub data_packs: DataPacks,

    #[serde(rename = "Player")]
    pub player: Option<Player>,

    #[serde(rename = "Version")]
    pub minecraft_version: MinecraftVersion,

    #[serde(rename = "WanderingTraderSpawnDelay")]
    pub wandering_trader_spawn_delay: i32,

    #[serde(rename = "Time")]
    pub time: i64,

    #[serde(rename = "DayTime")]
    pub day_time: i64,

    #[serde(rename = "SpawnAngle")]
    pub spawn_angle: f32,

    #[serde(rename = "BorderCenterZ")]
    pub border_center_z: f64,

    pub thundering: bool,

    #[serde(rename = "ServerBrands")]
    pub server_brands: Vec<String>,

    #[serde(rename = "BorderWarningTime")]
    pub border_warning_time: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldGenSettings {
    pub bonus_chest: bool,
    pub dimensions: HashMap<String, DimensionGenSettings>,
    pub generate_features: bool,
    pub seed: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DimensionGenSettings {
    #[serde(rename = "type")]
    pub ty: String,

    pub generator: Generator,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Generator {
    #[serde(rename = "minecraft:flat")]
    Flat(FlatGenerator),
    #[serde(rename = "minecraft:noise")]
    Noise(NoiseGenerator),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlatGenerator {
    pub settings: FlatSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlatSettings {
    pub structures: HashMap<String, HashMap<String, nbt::Value>>,
    pub layers: Vec<LayerSetting>,
    pub features: i8,
    pub lakes: i8,
    pub biome: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayerSetting {
    pub block: String,
    pub height: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoiseGenerator {
    pub biome_source: BiomeSource,
    pub seed: i64,
    pub settings: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BiomeSource {
    #[serde(rename = "type")]
    pub ty: String,
    pub seed: Option<i64>,
    pub preset: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum GameType {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DragonFight {
    #[serde(rename = "PreviouslyKilled")]
    pub previously_killed: bool,

    #[serde(rename = "Gateways")]
    pub gateways: Vec<i32>,

    #[serde(rename = "NeedsStateScanning")]
    pub needs_state_scanning: bool,

    #[serde(rename = "DragonKilled")]
    pub dragon_killed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataPacks {
    #[serde(rename = "Disabled")]
    pub disabled: Vec<String>,

    #[serde(rename = "Enabled")]
    pub enabled: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "Invulnerable")]
    pub invulnerable: bool,

    #[serde(rename = "FallFlying")]
    pub fall_flying: bool,

    #[serde(rename = "OnGround")]
    pub on_ground: bool,

    #[serde(rename = "Fire")]
    pub fire: i16,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,

    #[serde(rename = "Score")]
    pub score: i32,

    #[serde(rename = "HurtByTimestamp")]
    pub hurt_by_timestamp: i32,

    pub abilities: Abilities,

    #[serde(rename = "Motion")]
    pub motion: Vec<f64>,

    #[serde(rename = "EnderItems")]
    pub ender_items: Vec<nbt::Value>,

    #[serde(rename = "Brain")]
    pub brain: Brain,

    #[serde(rename = "SleepTimer")]
    pub sleep_timer: i16,

    #[serde(rename = "XpTotal")]
    pub xp_total: i32,

    #[serde(rename = "Air")]
    pub air: i16,

    #[serde(rename = "SelectedItemSlot")]
    pub selected_item_slot: i32,

    #[serde(rename = "PortalCooldown")]
    pub portal_cooldown: i32,

    #[serde(rename = "Inventory")]
    pub inventory: Vec<nbt::Value>,

    #[serde(rename = "playerGameType")]
    pub player_game_type: i32,

    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,

    #[serde(rename = "XpP")]
    pub xp_precent: f32,

    #[serde(rename = "foodTickTimer")]
    pub food_tick_timer: i32,

    #[serde(rename = "Rotation")]
    pub rotation: Vec<f32>,

    #[serde(rename = "FallDistance")]
    pub fall_distance: f32,

    #[serde(rename = "XpLevel")]
    pub xp_level: i32,

    #[serde(rename = "Pos")]
    pub pos: Vec<f64>,

    #[serde(rename = "seenCredits")]
    pub seen_credits: bool,

    #[serde(rename = "UUID")]
    pub uuid: Vec<i32>,

    #[serde(rename = "foodExhaustionLevel")]
    pub food_exhaustion_level: f32,

    #[serde(rename = "foodSaturationLevel")]
    pub food_saturation_level: f32,

    #[serde(rename = "Dimension")]
    pub dimension: String,

    #[serde(rename = "foodLevel")]
    pub food_level: i32,

    #[serde(rename = "HurtTime")]
    pub hurt_time: i16,

    #[serde(rename = "recipeBook")]
    pub recipe_book: RecipeBook,

    #[serde(rename = "XpSeed")]
    pub xp_seed: i32,

    #[serde(rename = "Health")]
    pub health: f32,

    #[serde(rename = "DeathTime")]
    pub death_time: i16,

    #[serde(rename = "AbsorptionAmount")]
    pub absorption_amount: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Abilities {
    #[serde(rename = "flySpeed")]
    pub fly_speed: f32,

    pub instabuild: bool,
    pub invulnerable: bool,

    #[serde(rename = "mayfly")]
    pub may_fly: bool,

    #[serde(rename = "walkSpeed")]
    pub walk_speed: f32,

    #[serde(rename = "mayBuild")]
    pub may_build: bool,

    pub flying: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Brain {
    pub memories: HashMap<String, nbt::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attribute {
    #[serde(rename = "Base")]
    pub base: f64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Modifiers")]
    pub modifiers: Option<Vec<nbt::Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeBook {
    pub recipes: Vec<String>,

    #[serde(rename = "isFilteringCraftable")]
    pub is_filtering_craftable: bool,

    #[serde(rename = "isBlastingFurnaceFilteringCraftable")]
    pub is_blasting_furnace_filtering_craftable: bool,

    #[serde(rename = "toBeDisplayed")]
    pub to_be_displayed: Vec<String>,

    #[serde(rename = "isFurnaceFilteringCraftable")]
    pub is_furnace_filtering_craftable: bool,

    #[serde(rename = "isBlastingFurnaceGuiOpen")]
    pub is_blasting_furnace_gui_open: bool,

    #[serde(rename = "isSmokerFilteringCraftable")]
    pub is_smoker_filtering_craftable: bool,

    #[serde(rename = "isFurnaceGuiOpen")]
    pub is_furnace_gui_open: bool,

    #[serde(rename = "isSmokerGuiOpen")]
    pub is_smoker_gui_open: bool,

    #[serde(rename = "isGuiOpen")]
    pub is_gui_open: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinecraftVersion {
    #[serde(rename = "Snapshot")]
    pub snapshot: bool,

    #[serde(rename = "Series")]
    pub series: String,

    #[serde(rename = "Id")]
    pub id: i32,

    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChunkData {
    #[serde(rename = "Heightmaps")]
    pub height_maps: HashMap<String, Vec<i64>>,

    pub structures: Structures,

    pub block_entities: Vec<nbt::Value>,
    pub block_ticks: Vec<nbt::Value>,
    pub fluid_ticks: Vec<nbt::Value>,

    #[serde(rename = "PostProcessing")]
    pub post_processing: Vec<Vec<nbt::Value>>,

    pub sections: Vec<Section>,

    #[serde(rename = "DataVersion")]
    pub data_version: i32,

    #[serde(rename = "InhabitiedTime")]
    pub inhabitied_time: Option<i64>,

    #[serde(rename = "isLightOn")]
    pub is_light_on: Option<bool>,

    #[serde(rename = "LastUpdate")]
    pub last_update: i64,

    #[serde(rename = "xPos")]
    pub x_pos: i32,

    #[serde(rename = "yPos")]
    pub y_pos: i32,

    #[serde(rename = "zPos")]
    pub z_pos: i32,

    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Section {
    pub biomes: Biomes,
    pub block_states: Option<BlockStates>,

    #[serde(rename = "SkyLight")]
    pub sky_light: Option<Vec<i8>>,

    #[serde(rename = "Y")]
    pub y: i8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Biomes {
    pub palette: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockStates {
    pub palette: Vec<BlockType>,
    pub data: Option<Vec<i64>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BlockType {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(default, rename = "Properties")]
    pub properties: BTreeMap<String, String>,
}

impl BlockType {
    pub fn new(name: &str) -> BlockType {
        BlockType {
            name: name.into(),
            properties: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Structures {
    #[serde(rename = "References")]
    pub references: HashMap<String, Vec<i64>>,
    pub starts: HashMap<String, nbt::Value>,
}
