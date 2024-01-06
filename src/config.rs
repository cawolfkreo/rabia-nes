pub struct Config {
    pub rom_path: Option<String>,
}

impl Config {
    pub fn new(rom_path: Option<String>) -> Self{
        Self {
            rom_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_config_struct_with_path() {
        // Arrange
        let test_path = Some (
            String::from("this/is/a/test/path/that/doesnt/exist")
        );

        // Act
        let test_config = Config::new(test_path.clone());

        // Assert
        assert!(test_config.rom_path.is_some());
        assert_eq!(test_path, test_config.rom_path);
        assert_eq!(test_path.unwrap(), test_config.rom_path.unwrap());

    }

    #[test]
    fn test_creates_config_struct_no_path() {
        // Arrange
        let test_path: Option<String> = None;

        // Act
        let test_config = Config::new(test_path.clone());

        // Assert
        assert!(test_config.rom_path.is_none());
    }
}