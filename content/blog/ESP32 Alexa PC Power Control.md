---
title: ESP32 Alexa PC Power Control
description: Use an ESP32 and the Espalexa library to turn your PC on and off with Alexa voice commands.
author: Arya Maddel
date: 2024-07-15
---

# ESP32 Alexa PC Power Control

This project uses an ESP32 with the Espalexa library to turn a PC on and off via Alexa voice commands. It works by connecting the ESP32 to the PC’s power switch pins on the motherboard.

## How It Works

- The ESP32 connects to WiFi and registers as an Alexa-compatible device.
- Alexa commands trigger a simulated power button press on the PC.
- An optocoupler reads the PC’s power status to determine if it’s on or off.
- The status LED reflects the current power state.

## Circuit Diagram

![Circuit Diagram](/blog/ESP32-Alexa-PC-Power-Control/esp-pc-control-circuit.png)

## Model Representation

![Model Representation](/blog/ESP32-Alexa-PC-Power-Control/esp-pc-control-model.png)

## [Code](https://github.com/aryamaddel/alexa-pc-control/blob/main/main.ino)
