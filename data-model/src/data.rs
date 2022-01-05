#[derive(Debug, PartialEq, Clone)]
pub struct Resources {
    pub energy: f64,
    pub minerals: f64,
    pub food: f64,
    pub physics_research: f64,
    pub society_research: f64,
    pub engineering_research: f64,
    pub influence: f64,
    pub unity: f64,
    pub consumer_goods: f64,
    pub alloys: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Income {}
#[derive(Debug, PartialEq, Clone)]
pub struct Expense {}
#[derive(Debug, PartialEq, Clone)]
pub struct Balance {}

#[derive(Debug, PartialEq, Clone)]
pub struct Budget {
    pub income: Income,
    pub expense: Expense,
    pub balance: Balance,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EmpireData {
    pub name: String,
    // pub budget: Budget,
    pub resources: Resources,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModelDataPoint {
    pub empires: Vec<EmpireData>,
}
