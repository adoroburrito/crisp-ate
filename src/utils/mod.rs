pub fn hex(num: u16) -> String {
    return format!("{:#04x?}", num)
}

pub fn vec_u16_from_u8_vec(nums: Vec<&u8>) -> Vec<u16> {
    let mut to_return: Vec<u16> = Vec::new();

    for num in nums {
        to_return.push((*num).try_into().unwrap())
    }

    to_return
}
