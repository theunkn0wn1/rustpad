from __future__ import annotations

import typing
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


def axis(name: str, key: int) -> typing.Set[Event]:
    return {Event(name=f"{name}Motion", code=key)}


def button(name: str, key: int) -> typing.Set[Event]:
    return {
        Event(name=f"{name}Pressed", code=key),
        Event(name=f"{name}Released", code=key),
    }


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
    event_names = [event.action.name for event in axis_events]
    event_names.extend(event.pressed.name for event in button_events)
    event_names.extend(event.released.name for event in button_events)

    print(event_names)
    enum_template = env.get_template("enum.jinja")
    decoder_template = env.get_template("decode.rs.jinja2")

    print(enum_template.render(events=sorted(event_names)))
    print(decoder_template.render(button_events=button_events))
    output_dir = Path() / "thrustmaster.rs"

cli()
