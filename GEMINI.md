# GEMINI.md: Project Context for `rust-lora-env`

This document provides context for the AI assistant to understand the `rust-lora-env` project.

## 1. Project Overview & Goal

This is an embedded firmware project written in `no_std` Rust for the Raspberry Pi Pico (RP2040).

The ultimate goal is to build a **Distributed LoRaWAN Environmental Sensor Network**. The project serves as a portfolio piece to demonstrate full-stack IoT engineering, from bare-metal firmware to cloud data processing and AI-driven analysis.

**Core Technologies:**
- **MCU:** Raspberry Pi Pico (RP2040)
- **Firmware:** `no_std` Rust with the `embassy` async framework.
- **Radio:** Semtech SX1262 LoRa module.
- **Sensor:** Bosch BME680 (Temperature, Humidity, Pressure, Gas).
- **Real-Time Clock (RTC):** DS3231 for low-power wake-ups.
- **Serialization:** `postcard` for efficient binary data packing.

The project philosophy emphasizes memory safety, power efficiency (targeting <500µA sleep current), and using modern, production-quality patterns even in the prototype stage.

## 2. Current Project Status

The project is currently in the **initial setup phase**.

- The development environment, toolchain, and build configuration are all in place for RP2040 development using Rust and `embassy`.
- The current `src/main.rs` is a simple "blinky" application that confirms the toolchain is working correctly. It blinks the Pico's onboard LED and logs messages via `defmt`.
- The detailed `README.md` file acts as the **architectural plan and development roadmap**. Most features described there (LoRaWAN, sensor integration, power management) are **not yet implemented**.

## 3. Building and Running

The project is built and flashed to the RP2040 using `probe-rs`.

**Prerequisites:**
1.  **Rust Toolchain:**
    ```bash
    rustup target add thumbv6m-none-eabi
    ```
2.  **Development Tools:**
    ```bash
    cargo install probe-rs --features cli
    cargo install flip-link
    ```

**Build & Run Commands:**
- For a debug build (with logging):
  ```bash
  cargo run
  ```
- For a release build (optimized, required for production LoRaWAN timing):
  ```bash
  cargo run --release
  ```

The `.cargo/config.toml` file is configured to use `probe-rs` as the runner for the `thumbv6m-none-eabi` target.

## 4. Project Structure & Key Files

- `README.md`: **The most important file.** It contains the detailed architectural plan, system diagrams, hardware BOM, pinouts, and the development roadmap. **Treat this as the source of truth for the project's goals.**
- `cargo.toml`: Defines the project and its dependencies. Notably, it includes `embassy` and `defmt`. It does *not* yet include LoRaWAN or sensor crates.
- `src/main.rs`: The main firmware entry point. Currently contains "blinky" code.
- `.cargo/config.toml`: Configures the build target (`thumbv6m-none-eabi`) and the `probe-rs` runner.
- `where.md`: A personal context file from the author explaining the motivation behind the project (portfolio piece for career advancement) and defining a Minimum Shippable Version (MSV).
- `docs/pico-lorawan/`: A git submodule containing a **C/C++** LoRaWAN implementation for the Pico and the older **SX1276** radio. This is likely for reference only and is **not** part of the main Rust build.

## 5. Development Conventions

- **Framework:** The `embassy` async framework is the required choice for all firmware development.
- **Configuration:** Sensitive information like LoRaWAN keys (`DEVEUI`, `APPEUI`, `APPKEY`) should be placed in a `src/config.rs` file, which is excluded from git.
- **Logging:** All logging should be done using the `defmt` framework.
- **Power Management:** A key goal is low power consumption using the RP2040's `DORMANT` mode, woken by an external DS3231 RTC.
- **Data Serialization:** Sensor data should be serialized using `postcard` before transmission to minimize Time-on-Air and thus save power.
