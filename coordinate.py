import pyautogui

print("Move your mouse to the Tournament tab and press 'Ctrl+C' to stop.")
try:
    while True:
        x, y = pyautogui.position()  # Get current mouse position
        print(f"X: {x}, Y: {y}", end="\r")
except KeyboardInterrupt:
    print("\nDone!")
