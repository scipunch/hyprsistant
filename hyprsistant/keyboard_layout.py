from collections import defaultdict
from enum import Enum

from hyprpy import Hyprland


_window_to_layout = defaultdict(lambda: "en")


def subscribe(instance: Hyprland):
    instance.signal_active_window_changed.connect(_window_changed)


def _window_changed(sender: Hyprland, active_window_address: str, **kwargs):
    print(f"Focus changed to {active_window_address=}, {type(sender)=}")
