from dataclasses import dataclass, field
from typing import ClassVar, Literal, Self

from zarr.core.chunk_key_encodings import ChunkKeyEncoding
from zarr.core.common import JSON, parse_named_configuration
from zarr.registry import register_chunk_key_encoding

from ._template_cke_py import PyInterpolator

__all__ = ["TemplateChunkKeyEncoding"]


@dataclass(frozen=True)
class TemplateChunkKeyEncoding(ChunkKeyEncoding):
    name: ClassVar[Literal["template"]] = "template"

    template: str
    separator: str | None = None
    _interpolator: PyInterpolator = field(init=False)

    def __post_init__(self):
        interp = PyInterpolator(self.template, self.separator)
        object.__setattr__(self, "_interpolator", interp)

    @classmethod
    def from_dict(cls, data: dict[str, JSON]) -> Self:
        _, config_parsed = parse_named_configuration(data, require_configuration=False)
        if config_parsed is None:
            raise ValueError("Config must exist")
        if "template" not in config_parsed and "format" in config_parsed:
            config_parsed["template"] = config_parsed.pop("format")
        fmt = config_parsed["template"]
        if not isinstance(fmt, str):
            raise ValueError("template must be a string")
        separator = config_parsed.get("separator")
        if separator is not None and not isinstance(separator, str):
            raise ValueError("separator must be a string if given")
        return cls(fmt, separator)

    def to_dict(self) -> dict[str, JSON]:
        c = {"template": self.template}
        if self.separator is not None:
            c["separator"] = self.separator
        return {"name": self.name, "configuration": c}

    def encode_chunk_key(self, chunk_coords: tuple[int, ...]) -> str:
        return self._interpolator.encode(chunk_coords)


register_chunk_key_encoding(TemplateChunkKeyEncoding.name, TemplateChunkKeyEncoding)
