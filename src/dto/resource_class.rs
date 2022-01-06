use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone)]
pub enum ResourceClass {
    Energy,
    Minerals,
    Food,
    Physics,
    Society,
    Engineering,
    Influence,
    Unity,
    ConsumerGoods,
    Alloys,
    Motes,
    Gasses,
    Crystals,
    LivingMetal,
    Zro,
    DarkMatter,
}
