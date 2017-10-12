
use na::{ Vector3, Vector2 };

use buff::{ Buff };
use ability::{ Ability };

pub struct Tag {
}

pub enum Terran {
    // Terran
    Armory,                 // CANCEL, HALT, CANCEL_LAST, RESEARCH_TERRANSHIPWEAPONS, RESEARCH_TERRANVEHICLEANDSHIPPLATING, RESEARCH_TERRANVEHICLEWEAPONS
    AutoTurret,             // SMART, STOP, ATTACK
    Banshee,                // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK, BEHAVIOR_CLOAKON, BEHAVIOR_CLOAKOFF
    Barracks,               // SMART, TRAIN_MARINE, TRAIN_REAPER, TRAIN_GHOST, TRAIN_MARAUDER, CANCEL, HALT, CANCEL_LAST, RALLY_UNITS, LIFT, BUILD_TECHLAB, BUILD_REACTOR
    BarracksFlying,         // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND, BUILD_TECHLAB, BUILD_REACTOR
    BarracksReactor,        // CANCEL
    BarracksTechLab,        // RESEARCH_STIMPACK, RESEARCH_COMBATSHIELD, RESEARCH_CONCUSSIVESHELLS, CANCEL, CANCEL_LAST
    BattleCruiser,          // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_YAMATOGUN, EFFECT_TACTICALJUMP, STOP, ATTACK
    Bunker,                 // SMART, EFFECT_SALVAGE, CANCEL, HALT, UNLOADALL, STOP, LOAD, RALLY_UNITS, ATTACK, EFFECT_STIM
    CommandCenter,          // SMART, TRAIN_SCV, MORPH_PLANETARYFORTRESS, MORPH_ORBITALCOMMAND, CANCEL, HALT, LOADALL, UNLOADALL, CANCEL_LAST, LIFT, RALLY_WORKERS
    CommandCenterFlying,    // SMART, MOVE, PATROL, HOLDPOSITION, LOADALL, UNLOADALL, STOP, LAND
    Cyclone,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_LOCKON, CANCEL, STOP, ATTACK
    EngineeringBay,         // RESEARCH_HISECAUTOTRACKING, RESEARCH_TERRANSTRUCTUREARMORUPGRADE, RESEARCH_NEOSTEELFRAME, CANCEL, HALT, CANCEL_LAST, RESEARCH_TERRANINFANTRYARMOR, RESEARCH_TERRANINFANTRYWEAPONS
    Factory,                // SMART, TRAIN_SIEGETANK, TRAIN_THOR, TRAIN_HELLION, TRAIN_HELLBAT, TRAIN_CYCLONE, TRAIN_WIDOWMINE, CANCEL, HALT, CANCEL_LAST, RALLY_UNITS, LIFT, BUILD_TECHLAB, BUILD_REACTOR
    FactoryFlying,          // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND, BUILD_TECHLAB, BUILD_REACTOR
    FactoryReactor,         // CANCEL
    FactoryTechLab,         // RESEARCH_INFERNALPREIGNITER, RESEARCH_DRILLINGCLAWS, RESEARCH_MAGFIELDLAUNCHERS, CANCEL, CANCEL_LAST
    FusionCore,             // RESEARCH_BATTLECRUISERWEAPONREFIT, CANCEL, HALT, CANCEL_LAST
    Ghost,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_NUKECALLDOWN, EFFECT_EMP, EFFECT_GHOSTSNIPE, CANCEL, STOP, ATTACK, BEHAVIOR_CLOAKON, BEHAVIOR_CLOAKOFF, BEHAVIOR_HOLDFIREON, BEHAVIOR_HOLDFIREOFF
    GhostAcademy,           // BUILD_NUKE, RESEARCH_PERSONALCLOAKING, CANCEL, HALT, CANCEL_LAST
    Hellion,                // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_HELLBAT, STOP, ATTACK
    HellionTank,            // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_HELLION, STOP, ATTACK
    Liberator,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_LIBERATORAGMODE, STOP, ATTACK
    LiberatorAg,            // SMART, MORPH_LIBERATORAAMODE, STOP, ATTACK
    Marauder,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK, EFFECT_STIM
    Marine,                 // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK, EFFECT_STIM
    Medivac,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_HEAL, EFFECT_MEDIVACIGNITEAFTERBURNERS, STOP, LOAD, UNLOADALLAT, ATTACK
    MissileTurret,          // SMART, CANCEL, HALT, STOP, ATTACK
    Mule,                   // SMART, MOVE, PATROL, HOLDPOSITION, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_REPAIR
    OrbitalCommand,         // SMART, EFFECT_CALLDOWNMULE, EFFECT_SUPPLYDROP, EFFECT_SCAN, TRAIN_SCV, CANCEL_LAST, LIFT, RALLY_WORKERS
    OrbitalCommandFlying,   // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND
    PlanetaryFortress,      // SMART, TRAIN_SCV, LOADALL, STOP, CANCEL_LAST, ATTACK, RALLY_WORKERS
    Raven,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_POINTDEFENSEDRONE, EFFECT_HUNTERSEEKERMISSILE, EFFECT_AUTOTURRET, STOP, ATTACK
    Reaper,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_KD8CHARGE, STOP, ATTACK
    Refinery,               // CANCEL, HALT
    Scv,                    // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_COMMANDCENTER, BUILD_SUPPLYDEPOT, BUILD_REFINERY, BUILD_BARRACKS, BUILD_ENGINEERINGBAY, BUILD_MISSILETURRET, BUILD_BUNKER, BUILD_SENSORTOWER, BUILD_GHOSTACADEMY, BUILD_FACTORY, BUILD_STARPORT, BUILD_ARMORY, BUILD_FUSIONCORE, HALT, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_SPRAY, EFFECT_REPAIR
    SensorTower,            // CANCEL, HALT
    SiegeTank,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_SIEGEMODE, STOP, ATTACK
    SiegeTankSieged,        // SMART, MORPH_UNSIEGE, STOP, ATTACK
    Starport,               // SMART, TRAIN_MEDIVAC, TRAIN_BANSHEE, TRAIN_RAVEN, TRAIN_BATTLECRUISER, TRAIN_VIKINGFIGHTER, TRAIN_LIBERATOR, CANCEL, HALT, CANCEL_LAST, RALLY_UNITS, LIFT, BUILD_TECHLAB, BUILD_REACTOR
    StarportFlying,         // SMART, MOVE, PATROL, HOLDPOSITION, STOP, LAND, BUILD_TECHLAB, BUILD_REACTOR
    StarportReactor,        // CANCEL
    StarportTechLab,        // RESEARCH_BANSHEECLOAKINGFIELD, RESEARCH_RAVENCORVIDREACTOR, RESEARCH_BANSHEEHYPERFLIGHTROTORS, RESEARCH_RAVENRECALIBRATEDEXPLOSIVES, RESEARCH_HIGHCAPACITYFUELTANKS, RESEARCH_ADVANCEDBALLISTICS, CANCEL, CANCEL_LAST
    SupplyDepot,            // MORPH_SUPPLYDEPOT_LOWER, CANCEL, HALT
    SupplyDepotLowered,     // MORPH_SUPPLYDEPOT_RAISE
    Thor,                   // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_THORHIGHIMPACTMODE, STOP, ATTACK
    ThorAp,                 // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_THOREXPLOSIVEMODE, CANCEL, STOP, ATTACK
    VikingAssault,          // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_VIKINGFIGHTERMODE, STOP, ATTACK
    VikingFighter,          // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_VIKINGASSAULTMODE, STOP, ATTACK
    WidowMine,              // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    WidowMineBurrowed,      // SMART, EFFECT_WIDOWMINEATTACK, BURROWUP

    // Terran non-interactive
    Kd8Charge,
    Nuke,
    PointDefenseDrone,
    Reactor,
    TechLab,
}

pub enum Zerg {
    // Zerg
    Baneling,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_EXPLODE, BEHAVIOR_BUILDINGATTACKON, BEHAVIOR_BUILDINGATTACKOFF, BURROWDOWN, STOP, ATTACK
    BanelingBurrowed,       // EFFECT_EXPLODE, BURROWUP
    BanelingCocoon,         // SMART, CANCEL_LAST, RALLY_UNITS
    BanelingNest,           // RESEARCH_CENTRIFUGALHOOKS, CANCEL, CANCEL_LAST
    Broodling,              // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    Broodlord,              // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    BroodlordCocoon,        // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL
    Changeling,             // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ChangelingMarine,       // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ChangelingMarineShield, // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ChangelingZealot,       // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ChangelingZergling,     // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    ChangelingZerglingWings,// SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    Corruptor,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_BROODLORD, EFFECT_CAUSTICSPRAY, STOP, ATTACK
    CreepTumor,             // CANCEL
    CreepTumorBurrowed,     // SMART, CANCEL, BUILD_CREEPTUMOR
    CreepTumorQueen,        // CANCEL
    Drone,                  // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_HATCHERY, BUILD_EXTRACTOR, BUILD_SPAWNINGPOOL, BUILD_EVOLUTIONCHAMBER, BUILD_HYDRALISKDEN, BUILD_SPIRE, BUILD_ULTRALISKCAVERN, BUILD_INFESTATIONPIT, BUILD_NYDUSNETWORK, BUILD_BANELINGNEST, BUILD_ROACHWARREN, BUILD_SPINECRAWLER, BUILD_SPORECRAWLER, BURROWDOWN, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_SPRAY
    DroneBurrowed,          // BURROWUP
    Egg,                    // SMART, CANCEL_LAST, RALLY_UNITS
    EvolutionChamber,       // CANCEL, CANCEL_LAST, RESEARCH_ZERGGROUNDARMOR, RESEARCH_ZERGMELEEWEAPONS, RESEARCH_ZERGMISSILEWEAPONS
    Extractor,              // CANCEL
    GreaterSpire,           // CANCEL_LAST, RESEARCH_ZERGFLYERARMOR, RESEARCH_ZERGFLYERATTACK
    Hatchery,               // SMART, MORPH_LAIR, RESEARCH_PNEUMATIZEDCARAPACE, RESEARCH_BURROW, TRAIN_QUEEN, CANCEL, CANCEL_LAST, RALLY_UNITS, RALLY_WORKERS
    Hive,                   // SMART, RESEARCH_PNEUMATIZEDCARAPACE, RESEARCH_BURROW, TRAIN_QUEEN, CANCEL_LAST, RALLY_UNITS, RALLY_WORKERS
    Hydralisk,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_LURKER, BURROWDOWN, STOP, ATTACK
    HydraliskBurrowed,      // BURROWUP
    HydraliskDen,           // RESEARCH_GROOVEDSPINES, RESEARCH_MUSCULARAUGMENTS, MORPH_LURKERDEN, CANCEL, CANCEL_LAST
    InfestationPit,         // RESEARCH_PATHOGENGLANDS, RESEARCH_NEURALPARASITE, CANCEL, CANCEL_LAST
    InfestedTerransEgg,     // SMART, MOVE, PATROL, HOLDPOSITION
    Infestor,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_FUNGALGROWTH, EFFECT_INFESTEDTERRANS, EFFECT_NEURALPARASITE, CANCEL, BURROWDOWN, STOP, ATTACK
    InfestorBurrowed,       // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_FUNGALGROWTH, EFFECT_INFESTEDTERRANS, EFFECT_NEURALPARASITE, CANCEL, BURROWUP, STOP, ATTACK
    InfestorTerran,         // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    Lair,                   // SMART, MORPH_HIVE, RESEARCH_PNEUMATIZEDCARAPACE, RESEARCH_BURROW, TRAIN_QUEEN, CANCEL, CANCEL_LAST, RALLY_UNITS, RALLY_WORKERS
    Larva,                  // TRAIN_DRONE, TRAIN_ZERGLING, TRAIN_OVERLORD, TRAIN_HYDRALISK, TRAIN_MUTALISK, TRAIN_ULTRALISK, TRAIN_ROACH, TRAIN_INFESTOR, TRAIN_CORRUPTOR, TRAIN_VIPER, TRAIN_SWARMHOST
    LocustMp,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    LocustMpFlying,         // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_LOCUSTSWOOP, STOP, ATTACK
    LurkerDenMp,            // RESEARCH_GROOVEDSPINES, RESEARCH_MUSCULARAUGMENTS, CANCEL_LAST
    LurkerMp,               // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    LurkerMpBurrowed,       // SMART, BURROWUP, STOP, ATTACK, BEHAVIOR_HOLDFIREON, BEHAVIOR_HOLDFIREOFF
    LurkerMpEgg,            // SMART, CANCEL, RALLY_UNITS
    Mutalisk,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    NydusCanal,             // SMART, UNLOADALL, STOP, LOAD, RALLY_UNITS
    NydusNetwork,           // SMART, BUILD_NYDUSWORM, CANCEL, UNLOADALL, STOP, LOAD, RALLY_UNITS
    Overlord,               // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_OVERSEER, BEHAVIOR_GENERATECREEPON, BEHAVIOR_GENERATECREEPOFF, MORPH_OVERLORDTRANSPORT, CANCEL, STOP, ATTACK
    OverlordCocoon,         // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL
    OverlordTransport,      // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_OVERSEER, BEHAVIOR_GENERATECREEPON, BEHAVIOR_GENERATECREEPOFF, STOP, LOAD, UNLOADALLAT, ATTACK
    Overseer,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_SPAWNCHANGELING, EFFECT_CONTAMINATE, STOP, ATTACK
    Queen,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_INJECTLARVA, EFFECT_TRANSFUSION, BURROWDOWN, STOP, ATTACK, BUILD_CREEPTUMOR
    QueenBurrowed,          // BURROWUP
    Ravager,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_CORROSIVEBILE, BURROWDOWN, STOP, ATTACK
    RavagerCocoon,          // SMART, CANCEL, RALLY_UNITS
    Roach,                  // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_RAVAGER, BURROWDOWN, STOP, ATTACK
    RoachBurrowed,          // SMART, MOVE, PATROL, HOLDPOSITION, BURROWUP, STOP, ATTACK
    RoachWarren,            // RESEARCH_GLIALREGENERATION, RESEARCH_TUNNELINGCLAWS, CANCEL, CANCEL_LAST
    SpawningPool,           // RESEARCH_ZERGLINGADRENALGLANDS, RESEARCH_ZERGLINGMETABOLICBOOST, CANCEL, CANCEL_LAST
    SpineCrawler,           // SMART, CANCEL, STOP, ATTACK, MORPH_UPROOT
    SpineCrawlerUprooted,   // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL, STOP, ATTACK, MORPH_ROOT
    Spire,                  // MORPH_GREATERSPIRE, CANCEL, CANCEL_LAST, RESEARCH_ZERGFLYERARMOR, RESEARCH_ZERGFLYERATTACK
    SporeCrawler,           // SMART, CANCEL, STOP, ATTACK, MORPH_UPROOT
    SporeCrawlerUprooted,   // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL, STOP, ATTACK, MORPH_ROOT
    SwarmHostBurrowedMp,    // SMART, EFFECT_SPAWNLOCUSTS, BURROWUP
    SwarmHostMp,            // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_SPAWNLOCUSTS, BURROWDOWN, STOP, ATTACK
    TransportOverlordCocoon,// SMART, MOVE, PATROL, HOLDPOSITION, CANCEL
    Ultralisk,              // SMART, MOVE, PATROL, HOLDPOSITION, BURROWDOWN, STOP, ATTACK
    UltraliskCavern,        // RESEARCH_CHITINOUSPLATING, CANCEL, CANCEL_LAST
    Viper,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_BLINDINGCLOUD, EFFECT_ABDUCT, EFFECT_VIPERCONSUME, EFFECT_PARASITICBOMB, STOP, ATTACK
    Zergling,               // SMART, MOVE, PATROL, HOLDPOSITION, TRAIN_BANELING, BURROWDOWN, STOP, ATTACK
    ZerglingBurrowed,       // BURROWUP

    // Zerg non-interactive
    ParasiticBombDummy,
}

pub enum Protoss {
    // Protoss
    Adept,                  // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_ADEPTPHASESHIFT, CANCEL, STOP, RALLY_UNITS, ATTACK
    AdeptPhaseShift,        // SMART, MOVE, PATROL, HOLDPOSITION, CANCEL, STOP, ATTACK
    Archon,                 // SMART, MOVE, PATROL, HOLDPOSITION, STOP, RALLY_UNITS, ATTACK
    Assimilator,            // CANCEL
    Carrier,                // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_INTERCEPTORS, STOP, CANCEL_LAST, ATTACK
    Colossus,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    CyberneticScore,        // RESEARCH_WARPGATE, CANCEL, CANCEL_LAST, RESEARCH_PROTOSSAIRARMOR, RESEARCH_PROTOSSAIRWEAPONS
    DarkShrine,             // RESEARCH_SHADOWSTRIKE, CANCEL, CANCEL_LAST
    DarkTemplar,            // SMART, MOVE, PATROL, HOLDPOSITION, STOP, RALLY_UNITS, ATTACK, EFFECT_BLINK
    Disruptor,              // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_PURIFICATIONNOVA, STOP, ATTACK
    DisruptorPhased,        // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    FleetBeacon,            // RESEARCH_INTERCEPTORGRAVITONCATAPULT, RESEARCH_PHOENIXANIONPULSECRYSTALS, CANCEL, CANCEL_LAST
    Forge,                  // CANCEL, CANCEL_LAST, RESEARCH_PROTOSSGROUNDARMOR, RESEARCH_PROTOSSGROUNDWEAPONS, RESEARCH_PROTOSSSHIELDS
    Gateway,                // SMART, TRAIN_ZEALOT, TRAIN_STALKER, TRAIN_HIGHTEMPLAR, TRAIN_DARKTEMPLAR, TRAIN_SENTRY, TRAIN_ADEPT, MORPH_WARPGATE, CANCEL, CANCEL_LAST, RALLY_UNITS
    HighTemplar,            // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_FEEDBACK, EFFECT_PSISTORM, STOP, RALLY_UNITS, ATTACK
    Immortal,               // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_IMMORTALBARRIER, STOP, ATTACK
    Interceptor,            // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    Mothership,             // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_PHOTONOVERCHARGE, EFFECT_TIMEWARP, STOP, ATTACK, EFFECT_MASSRECALL
    MothershipCore,         // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_MOTHERSHIP, EFFECT_PHOTONOVERCHARGE, EFFECT_TIMEWARP, CANCEL, STOP, ATTACK, EFFECT_MASSRECALL
    Nexus,                  // SMART, EFFECT_CHRONOBOOST, TRAIN_PROBE, TRAIN_MOTHERSHIPCORE, CANCEL, CANCEL_LAST, RALLY_WORKERS
    Observer,               // SMART, MOVE, PATROL, HOLDPOSITION, STOP, ATTACK
    Oracle,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_ORACLEREVELATION, BEHAVIOR_PULSARBEAMON, BEHAVIOR_PULSARBEAMOFF, BUILD_STASISTRAP, CANCEL, STOP, ATTACK
    OracleStasisTrap,       // CANCEL
    Phoenix,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_GRAVITONBEAM, CANCEL, STOP, ATTACK
    PhotonCannon,           // SMART, CANCEL, STOP, ATTACK
    Probe,                  // SMART, MOVE, PATROL, HOLDPOSITION, BUILD_NEXUS, BUILD_PYLON, BUILD_ASSIMILATOR, BUILD_GATEWAY, BUILD_FORGE, BUILD_FLEETBEACON, BUILD_TWILIGHTCOUNCIL, BUILD_PHOTONCANNON, BUILD_STARGATE, BUILD_TEMPLARARCHIVE, BUILD_DARKSHRINE, BUILD_ROBOTICSBAY, BUILD_ROBOTICSFACILITY, BUILD_CYBERNETICSCORE, STOP, HARVEST_GATHER, HARVEST_RETURN, ATTACK, EFFECT_SPRAY
    Pylon,                  // CANCEL
    PylonOvercharged,       // SMART, STOP, ATTACK
    RoboticsBay,            // RESEARCH_GRAVITICBOOSTER, RESEARCH_GRAVITICDRIVE, RESEARCH_EXTENDEDTHERMALLANCE, CANCEL, CANCEL_LAST
    RoboticsFacility,       // SMART, TRAIN_WARPPRISM, TRAIN_OBSERVER, TRAIN_COLOSSUS, TRAIN_IMMORTAL, TRAIN_DISRUPTOR, CANCEL, CANCEL_LAST, RALLY_UNITS
    Sentry,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_GUARDIANSHIELD, HALLUCINATION_ARCHON, HALLUCINATION_COLOSSUS, HALLUCINATION_HIGHTEMPLAR, HALLUCINATION_IMMORTAL, HALLUCINATION_PHOENIX, HALLUCINATION_PROBE, HALLUCINATION_STALKER, HALLUCINATION_VOIDRAY, HALLUCINATION_WARPPRISM, HALLUCINATION_ZEALOT, EFFECT_FORCEFIELD, HALLUCINATION_ORACLE, HALLUCINATION_DISRUPTOR, HALLUCINATION_ADEPT, STOP, RALLY_UNITS, ATTACK
    Stalker,                // SMART, MOVE, PATROL, HOLDPOSITION, STOP, RALLY_UNITS, ATTACK, EFFECT_BLINK
    Stargate,               // SMART, TRAIN_PHOENIX, TRAIN_CARRIER, TRAIN_VOIDRAY, TRAIN_ORACLE, TRAIN_TEMPEST, CANCEL, CANCEL_LAST, RALLY_UNITS
    Tempest,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_TEMPESTDISRUPTIONBLAST, CANCEL, STOP, ATTACK
    TemplarArchive,         // RESEARCH_PSISTORM, CANCEL, CANCEL_LAST
    TwilightCouncil,        // RESEARCH_CHARGE, RESEARCH_BLINK, RESEARCH_ADEPTRESONATINGGLAIVES, CANCEL, CANCEL_LAST
    VoidRay,                // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_VOIDRAYPRISMATICALIGNMENT, STOP, ATTACK
    WarpGate,               // SMART, TRAINWARP_ZEALOT, TRAINWARP_STALKER, TRAINWARP_HIGHTEMPLAR, TRAINWARP_DARKTEMPLAR, TRAINWARP_SENTRY, TRAINWARP_ADEPT, MORPH_GATEWAY
    WarpPrism,              // SMART, MOVE, PATROL, HOLDPOSITION, MORPH_WARPPRISMPHASINGMODE, STOP, LOAD, UNLOADALLAT, ATTACK
    WarpPrismPhasing,       // SMART, MORPH_WARPPRISMTRANSPORTMODE, STOP, LOAD, UNLOADALLAT
    Zealot,                 // SMART, MOVE, PATROL, HOLDPOSITION, EFFECT_CHARGE, STOP, RALLY_UNITS, ATTACK
}

pub enum Neutral {
    // Neutral
    CollapsibleRockTowerDebris,
    CollapsibleRockTowerDiagonal,
    CollapsibleRockTowerPushUnit,
    CollapsibleTerranTowerDebris,
    CollapsibleTerranTowerDiagonal,
    CollapsibleTerranTowerPushUnit,
    CollapsibleTerranTowerPushUnitRampLeft,
    CollapsibleTerranTowerPushUnitRampRight,
    CollapsibleTerranTowerRampLeft,
    CollapsibleTerranTowerRampRight,
    DebrisRampLeft,
    DebrisRampRight,
    DestructibleDebris6x6,
    DestructibleDebrisRampDiagonalHugeBlur,
    DestructableRock6x6,
    DestructibleRockEx1DiagonalHugeBlur,
    ForceField,
    KarakFemale,
    LabMineralField,
    LabMineralField750,
    MineralField,
    MineralField750,
    ProtossVespeneGeyser,
    RichMineralField,
    RichMineralField750,
    Scantipede,
    SpacePlatformGeyser,
    UnbuildableBricksDestructible,
    UnbuildablePlatesDestructible,
    UtilityBot,
    VespeneGeyser,
    XelNagaTower,
}

pub enum UnitType {
    Invalid,

    Terran(Terran),
    Zerg(Zerg),
    Protoss(Protoss),
    Neutral(Neutral),
}

pub enum DisplayType {
    Visible,
    Snapshot,
    Hidden,
}

pub enum Alliance {
    Domestic,
    Ally,
    Neutral,
    Enemy,
}

pub enum CloakState {
    Cloaked,
    CloakedDetected,
    NotCloaked,
    Unknown,
}

pub struct UnitOrder {
    ability:                Ability,
    target_unit_tag:        Tag,
    target_pos:             Vector2<f32>,
}

pub struct PassengerUnit {
    tag:                    Tag,
    health:                 f32,
    health_max:             f32,
    shield:                 f32,
    energy:                 f32,
    unit_type:              UnitType,
}

pub struct Unit {
    display_type:           DisplayType,
    alliance:               Alliance,

    tag:                    Tag,
    unit_type:              UnitType,
    owner:                  u32,

    pos:                    Vector3<f32>,
    facing:                 f32,
    radius:                 f32,
    build_progress:         f32,

    cloak:                  CloakState,

    detect_range:           f32,
    radar_range:            f32,

    is_selected:            bool,
    is_on_screen:           bool,
    is_blip:                bool,

    health:                 f32,
    health_max:             f32,
    shield:                 f32,
    energy:                 f32,
    mineral_contents:       u32,
    vespene_contents:       u32,
    is_flying:              bool,
    is_burrowed:            bool,
    weapon_cooldown:        f32,

    orders:                 Vec<UnitOrder>,
    add_on_tag:             Tag,
    passengers:             Vec<PassengerUnit>,
    cargo_space_taken:      u32,
    cargo_space_max:        u32,
    assigned_harvesters:    u32,
    ideal_harvesters:       u32,
    engaged_target_tag:     Tag,
    buffs:                  Vec<Buff>,
    is_powered:             bool,

    is_alive:               bool,
    last_seen_game_loop:    u32,
}
