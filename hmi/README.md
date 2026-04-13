## Data Flow


## Drivers
### LCD Backlight
#### Resources:
- LCD EN pin
- LCD BL pin
- PWM slice

#### Commands:
- On/off
- Set brightness

### Touch Sensor
#### Resources:
- I2C Device
- Touch INT pin
- Touch RST pin

#### Commands:
- initialize
- reset
- read_product_id
- wait for interrupt
- read points

## Tasks
