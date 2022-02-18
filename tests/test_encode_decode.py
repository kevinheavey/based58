# note: these tests are mostly copied from the base58 library
# https://github.com/keis/base58/blob/master/test_base58.py
from pytest import raises, fixture, mark
from itertools import product
from random import getrandbits
from based58 import (
    b58encode,
    b58decode,
    b58encode_check,
    b58decode_check,
    Alphabet,
)


@fixture(params=[Alphabet.BITCOIN, Alphabet.RIPPLE])
def alphabet(request) -> str:
    return request.param


def test_simple_encode():
    data = b58encode(b"hello world")
    assert data == b"StV1DL6CwTryKyV"


def test_leadingz_encode():
    data = b58encode(b"\0\0hello world")
    assert data == b"11StV1DL6CwTryKyV"


def test_encode_empty():
    data = b58encode(b"")
    assert data == b""


def test_simple_decode():
    data = b58decode(b"StV1DL6CwTryKyV")
    assert data == b"hello world"


def test_simple_decode_bytes():
    data = b58decode(b"StV1DL6CwTryKyV")
    assert data == b"hello world"


def test_leadingz_decode():
    data = b58decode(b"11StV1DL6CwTryKyV")
    assert data == b"\0\0hello world"


def test_leadingz_decode_bytes():
    data = b58decode(b"11StV1DL6CwTryKyV")
    assert data == b"\0\0hello world"


def test_empty_decode():
    data = b58decode(b"1")
    assert data == b"\0"


def test_empty_decode_bytes():
    data = b58decode(b"1")
    assert data == b"\0"


def test_check_str():
    data = b"hello world"
    out = b58encode_check(data)
    assert out == b"3vQB7B6MrGQZaxCuFg4oh"
    back = b58decode_check(out)
    assert back == b"hello world"


def test_check_failure():
    data = b"3vQB7B6MrGQZaxCuFg4oH"
    with raises(ValueError) as excinfo:
        b58decode_check(data)
    msg = (
        "invalid checksum, calculated checksum: '[188, 98, 212, 184]', "
        "expected checksum: [188, 98, 212, 160]"
    )
    assert excinfo.value.args[0] == msg


def test_check_identity(alphabet):
    data = b"hello world"
    out = b58decode_check(b58encode_check(data, alphabet=alphabet), alphabet=alphabet)
    assert out == data


def test_round_trips(alphabet):
    possible_bytes = [b"\x00", b"\x01", b"\x10", b"\xff"]
    for length in range(0, 5):
        for bytes_to_test in product(possible_bytes, repeat=length):
            bytes_in = b"".join(bytes_to_test)
            bytes_out = b58decode(
                b58encode(bytes_in, alphabet=alphabet), alphabet=alphabet
            )
            assert bytes_in == bytes_out


def test_invalid_input():
    data = b"xyz\b"  # backspace is not part of the bitcoin base58 alphabet
    with raises(ValueError) as excinfo:
        b58decode(data)
    assert (
        excinfo.value.args[0]
        == "provided string contained invalid character '\\u{8}' at byte 3"
    )


@mark.parametrize("length", [8, 32, 256, 1024])
def test_encode_random(length) -> None:
    data = getrandbits(length * 8).to_bytes(length, byteorder="big")
    encoded = b58encode(data)
    assert b58decode(encoded) == data


@mark.parametrize("length", [8, 32, 256, 1024])
def test_decode_random(length) -> None:
    origdata = getrandbits(length * 8).to_bytes(length, byteorder="big")
    encoded = b58encode(origdata)
    data = b58decode(encoded)
    assert data == origdata


def test_invalid_checksum():
    with raises(ValueError) as excinfo:
        b58decode_check(b"4vQB7B6MrGQZaxCuFg4oh")
    msg = (
        "invalid checksum, calculated checksum: '[4, 49, 3, 121]', "
        "expected checksum: [189, 114, 212, 184]"
    )
    assert excinfo.value.args[0] == msg


def test_invalid_character():
    with raises(ValueError) as excinfo:
        b58decode(b"hello world")
    msg = "provided string contained invalid character 'l' at byte 2"
    assert excinfo.value.args[0] == msg
