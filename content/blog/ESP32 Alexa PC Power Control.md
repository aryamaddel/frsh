---
title: ESP32 Alexa PC Power Control
description: Use an ESP32 and the Espalexa library to turn your PC on and off with Alexa voice commands.
author: Arya Maddel
date: 2024-07-15
---

# ESP32 Alexa PC Power Control

This project uses an ESP32 with the Espalexa library to turn a PC on and off via Alexa voice commands. It works by connecting the ESP32 to the PC’s power switch pins on the motherboard.

# ESP32 Alexa PC Power Control

This project uses an ESP32 with the Espalexa library to turn a PC on and off via Alexa voice commands. It works by connecting the ESP32 to the PC’s power switch pins on the motherboard.

## How It Works

- The ESP32 connects to WiFi and registers as an Alexa-compatible device.
- Alexa commands trigger a simulated power button press on the PC.
- An optocoupler reads the PC’s power status to determine if it’s on or off.
- The status LED reflects the current power state.

## Circuit Diagram

![Circuit Diagram](https://private-user-images.githubusercontent.com/84805906/351709362-cf16a01b-876c-499b-8cb4-d6dbbf6bfb73.jpg?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3Mzg0Mjk1OTgsIm5iZiI6MTczODQyOTI5OCwicGF0aCI6Ii84NDgwNTkwNi8zNTE3MDkzNjItY2YxNmEwMWItODc2Yy00OTliLThjYjQtZDZkYmJmNmJmYjczLmpwZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTAyMDElMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwMjAxVDE3MDEzOFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWJhOTk5MjZkZWVkNzE0YTIzODYxNzEzZmZiNDVhNTQzYmEyNDYwOGE1ODM0YWUzZDllMDU3ZmQwNGQwNGMxMGYmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.5Czze4wI1XyQPCvBKceqdY8HEcvCt6NtrbS_eRblde8)  
_(Ignore the rookie circuit diagram 😅😅)_

## Model Representation

![Model Representation](https://private-user-images.githubusercontent.com/84805906/351709350-e1e2b660-7063-4d3c-94f0-e8d42c636f17.jpg?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3Mzg0Mjk1OTgsIm5iZiI6MTczODQyOTI5OCwicGF0aCI6Ii84NDgwNTkwNi8zNTE3MDkzNTAtZTFlMmI2NjAtNzA2My00ZDNjLTk0ZjAtZThkNDJjNjM2ZjE3LmpwZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTAyMDElMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwMjAxVDE3MDEzOFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTliYzk0ZWVhYTMwZTI3NjRjNTgxMmZhMjc4YTYwMWJhZWZjYWY2OGRjMDMwZDEzMmM3YmM3MzA0OGFmOTk3N2MmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.SnB5NLv72zVATbAZUirGXkZzMl6hhXwAWTLaS31cMBQ)
