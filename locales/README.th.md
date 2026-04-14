<div align="center">

# chronos

**เลเยอร์การรับรู้ AI แบบกราฟพร้อมการพิสูจน์อย่างเป็นทางการ**

[![CI](https://github.com/cntm-labs/chronos/actions/workflows/ci.yml/badge.svg)](https://github.com/cntm-labs/chronos/actions/workflows/ci.yml)
[![Security](https://github.com/cntm-labs/chronos/actions/workflows/security.yml/badge.svg)](https://github.com/cntm-labs/chronos/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-active-success)](./)

<!-- Language Badges -->
![Rust](https://img.shields.io/badge/language-Rust-orange.svg) ![Mojo](https://img.shields.io/badge/language-Mojo-red.svg) ![Lean](https://img.shields.io/badge/language-Lean-purple.svg)

<!-- LOD Badges -->
![Rust LOD](https://img.shields.io/badge/Rust_LOD-0-blue.svg) ![Total LOD](https://img.shields.io/badge/Total_LOD-0-brightgreen.svg)

</div>

---

[ [English](../README.md) | ภาษาไทย | [日本語](./README.ja.md) | [简体中文](./README.zh.md) ]

เลเยอร์การจัดการ (orchestration layer) ที่ผสานสัญลักษณ์ทางคณิตศาสตร์เข้ากับโครงข่ายประสาทเทียม เพื่อให้ AI มีหน่วยความจำและการให้เหตุผลที่ตรวจสอบได้ แก้ปัญหาการสูญเสียความจำและอาการประสาทหลอนของ AI ด้วยการสร้างเลเยอร์หน่วยความจำที่มีการพิสูจน์ความถูกต้องอย่างเป็นทางการ

## ✨ คุณสมบัติหลัก

- 🚀 **เครื่องมือพิสูจน์ความถูกต้อง (Lean-powered)** — กราฟความรู้ที่ปราศจากข้อผิดพลาดโดยใช้คณิตศาสตร์ขั้นสูงเพื่อตรวจสอบตรรกะภายในกราฟ
- 🛡️ **การติดตามความรู้ตามกาลเวลา** — การบันทึกและสืบค้นความสัมพันธ์ของความรู้ตามลำดับเวลาผ่าน BlowTime integration
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
