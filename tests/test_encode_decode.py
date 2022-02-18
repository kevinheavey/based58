# note: these tests are mostly copied from the base58 library
# https://github.com/keis/base58/blob/master/test_base58.py
import pytest
from itertools import product
from random import getrandbits
from hamcrest import assert_that, equal_to, calling, raises
from based58 import (
    b58encode,
    b58decode,
    b58encode_check,
    b58decode_check,
    Alphabet,
)


@pytest.fixture(params=[Alphabet.BITCOIN, Alphabet.RIPPLE])
def alphabet(request) -> str:
    return request.param


def test_simple_encode():
    data = b58encode(b"hello world")
    assert_that(data, equal_to(b"StV1DL6CwTryKyV"))


def test_leadingz_encode():
    data = b58encode(b"\0\0hello world")
    assert_that(data, equal_to(b"11StV1DL6CwTryKyV"))


def test_encode_empty():
    data = b58encode(b"")
    assert_that(data, equal_to(b""))


def test_simple_decode():
    data = b58decode(b"StV1DL6CwTryKyV")
    assert_that(data, equal_to(b"hello world"))


def test_simple_decode_bytes():
    data = b58decode(b"StV1DL6CwTryKyV")
    assert_that(data, equal_to(b"hello world"))


def test_leadingz_decode():
    data = b58decode(b"11StV1DL6CwTryKyV")
    assert_that(data, equal_to(b"\0\0hello world"))


def test_leadingz_decode_bytes():
    data = b58decode(b"11StV1DL6CwTryKyV")
    assert_that(data, equal_to(b"\0\0hello world"))


def test_empty_decode():
    data = b58decode(b"1")
    assert_that(data, equal_to(b"\0"))


def test_empty_decode_bytes():
    data = b58decode(b"1")
    assert_that(data, equal_to(b"\0"))


def test_check_str():
    data = b"hello world"
    out = b58encode_check(data)
    assert_that(out, equal_to(b"3vQB7B6MrGQZaxCuFg4oh"))
    back = b58decode_check(out)
    assert_that(back, equal_to(b"hello world"))


def test_check_failure():
    data = b"3vQB7B6MrGQZaxCuFg4oH"
    assert_that(calling(b58decode_check).with_args(data), raises(ValueError))


def test_check_identity(alphabet):
    data = b"hello world"
    out = b58decode_check(b58encode_check(data, alphabet=alphabet), alphabet=alphabet)
    assert_that(out, equal_to(data))


def test_round_trips(alphabet):
    possible_bytes = [b"\x00", b"\x01", b"\x10", b"\xff"]
    for length in range(0, 5):
        for bytes_to_test in product(possible_bytes, repeat=length):
            bytes_in = b"".join(bytes_to_test)
            bytes_out = b58decode(
                b58encode(bytes_in, alphabet=alphabet), alphabet=alphabet
            )
            assert_that(bytes_in, equal_to(bytes_out))


def test_invalid_input():
    data = b"xyz\b"  # backspace is not part of the bitcoin base58 alphabet
    with pytest.raises(ValueError) as excinfo:
        b58decode(data)
    assert (
        excinfo.value.args[0]
        == "provided string contained invalid character '\\u{8}' at byte 3"
    )


@pytest.mark.parametrize("length", [8, 32, 256, 1024])
def test_encode_random(length) -> None:
    data = getrandbits(length * 8).to_bytes(length, byteorder="big")
    encoded = b58encode(data)
    assert_that(b58decode(encoded), equal_to(data))


@pytest.mark.parametrize("length", [8, 32, 256, 1024])
def test_decode_random(length) -> None:
    origdata = getrandbits(length * 8).to_bytes(length, byteorder="big")
    encoded = b58encode(origdata)
    data = b58decode(encoded)
    assert_that(data, equal_to(origdata))


def test_invalid_checksum():
    with pytest.raises(ValueError) as excinfo:
        b58decode_check(b"4vQB7B6MrGQZaxCuFg4oh")
    msg = (
        "invalid checksum, calculated checksum: '[4, 49, 3, 121]', "
        "expected checksum: [189, 114, 212, 184]"
    )
    assert excinfo.value.args[0] == msg
