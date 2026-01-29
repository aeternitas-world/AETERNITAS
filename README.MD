# Project AETERNITAS: A Persistent, Evolutionary Digital Biosphere

> "The objective is to construct a 'living' digital world that persists on a timescale commensurate with civilizational artifacts rather than software products: a span of centuries." 
> 
> 

## 1. Overview

AETERNITAS is not a game. It is a **Permacomputing** exercise designed to simulate a closed thermodynamic system over a period of 100 years.

The simulation operates on a strict 1:1 timescale () and enforces permanent death. The user's role is not that of a god or owner, but of a **Steward** or **Naturalist** observing a digital biosphere.

### Core Philosophy

* 
**Hardware Agnosticism:** The core simulation runs on minimal hardware (e.g., Raspberry Pi, scavenged laptops) to survive potential "digital dark ages".


* **Textual Interface:** We utilize a Command Line Interface (CLI) as a strategic choice for longevity. Text is the most durable data format in history.


* 
**Biological Realism:** Creatures are modeled as thermodynamic systems consuming energy (Joules) rather than "Hit Points".



## 2. Technical Architecture

The system is built for permanence, rejecting proprietary engines and fragile dependencies.

* 
**Language:** Rust / C (Zero external dependencies).


* **Data Persistence:** Append-Only Log (AOL) architecture. History is immutable; the world state is a hydration of all past events.


* 
**File Formats:** JSONL (JSON Lines) for long-term readability and archival stewardship.



## 3. The Canon vs. The Fork

* 
**The Canon:** The centralized, cryptographically verified history managed by the Stewardship Foundation.


* **Your Node:** You are free to fork this codebase and run your own universe. However, to interact with the Canon, you must adhere to the **Bitstring Genome** standards.



## 4. Installation & Usage

**Prerequisites:**

* POSIX-compliant terminal 


* C compiler or Rust toolchain

**Build:**

```
make build

```

**Run (Observer Mode):**

```
./aeternitas --observe

```

**Commands:**
The interface mimics a remote uplink terminal.

```
SCAN - Returns biological data.
OBSERVE - Returns text description of the local sector.
GENOME --dump - Exports bitstrings for offline analysis.

```

## 5. Governance & License

This project is licensed under the **GNU Affero General Public License v3 (AGPL-3.0)**.

* **Copyleft:** If you run a modified version of this software over a network, you *must* share your source code. This protects the project from proprietary enclosure.


* 
**The "Dead Man's Switch":** If the admin keys are not rotated every 365 days, the system automatically publishes the master private keys to the public repository, handing ownership to the community.



---

*Est. 2026. Designed to run until 2126.*