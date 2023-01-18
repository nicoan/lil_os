mod ibm_pc_at_8259;
pub(crate) mod pic8259;

use self::ibm_pc_at_8259::IBMPcAt8259;
use crate::os_core::spinlock::Mutex;

// TODO The idea here is to define a trait that abstacts away the interruption habdling either if
// it is with the IBM PC/AT 8259 Architecture or with the APIC interface.
//
// TODO: Create a trait that abstracs aways if we are using a pic8259 or an apic or whatever
// interrupt controller we need o use.

// pub static PICS: Mutex<IBMPcAt8259> = Arc::new(Mutex::new(unsafe { IBMPcAt8259::new() }));
