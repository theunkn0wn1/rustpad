from __future__ import annotations

from pathlib import Path
from typing import List

import attr
import click
import jinja2
import toml

env = jinja2.Environment(loader=jinja2.PackageLoader("generator"), trim_blocks=True, lstrip_blocks=True)


@attr.dataclass(frozen=True, slots=True)
class Event:
    code: int = attr.ib(validator=attr.validators.instance_of(int), converter=int)
    name: str = attr.ib(validator=attr.validators.instance_of(str))
    """ underlying event kind, OR'ed against the code """

    def encode(self, kind: int):
        return kind << 16 | self.code


@attr.dataclass(frozen=True)
class ButtonEvent:
    pressed: Event
    released: Event


@attr.dataclass(frozen=True)
class AxisEvent:
    action: Event


@attr.dataclass(frozen=True)
class TwoWaySwitchEvent:
    high: Event
    neutral: Event


@attr.dataclass(frozen=True)
class ThreeWaySwitchEvent(TwoWaySwitchEvent):
    low: Event


@click.command("build")
@click.argument("path")
def cli(path: str):
    target = Path(path)
    del path
    if not target.exists():
        raise click.Abort(f"{target.absolute()} does not exist.")

    data = toml.load(target)
    axis_events: List[AxisEvent] = [AxisEvent(action=Event(name=f"{base}Action", code=key)) for key, base in
                                    data['axes'].items()]

    button_events: List[ButtonEvent] = []

    for key, base in data['buttons'].items():
        button_events.append(ButtonEvent(
            pressed=Event(name=f"{base}Pressed", code=key),
            released=Event(name=f"{base}Released", code=key)
        ))

    two_way_events: List[TwoWaySwitchEvent] = []
    for obj in data['two_way'].values():
        neutral_name = obj['neutral']
        del obj['neutral']
        code = int(list(obj.keys())[0])
        neutral_event = Event(name=neutral_name, code=code)
        high_event = Event(name=obj[f"{code}"], code=code)
        two_way_events.append(TwoWaySwitchEvent(high=high_event, neutral=neutral_event))

    event_names = [event.action.name for event in axis_events]

    for event in button_events:
        event_names.append(event.pressed.name)
        event_names.append(event.released.name)
    for event in two_way_events:
        event_names.append(event.high.name)
        event_names.append(event.neutral.name)

    print(event_names)
    enum_template = env.get_template("enum.jinja")
    decoder_template = env.get_template("decode.rs.jinja2")
    module_template = env.get_template("mod.rs.template")

    rendered_enum = enum_template.render(events=sorted(event_names))
    rendered_decoder = decoder_template.render(button_events=button_events, two_way_events=two_way_events)
    rendered_module = module_template.render(enum=rendered_enum, decoder=rendered_decoder, joystick="warthog")
    output_dir = Path() / "src" / "thrustmaster.rs"
    output_dir.write_text(rendered_module)


cli()
