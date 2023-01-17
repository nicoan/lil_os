mod ibm_pc_at_8259;
pub(crate) mod pic8259;

// TODO The idea here is to define a trait that abstacts away the interruption habdling either if
// it is with the IBM PC/AT 8259 Architecture or with the APIC interface.
//
// TODO: Create a trait that abstracs aways if we are using a pic8259 or an apic or whatever
// interrupt controller we need o use.
