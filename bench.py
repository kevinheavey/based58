import based58
import base58

TO_DECODE = (
    b"5MaiiCavjCmn9Hs1o3eznqDEhRwxo7pXiAYez"
    b"7keQUviUkauRiTMD8DrESdrNjN8zd9mTmVhRvBJeg5vhyvgrAhG"
)
TO_ENCODE = bytes(
    [
        217,
        209,
        11,
        117,
        20,
        116,
        176,
        35,
        106,
        179,
        68,
        60,
        223,
        114,
        201,
        199,
        74,
        254,
        15,
        14,
        253,
        15,
        137,
        41,
        116,
        68,
        148,
        190,
        213,
        78,
        166,
        97,
        71,
        153,
        208,
        10,
        192,
        117,
        37,
        246,
        54,
        188,
        173,
        16,
        84,
        68,
        22,
        36,
        78,
        90,
        214,
        115,
        15,
        225,
        215,
        92,
        166,
        174,
        42,
        53,
        72,
        101,
        107,
        155,
    ]
)

TO_ENCODE_CHECK = b"hello world"
TO_DECODE_CHECK = b"3vQB7B6MrGQZaxCuFg4oh"


def test_based58_decode(benchmark):
    benchmark(based58.b58decode, TO_DECODE)


def test_based58_decode_vec(benchmark):
    benchmark(based58.b58decode_vec, TO_DECODE)


def test_base58_decode(benchmark):
    benchmark(base58.b58decode, TO_DECODE)


def test_based58_encode(benchmark):
    benchmark(based58.b58encode, TO_ENCODE)


def test_base58_encode(benchmark):
    benchmark(base58.b58encode, TO_ENCODE)


def test_based58_decode_check(benchmark):
    benchmark(based58.b58decode_check, TO_DECODE_CHECK)


def test_base58_decode_check(benchmark):
    benchmark(base58.b58decode_check, TO_DECODE_CHECK)


def test_based58_encode_check(benchmark):
    benchmark(based58.b58encode_check, TO_ENCODE_CHECK)


def test_base58_encode_check(benchmark):
    benchmark(base58.b58encode_check, TO_ENCODE_CHECK)
