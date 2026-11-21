use solana_sbpf::{
    ebpf::MM_INPUT_START,
    elf::Executable,
    memory_region::{MemoryMapping, MemoryRegion},
    program::{BuiltinProgram, SBPFVersion},
    verifier::RequisiteVerifier,
    vm::{CallFrame, Config, ContextObject, EbpfVm, ExecutionMode},
};
use std::{env, fs, path::PathBuf, ptr::NonNull, sync::Arc};

struct Context {
    mapping: MemoryMapping,
    remaining: u64,
}

impl ContextObject for Context {
    fn consume(&mut self, amount: u64) {
        self.remaining = self.remaining.saturating_sub(amount);
    }

    fn get_remaining(&self) -> u64 {
        self.remaining
    }

    fn active_mapping_ptr(&mut self) -> NonNull<MemoryMapping> {
        NonNull::from(&mut self.mapping)
    }
}

#[test]
fn execute_deployed_program() {
    let dir = PathBuf::from(env::var_os("SBF_OUT_DIR").expect("run with cargo test-sbf"));
    let bytes = fs::read(dir.join("sbf_smoke.so")).unwrap();
    assert_eq!(&bytes[..6], b"\x7fELF\x02\x01");
    assert_eq!(u16::from_le_bytes(bytes[16..18].try_into().unwrap()), 3);
    assert_eq!(u16::from_le_bytes(bytes[18..20].try_into().unwrap()), 247);
    assert_eq!(u32::from_le_bytes(bytes[48..52].try_into().unwrap()), 3);

    let config = Config::default();
    let loader = Arc::new(BuiltinProgram::new_loader(config.clone()));
    let exe = Executable::<Context>::from_elf(&bytes, loader.clone()).unwrap();
    assert_eq!(exe.get_sbpf_version(), SBPFVersion::V3);
    exe.verify::<RequisiteVerifier>().unwrap();

    for (instruction, expected) in [(42, 0), (7, 1)] {
        // Solana loader input: two u64 lengths, the instruction, and the program ID.
        let mut input = [0u8; 8 + 8 + 1 + 32];
        input[8] = 1;
        input[16] = instruction;
        let regions = vec![
            exe.get_ro_region(),
            MemoryRegion::new(&raw mut input, MM_INPUT_START),
        ];
        // Both the ELF and the input buffer outlive the VM.
        let mapping = unsafe { MemoryMapping::new(regions, &config, SBPFVersion::V3).unwrap() };
        let mut context = Context { mapping, remaining: 1000 };
        let mut vm = EbpfVm::new(loader.clone(), SBPFVersion::V3, &mut context, 0);
        vm.registers[1] = MM_INPUT_START;
        let mut frames = vec![CallFrame::default(); config.max_call_depth];
        let (_, result) = vm.execute_program(&exe, &mut ExecutionMode::Interpreted, &mut frames);
        assert_eq!(result.unwrap(), expected);
    }
}
