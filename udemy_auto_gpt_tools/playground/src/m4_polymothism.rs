
use ethers::types::Address;
use std::str::FromStr;


trait EthAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthAddress for &str {

    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid eth address string")
        }
    }
}

impl EthAddress for Address {

    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_eth_data<T: EthAddress>(address: T) -> Address {
    let converted_address = address.convert_address().unwrap();
    converted_address
}


#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_polym -- --nocapture

    #[test]
    fn tests_polym(){
        dbg!("tests_polym");


        let addr = Address::from_str("0x71C7656EC7ab88b098defB751B7401B5f6d8976F").unwrap();
        let new_a = get_eth_data(addr);

        dbg!(addr);
        assert_eq!(new_a, Address::from_str("0x71C7656EC7ab88b098defB751B7401B5f6d8976F").unwrap());

        let new_a2 = get_eth_data("0x71C7656EC7ab88b098defB751B7401B5f6d8976F");
        assert_eq!(new_a2, Address::from_str("0x71C7656EC7ab88b098defB751B7401B5f6d8976F").unwrap());

    }
}