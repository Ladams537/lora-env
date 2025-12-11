# Distributed LoRaWAN Environmental Sensor Network

![Build Status](https://img.shields.io/badge/build-passing-brightgreen) ![Stack](https://img.shields.io/badge/stack-no__std%20Rust-orange) ![Architecture](https://img.shields.io/badge/architecture-embedded%20to%20cloud-blue)

A high-assurance, distributed sensor network spanning **embedded firmware to cloud-native AI agents**. This project demonstrates full-stack IoT engineering—from bare-metal async Rust on the RP2040, through LoRaWAN mesh networking, to intelligent data processing with PydanticAI agents—all optimized for memory safety, fault tolerance, and extreme power efficiency.

**What makes this project unique:** It's not just another sensor network. It's a complete demonstration of modern systems thinking across the entire stack, from sub-milliamp power budgets to LLM-powered decision making.

---

## 🎯 Project Philosophy

This is a **proof of concept** that showcases:
- **Cross-domain integration:** Hardware, networking, backend, and AI in one coherent system
- **Production-quality patterns:** Even in prototype form, we use the right tools (Embassy, postcard, PydanticAI)
- **Iterative development:** MSV (Minimum Shippable Version) first, optimization later
- **Learning through building:** Documentation reflects the actual development journey

**This is not over-engineered.** Each technology choice solves a specific constraint (power, bandwidth, type safety, etc.).

---

## 🏗️ System Architecture

The system follows a **"Store and Forward"** topology optimized for intermittent connectivity, extreme low power, and intelligent processing.

```
┌─────────────────┐
│  Sensor Node    │ (RP2040 + BME680 + SX1262)
│  - Wake from RTC│
│  - Read sensors │ 
│  - Pack with    │
│    postcard     │
│  - LoRaWAN TX   │
│  - Deep sleep   │
└────────┬────────┘
         │ LoRaWAN (868/915 MHz)
         │ ~12 bytes/packet
         │ 80% reduction vs JSON
         ▼
┌─────────────────┐
│  LoRa Gateway   │ (Raspberry Pi + RAK2245)
│  - Packet RX    │
│  - Forward to   │
│    Network      │
│    Server       │
└────────┬────────┘
         │ TCP/IP
         │
         ▼
┌─────────────────┐
│  TTN / Chirp    │ (The Things Network)
│  Stack          │
│  - LoRaWAN MAC  │
│  - Routing      │
│  - Webhooks     │
└────────┬────────┘
         │ HTTPS POST
         │
         ▼
┌─────────────────┐
│  Backend API    │ (Rust + Axum)
│  - Webhook RX   │
│  - Deserialize  │
│    postcard     │
│  - Write to DB  │
└────────┬────────┘
         │
         ├──────────────┐
         │              │
         ▼              ▼
┌─────────────────┐  ┌──────────────────┐
│  InfluxDB       │  │  PydanticAI      │
│  - Time-series  │  │  Agent           │
│  - Historical   │  │  - Analyze data  │
│    data         │  │  - Detect        │
│  - Grafana      │  │    anomalies     │
│    dashboards   │  │  - Generate      │
└─────────────────┘  │    reports       │
                     │  - Make decisions│
                     └──────────────────┘
```

### Data Flow (End-to-End)

1. **Node Wake:** RP2040 wakes from **Dormant** mode (~180µA) via DS3231 RTC interrupt
2. **Sense:** Reads BME680 environmental data via I2C (~50ms)
3. **Process:** Serializes to binary using `postcard` (12 bytes vs 60+ bytes JSON)
4. **Transmit:** LoRaWAN Uplink via SX1262 (OTAA, ADR enabled)
5. **Sleep:** Powers down all oscillators, enters Dormant mode
6. **Gateway:** Receives packet, forwards to Network Server
7. **Backend:** Deserializes postcard binary, writes to InfluxDB
8. **AI Agent:** Queries recent data, analyzes trends, triggers alerts if needed

**Key Insight:** By using `postcard` instead of JSON, we reduce Time-on-Air by ~80%, which translates to:
- **5x longer battery life** (less TX time = less power)
- **Better compliance** with duty cycle limits (EU868: 1% duty cycle)
- **Reduced collision probability** in dense networks

---

## 🛠️ Hardware Bill of Materials (BOM)

| Component | Specification | Function | Cost (approx) |
|:----------|:--------------|:---------|:--------------|
| **MCU** | Raspberry Pi Pico (RP2040) | Application logic & LoRaWAN MAC | $4 |
| **Radio** | Semtech SX1262 Module | LoRa transceiver (868/915 MHz) | $8 |
| **Sensor** | Bosch BME680 | Temp, Humidity, Pressure, Gas (I2C) | $12 |
| **RTC** | DS3231 (Precision) | External wake-up trigger | $3 |
| **Power** | LiPo 3.7V 2000mAh + 3.3V LDO | Battery power supply | $8 |
| **Gateway** | Raspberry Pi 3/4 + RAK2245 | LoRa gateway (optional if using TTN) | $150 |

**Total per node:** ~$35  
**Expected battery life:** 6-12 months on 2000mAh LiPo (15min TX interval)

### Why These Components?

- **RP2040:** Dual-core ARM Cortex-M0+, native USB, cheap, excellent Rust support via Embassy
- **SX1262:** Lower power than SX127x, better sensitivity, longer range
- **BME680:** Four sensors in one, industry-standard I2C interface
- **DS3231:** Required for Dormant wake-up, maintains time during deep sleep

---

## 📌 Pin Configuration

> **⚠️ Critical:** The **SX1262 BUSY pin** must be connected. Without it, the driver will hang indefinitely waiting for the radio to become ready. This is the #1 cause of "radio not working" issues.

| RP2040 Pin | Function | Device Pin | Notes |
|:-----------|:---------|:-----------|:------|
| **GP18** | SPI0 SCK | SX1262 SCK | |
| **GP19** | SPI0 MOSI | SX1262 MOSI | |
| **GP16** | SPI0 MISO | SX1262 MISO | |
| **GP17** | GPIO Out | SX1262 NSS | Software chip select |
| **GP20** | GPIO In | **SX1262 BUSY** | **Critical** - flow control |
| **GP21** | GPIO In | SX1262 DIO1 | IRQ (RxDone/TxDone) |
| **GP22** | GPIO Out | SX1262 RST | Active-low reset |
| **GP4** | I2C0 SDA | BME680 SDA | Requires 4.7kΩ pull-up |
| **GP5** | I2C0 SCL | BME680 SCL | Requires 4.7kΩ pull-up |
| **GP6** | I2C0 SDA | DS3231 SDA | Shared I2C bus |
| **GP7** | I2C0 SCL | DS3231 SCL | Shared I2C bus |
| **GP10** | GPIO In | DS3231 SQW | Wake interrupt (falling edge) |

**Power Connections:**
- 3.3V to all VCC pins
- GND to all GND pins
- LiPo to LDO input (3.7-4.2V)
- LDO output to RP2040 VSYS (3.3V regulated)

---

## ⚡ Firmware Stack

The firmware leverages **Embassy**, a modern async framework for embedded Rust that brings ergonomic async/await to bare-metal systems.

### Core Technologies

- **Executor:** `embassy-executor` (cooperative multitasking, automatic WFE when idle)
- **HAL:** `embassy-rp` (RP2040 peripheral access with async APIs)
- **LoRaWAN:** `lora-rs` (SX1262 driver + LoRaWAN 1.0.3 MAC)
- **Serialization:** `postcard` (zero-copy binary format with Varints)
- **Logging:** `defmt` (efficient logging over probe-rs)
- **Storage:** `sequential-storage` (flash persistence for frame counters)

### Why Embassy Over RTIC or Bare Metal?

**Embassy** gives you:
- Async/await syntax (no callback hell)
- Zero-cost abstractions (compiles to efficient state machines)
- Excellent driver ecosystem
- Active development and community

**Example:** Reading a sensor and transmitting is just:

```rust
loop {
    let data = sensor.read().await?;
    radio.send(data).await?;
    Timer::after_secs(900).await; // 15 min
}
```

Compare this to manual state machines or RTIC resources. Embassy makes the code readable and maintainable.

### Power Strategy: Dormant Mode Deep Dive

Standard RP2040 sleep modes are insufficient for battery operation:
- **Active:** ~20mA (too high)
- **Sleep (WFI):** ~1.3mA (still too high for >6 month battery life)
- **Dormant:** ~180µA (acceptable, but with caveats)

**Dormant Mode** stops the internal crystal oscillator (XOSC), which means:
- ✅ Ultra-low power consumption
- ✅ All state retained in RAM
- ❌ **No internal timers work** (they need XOSC)
- ❌ **Cannot self-wake**

**Solution:** External DS3231 RTC configured to generate an interrupt at the next wake time. The falling edge on GP10 restarts the RP2040 clock tree, and execution resumes from the `WFE` instruction.

**Power Budget Calculation:**
```
Average current = (I_active × T_active + I_dormant × T_dormant) / T_total

For 15min TX interval:
- Active phase: 50mA for 5 seconds (sensor + radio TX)
- Dormant phase: 0.18mA for 895 seconds

I_avg = (50mA × 5s + 0.18mA × 895s) / 900s
I_avg = (250 + 161) / 900
I_avg = 0.46mA

2000mAh battery / 0.46mA = 4,347 hours = 181 days ≈ 6 months
```

Add margins for temperature, aging, etc. → realistic 4-5 month battery life per charge.

---

## 📡 Data Protocol & Serialization

### Why Not JSON?

JSON is great for humans, terrible for constrained devices:

```json
{"temp":24.5,"humidity":45,"pressure":1013,"gas":1250}
```
**Size:** 61 bytes

With `postcard` (binary format):
```rust
#[derive(Serialize)]
struct Telemetry {
    #[serde(rename = "t")]
    temp_c_x100: i16,    // 2450 = 24.50°C (Varint encoding)
    #[serde(rename = "h")]
    humidity: u8,        // 0-100%
    #[serde(rename = "p")]
    pressure_hpa: u16,   // 1013 hPa
    #[serde(rename = "g")]
    gas_ohms: u16,       // 1250 Ω
}
```
**Size:** 9-12 bytes (depending on Varint compression)

**Benefits:**
- 80% smaller → 80% less Time-on-Air → 80% less power
- Type-safe (compile-time checks)
- Zero-copy deserialization
- No parsing overhead

**Trade-off:** Requires custom decoder on backend (we provide this in `backend/src/decoder.rs`)

### LoRaWAN Configuration

```rust
// Optimized for battery life
const LORAWAN_CONFIG: Config = Config {
    region: Region::EU868,          // or US915
    join_mode: JoinMode::OTAA,      // Secure activation
    adr: true,                       // Adaptive Data Rate
    max_eirp: 16,                    // dBm (limit power)
    rx1_delay: 5,                    // seconds
    rx2_datarate: DataRate::SF12,   // Fallback for join
};
```

**Spreading Factor Strategy:**
- Start with SF12 (max range, slow)
- Network Server adjusts down via ADR (faster, less power)
- Typical settled state: SF7-SF9

---

## 🚀 Getting Started

### Prerequisites

1. **Rust Toolchain:**
```bash
rustup target add thumbv6m-none-eabi
```

2. **Development Tools:**
```bash
cargo install probe-rs --features cli
cargo install flip-link  # Stack overflow protection
```

3. **Hardware:**
- Picoprobe or any CMSIS-DAP debugger
- Wired to RP2040 SWD pins (SWDIO, SWCLK, GND)

### Configuration

Create `src/config.rs` (do not commit):

```rust
use lorawan_device::region::Region;

pub const LORAWAN_REGION: Region = Region::EU868;

// Get these from your LoRaWAN provider (TTN, Chirpstack, etc.)
pub const DEVEUI: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const APPEUI: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const APPKEY: [u8; 16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

// RTC wake interval (seconds)
pub const WAKE_INTERVAL_SECS: u32 = 900; // 15 minutes
```

### Build and Flash

```bash
# Debug build (includes logging, slower)
cargo run

# Release build (required for production, strict LoRaWAN timing)
cargo run --release
```

**Flashing happens automatically** via probe-rs. Logs will stream to your terminal:

```plaintext
INFO  [main] System initialized
INFO  [main] LoRaWAN joining...
INFO  [lorawan] Join Accept received
INFO  [sensor] BME680: T=24.5°C H=45% P=1013hPa G=1250Ω
INFO  [radio] Encoding with postcard: 12 bytes
INFO  [radio] TX queued (SF7, 125kHz)
INFO  [radio] TX Done (ToA: 41ms)
INFO  [power] Setting DS3231 alarm for +900s
INFO  [power] Entering Dormant mode
```

### Debugging Tips

**Problem:** I2C not working
- Check pull-up resistors (4.7kΩ on SDA/SCL)
- Verify I2C address: `i2cdetect` on Linux, logic analyzer, or try both 0x76/0x77
- Slow down clock: `Config::default().frequency(100_000)` (100kHz)

**Problem:** LoRa radio not transmitting
- **First:** Is BUSY pin connected? 99% of issues
- Check SPI wiring (MISO/MOSI easy to swap)
- Verify frequency matches gateway (868 vs 915 MHz)
- Check antenna is connected (operating without antenna damages radio)

**Problem:** Gateway not receiving
- Use SF12 for first test (max range, most robust)
- Place node <5m from gateway initially
- Check gateway logs (`journalctl -u chirpstack-gateway-bridge -f`)

**Problem:** Code compiles but doesn't run
- Verify `memory.x` is correct for your chip
- Check stack size in `.cargo/config.toml`
- Use `defmt::error!()` liberally to trace execution

---

## ☁️ Backend Architecture

The backend is a lightweight Rust service that bridges LoRaWAN to time-series storage and AI processing.

### Technology Stack

- **Web Framework:** Axum (async, fast, type-safe)
- **Database:** InfluxDB 2.x (optimized for time-series)
- **Deserialization:** `postcard` (matching embedded side)
- **Monitoring:** Grafana (dashboards for sensor trends)

### API Endpoints

**POST /ingest** - Webhook receiver for TTN/Chirpstack
```json
{
  "end_device_ids": {
    "device_id": "node-001",
    "dev_eui": "0000000000000001"
  },
  "uplink_message": {
    "frm_payload": "AYoALQP1BNI=",  // Base64 postcard
    "rx_metadata": [...],
    "settings": {...}
  }
}
```

**Backend processes:**
1. Decodes Base64 to binary
2. Deserializes with `postcard`
3. Writes point to InfluxDB:
```rust
Point::new("environment")
    .tag("device_id", device_id)
    .field("temperature", temp_c)
    .field("humidity", humidity)
    .field("pressure", pressure)
    .field("gas_resistance", gas)
    .timestamp(timestamp)
```

**GET /metrics** - Prometheus metrics for monitoring
- Ingestion rate
- Decode errors
- Database write latency

### Running the Backend

```bash
cd backend

# Configure via environment
export INFLUXDB_URL="http://localhost:8086"
export INFLUXDB_TOKEN="your-token"
export INFLUXDB_ORG="your-org"
export INFLUXDB_BUCKET="sensors"

# Run
cargo run --release
```

**Production:** Deploy via Docker/Kubernetes, use systemd, or serverless (e.g., Fly.io)

---

## 🤖 AI Agent Integration

The PydanticAI agent adds intelligence to the system, moving beyond simple data collection to **automated analysis and decision-making**.

### Agent Capabilities

1. **Anomaly Detection:** Identifies unusual sensor patterns
2. **Trend Analysis:** Compares current data to historical baselines
3. **Alert Generation:** Triggers notifications for critical events
4. **Report Generation:** Creates human-readable summaries
5. **Predictive Insights:** Forecasts potential issues

### Example Agent Code

```python
from pydantic_ai import Agent
from datetime import datetime, timedelta

agent = Agent(
    model="openai:gpt-4o",  # or anthropic:claude-sonnet-4
    system_prompt="""You are an environmental monitoring expert.
    Analyze sensor data and provide actionable insights."""
)

async def analyze_recent_data(device_id: str):
    # Query last 24h from InfluxDB
    data = await query_influxdb(
        device=device_id,
        start=datetime.now() - timedelta(days=1)
    )
    
    result = await agent.run(
        f"""Analyze this sensor data:
        
        Device: {device_id}
        Readings: {data}
        
        1. Identify any anomalies
        2. Compare to typical patterns
        3. Suggest actions if needed
        """
    )
    
    if result.data.get("alert_level") == "high":
        await send_notification(result.data["message"])
    
    return result
```

### Integration Points

**Trigger Options:**
- **Scheduled:** Cron job runs analysis every hour
- **Event-driven:** Lambda/Cloud Function on new data
- **On-demand:** API endpoint for manual queries

**Storage:**
```
InfluxDB → Agent reads historical context
       ↓
   Analysis runs
       ↓
  Results logged to InfluxDB (separate measurement)
       ↓
  Grafana displays both raw data + agent insights
```

### Example Insights

**Input:** 24 hours of temperature data from a greenhouse sensor
**Agent Output:**
```
ALERT: Temperature spike detected at 14:30 (32°C, +8°C from baseline)
PATTERN: Daily temp range has increased 15% over past week
RECOMMENDATION: Check HVAC system and door seals
PREDICTION: If trend continues, expect equipment strain within 3 days
```

### Configuration

```python
# config/agent.yaml
agent:
  model: "anthropic:claude-sonnet-4"
  temperature: 0.3  # Lower = more consistent
  max_tokens: 1000
  
checks:
  - type: "anomaly"
    threshold: 2.0  # Standard deviations
    window: "1h"
    
  - type: "trend"
    metric: "temperature"
    lookback: "7d"
    alert_if: ">10% change"
    
notifications:
  - type: "email"
    recipients: ["alert@example.com"]
    severity: ["high", "critical"]
```

---

## 🛡️ Failure Recovery & Resilience

Production systems fail. This project handles failure gracefully.

### Hardware Watchdog

```rust
let mut watchdog = Watchdog::new(p.WATCHDOG);
watchdog.start(Duration::from_secs(8));

loop {
    watchdog.feed();  // Reset timer
    
    // If this loop hangs (SPI deadlock, etc),
    // watchdog triggers hard reset after 8s
}
```

**Why 8 seconds?**
- LoRaWAN TX can take 4-6s at SF12
- Sensor read takes ~500ms
- Leaves margin for other operations

### LoRaWAN Duty Cycle Management

EU868 has strict duty cycle limits (1% per sub-band). If violated:

```rust
match lorawan.send(&payload).await {
    Ok(_) => info!("TX Success"),
    
    Err(Error::DutyCycleRestricted(wait_ms)) => {
        warn!("Duty cycle hit, backing off {}ms", wait_ms);
        
        // Calculate actual wake time
        let backoff = Duration::from_millis(wait_ms);
        set_rtc_alarm(backoff + NORMAL_INTERVAL);
        
        // Sleep immediately (don't waste power waiting)
        enter_dormant();
    }
    
    Err(e) => error!("TX failed: {:?}", e),
}
```

### Flash Persistence

Frame counters and DevNonce must survive power loss:

```rust
use sequential_storage::{cache::NoCache, map::Map};

// Write frame counter to flash before sleep
async fn persist_state(counter: u32) {
    let mut map = Map::new(flash, flash_range, cache);
    map.store_item("frame_counter", &counter.to_le_bytes()).await?;
}

// Restore on wake
async fn restore_state() -> Option<u32> {
    let mut buf = [0u8; 4];
    map.fetch_item("frame_counter", &mut buf).await.ok()?;
    Some(u32::from_le_bytes(buf))
}
```

**Why this matters:** Without persistence, the node would re-join after every battery swap, wasting network resources and confusing the Network Server.

### Error Handling Philosophy

```rust
// Don't panic on recoverable errors
match sensor.read().await {
    Ok(data) => process(data),
    Err(e) => {
        error!("Sensor read failed: {:?}", e);
        // Log error, continue to next cycle
        // Don't crash the entire system
    }
}
```

**Principle:** Degrade gracefully. One bad sensor reading shouldn't kill the node.

---

## 📊 Monitoring & Observability

### Embedded Logging (defmt)

```rust
use defmt::*;

info!("System boot"); // Green
warn!("Low battery"); // Yellow  
error!("Radio timeout"); // Red
debug!("SPI: {:02x}", data); // Detailed
```

**Advantages over printf:**
- Zero cost when disabled
- Type-safe formatting
- Deferred formatting (smaller binary)

### Backend Metrics

```rust
// Prometheus metrics
metrics::counter!("packets_received_total").increment(1);
metrics::histogram!("decode_duration_ms").record(duration.as_millis());
metrics::gauge!("active_devices").set(count);
```

Scrape with Prometheus, visualize in Grafana.

### Grafana Dashboard Queries

```flux
// Temperature over time
from(bucket: "sensors")
  |> range(start: -24h)
  |> filter(fn: (r) => r._measurement == "environment")
  |> filter(fn: (r) => r._field == "temperature")

// Packet loss rate
from(bucket: "sensors")
  |> range(start: -1h)
  |> filter(fn: (r) => r._measurement == "lorawan")
  |> aggregateWindow(every: 5m, fn: count)
  |> yield(name: "packets_per_5min")
```

---

## 📦 Project Structure

```
.
├── firmware/               # Embedded Rust (RP2040)
│   ├── src/
│   │   ├── main.rs        # Entry point, main loop
│   │   ├── config.rs      # LoRaWAN keys (gitignored)
│   │   ├── sensor.rs      # BME680 driver wrapper
│   │   ├── radio.rs       # SX1262 + LoRaWAN
│   │   ├── power.rs       # Dormant mode, RTC
│   │   └── protocol.rs    # Postcard serialization
│   ├── Cargo.toml
│   └── memory.x           # Linker script
│
├── backend/               # Rust + Axum
│   ├── src/
│   │   ├── main.rs        # HTTP server
│   │   ├── ingest.rs      # Webhook handler
│   │   ├── decoder.rs     # Postcard deserializer
│   │   └── influx.rs      # InfluxDB client
│   └── Cargo.toml
│
├── agent/                 # Python + PydanticAI
│   ├── agent.py           # AI analysis logic
│   ├── config.yaml        # Agent configuration
│   └── requirements.txt
│
├── gateway/               # Configuration for LoRa gateway
│   ├── chirpstack/
│   └── ttn/
│
├── docs/                  # Additional documentation
│   ├── WIRING.md          # Detailed wiring diagrams
│   ├── TROUBLESHOOTING.md # Common issues + solutions
│   └── DEVELOPMENT.md     # Development workflow
│
└── README.md              # This file
```

---

## 🎯 Development Roadmap

### ✅ Phase 1: MSV (Minimum Shippable Version)
- [x] Blink LED (prove toolchain works)
- [x] Read BME680 via I2C
- [x] Send LoRa packet (basic mode)
- [x] LoRaWAN OTAA join
- [x] End-to-end data flow (sensor → cloud)

### ✅ Phase 2: Power Optimization
- [x] Implement Dormant mode
- [x] DS3231 RTC integration
- [x] Battery life testing
- [x] Duty cycle backoff

### ✅ Phase 3: Data Processing
- [x] Postcard serialization
- [x] Backend webhook receiver
- [x] InfluxDB integration
- [x] Grafana dashboards

### 🚧 Phase 4: Intelligence (Current)
- [x] PydanticAI agent framework
- [ ] Anomaly detection
- [ ] Trend analysis
- [ ] Alert system

### 📋 Phase 5: Production Hardening
- [ ] OTA firmware updates
- [ ] Multi-node fleet management
- [ ] Encrypted storage (DevNonce, keys)
- [ ] Network time sync (GPS/LoRaWAN MAC)

### 🔮 Future Ideas
- [ ] Solar panel + supercapacitor
- [ ] Mesh networking (node-to-node relay)
- [ ] Edge ML (TinyML on RP2040)
- [ ] BLE provisioning app

---

## 🧪 Testing Strategy

### Unit Tests (Embedded)

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_postcard_encoding() {
        let data = Telemetry {
            temp_c_x100: 2450,
            humidity: 45,
            pressure_hpa: 1013,
            gas_ohms: 1250,
        };
        
        let encoded = postcard::to_vec(&data).unwrap();
        assert!(encoded.len() <= 12);
        
        let decoded: Telemetry = postcard::from_bytes(&encoded).unwrap();
        assert_eq!(decoded.temp_c_x100, 2450);
    }
}
```

### Integration Tests (Hardware-in-Loop)

```bash
# Flash test firmware
cargo test --release --target thumbv6m-none-eabi

# Automated tests via probe-rs
probe-rs test --chip RP2040
```

### Backend Tests

```rust
#[tokio::test]
async fn test_ingest_endpoint() {
    let app = create_app();
    
    let response = app
        .oneshot(Request::post("/ingest")
            .json(&sample_payload())
            .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
```

---

## 🤝 Contributing

This is primarily a learning/portfolio project, but contributions are welcome!

**Good first issues:**
- Documentation improvements
- Additional sensor drivers (SHT31, BMP280, etc.)
- Power consumption measurements
- Alternative backend implementations (Go, Python)

**Before submitting a PR:**
- Run `cargo fmt` (both firmware and backend)
- Run `cargo clippy -- -D warnings`
- Test on real hardware (not just simulator)
- Update relevant documentation