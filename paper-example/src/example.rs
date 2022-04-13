use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use actix_telepathy::prelude::*;
use actix_rt;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;


#[derive(RemoteMessage, Serialize, Deserialize)]
struct MyMessage {}

#[derive(RemoteActor)]
#[remote_messages(MyMessage)]
struct MyActor {
    pub state: usize
}

impl Actor for MyActor {
type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.register(ctx.address().recipient());
        // optional subscription to receive cluster news
        self.subscribe_system_async::<ClusterLog>(ctx);
    }
}

impl Handler<MyMessage> for MyActor {
    type Result = ();

    fn handle(&mut self,
              _msg: MyMessage,
              _ctx: &mut Self::Context)
    -> Self::Result {
        println!("MyMessage received");
    }
}

// optional handler for receiving cluster news
impl Handler<ClusterLog> for MyActor {
    type Result = ();

    fn handle(&mut self,
              msg: ClusterLog,
              _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClusterLog::NewMember(_ip_addr,
                                  mut remote_addr) => {
                remote_addr.change_id(
                    Self::ACTOR_ID.to_string());
                remote_addr.do_send(MyMessage {});
            },
            ClusterLog::MemberLeft(_ip_addr) => ()
        }
    }
}
impl ClusterListener for MyActor {}

#[actix_rt::main]
pub async fn main(own_addr: SocketAddr,
                  seed_nodes: Vec<SocketAddr>) {
    let _local_addr = MyActor { state: 0 }.start();
    let _cluster = Cluster::new(own_addr, seed_nodes);
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}
