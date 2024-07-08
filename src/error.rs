#[derive(thiserror::Error, Debug)]
pub enum VenusError {}

pub type VenusResult<T, E = VenusError> = Result<T, E>;
