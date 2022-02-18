import based58
import base58

TO_DECODE = b"he11owor1d"
TO_ENCODE = bytes([0x04, 0x30, 0x5E, 0x2B, 0x24, 0x73, 0xF0, 0x58])


def test_based58_decode(benchmark):
    benchmark(based58.b58decode, TO_DECODE)


def test_base58_decode(benchmark):
    benchmark(base58.b58decode, TO_DECODE)


def test_based58_encode(benchmark):
    benchmark(based58.b58encode, TO_ENCODE)


def test_base58_encode(benchmark):
    benchmark(base58.b58encode, TO_ENCODE)
