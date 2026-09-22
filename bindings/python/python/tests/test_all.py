from .common import FORMAT, SEPARATOR


def test_import():
    from template_cke_py import TemplateChunkKeyEncoding  # noqa


def test_sum_as_string():
    from template_cke_py import TemplateChunkKeyEncoding

    cke = TemplateChunkKeyEncoding(FORMAT, SEPARATOR)
    encoded = cke.encode_chunk_key((0, 1, 2, 3, 4, 5, 6))
    assert encoded == "potato0-2/004_1:3:5,6suffix"


def test_from_dict():
    from template_cke_py import TemplateChunkKeyEncoding

    cke = TemplateChunkKeyEncoding.from_dict(
        {
            "name": "template",
            "configuration": {
                "format": FORMAT,
                "separator": SEPARATOR,
            },
        }
    )
    assert type(cke) is TemplateChunkKeyEncoding
    assert cke.format == FORMAT
    assert cke.separator == SEPARATOR


def test_to_dict():
    from template_cke_py import TemplateChunkKeyEncoding

    cke = TemplateChunkKeyEncoding(FORMAT, SEPARATOR)
    d = cke.to_dict()
    assert d == {
        "name": "template",
        "configuration": {
            "format": FORMAT,
            "separator": SEPARATOR,
        },
    }
