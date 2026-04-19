<div align="center">

# chronos

**เลเยอร์การรับรู้ AI แบบกราฟพร้อมการพิสูจน์อย่างเป็นทางการ**



[![CI](https://github.com/cntm-labs/chronos/actions/workflows/ci.yml/badge.svg)](https://github.com/cntm-labs/chronos/actions/workflows/ci.yml)
[![Security](https://github.com/cntm-labs/chronos/actions/workflows/security.yml/badge.svg)](https://github.com/cntm-labs/chronos/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

![Rust LOD](https://img.shields.io/badge/Rust_LOD-0-dea584.svg) ![Mojo LOD](https://img.shields.io/badge/Mojo_LOD-0-CC0000.svg) ![Lean LOD](https://img.shields.io/badge/Lean_LOD-0-7B68EE.svg) ![Total LOD](https://img.shields.io/badge/Total_LOD-0-brightgreen.svg)

[![Rust](https://img.shields.io/badge/Rust-dea584?logo=rust&logoColor=white)](./) [![Mojo](https://img.shields.io/badge/Mojo-CC0000?logo=mojo&logoColor=white)](./) [![Lean](https://img.shields.io/badge/Lean-7B68EE?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAiIGhlaWdodD0iMjAiIHZpZXdCb3g9IjAgMCAyMCAyMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48dGV4dCB4PSIyIiB5PSIxNSIgZm9udC1zaXplPSIxNCIgZmlsbD0id2hpdGUiPkw8L3RleHQ+PC9zdmc+&logoColor=white)](./) [![CUDA](https://img.shields.io/badge/CUDA-76B900?logo=nvidia&logoColor=white)](./) [![Axum](https://img.shields.io/badge/Axum-dea584?logo=rust&logoColor=white)](./) [![PyTorch](https://img.shields.io/badge/PyTorch-EE4C2C?logo=pytorch&logoColor=white)](./)

</div>

---

[ [English](../README.md) | ภาษาไทย | [日本語](./README.ja.md) | [简体中文](./README.zh.md) ]

เลเยอร์การจัดการ (orchestration layer) ที่ผสานสัญลักษณ์ทางคณิตศาสตร์เข้ากับโครงข่ายประสาทเทียม เพื่อให้ AI มีหน่วยความจำและการให้เหตุผลที่ตรวจสอบได้ แก้ปัญหาการสูญเสียความจำและอาการประสาทหลอนของ AI ด้วยการสร้างเลเยอร์หน่วยความจำที่มีการพิสูจน์ความถูกต้องอย่างเป็นทางการ

## ✨ คุณสมบัติหลัก

- 🚀 **เครื่องมือพิสูจน์ความถูกต้อง (Lean-powered)** — กราฟความรู้ที่ปราศจากข้อผิดพลาดโดยใช้คณิตศาสตร์ขั้นสูงเพื่อตรวจสอบตรรกะภายในกราฟ
- 🛡️ **การติดตามความรู้ตามกาลเวลา** — การบันทึกและสืบค้นความสัมพันธ์ของความรู้ตามลำดับเวลาผ่าน Isotime integration
- 📊 **โมเดลการรับรู้เชิงทำนาย (Mojo/CUDA)** — การพยากรณ์เหตุการณ์ในอนาคตตามความสัมพันธ์ของกราฟด้วย neural-symbolic fusion

## 🛠️ เริ่มต้นใช้งาน

```bash
# สร้าง orchestration layer
cargo build --release

# รันชุดทดสอบ
cargo test

# รัน Lean proofs
lake build
```

## 🗺️ เมนูนำทาง

- 🏗️ **[สถาปัตยกรรม](../ARCHITECTURE.md)** — การออกแบบและส่วนประกอบหลัก
- 📅 **[แผนงาน](../ROADMAP.md)** — ไทม์ไลน์และเป้าหมาย
- 🤝 **[การมีส่วนร่วม](../CONTRIBUTING.md)** — วิธีเข้าร่วมและช่วยเหลือ
- 🌳 **[โครงสร้างโปรเจกต์](../STRUCTURE.tree)** — แผนที่ไฟล์ทั้งหมด

## ⚖️ สัญญาอนุญาต

[MIT](../LICENSE)
