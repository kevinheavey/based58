Introduction
============

``based58`` is a fast Python library for
`Base58 <https://en.wikipedia.org/wiki/Binary-to-text_encoding#Base58>`_
encoding and decoding. It includes support for Base58Check and configurable alphabets.

It is
`significantly faster <https://gist.github.com/kevinheavey/2abad728d7658c136de0078d667d7267>`_
than the pure-Python
`base58 library <https://pypi.org/project/base58/>`_,
as it calls the Rust `bs58 library <https://github.com/mycorrhiza/bs58-rs>`_
under the hood.

The API mimics that of the ``base58`` library, with the exception that string inputs are not
supported, only bytes.

Installation
^^^^^^^^^^^^

::

    pip install based58

.. note:: requires Python >= 3.7.

Usage
^^^^^

.. code::

   >>> import based58
   >>> data = [1, 2, 3]
   >>> based58.b58encode(b'hello world')
   b'StV1DL6CwTryKyV'
   >>> based58.b58decode(b'StV1DL6CwTryKyV')
   b'hello world'
   >>> based58.b58encode_check(b'hello world')
   b'3vQB7B6MrGQZaxCuFg4oh'
   >>> based58.b58decode_check(b'3vQB7B6MrGQZaxCuFg4oh')
   b'hello world'
   >>> based58.b58encode(b'hello world', alphabet=based58.Alphabet.RIPPLE)
   b'StVrDLaUATiyKyV'
   >>> based58.b58decode(b'StVrDLaUATiyKyV', alphabet=based58.Alphabet.RIPPLE)
   b'hello world'


Documentation credit
^^^^^^^^^^^^^^^^^^^^

Most of this documentation is copied from
the `base58 docs <https://github.com/keis/base58>`_
and `bs58 docs <https://docs.rs/bs58/latest/bs58/>`_.




.. toctree::
   :maxdepth: 3
   :caption: Contents:

   self
   api_reference
