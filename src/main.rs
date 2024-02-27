use std::{fs::File, io::Write};

use msbt::msbt;
use ::msbt::structs::{Header, ATR1, LBL1, TXT2};

fn main() -> ::msbt::Result<()> {
    // let mut file = File::open("agb.msbt")?;
    // let msbt = msbt::from_binary(&mut file)?;
    // let strings = msbt::get_strings(msbt.clone())?;
    // let new_msbt = msbt::to_binary(strings, msbt.endianness)?;
    // let mut result = File::create("foo.msbt")?;
    // result.write(&new_msbt)?;
    
    //Clappy Trio
    // let string = [0x0E, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0xCD, 0x50, 0x00, 0x72, 0x00, 0x65, 0x00, 0x73, 0x00, 0x73, 0x00, 0x20, 0x00, 0x00, 0xE0, 0x20, 0x00, 0x66, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x20, 0x00, 0x74, 0x00, 0x68, 0x00, 0x65, 0x00, 0x20, 0x00, 0x74, 0x00, 0x68, 0x00, 0x69, 0x00, 0x72, 0x00, 0x64, 0x00, 0x20, 0x00, 0x63, 0x00, 0x6C, 0x00, 0x61, 0x00, 0x70, 0x00, 0x2E, 0x00, 0x0F, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x0E, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xCD, 0x0E, 0x00, 0x00, 0x00, 0x03, 0x00, 0x04, 0x00, 0x3B, 0xDB, 0x3E, 0xFF, 0x54, 0x00, 0x61, 0x00, 0x70, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x03, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x20, 0x00, 0x66, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x20, 0x00, 0x74, 0x00, 0x68, 0x00, 0x65, 0x00, 0x20, 0x00, 0x74, 0x00, 0x68, 0x00, 0x69, 0x00, 0x72, 0x00, 0x64, 0x00, 0x20, 0x00, 0x63, 0x00, 0x6C, 0x00, 0x61, 0x00, 0x70, 0x00, 0x2E, 0x00, 0x0F, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    //Quiz Show
    // let string: [u8;118] = [0x54,0x00,0x68,0x00,0x65,0x00,0x20,0x00,0x6B,0x00,0x65,0x00,0x79,0x00,0x20,0x00,0x68,0x00,0x65,0x00,0x72,0x00,0x65,0x00,0x20,0x00,0x69,0x00,0x73,0x00,0x20,0x00,0x74,0x00,0x6F,0x00,0x20,0x00,0x0E,0x00,0x00,0x00,0x03,0x00,0x04,0x00,0xE4,0x00,0x00,0xFF,0x75,0x00,0x73,0x00,0x65,0x00,0x0A,0x00,0x74,0x00,0x68,0x00,0x65,0x00,0x20,0x00,0x72,0x00,0x68,0x00,0x79,0x00,0x74,0x00,0x68,0x00,0x6D,0x00,0x20,0x00,0x74,0x00,0x6F,0x00,0x20,0x00,0x72,0x00,0x65,0x00,0x6D,0x00,0x65,0x00,0x6D,0x00,0x62,0x00,0x65,0x00,0x72,0x00,0x0E,0x00,0x00,0x00,0x03,0x00,0x04,0x00,0x00,0x00,0x00,0xFF,0x21,0x00,0x00,0x00];
    // let constructed_string = ::msbt::structs::TXT2::parse_binary(string.to_vec(), bytestream::ByteOrder::LittleEndian);
    // println!("{}", constructed_string);

    // let string = "[RawCmd 4.0 00CD]Press [!A_button_3DS] for the third clap.[/RawCmd 4.0][RawCmd 4.1]\n[RawCmd 4.0 01CD][RawCmd 0.3 3BDB3EFF]Tap[RawCmd 0.3 000000FF] for the third clap.[/RawCmd 4.0]";
    // let constructed_string = ::msbt::structs::TXT2::parse_string(string, bytestream::ByteOrder::LittleEndian);

    // let code = "[/RawCmd 4.0]".to_string();
    // let raw_code = ::msbt::structs::TXT2::convert_control_code_close(&code, bytestream::ByteOrder::LittleEndian);
    // println!("{:02X?}", raw_code);
    Ok(())
}