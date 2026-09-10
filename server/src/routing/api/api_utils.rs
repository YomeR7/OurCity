//helper func to convert i32 to NonZeroU32
pub fn convert_i32_nonzerou32(val: i32) -> Result<std::num::NonZeroU32, String> {
    let c_val: u32 = match u32::try_from(val) {
        Ok(c_val) => c_val,
        Err(e) => return Err(format!("Failed to convert i32 -> u32: {} {}", val, e)),
    };
    match std::num::NonZeroU32::new(c_val) {
        Some(c_val) => Ok(c_val),
        None => return Err(format!("Random zero shit is stored in db: {}", val)),
    }
}
