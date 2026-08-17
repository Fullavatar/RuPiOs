use core::arch::global_asm;

global_asm!(
    r#"
    .section .text._start
    .global _start

_start:
    mrs x0, mpidr_el1
    and x0, x0, #0xff
    cbz x0, primary_core

secondary_core:
    wfe
    b secondary_core

primary_core:
    ldr x0, =__stack_top
    mov sp, x0

    mrs x0, CurrentEL

    lsr x0, x0, #2
    and x0, x0, #0b11

    cmp x0, #3
    b.eq boot_from_el3

    cmp x0, #2
    b.eq boot_from_el2

    cmp x0, #1
    b.eq boot_continue

    b boot_error

boot_from_el3:
    msr cptr_el3, xzr

    mov x0, #0x401
    msr scr_el3, x0

    mov x0, sp
    msr sp_el2, x0

    adr x0, boot_from_el2
    msr elr_el3, x0

    mov x0, #0x3c9
    msr spsr_el3, x0

    eret

boot_from_el2:
    msr cptr_el2, xzr

    mrs x0, hcr_el2
    orr x0, x0, #(1 << 31)
    msr hcr_el2, x0

    mrs x0, cnthctl_el2
    orr x0, x0, #3
    msr cnthctl_el2, x0

    msr sctlr_el1, xzr

    mov x0, sp
    msr sp_el1, x0

    adr x0, el1_entry
    msr elr_el2, x0

    mov x0, #0x3c5
    msr spsr_el2, x0

    eret

el1_entry:
    mov x0, #(0b11 << 20)
    msr cpacr_el1, x0
    isb

    adr x0, exception_vectors
    msr vbar_el1, x0
    isb

    b boot_continue

boot_continue:
    bl kernel_main

boot_halt:
    wfe
    b boot_halt

boot_error:
    wfe
    b boot_error

    .balign 2048
    .global exception_vectors

exception_vectors:
    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt_sync_current_spx
    .space 124

    b exception_irq_current_spx
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

    b exception_halt
    .space 124

exception_halt_sync_current_spx:
    mrs x0, esr_el1
    mrs x1, elr_el1
    mrs x2, far_el1

    bl exception_handler

    b exception_halt

exception_irq_current_spx:
    bl irq_handler
    b exception_halt

exception_halt:
    wfe
    b exception_halt
"#
);