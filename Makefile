KERNEL_NAME := rupios
TARGET := aarch64-unknown-none

ELF := target/$(TARGET)/debug/$(KERNEL_NAME)
IMG := kernel8.img

QEMU := qemu-system-aarch64
QEMU_MACHINE := -M raspi4b

LOG_ELF := qemu.log
LOG_RAW := qemu_raw.log

KERNEL_LOG_RANGE := 0x80000..0x80fff


.PHONY: build
build: ## Build the AArch64 bare-metal ELF
	cargo build


.PHONY: clean
clean: ## Remove Cargo build artifacts and generated files
	cargo clean
	powershell -NoProfile -Command "$$files = @('$(IMG)', '$(LOG_ELF)', '$(LOG_RAW)'); $$files | Where-Object { Test-Path $$_ } | Remove-Item -Force"


.PHONY: img
img: ## Generate the raw Raspberry Pi kernel8.img
	$(MAKE) build
	cargo objcopy -- -O binary --remove-section=.eh_frame $(IMG)


.PHONY: run-elf
run-elf: ## Run the ELF in QEMU
	$(MAKE) build
	$(QEMU) $(QEMU_MACHINE) -kernel "$(ELF)" -display none -serial stdio -monitor none


.PHONY: run-img
run-img: ## Run kernel8.img in QEMU
	$(MAKE) img
	$(QEMU) $(QEMU_MACHINE) -kernel "$(IMG)" -display none -serial stdio -monitor none


.PHONY: log-elf
log-elf: ## Log ELF ARM64 instructions to qemu.log
	$(MAKE) build
	$(QEMU) $(QEMU_MACHINE) -kernel "$(ELF)" -nographic -d in_asm -D "$(LOG_ELF)" -dfilter $(KERNEL_LOG_RANGE)


.PHONY: log-img
log-img: ## Log raw-image ARM64 instructions to qemu_raw.log
	$(MAKE) img
	$(QEMU) $(QEMU_MACHINE) -kernel "$(IMG)" -nographic -d in_asm -D "$(LOG_RAW)" -dfilter $(KERNEL_LOG_RANGE)


.PHONY: disasm
disasm: ## Disassemble the generated ELF
	$(MAKE) build
	cargo objdump -- --disassemble --no-show-raw-insn


.PHONY: sections
sections: ## Display ELF section headers
	$(MAKE) build
	cargo objdump -- --section-headers


.PHONY: hex
hex: ## Display kernel8.img bytes
	$(MAKE) img
	powershell -NoProfile -Command "Format-Hex '.\$(IMG)'"


.PHONY: size
size: ## Display kernel8.img size
	$(MAKE) img
	powershell -NoProfile -Command "Get-Item '.\$(IMG)' | Select-Object Name, Length, LastWriteTime"