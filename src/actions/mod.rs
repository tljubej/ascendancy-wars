use world::Id;

pub enum Actions {
    Attack(Id),
    PledgeTo(Id),
    DemandFealtyFrom(Id),
    DoNothing,
}
