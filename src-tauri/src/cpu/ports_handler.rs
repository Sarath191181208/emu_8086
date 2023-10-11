use serde::{ser::SerializeSeq, Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Ports {
    list: [u8; 0xFF],
}

impl Default for Ports {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Deserialize<'a> for Ports {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let list = Vec::<u8>::deserialize(deserializer)?;
        let mut ports = Ports::new();
        for (i, value) in list.iter().enumerate() {
            ports.set(i as u8, *value);
        }
        Ok(ports)
    }
}

impl Serialize for Ports {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.list.len()))?;
        for element in &self.list {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl Ports {
    pub fn new() -> Self {
        Self { list: [0; 0xFF] }
    }

    pub fn reset(&mut self){
        self.list = [0; 0xFF];
    }

    pub fn get(&self, port: u8) -> u8 {
        self.list[port as usize]
    }

    pub fn set(&mut self, port: u8, value: u8) {
        self.list[port as usize] = value;
    }
}

impl Ports {
    pub fn print_non_empty_prots(&self) {
        for (i, port) in self.list.iter().enumerate() {
            if *port != 0 {
                println!("Port: {:#X} Value: {:#X}", i, port);
            }
        }
    }
}
