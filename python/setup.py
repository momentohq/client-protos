import setuptools
import os

version = os.getenv("PYPI_MOMENTO_WIRE_TYPE_VERSION")

assert version != None

with open("../README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    version=version,
    long_description=long_description,
    long_description_content_type="text/markdown",
    license_files = ('../LICENSE.txt',),
)
