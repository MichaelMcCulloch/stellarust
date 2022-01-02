#[derive(Debug, PartialEq, Clone)]
pub struct EmpireData {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ModelDataPoint {
    pub empires: Vec<EmpireData>,
}
