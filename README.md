# Bonding Curve Contract

### Description
The Bonding Curve is a decentralized mechanism to launch and trade tokens directly on-chain. It allows users to buy and sell tokens against an **automated price curve**, ensuring fair pricing, smooth token distribution, and predictable fundraising. The curve supports configurable limits, fees, and virtual reserves for price stability, making it ideal for controlled token launches or continuous bonding curves.

---

## Table of Contents
- [Bonding Curve Contract](#bonding-curve-contract)
    - [Description](#description)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [How It Works](#how-it-works)
    - [1. **Two-sided Liquidity Pool**](#1-two-sided-liquidity-pool)
    - [2. **Virtual Reserves**](#2-virtual-reserves)
    - [3. **Curve Limit**](#3-curve-limit)
    - [4. **Fees**](#4-fees)
    - [5. **Reserved Space**](#5-reserved-space)
  - [Configuration Parameters](#configuration-parameters)
  - [Initialization Example](#initialization-example)

---

## Introduction

`SUBH` Token Bonding Curve enables a fully on-chain token sale without relying on a centralized exchange. The bonding curve automatically adjusts the token price based on liquidity and supply, providing a fair and transparent market for early buyers and long-term holders.

---

## How It Works

Let’s assume you have a token named `MOONBEANS`.

### 1. **Two-sided Liquidity Pool**
- MOONBEANS tokens (real + virtual reserves)
- SOL liquidity (real + virtual reserves)

Users send SOL to buy MOONBEANS or send MOONBEANS to receive SOL.

---

### 2. **Virtual Reserves**
- Virtual MOONBEANS and SOL smooth out the price curve.
- Prevents early buyers from experiencing extreme volatility or price jumps.

---

### 3. **Curve Limit**
- Sets a maximum SOL cap (e.g., fundraising goal).
- Stops trading once the cap is hit, locking further buys.

---

### 4. **Fees**
- Buy, sell, and migration fees are automatically collected.
- Sent to the configured fee recipient wallet.

---

### 5. **Reserved Space**
- Reserved bytes allow future upgrades without breaking the on-chain config account layout.

---

## Configuration Parameters

| Parameter | Description | Example |
|-----------|-------------|---------|
| `authority` | Admin wallet controlling the config | `YourWalletPubkey` |
| `fee_recipient` | Wallet receiving collected fees | `TeamWalletPubkey` |
| `curve_limit` | Max SOL the curve can collect | `500 SOL` |
| `initial_real_token_reserve` | Actual tokens in the curve | `800,000,000 MOONBEANS` |
| `initial_virtual_token_reserve` | Virtual tokens to smooth curve | `200,000,000 MOONBEANS` |
| `initial_real_sol_reserve` | SOL liquidity provided by creator | `10 SOL` |
| `initial_virtual_sol_reserve` | Virtual SOL to smooth curve | `1,000 SOL` |
| `total_token_supply` | Total minted tokens | `1,000,000,000 MOONBEANS` |
| `buy_fee_percentage` | Fee on buys | `1%` |
| `sell_fee_percentage` | Fee on sells | `1%` |
| `migration_fee_percentage` | Fee for migration/redemption | `0.5%` |
| `reserved` | Future upgrade space | `[[0;8];8]` |

---

## Initialization Example

```json
{
  "authority": "YourPubkeyHere",
  "fee_recipient": "YourTeamWalletHere",
  "curve_limit": 500000000000,
  "initial_virtual_token_reserve": 200000000,
  "initial_virtual_sol_reserve": 1000000000000,
  "initial_real_token_reserve": 800000000,
  "total_token_supply": 1000000000,
  "buy_fee_percentage": 0.01,
  "sell_fee_percentage": 0.01,
  "migration_fee_percentage": 0.005,
  "reserved": [
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0]
  ]
}
