#![no_std]
use embedded_hal as hal;
use hal::blocking::{
    delay::{DelayMs},
};

pub trait Interface {
    fn write_command(&self, command: u16);
    fn write_data(&self, data: u16);
    fn read_data(&self, data: &mut u16);
    fn reset(&self);
}

#[derive(Copy, Clone, Default)]
pub struct DisplayIdentification {
    raw: u16,
}

#[derive(Copy, Clone)]
pub struct Controller<T>
    where T: Interface
{
    iface: T,
    width: u16,
    height: u16,
}

impl<T: Interface> Controller<T> 
    where T: Interface
{
    pub fn new(iface: T, width: u16, height: u16) -> Self {
        Controller {
            iface: iface,
            width: width,
            height: height,
        }
    }

    fn write_command(&self, command: u16) {
        self.iface.write_command(command);
    }

    fn write_data(&self, data: u16) {
        self.iface.write_data(data);
    }

    fn write_cmd_data(&self, command: u16, data: u16) {
        self.write_command(command);
        self.write_data(data);
    }

    fn read_data(&self, data: &mut u16) {
        self.iface.read_data(data);
    }

    pub fn read_id(&self) -> DisplayIdentification {
        let mut result = DisplayIdentification::default();
        self.write_command(0x0000);
        self.read_data(&mut result.raw);
        result
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn init<DELAY>(&self, delay: &mut DELAY)
        where
            DELAY: DelayMs<u8>,
    {
        self.iface.reset();
        self.write_cmd_data(0x0001, 0x0100);/* Driver Output Contral Register */
        self.write_cmd_data(0x0002, 0x0700);/* LCD Driving Waveform Contral */
        self.write_cmd_data(0x0003, 0x1030);/* Entry Mode setting */

        self.write_cmd_data(0x0004, 0x0000);/* Scalling Control register */
        self.write_cmd_data(0x0008, 0x0207);/* Display Control 2 */
        self.write_cmd_data(0x0009, 0x0000);/* Display Control 3 */
        self.write_cmd_data(0x000A, 0x0000);/* Frame Cycle Control */
        self.write_cmd_data(0x000C, 0x0000);/* External Display Interface Control 1 */
        self.write_cmd_data(0x000D, 0x0000);/* Frame Maker Position */
        self.write_cmd_data(0x000F, 0x0000);/* External Display Interface Control 2 */
        delay.delay_ms(50);
        self.write_cmd_data(0x0007, 0x0101);/* Display Control */
        delay.delay_ms(50);
        self.write_cmd_data(0x0010, 0x16B0);/* Power Control 1 */
        self.write_cmd_data(0x0011, 0x0001);/* Power Control 2 */
        self.write_cmd_data(0x0017, 0x0001);/* Power Control 3 */
        self.write_cmd_data(0x0012, 0x0138);/* Power Control 4 */
        self.write_cmd_data(0x0013, 0x0800);/* Power Control 5 */
        self.write_cmd_data(0x0029, 0x0009);/* NVM read data 2 */
        self.write_cmd_data(0x002a, 0x0009);/* NVM read data 3 */
        self.write_cmd_data(0x00a4, 0x0000);
        self.write_cmd_data(0x0050, 0x0000);
        //self.write_cmd_data(0x0051,0x013F);
        self.write_cmd_data(0x0051, self.width - 1);
        self.write_cmd_data(0x0052, 0x0000);
        self.write_cmd_data(0x0053, self.height - 1);
        //self.write_cmd_data(0x0053,0x00EF);

        self.write_cmd_data(0x0060, 0xA700);/* Driver Output Control */
        self.write_cmd_data(0x0061, 0x0003);/* Driver Output Control */
        self.write_cmd_data(0x006A, 0x0000);/* Vertical Scroll Control */

        self.write_cmd_data(0x0080, 0x0000);/* Display Position Partial Display 1 */
        self.write_cmd_data(0x0081, 0x0000);/* RAM Address Start Partial Display 1 */
        self.write_cmd_data(0x0082, 0x0000);/* RAM address End - Partial Display 1 */
        self.write_cmd_data(0x0083, 0x0000);/* Display Position Partial Display 2 */
        self.write_cmd_data(0x0084, 0x0000);/* RAM Address Start Partial Display 2 */
        self.write_cmd_data(0x0085, 0x0000);/* RAM address End Partail Display2 */
        self.write_cmd_data(0x0090, 0x0013);/* Frame Cycle Control */
        self.write_cmd_data(0x0092, 0x0000);/* Panel Interface Control 2 */
        self.write_cmd_data(0x0093, 0x0003);/* Panel Interface control 3 */
        self.write_cmd_data(0x0095, 0x0110);/* Frame Cycle Control */
        self.write_cmd_data(0x0007, 0x0173);
    }
}
