/*
pub fn create_opcode<'a>(
    cpu: &'a Rc<RefCell<CPU>>,
    instruction: impl Instruction + 'a,
) -> impl Generator + 'a {
    move || {
        yield format!("Implied {:?}", instruction);
        instruction.execute(cpu);
        return;
    }
}
*/
