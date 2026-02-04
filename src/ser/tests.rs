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

#[test]
fn serialize_u32() {
    let input: &[u32] = &[
        0, 1, 10, 23, 24, 25, 100, 255,       // Fit in u8
        256, 1000, 65535,                     // Fit in u16
        65536, 1000000, 4294967295            // Fit in u32
    ];
    let expected: &[u8] = &[
        0x00,                         // 0
        0x01,                         // 1
        0x0A,                         // 10
        0x17,                         // 23
        0x18, 0x18,                   // 24
        0x18, 0x19,                   // 25
        0x18, 0x64,                   // 100
        0x18, 0xFF,                   // 255
        0x19, 0x01, 0x00,             // 256
        0x19, 0x03, 0xE8,             // 1000
        0x19, 0xFF, 0xFF,             // 65535
        0x1A, 0x00, 0x01, 0x00, 0x00, // 65536
        0x1A, 0x00, 0x0F, 0x42, 0x40, // 1_000_000
        0x1A, 0xFF, 0xFF, 0xFF, 0xFF  // 4_294_967_295
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

#[test]
fn serialize_u64() {
    let input: &[u64] = &[
        0, 5, 20, 23, 30, 50, 200, 250,           // Fit in u8
        300, 1024, 60000,                         // Fit in u16
        70000, 2_000_000, 4_000_000_000,          // Fit in u32
        4_500_000_000, 900_000_000_000, u64::MAX  // Fit in u64
    ];
    let expected: &[u8] = &[
        0x00,                                                 // 0
        0x05,                                                 // 5
        0x14,                                                 // 20
        0x17,                                                 // 23
        0x18, 0x1E,                                           // 30
        0x18, 0x32,                                           // 50
        0x18, 0xC8,                                           // 200
        0x18, 0xFA,                                           // 250
        0x19, 0x01, 0x2C,                                     // 300
        0x19, 0x04, 0x00,                                     // 1024
        0x19, 0xEA, 0x60,                                     // 60000
        0x1A, 0x00, 0x01, 0x11, 0x70,                         // 70000
        0x1A, 0x00, 0x1E, 0x84, 0x80,                         // 2_000_000
        0x1A, 0xEE, 0x6B, 0x28, 0x00,                         // 4_000_000_000
        0x1B, 0x00, 0x00, 0x00, 0x01, 0x0C, 0x38, 0x8D, 0x00, // 4_500_000_000
        0x1B, 0x00, 0x00, 0x00, 0xD1, 0x8C, 0x2E, 0x28, 0x00, // 900_000_000_000
        0x1B, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF  // u64::MAX
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
