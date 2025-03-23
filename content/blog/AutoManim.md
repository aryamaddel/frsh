---
title: AutoManim
description: A Flask-based web application that converts natural language into Manim animations.
author: Arya Maddel
date: 2025-03-13
---
# AutoManim
This project combines Flask and the Groq API to create a web interface that transforms text descriptions into mathematical animations using Manim.

## How It Works
- User enters a description of the desired animation
- Groq LLM converts the description to Manim Python code
- Manim code is executed and renders the animation

## Setup Guide

1. **Clone the repository**
    ```
    git clone https://github.com/aryamaddel/manim-web-generator
    cd manim-web-generator
    ```

2. **Install Manim and dependencies using UV package manager (recommended)**
    - Install UV from their [official website](https://docs.astral.sh/uv/)
    - After installation, run the following command to install Manim and other dependencies:
    ```bash
    uv add manim flask groq
    ```
    - Ensure all Manim requirements are installed by following their [official installation guide](https://docs.manim.community/en/stable/installation.html).

3. **Run the project**
    ```bash
    flask run
    ```


## [Code](https://github.com/aryamaddel/manim-web-generator)