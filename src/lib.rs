#[cfg(not(target_arch = "x86_64"))]
mod fallback;

#[cfg(not(target_arch = "x86_64"))]
pub use fallback::*;

#[cfg(target_arch = "x86_64")]
mod consts;

#[cfg(target_arch = "x86_64")]
mod x86;

#[cfg(target_arch = "x86_64")]
pub use x86::*;

#[inline]
pub fn despace_string(incoming: &mut String) {
    let len = unsafe { de_space_str(incoming) };
    incoming.truncate(len);
}

#[cfg(test)]
mod tests {
    use crate::{count_spaces, despace_string};

    #[test]
    fn it_works() {
        let mut string = "a bcdefg h i j k lmnopqrstuvwxyz1234567890a bcdefg h i j k lmnopqrstuvwxyz1234567890a bcdefg h i j k lmnopqrstuvwxyz1234567890a bcdefg h i j k lmnopqrstuvwxyz1234567890a bcdefg h i j k lmnopqrstuvwxyz1234567890a bcdefg h i j k lmnopqrstuvwxyz1234567890a bcdefg h i j k lmnopqrstuvwxyz123456789
    
    
    
    
    0a bcdefg h i j k lmnopqrstuvwxyz1234567890a bcdefg h i j k lmnopqrstuvwxyz1234567890".to_string();

        despace_string(&mut string);
        assert_eq!(count_spaces(&string), 0);
    }
}
