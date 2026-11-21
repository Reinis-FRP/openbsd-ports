/// Accept instruction 42 with no accounts and return custom error 1 otherwise.
///
/// # Safety
/// The Solana loader supplies a serialized instruction at `input`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let header = input.cast::<u64>();
    unsafe {
        if header.read_unaligned() == 0
            && header.add(1).read_unaligned() == 1
            && input.add(16).read() == 42
        {
            0
        } else {
            1
        }
    }
}
