pub const BUTTON_A: u8 = 1 << 7;
pub const BUTTON_B: u8 = 1 << 6;
pub const BUTTON_SELECT: u8 = 1 << 5;
pub const BUTTON_START: u8 = 1 << 4;
pub const BUTTON_UP: u8 = 1 << 3;
pub const BUTTON_DOWN: u8 = 1 << 2;
pub const BUTTON_LEFT: u8 = 1 << 1;
pub const BUTTON_RIGHT: u8 = 1 << 0;

pub enum ControllerState {
    On = 0xFF,
    Off = 0x00,
}

pub struct Controller {
    controller_state: u8,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            controller_state: 0x00
        }
    }

    pub fn get_controller_state(& self) -> u8 {
        self.controller_state
    }

    pub fn set_controller_flag(&mut self, button_mask: u8, state: ControllerState) {
        self.controller_state = (self.controller_state & !button_mask)  | (button_mask & state as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_controller_flag() {
        // Arrange
        let mut mask: u8 = 0x01;

        for _ in 0..8 {
            // Arrange
            let mut controller = Controller::new();

            // Act
            controller.set_controller_flag(mask, ControllerState::On);

            // Test
            assert_eq!(controller.controller_state, mask, "The controller flag has a wrong state after flag set!");

            mask = mask << 1;
        }
    }

    #[test]
    fn clear_controller_flag() {
        // Arrange
        let mut mask: u8 = 0x01;

        for _ in 0..8 {
            // Arrange
            let mut controller = Controller::new();
            controller.controller_state = 0xFF;

            // Act
            controller.set_controller_flag(mask, ControllerState::Off);

            // Test
            assert_eq!(controller.controller_state, !mask, "The controller flag has a wrong state after flag clear!");

            mask = mask << 1;
        }
    }

    #[test]
    fn get_controller_state() {
        // Arrange
        let mask: u8 = 0x01;
        let mut controller = Controller::new();

        // Act
        controller.set_controller_flag(mask, ControllerState::On);
        let state = controller.get_controller_state();

        // Test
        assert_eq!(state, !mask, "The controller flag has a wrong state!");
    }
}