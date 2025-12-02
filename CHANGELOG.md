# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-02

### Added

- **程序監控功能**
  - 顯示執行中的程序資訊（PID、名稱、CPU 使用率、記憶體使用、磁碟 I/O）
  - 支援顯示前 N 個程序（`--top <N>`）
  - 支援依 CPU 或記憶體排序（`--sort-cpu`）
  - 預設顯示包含 top 10 程序（依 CPU 排序）

- **Hosts 檔案顯示**
  - 顯示 hosts 檔案內容（IP 位址和主機名稱）
  - 自動過濾註解（可選 `--show-comments` 顯示）
  - 跨平台支援（Windows/Linux/macOS）
  - 優雅處理權限錯誤

- **架構資訊顯示**
  - System Information 新增架構欄位（32-bit/64-bit）
  - CPU Information 新增詳細架構資訊（如：64-bit (x86_64)）

### Improved

- **電池資訊增強**
  - 無電池時顯示友善提示訊息（如：桌上型電腦）
  - 32-bit Windows 顯示不支援訊息
  - 錯誤時顯示具體失敗原因

- **文檔更新**
  - README.md 改為中英雙語版本
  - 新增遠端安裝說明（從 GitHub Release）
  - 新增從原始碼編譯說明
  - 更新所有功能說明和範例

### Changed

- BatteryInfo 結構調整為包含 data 和 error 欄位
- 預設顯示現在包含程序資訊（top 10，依 CPU 排序）

## [0.1.0] - 2025-01-XX

### Added

- **系統資訊收集**

  - CPU 資訊（型號、核心數、使用率、頻率）
  - 記憶體資訊（總量、已用、可用、使用率）
  - 系統資訊（作業系統、內核版本、主機名）

- **硬體資訊收集**

  - 電池資訊（電量、充電狀態、健康度、溫度）
  - 磁盤資訊（掛載點、容量、檔案系統、使用率）
  - 網路資訊（介面名稱、傳輸/接收流量、封包統計）
  - 溫度監控（CPU 和組件溫度、最高溫度、臨界溫度）

- **輸出功能**

  - 彩色表格輸出
  - JSON 格式輸出
  - 選擇性顯示特定資訊
  - 即時監控模式（`--watch`）
  - 自訂監控間隔（`--interval`）

- **跨平台支援**
  - Windows (x86_64, i686)
  - Linux (x86_64, i686, ARMv7, ARM64) - glibc 版本
  - Linux (x86_64, i686, ARMv7, ARM64) - musl 靜態鏈接版本
  - macOS (Intel x86_64, Apple Silicon ARM64)

### Technical Details

- 使用 pico-args 實現輕量級 CLI 參數解析
- 使用 sysinfo 收集系統資訊
- 使用 battery crate 收集電池資訊
- 使用 comfy-table 實現美觀的表格輸出
- 懶加載設計，僅收集請求的資訊
