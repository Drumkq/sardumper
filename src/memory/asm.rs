pub const unsafe fn get_virtual_absolute_address<T>(
    va_base: isize,
    data: &[T],
    offset: isize,
    instruction_sz: isize,
) -> isize {
    let jmp_rva = data
        .as_ptr()
        .offset((offset + instruction_sz) as isize)
        .cast::<i32>()
        .read_unaligned();

    let next_instruction_rva = offset + instruction_sz + size_of::<i32>() as isize;

    if jmp_rva > i32::MAX {
        va_base + next_instruction_rva - jmp_rva as isize
    } else {
        va_base + next_instruction_rva + jmp_rva as isize
    }
}
