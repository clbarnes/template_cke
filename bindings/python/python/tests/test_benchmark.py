from template_cke import TemplateChunkKeyEncoding

from .common import SEPARATOR, TEMPLATE


def test_bench_instantiation(benchmark):
    benchmark(TemplateChunkKeyEncoding, TEMPLATE, SEPARATOR)


def test_bench_encode(benchmark):
    cke = TemplateChunkKeyEncoding(TEMPLATE, SEPARATOR)
    chunk_idx = (0, 1, 2, 3, 4, 5, 6)
    benchmark(cke.encode_chunk_key, chunk_idx)
