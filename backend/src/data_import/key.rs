use stellarust::dto::{BudgetComponent, ResourceClass};

pub trait Key {
    fn key<'a>(&'a self) -> &'a str;
}

impl Key for ResourceClass {
    fn key<'a>(&'a self) -> &'a str {
        match self {
            ResourceClass::Energy => "energy",
            ResourceClass::Minerals => "minerals",
            ResourceClass::Food => "food",
            ResourceClass::Physics => "physics_research",
            ResourceClass::Society => "society_research",
            ResourceClass::Engineering => "engineering_research",
            ResourceClass::Influence => "influence",
            ResourceClass::Unity => "unity",
            ResourceClass::ConsumerGoods => "consumer_goods",
            ResourceClass::Alloys => "alloys",
            ResourceClass::Motes => "volatile_motes",
            ResourceClass::Gasses => "exotic_gases",
            ResourceClass::Crystals => "rare_crystals",
            ResourceClass::LivingMetal => "sr_living_metal",
            ResourceClass::Zro => "sr_zro",
            ResourceClass::DarkMatter => "sr_dark_matter",
        }
    }
}

impl Key for BudgetComponent {
    fn key<'a>(&'a self) -> &'a str {
        match self {
            BudgetComponent::Income => "income",
            BudgetComponent::Expenses => "expenses",
            BudgetComponent::Balance => "balance",
        }
    }
}
