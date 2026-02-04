use serde::Serialize;
use crate::ser::basic::BasicSerializer;

#[test]
fn serialize_u8() {
    let input: &[u8] = &[
        0, 1, 10, 23, 24, 25, 100, 255
    ];
    let expected: &[u8]= &[
        0x00,       // 0
        0x01,       // 1
        0x0A,       // 10
        0x17,       // 23
        0x18, 0x18, // 24
        0x18, 0x19, // 25
        0x18, 0x64, // 100
        0x18, 0xFF, // 255
    ];
    let mut output = Vec::new();
    {
        let mut serializer = BasicSerializer::new(&mut output);
        for value in input {
            value.serialize(&mut serializer).unwrap();
        }
    }
    assert_eq!(expected, output.as_slice())
}

#[test]
fn serialize_u16() {
    let input: &[u16] = &[
        0, 1, 10, 23, 24, 25, 100, 255,  // Fit in u8
        256, 1000, 65535                 // Fit in u16
    ];
    let expected: &[u8] = &[
        0x00,             // 0
        0x01,             // 1
        0x0A,             // 10
        0x17,             // 23
        0x18, 0x18,       // 24
        0x18, 0x19,       // 25
        0x18, 0x64,       // 100
        0x18, 0xFF,       // 255
        0x19, 0x01, 0x00, // 256
        0x19, 0x03, 0xE8, // 1000
        0x19, 0xFF, 0xFF, // 65535
    ];
    let mut output = Vec::new();
    {
        let mut serializer = BasicSerializer::new(&mut output);
        for &value in input {
            value.serialize(&mut serializer).unwrap();
        }
    }
    assert_eq!(expected, output.as_slice())
}
