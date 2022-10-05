/* Driver manages three device types:
- Fan: the fan(s) managed by the device.
- TempSensor (a type of sensor): the temperature sensor(s) of the device.
- CoolantSensor (a type of sensor): the coolant level sensor on the device.
- FlowSensor (a type of sensor): the flow rate sensor of the pump.
*/

use hidapi::HidDevice;

pub struct TempSensor {
    temp: i64
}
pub struct CoolantSensor {
    level_ok: bool
}

pub struct FlowSensor {
    flow: f64
}

pub struct Fan {
    rpm: f64,
    pwm: f64,

    fan_channel: [u8; 2]
}

pub struct EKLoopConnectDevice {
    hid_device: HidDevice,
    pub fans: [Fan; 6],
    pub temperature_sensors: [TempSensor; 3],
    pub flow_sensor: FlowSensor,
    pub coolant_sensor: CoolantSensor
}

impl EKLoopConnectDevice {
    fn sensor_response_object(&self) {
    }

    fn read_sensors(&self) {} 
}

pub fn initialize_EKLCDevice(hid_device: HidDevice) -> EKLoopConnectDevice {
    EKLoopConnectDevice { hid_device: hid_device, fans: (), temperature_sensors: (), flow_sensor: (), coolant_sensor: () }
}
pub fn read_sensors() {
    let request = generate_request([0xa2, 0x20]);

    // Todo: Send to the HID thing and then receive, parse response
}

pub fn read_fan(fan_channel: [u8; 2]) {
    let request =generate_request(fan_channel);
}

///TODO: This needs to be made a private function
pub fn generate_request(fan_channel: [u8; 2]) -> [u8; 64] {
    let hid_request: [u8; 64] = [0; 64]; // HIDAPI requires 0x00 at byte 1

    // section conforming to the EK spec (request size is 63 bytes (zero-padded))
    hid_request[1..][0..6].swap_with_slice(&mut [0x10, 0x12, 0x08, 0xaa, 0x01, 0x03]);
    hid_request[1..][6..8].swap_with_slice(&mut fan_channel);
    hid_request[1..][8..10].swap_with_slice(&mut [0x00, 0x20]);
    hid_request[1..][10..12].swap_with_slice(&mut [0x66, 0x67]); //sensor channel doesn't care

    hid_request
}