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
    b.eq el1_entry

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

.macro save_context
    sub sp, sp, #800

    stp x0,  x1,  [sp, #0]
    stp x2,  x3,  [sp, #16]
    stp x4,  x5,  [sp, #32]
    stp x6,  x7,  [sp, #48]
    stp x8,  x9,  [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30,      [sp, #240]

    mrs x0, fpcr
    mrs x1, fpsr
    stp x0, x1, [sp, #256]

    mrs x0, elr_el1
    mrs x1, spsr_el1
    stp x0, x1, [sp, #272]

    stp q0,  q1,  [sp, #288]
    stp q2,  q3,  [sp, #320]
    stp q4,  q5,  [sp, #352]
    stp q6,  q7,  [sp, #384]
    stp q8,  q9,  [sp, #416]
    stp q10, q11, [sp, #448]
    stp q12, q13, [sp, #480]
    stp q14, q15, [sp, #512]
    stp q16, q17, [sp, #544]
    stp q18, q19, [sp, #576]
    stp q20, q21, [sp, #608]
    stp q22, q23, [sp, #640]
    stp q24, q25, [sp, #672]
    stp q26, q27, [sp, #704]
    stp q28, q29, [sp, #736]
    stp q30, q31, [sp, #768]
.endm

.macro restore_context
    ldp q0,  q1,  [sp, #288]
    ldp q2,  q3,  [sp, #320]
    ldp q4,  q5,  [sp, #352]
    ldp q6,  q7,  [sp, #384]
    ldp q8,  q9,  [sp, #416]
    ldp q10, q11, [sp, #448]
    ldp q12, q13, [sp, #480]
    ldp q14, q15, [sp, #512]
    ldp q16, q17, [sp, #544]
    ldp q18, q19, [sp, #576]
    ldp q20, q21, [sp, #608]
    ldp q22, q23, [sp, #640]
    ldp q24, q25, [sp, #672]
    ldp q26, q27, [sp, #704]
    ldp q28, q29, [sp, #736]
    ldp q30, q31, [sp, #768]

    ldp x0, x1, [sp, #272]
    msr elr_el1, x0
    msr spsr_el1, x1

    ldp x0, x1, [sp, #256]
    msr fpcr, x0
    msr fpsr, x1

    ldp x0,  x1,  [sp, #0]
    ldp x2,  x3,  [sp, #16]
    ldp x4,  x5,  [sp, #32]
    ldp x6,  x7,  [sp, #48]
    ldp x8,  x9,  [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30,      [sp, #240]

    add sp, sp, #800
.endm

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
    save_context
    bl irq_handler
    restore_context

    eret

exception_halt:
    wfe
    b exception_halt

"#
);