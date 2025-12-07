//use std::{io};
use iced::{Task};
//use wfp::{ActionType, FilterBuilder, FilterEngineBuilder, Layer, SubLayerBuilder, Transaction,PortConditionBuilder };


mod structs;

use crate::structs::{app::App};

fn main() -> iced::Result {
    

    iced::application("Ping Example", App::update, App::view)
        .subscription(App::subscription).run_with(|| (App::new(), Task::none()))
}

    /*println!("Creating WFP filter engine...");

    let mut engine = FilterEngineBuilder::default().dynamic().open()?;

    std::thread::spawn(move || {
        println!("Starting transaction...");
        let transaction = Transaction::new(&mut engine)?;

        // Create a custom sublayer for organizing our filters
        println!("Adding custom sublayer...");
        SubLayerBuilder::default()
            .name("Example SubLayer")
            .description("Custom sublayer for example filters")
            .weight(100)
            .add(&transaction)?;

        // Create a blocking filter for IPv4 outbound connections
        println!("Adding blocking filter...");
        FilterBuilder::default()
            .name("Example Block Filter")
            .description("Blocks all outbound IPv4 connections on port 80")
            .action(ActionType::Block)
            .layer(Layer::ConnectV4)
            .condition(
                PortConditionBuilder::remote()
                    .equal(80)
                    .build(),
            )
            .add(&transaction)?;

        println!("Committing transaction...");
        transaction.commit()?;

        println!("Filter successfully added!");
        Ok::<(), io::Error>(())
    })
    .join()
    .unwrap()?;

    println!("Example completed successfully!");
    Ok(())*/

// loop {
//         match ping_once("8.8.8.8").await {
//             Ok(latency) => println!("Ping OK : {:?}", latency),
//             Err(err) => println!("Ping FAIL : {}", err),
//         }

//         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//     }