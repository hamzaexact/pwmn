use crate::{
    error::SessionErr,
    session::{self, SessionConn},
};

pub struct Disconnect<'a> {
    pub session: &'a mut SessionConn,
}

impl<'a> Disconnect<'a> {
    pub fn disconnect(&mut self) -> Result<(), SessionErr> {
        if !self.session.is_connected() {
            return Err(SessionErr::SessionNotConnected);
        }
        self.session.disconnect_from();
        println!("Session disconnected successfully");
        Ok(())
    }
}
