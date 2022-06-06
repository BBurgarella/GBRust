// All the operations related to bits


#[macro_export]
macro_rules! upper {
    ($self:tt.$regi: ident) => {
        (($self.$regi & 0xFF00)>>8) as u8
    }
}
#[macro_export]
macro_rules! lower {
    ($self:tt.$regi: ident) => {
        ($self.$regi & 0x00FF) as u8
    }
}   