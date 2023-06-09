// use std::thread;
// use std::time::Duration;
// use actix::prelude::*;
//
// #[derive(Message)]
// #[rtype(result = "()")]
// struct OrderShipped(usize);
//
// #[derive(Message)]
// #[rtype(result = "()")]
// struct Ship(usize);
//
// /// Subscribe to order shipped event.
// #[derive(Message)]
// #[rtype(result = "()")]
// struct Subscribe(pub Recipient<OrderShipped>);
//
// /// Actor that provides order shipped event subscriptions
// struct OrderEvents {
//     subscribers: Vec<Recipient<OrderShipped>>,
// }
//
// impl OrderEvents {
//     fn new() -> Self {
//         OrderEvents {
//             subscribers: vec![]
//         }
//     }
// }
//
// impl Actor for OrderEvents {
//     type Context = Context<Self>;
// }
//
// impl OrderEvents {
//     /// Send event to all subscribers
//      fn notify(&mut self, order_id: usize) {
//         for subscr in &self.subscribers {
//             subscr.try_send(OrderShipped(order_id)).unwrap();
//         }
//     }
// }
//
// /// Subscribe to shipment event
// impl Handler<Subscribe> for OrderEvents {
//     type Result = ();
//
//     fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
//         self.subscribers.push(msg.0);
//     }
// }
//
// /// Subscribe to ship message
// impl Handler<Ship> for OrderEvents {
//     type Result = ();
//     fn handle(&mut self, msg: Ship, ctx: &mut Self::Context) -> Self::Result {
//         println!("OrderEvents  handling Ship");
//         self.notify(msg.0);
//     }
// }
//
// /// Email Subscriber
// struct EmailSubscriber;
// impl Actor for EmailSubscriber {
//     type Context = Context<Self>;
//
//     fn started(&mut self, _ctx: &mut Context<Self>) {
//         println!("EmailSubscriber  is alive");
//     }
//
//     fn stopped(&mut self, _ctx: &mut Context<Self>) {
//         println!("EmailSubscriber is stopped");
//     }
// }
//
//
//
// impl Handler<OrderShipped> for EmailSubscriber {
//     type Result = ();
//     fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
//         println!("Email sent for order {}", msg.0);
//     }
//
// }
// struct SmsSubscriber;
// impl Actor for SmsSubscriber {
//     type Context = Context<Self>;
//
//     fn started(&mut self, _ctx: &mut Context<Self>) {
//         println!("SmsSubscriber  is alive");
//     }
//
//     fn stopped(&mut self, _ctx: &mut Context<Self>) {
//         println!("SmsSubscriber is stopped");
//     }
// }
//
// impl Handler<OrderShipped> for SmsSubscriber {
//     type Result = ();
//     fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
//         println!("SMS sent for order {}", msg.0);
//     }
//
// }
//
// #[actix_rt::main]
// async fn main() {
//     let email_subscriber = Subscribe(EmailSubscriber{}.start().recipient());
//     let sms_subscriber = Subscribe(SmsSubscriber{}.start().recipient());
//     let order_event = OrderEvents::new().start();
//     let r1=order_event.send(email_subscriber).await;
//     let r2= order_event.send(sms_subscriber).await;
//     let r3= order_event.send(Ship(1)).await;
//     if r1.is_err() || r2.is_err() || r3.is_err(){
//         println!("actor fail!");
//     }
//
//
// }