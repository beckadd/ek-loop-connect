/* Driver manages three device types:
- Fan: the fan(s) managed by the device.
- TempSensor (a type of sensor): the temperature sensor(s) of the device.
- CoolantSensor (a type of sensor): the coolant level sensor on the device.
- FlowSensor (a type of sensor): the flow rate sensor of the pump.
*/

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

    fan_channel: FanChannel
}

pub enum FanChannel {
    F1(u8, u8),
    F2(u8, u8),
    F3(u8, u8),
    F4(u8, u8), 
    F5(u8, u8),
    F6(u8, u8)
}

/*
* request size is 63 bytes (zero-padded)
*/
pub struct EKLoopConnectDevice {
    pub fans: [Fan; 6],
    pub temperature_sensors: [TempSensor; 3],
    pub flow_sensor: FlowSensor,
    pub coolant_sensor: CoolantSensor
}

impl EKLoopConnectDevice {


    fn __sensor_response_object(&self) {
    }

    fn read_sensors(&self) {} 
}

pub fn read_sensors() {
    let mut request: [u8; 63] = [0; 63];

    request[0..6].swap_with_slice(&mut [0x10, 0x12, 0x08, 0xaa, 0x01, 0x03]);
    request[6..8].swap_with_slice(&mut [0xa2, 0x20]); //sensor channel read req
    request[8..10].swap_with_slice(&mut [0x00, 0x20]);
    request[10..12].swap_with_slice(&mut [0x66, 0x67]); //sensor channel doesn't care

    // Todo: Send to the HID thing and then receive, parse response
    println!("{:?}", request)
}