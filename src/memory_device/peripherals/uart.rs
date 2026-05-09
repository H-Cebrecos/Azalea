pub mod backends;
mod devices;
pub use devices::*;

pub trait UartBackend {
    fn write_byte(&mut self, byte: u8);

    fn read_byte(&mut self) -> Option<u8>;

    fn rx_ready(&mut self) -> bool;
}
