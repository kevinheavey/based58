from pytest import raises

from based58 import Alphabet


def test_duplicate_character() -> None:
    chars = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    with raises(ValueError) as excinfo:
        Alphabet(chars)
    expected = "alphabet contained a duplicate character `a` at indexes 0 and 1"
    assert excinfo.value.args[0] == expected
