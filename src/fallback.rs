pub fn count_spaces(string: &str) -> usize {
    crate::jump_tables::count_spaces(string)
}

pub fn despace_string(string: &mut str) -> usize {
    unsafe { crate::jump_tables::de_space_str(string) }
}
