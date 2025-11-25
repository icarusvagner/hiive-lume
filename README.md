# 🌟 Hiive Lume

> **Modernizing Human Resource Operations with Precision, Speed, and Native Rust UI.**

Hiive Lume is a next‑generation desktop Human Resource & Operations Management system built using **Rust**, **gpui**, and **gpui-component**. Inspired by Morphiq Lume but architected for a fully declarative, component‑driven UI, Hiive Lume delivers a fast, clean, and modern HR experience.

---

## 📖 Theory Behind Hiive Lume

> **“Clarity Through Component‑Driven Intelligence”**

Traditional HRIS platforms often suffer from cluttered workflows and slow interfaces. Hiive Lume introduces a new philosophy:

✅ **"Visual Clarity, Structural Simplicity"**

Where:

* **Hiive** = A harmonious hub where modular components work together
* **Lume** = Clarity, insight, and well‑organized human workflows

Together they form a system that is **clean, fast, adaptive, and pleasant to use**.

---

## 🧩 Core Features

| Module                        | Description                                            |
| ----------------------------- | ------------------------------------------------------ |
| 🧑‍💼 **Employee Management** | Rich profiles, departments, ranks, tagging, and search |
| ⏱️ **Attendance Tracking**    | Manual input + device‑sync‑ready interface             |
| 💰 **Payroll Logic**          | Sync with attendance; rule‑driven computations         |
| 🗓️ **Events & Scheduling**   | Manage events, invitations, and participation          |
| 🌴 **Leave Management**       | Leave types, approvals, tracking, and quota logic      |
| 📄 **Document Inbox**         | Intake for resumes, letters, and internal memos        |

---

## 🌐 Optional & Upcoming Modules

* 🔐 Biometric integrations (ZKTeco, Suprema)
* 📊 gpui‑based dashboards & charts
* 🧾 Asset issuance & tracking
* 🎯 Performance Management
* 🧪 Applicant Tracking & Recruitment

---

## 🛠️ Project Stack

* 🦀 **Rust** — memory‑safe, high‑performance, offline‑first
* 🐝 **gpui** — high‑performance declarative native UI
* 📦 **gpui-component** — reusable UI toolkit for consistent styling
* 🗂️ SQLite or PostgreSQL (via SQLx) for storage
* 🔄 `serde`, `uuid`, `chrono` for serialization & data handling

---

## 📦 Getting Started

### 🔧 Requirements

* Rust toolchain (stable)
* Linux 🐧 / Windows 🪟 / macOS 🍎 support

### 🏃 Run It

```bash
git clone https://github.com/your-org/hiive_lume.git
cd hiive_lume
cargo run --release
```

---

## 💻 Screenshots

Coming soon… ✨

---

## 🧠 System Philosophy

### 📘 "Clarity Through Component Architecture"

Each Hiive Lume feature is built from small, reusable components that:

* Encourage UI clarity
* Reduce complexity
* Improve iteration speed

### 🧬 Architecture Highlights

| Principle        | Implementation                                                     |
| ---------------- | ------------------------------------------------------------------ |
| Component‑Driven | All views composed from `gpui-component` widgets                   |
| Native‑Fast      | Zero Electron, zero web engine—full Rust performance               |
| Modular          | Screens separated into `ui/screens`, components in `ui/components` |
| Secure           | No telemetry, offline‑first, local database encrypted (optional)   |

---

## 🧱 Component Catalog (gpui + gpui-component)

Hiive Lume uses a library of consistent components.

---

## 🛠️ Packaging Metadata (Cargo.toml)

### Debian

```toml
[package.metadata.deb]
maintainer = "Your Name <you@example.com>"
license-file = ["LICENSE"]
assets = [["assets/icon.png", "/usr/share/pixmaps/hiive-lume.png", "644"]]
```

### Windows (.msi)

```toml
[package.metadata.wix]
upgrade-guid = "YOUR-GUID"
path-guid = "YOUR-GUID"
```

### macOS

```toml
[package.metadata.bundle]
bundle_name = "Hiive Lume"
icon = "assets/icon.icns"
```

---

## 🤝 Contributing

We welcome:

* Rust developers
* gpui component creators
* UX designers

---

## 📜 License

Choose between:

* MIT
* Apache 2.0

---

## 👤 Maintainers

* **Akaza Ruthven** – Founder, Lead Engineer
* **Devixion** – UI/Architecture

---

## 📬 Contact

📧 [info@hiive.com](mailto:info@hiive.com)

---

> *“Clean. Native. Fast. A new era of Rust-powered HR systems.”*

