import setuptools
import os

version = os.getenv("PYPI_MOMENTO_WIRE_TYPE_VERSION")

assert version != None

setuptools.setup(
    version=version,
    license_files = ('../LICENSE.txt',),
)
