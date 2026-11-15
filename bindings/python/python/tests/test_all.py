from .common import SEPARATOR, TEMPLATE


def test_import():
    from template_cke import TemplateChunkKeyEncoding  # noqa


def test_sum_as_string():
    from template_cke import TemplateChunkKeyEncoding

    cke = TemplateChunkKeyEncoding(TEMPLATE, SEPARATOR)
    encoded = cke.encode_chunk_key((0, 1, 2, 3, 4, 5, 6))
    assert encoded == "potato0-2/004_1:3:5,6suffix"


def test_from_dict():
    from template_cke import TemplateChunkKeyEncoding

    cke = TemplateChunkKeyEncoding.from_dict(
        {
            "name": "template",
            "configuration": {
                "template": TEMPLATE,
                "separator": SEPARATOR,
            },
        }
    )
    assert type(cke) is TemplateChunkKeyEncoding
    assert cke.template == TEMPLATE
    assert cke.separator == SEPARATOR

def test_from_dict_alias():
    from template_cke import TemplateChunkKeyEncoding

    cke = TemplateChunkKeyEncoding.from_dict(
        {
            "name": "template",
            "configuration": {
                "format": TEMPLATE,
                "separator": SEPARATOR,
            },
        }
    )
    assert type(cke) is TemplateChunkKeyEncoding
    assert cke.template == TEMPLATE
    assert cke.separator == SEPARATOR


def test_to_dict():
    from template_cke import TemplateChunkKeyEncoding

    cke = TemplateChunkKeyEncoding(TEMPLATE, SEPARATOR)
    d = cke.to_dict()
    assert d == {
        "name": "template",
        "configuration": {
            "template": TEMPLATE,
            "separator": SEPARATOR,
        },
    }
