use btleplug::api::{CharPropFlags,  Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use futures::stream::StreamExt;
use glib::Sender;

use crate::utils::data_parse;

const NOTIFIY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x0000ffe1_0000_1000_8000_00805f9b34fb);

pub async fn find_module(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("BluetoothModule"))
            {
                return Some(p);
            }
    }
    None
}

pub async fn connect_module() -> Result<Peripheral, Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    central.start_scan(ScanFilter::default()).await?;
    time::sleep(Duration::from_secs(2)).await;

    let module = find_module(&central).await.expect("No module found");
    module.connect().await?;
    module.discover_services().await?;

    Ok(module)
}

pub async fn subscribe_to_temp(module: &Peripheral) -> Result<(), Box<dyn Error>> {
    let chars = module.characteristics();

    for char in chars {
        if char.uuid == NOTIFIY_CHARACTERISTIC_UUID
            && char.properties.contains(CharPropFlags::NOTIFY)
        {
            println!("Subscribing to characteristic: {:?}, {:?}", char.uuid, char);
            module.subscribe(&char).await?;
        }
    }

    Ok(())
}

pub async fn bluetooth_handler(module: Peripheral, tx: Sender<f64>) -> f64 {
    let _tx = tx.clone();

    let mut notification_stream = 
    module.notifications().await.expect("No notifiers present");

    while let Some(data) = notification_stream.next().await {
        println!(
            "Recieved data from 'BLUETOOTHMODULE' [{:?}]",
            data.uuid
        );

        println!("{}", data_parse(&data.value));
        tx.send(data_parse(&data.value)).expect(
            "Could not send data over channel");
    }

    module.disconnect().await.expect("Error disconnecting...");

    0.0
}

pub async fn disconnect_bluetooth(module: &Peripheral) {
    module.disconnect().await.expect("Error disconnecting...");
}