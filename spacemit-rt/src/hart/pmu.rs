// K1: OpenSBI fc02b891, k1x_evb.h and k1x/underly_implement.c::spacemit_wakeup_cpu.
// https://github.com/spacemit-com/opensbi/blob/fc02b891b17b8bdc1273a39f80aa374cd99ba9a2/lib/utils/psci/spacemit/plat/k1x/underly_implement.c
// K3: OpenSBI 8bd2cbdf, k3.h and k3_corepm.c::spacemit_wakeup_core.
// https://github.com/spacemit-com/opensbi/blob/8bd2cbdf9856dbc1a990d36e26bf47411f356c42/platform/generic/spacemit/k3_corepm.c

#[inline]
pub(super) unsafe fn release(id: usize) {
    cfg_if::cfg_if! {
        if #[cfg(all(target_os = "none", target_arch = "riscv64", any(
            feature = "k3-bootrom", feature = "k3-cpu", feature = "k3-ai"
        )))] {
            let start = if id < 8 {
                crate::arch::spacemit_x100::start
            } else {
                crate::arch::spacemit_a100::start
            };
        } else {
            let start = crate::start;
        }
    }
    let entry = start as *const () as usize as u64;
    // SAFETY: Hart::spawn reserves these mapped CIU and APMU registers; exact writes
    // program the vector halves and then issue the write-one wakeup request.
    unsafe {
        let vector = if cfg!(any(feature = "k1-bootrom", feature = "k1-cpu")) {
            let ciu = &*crate::soc::k1::CIU::ptr();
            [&ciu.cluster0_reset_vector, &ciu.cluster1_reset_vector][id / 4]
        } else {
            let ciu = &*crate::soc::k3::CIU::ptr();
            [
                &ciu.cluster0_reset_vector,
                &ciu.cluster1_reset_vector,
                &ciu.cluster2_reset_vector,
                &ciu.cluster3_reset_vector,
            ][id / 4]
        };
        vector.low.write(entry as u32);
        vector.high.write((entry >> 32) as u32);
        riscv::asm::fence();
        // K1 indexes the request register by the initiator; K3 uses the target.
        if cfg!(any(feature = "k1-bootrom", feature = "k1-cpu")) {
            let initiator = riscv::register::mhartid::read();
            let apmu = &*crate::soc::k1::APMU::ptr();
            let wakeup = [&apmu.cluster0_wakeup, &apmu.cluster1_wakeup][initiator / 4];
            wakeup[initiator % 4].write(1 << id);
        } else {
            let apmu = &*crate::soc::k3::APMU::ptr();
            let wakeup = [
                &apmu.cluster0_wakeup,
                &apmu.cluster1_wakeup,
                &apmu.cluster2_wakeup,
                &apmu.cluster3_wakeup,
            ][id / 4];
            wakeup[id % 4].write(1 << id);
        }
        riscv::asm::fence();
    }
}
