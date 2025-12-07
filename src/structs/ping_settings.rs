use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

#[derive(Clone,Copy)]
pub struct PingSettings {
    address: IpAddr,
    interval_ping: Duration,
}



impl PingSettings {
    pub fn new() -> Self {
        PingSettings {
            address: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            interval_ping: Duration::from_secs(1),
        }
    }

    

    pub fn get_address(&self) -> IpAddr {
        self.address
    }

    pub fn get_interval_ping(&self) -> Duration {
        self.interval_ping
    }


    // pub async fn ping_continuous(&mut self,) -> () {
    //     self.is_pinging = true;
    //     while self.is_pinging{
    //         match self.ping_once().await {
    //             Ok(rtt) => println!("Ping réussi: {}ms" , rtt),
    //             Err(e) => println!("Pring échoué: {}",e),
    //         }
    //         tokio::time::sleep(self.interval_ping).await;
    //     }
    // }
}