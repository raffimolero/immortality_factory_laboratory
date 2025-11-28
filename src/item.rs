use Item::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Item {
    #[default]
    Empty = -1,
    RichAir = 0,
    ManaCrystal,
    LiquidCurse,
    ManaDust,
    SilicaPowder,
    ObsidianPlate,
    CopperCoin,
    SilverCoin,
    GoldCoin,
    TaintedWater,
    ChaosSalt,
    VialOfBlood,
    LifeForce,
    CongealedFleshmatter,
    SentientMeat,
    WeakManaGem,
    PureManaGem,
    GloomShard,
    BrightShard,
    EqualizedOrb,
    AdamantineBar,
    AstralSheet,
    ElementalChassis,
    EnergizedSpark,
    ImmaculateSoul,
    Phylactery,
}

impl Item {
    pub const ITEMS: &[Item] = &[
        RichAir,
        ManaCrystal,
        LiquidCurse,
        ManaDust,
        SilicaPowder,
        ObsidianPlate,
        CopperCoin,
        SilverCoin,
        GoldCoin,
        TaintedWater,
        ChaosSalt,
        VialOfBlood,
        LifeForce,
        CongealedFleshmatter,
        SentientMeat,
        WeakManaGem,
        PureManaGem,
        GloomShard,
        BrightShard,
        EqualizedOrb,
        AdamantineBar,
        AstralSheet,
        ElementalChassis,
        EnergizedSpark,
        ImmaculateSoul,
        Phylactery,
    ];
}

impl TryFrom<i8> for Item {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            ..-1 => Err(()),
            -1 => Ok(Empty),
            i => Self::ITEMS.get(i as usize).copied().ok_or(()),
        }
    }
}
