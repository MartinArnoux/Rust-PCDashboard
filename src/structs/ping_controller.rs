use std::time::Duration;

use circular_buffer::CircularBuffer;

use crate::structs::ping_settings::{PingSettings};


pub struct PingController {
    pings: CircularBuffer<10,Result<u32,PingControllerError>>,
    is_pinging: bool,
    receive: bool,
}

#[derive(Clone)]
pub enum PingControllerError {
    NoLastPing,
    ErrConnection,
}


impl PingController{
    pub fn new() -> PingController{
        PingController { pings: CircularBuffer::<10,Result<u32,PingControllerError>>::new(), is_pinging: false, receive: true }
    }

    pub fn get_last_ping(&self) -> Result<u32,PingControllerError>{
        let last_ping: Result<u32,PingControllerError> = self.pings.back()
        .cloned()
        .ok_or(PingControllerError::NoLastPing)?;

        match last_ping {
            Ok(v) => Ok(v),
            Err(e) => Err(e)
        }
    }

    pub fn had_receive(&self) -> bool {
        self.receive
    }
    
    pub fn start_stop_pinging(&mut self) {
        self.is_pinging = !self.is_pinging;
    }

    pub fn is_pinging(&self) -> bool {
        self.is_pinging
    }


    pub fn ping_once(&mut self,ping_settings:PingSettings) -> () {

        self.receive = false;
        let addr = ping_settings.get_address();
        let data = [1,2,3,4];  // ping data
        let timeout = Duration::from_secs(3);
        let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
        let result = ping_rs::send_ping(&addr, timeout, &data, Some(&options));
        match result {//println!("Reply from {}: bytes={} time={}ms TTL={}", reply.address, data.len(), reply.rtt, options.ttl)
            Ok(reply) => { self.pings.push_back(Ok(reply.rtt.clone())); },
            Err(e) =>  { println!("Erreur: {:?}",e);self.pings.push_back(Err(PingControllerError::ErrConnection)); }
        };
        self.receive = true;
    }

}