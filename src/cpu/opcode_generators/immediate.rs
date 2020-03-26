/*
pub fn create_opcode<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: impl Instruction + 'a,
) -> impl Generator + 'a {
    move || {
        yield format!("Immediate {:?}", instruction);
        cpu.borrow_mut().registers.pc += 1;
        instruction.execute(cpu);
        return;
    }
}
*/
