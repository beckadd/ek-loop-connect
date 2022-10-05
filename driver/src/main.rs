use driver_types::{generate_request, EKLoopConnectDevice};
use hidapi::{HidApi, HidDevice, HidResult};

mod driver_types;

const EKWB_VENDOR_ID: u16 = 0x0483;
const EKWB_LOOP_CONNECT_PROD_ID: u16 = 0x5750;

fn main() {
    //TODO: need to try this on my computer @ home - I can't test this without having HID devices connected :)
    match HidApi::new() {
        Ok(api) => {

            //TEST:
            println!("Printing devices...");
            for device in api.device_list() {
                println!("device: {:04x}:{:04x}", device.vendor_id(), device.product_id());
            }
            println!("All devices shown.");

            connect(api);
        },
        Err(e) => {
            eprintln!("Error! : {}", e)
        }
    }
}

// Establishes connection with the EK Loop Connect Device.
// fn connect(api: HidApi) -> EKLoopConnectDevice {
//     match api.open(EKWB_VENDOR_ID, EKWB_LOOP_CONNECT_PROD_ID) {
//         Ok(device) => {
//         },
//         Err(e) => {}
//     }
// }