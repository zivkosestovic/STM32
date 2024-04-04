In order to test each MCU these steps are to be followed:

1. In the system/src/core_headers folder pick the appropriate MCU folder's name and copy its core_header.rs file into the system/src folder.

2. In the system/src/linker_scripts folder pick the appropriate MCU folder's name and copy its memory.rs file into the rust_tests folder.

3. In the rust_tests/Cargo.rs file:

stm32f4xx-hal = { version = "0.20.0",  features = ["stm32f411"], path = "dependencies/stm32f4xx-hal-0.20.0" }

stm32f4xx-hal = { version = "0.20.0",  features = ["stm32f469"], path = "dependencies/stm32f4xx-hal-0.20.0" }

stm32f7xx-hal = { version = "0.7.0",   features = ["stm32f723"], path = "dependencies/stm32f7xx-hal-0.7.0"  }

stm32f1xx-hal = { version = "0.10.0",  features = ["stm32f103"], path = "dependencies/stm32f1xx-hal-0.10.0" }

Pick the appropriate HAL dependency for the MCU and comment out the rest.

4. In the main.rs of the rust tests folder comment/uncomment each test to be executed.

5. While in the root folder (rust_tests) of VSC terminal type in the next command:

cargo flash --chip ((chip name)) --features ((chip feature name)) --target thumbv7em-none-eabihf --probe ((your ST-LINK ID))

*=========== targets ==========*

--target thumbv6m-none-eabi - stm32f042c6, stm32l073rz, stm32g070rb
--thumbv7em-none-eabihf     - for all other MCU's

*========================= ST-LINK ID =========================*

Install probe-rs with: cargo install probe-rs --features cli
Type probe-rs list in the VSC terminal, a list will pop out and you can check your ST-LINK ID.

Example:

> probe-rs list
The following debug probes were found:
[0]: T1-C-7 [CMSIS-DAP] (VID: 2dbc, PID: 1924, Serial: ME2023089880002, CmsisDap)
[1]: STLink V2 (VID: 0483, PID: 3748, Serial: 36263338324337463626302631, StLink)

Example:

cargo flash --chip stm32f732zetx --features stm32f723ze --target thumbv7em-none-eabihf --probe 0483:3748

*==============================   FPU TESTING   ==============================*

1. In the Cargo.toml file turn off the optimizations:

Example:

[profile.release]            

opt-level = 0

lto = false

[profile.dev]

opt-level = 0

lto = false

2. Comment out the conditional compilation for the appropriate MCU in the main.rs file, except for the "use stm32f4xx_hal as _":

Example:

// #[cfg(feature = "stm32f469ii")]

// pub mod test_stm32f469ii;

// #[cfg(feature = "stm32f469ii")]

// pub use test_stm32f469ii as mcu_test;

// #[cfg(feature = "stm32f469ii")]

use stm32f4xx_hal as _;

3. Uncomment the equation below the "FPU testing" banner in the main.rs file

3. In the root folder (rust tests) type in the next commands:

- cargo clean
- cargo rustc --release -- --emit asm -C "codegen-units=1"

4. Navigate to target/thumbv7em-none-eabihf/release/deps/rusty_tests-9396662dcc958b82.s

5. Enter this *.s file and look for FPU operations such as "vmov", "vldr", etc.

*==============================   STM32G070RB   ==============================*

1. cargo build --release  --features stm32g070rb --target thumbv6m-none-eabi

2. Install arm-none-eabi and navigate to the thumbv6m-none-eabi/release folder in cmd and enter the next command: arm-none-eabi-objcopy -O elf32-littlearm rusty_tests rusty_tests.elf

3. Flash the elf file in the STM32CubeProgrammer.