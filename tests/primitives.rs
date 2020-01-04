use bytearray::{from_byte_vec, to_byte_vec};

#[test]
fn test_vec_u32_roundtrip() {
    let v: Vec<u32> = vec![0, 1, 2, 3, 4, 8];
    let bytes: Vec<u8> = to_byte_vec(v.clone());
    let result: Vec<u32> = from_byte_vec(bytes).unwrap();

    assert_eq!(v, result);
}

#[test]
fn test_bytes_from_vec_u32() {
    let v: Vec<u32> = vec![0, 1, 2, 1 << 8, (1 << 16) + (1 << 31)];
    let bytes: Vec<u8> = to_byte_vec(v);

    #[rustfmt::skip]
    assert_eq!(bytes, vec![
        0u8, 0, 0, 0,
        1, 0, 0, 0,
        2, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 128
    ]);
}

#[test]
fn test_bytes_to_vec_u32() {
    let bytes = vec![
        0u8, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 128,
    ];
    let v: Vec<u32> = from_byte_vec(bytes).unwrap();

    assert_eq!(v, vec![0, 1, 2, 1 << 8, (1 << 16) + (1 << 31)]);
}

#[test]
fn test_empty_vec() {
    let v: Vec<u32> = Vec::new();
    let bytes: Vec<u8> = to_byte_vec(v);
    assert_eq!(bytes, vec![]);

    let bytes: Vec<u8> = Vec::new();
    let v: Vec<u32> = from_byte_vec(bytes).unwrap();
    assert_eq!(v, vec![]);
}

#[test]
fn test_wrong_size_bytes() {
    #[rustfmt::skip]
    let bytes = vec![
        0u8, 0, 0, 0,
        1, 0, 0, 0,
        2, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1,
    ];
    let result: Result<Vec<u32>, String> = from_byte_vec(bytes);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Byte vector capacity 19 must be a multiple of u32 size: 4"
    );
}

#[test]
fn test_zero_size() {
    #[rustfmt::skip]
    let bytes = vec![0u8, 1, 2, 3];
    let result: Result<Vec<()>, String> = from_byte_vec(bytes);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Cannot construct size 0 vector (type ()) from bytes."
    );
}

#[test]
fn test_vec_u64_roundtrip() {
    let v: Vec<u64> = vec![0, 1, 2, 3, 5 << 31, 3 << 60];
    let bytes: Vec<u8> = to_byte_vec(v.clone());
    let result: Vec<u64> = from_byte_vec(bytes).unwrap();

    assert_eq!(v, result);
}

#[test]
fn test_vec_i64_roundtrip() {
    let v: Vec<i64> = vec![0, 1, 2, -3, -5 << 31, 3 << 60];
    let bytes: Vec<u8> = to_byte_vec(v.clone());
    let result: Vec<i64> = from_byte_vec(bytes).unwrap();

    assert_eq!(v, result);
}

#[test]
fn test_vec_f32_roundtrip() {
    let v: Vec<f32> = vec![0., 1., 2., -3., -5.25e9, 8.4e-9];
    let bytes: Vec<u8> = to_byte_vec(v.clone());
    let result: Vec<f32> = from_byte_vec(bytes).unwrap();

    assert_eq!(v, result);
}
