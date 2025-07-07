# Example User Flow

This document outlines a typical user interaction with the weather bot, demonstrating the main features.

## Flow Pseudocode

```
USER: START BOT (/start)
BOT: Shows main menu with "Current weather", "Forecast", "Interested towns", "Weather Alerts"

USER: SELECTS "Interested towns"
BOT: Shows towns menu with "Set Home Town", "Add Interested Town"

USER: SELECTS "Set Home Town"
BOT: Asks for home town name.

USER: SENDS "Kyiv"
BOT: Confirms "Home town set to: Kyiv" and shows towns menu. Home town is now visible.

USER: SELECTS "Add Interested Town"
BOT: Asks for interested town name.

USER: SENDS "Lviv"
BOT: Confirms "Added 'Lviv' to your interested towns" and shows towns menu with "Lviv" listed.

USER: SELECTS "Current weather" from main menu (via "Back to Main Menu")
BOT: Shows current weather menu.

USER: SELECTS "For home"
BOT: Shows current weather for Kyiv.

USER: SELECTS "Forecast" from main menu
BOT: Shows forecast menu.

USER: SELECTS "For any city"
BOT: Asks for city name.

USER: SENDS "Odesa"
BOT: Shows 3-day forecast for Odesa.

USER: SELECTS "Weather Alerts" from main menu
BOT: Shows alerts menu.

USER: SELECTS "Add Alert"
BOT: Shows alert type selection.

USER: SELECTS "Temperature Alert"
BOT: Asks for city name for the alert.

USER: SENDS "Kharkiv"
BOT: Asks for minimum temperature.

USER: SENDS "10"
BOT: Asks for maximum temperature.

USER: SENDS "30"
BOT: Asks for forecast horizon in hours.

USER: SENDS "24"
BOT: Confirms alert creation for Kharkiv and shows alerts menu. 