use btleplug::api::{CharPropFlags,  Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use futures::stream::StreamExt;

use crate::utils::data_parse;

const NOTIFIY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x0000ffe1_0000_1000_8000_00805f9b34fb);

pub struct BluetoothManager {
    pub module: Option<Peripheral>,
    pub tx: Option<glib::Sender<f64>>,
}

impl BluetoothManager {
    pub async fn find_module(&self, central: &Adapter) -> Option<Peripheral> {
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
    
    pub async fn connect_module(&mut self) -> Result<(), Box<dyn Error>> {
        let manager = Manager::new().await.unwrap();
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().nth(0).unwrap();
    
        central.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(2)).await;
    
        self.module = Some(self.find_module(&central).await.expect("No module found"));
        self.module.as_ref().unwrap().connect().await?;
        self.module.as_ref().unwrap().discover_services().await?;

        Ok(())
    }
    
    pub async fn subscribe_to_temp(&self) -> Result<(), Box<dyn Error>> {
        let chars = self.module.as_ref().unwrap().characteristics();
    
        for char in chars {
            if char.uuid == NOTIFIY_CHARACTERISTIC_UUID
                && char.properties.contains(CharPropFlags::NOTIFY)
            {
                println!("Subscribing to characteristic: {:?}, {:?}", char.uuid, char);
                self.module.as_ref().unwrap().subscribe(&char).await?;
            }
        }
    
        Ok(())
    }
    
    pub async fn bluetooth_handler(&self) -> f64 {
        let mut notification_stream = 
        self.module.as_ref().unwrap().notifications().await.expect("No notifiers present");
    
        while let Some(data) = notification_stream.next().await {
            println!(
                "Recieved data from 'BLUETOOTHMODULE' [{:?}]",
                data.uuid
            );
    
            println!("{}", data_parse(&data.value));
            self.tx.as_ref().unwrap().send(data_parse(&data.value)).expect(
                "Could not send data over channel");
        }
    
        self.module.as_ref().unwrap().disconnect().await.expect("Error disconnecting...");
    
        0.0
    }

    pub async fn disconnect_bluetooth(&self) {
        self.module.as_ref().unwrap().disconnect().await.expect("Error disconnecting...");
    } 
}
