from hyprpy import Hyprland

from hyprsistant import keyboard_layout

def main():
    instance = Hyprland()
    keyboard_layout.subscribe(instance)
    instance.watch()


if __name__ == "__main__":
    main()
