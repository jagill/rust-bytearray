use bytearray::{from_byte_vec, to_byte_vec};
use zerocopy::{AsBytes, FromBytes};

#[derive(AsBytes, FromBytes, Copy, Debug, PartialEq, Clone)]
#[repr(packed)]
struct PackedCoord {
    x: f32,
    y: f32,
}

#[derive(AsBytes, FromBytes, Copy, Debug, PartialEq, Clone)]
#[repr(C)]
struct CCoord {
    x: f32,
    y: f32,
}

#[derive(AsBytes, FromBytes, Copy, Debug, PartialEq, Clone)]
#[repr(packed)]
struct Uneven {
    a: u16,
    b: u8,
}

#[derive(AsBytes, FromBytes, Copy, Debug, PartialEq, Clone)]
#[repr(transparent)]
struct Transparent {
    a: u8,
}

#[derive(AsBytes, FromBytes, Copy, Debug, PartialEq, Clone)]
#[repr(C)]
struct Empty {}

fn test_roundtrip<C>(v: Vec<C>)
where
    C: AsBytes + FromBytes + PartialEq + Clone + std::fmt::Debug,
{
    let bytes: Vec<u8> = to_byte_vec(v.clone());
    let result: Vec<C> = from_byte_vec(bytes).unwrap();

    assert_eq!(v, result);
}

#[test]
fn test_vec_packed_roundtrip() {
    let v: Vec<PackedCoord> = vec![
        PackedCoord { x: 1., y: 2. },
        PackedCoord { x: -1., y: 10. },
        PackedCoord { x: 1.6, y: 4.5 },
        PackedCoord {
            x: 9.9e9,
            y: -9.9e-9,
        },
    ];
    test_roundtrip(v);
}

#[test]
fn test_vec_ccoord_roundtrip() {
    let v: Vec<CCoord> = vec![
        CCoord { x: 1., y: 2. },
        CCoord { x: -1., y: 10. },
        CCoord { x: 1.6, y: 4.5 },
        CCoord {
            x: 9.9e9,
            y: -9.9e-9,
        },
    ];
    test_roundtrip(v);
}

#[test]
fn test_vec_uneven_roundtrip() {
    let v: Vec<Uneven> = vec![
        Uneven { a: 1, b: 2 },
        Uneven {
            a: 1 << 12,
            b: 1 << 4,
        },
        Uneven { a: 3, b: 255 },
    ];
    test_roundtrip(v);
}

#[test]
fn test_vec_transparent_roundtrip() {
    let v: Vec<Transparent> = vec![
        Transparent { a: 1 },
        Transparent { a: 1 << 4 },
        Transparent { a: 3 },
    ];
    test_roundtrip(v);
}

#[test]
fn test_vec_empty_struct() {
    let v: Vec<Empty> = vec![Empty {}, Empty {}, Empty {}];
    let bytes: Vec<u8> = to_byte_vec(v);
    assert!(bytes.is_empty());
    let result: Result<Vec<()>, String> = from_byte_vec(bytes);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Cannot construct size 0 vector (type ()) from bytes."
    );
}
