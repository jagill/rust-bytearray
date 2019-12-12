use bytearray::{from_byte_vec, to_byte_vec};

#[test]
fn test_vec_u32() {
    let v: Vec<u32> = vec![0, 1, 2, 3, 4, 8];
    let bytes: Vec<u8> = to_byte_vec(v.clone());
    let result: Vec<u32> = from_byte_vec(bytes).unwrap();

    assert_eq!(v, result);
}
