import setuptools
import os

version = os.getenv("PYPI_MOMENTO_WIRE_TYPE_VERSION")

assert version != None

with open("PYPIREADME.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    version=version,
    license_files = ('../LICENSE.txt',),
    long_description=long_description,
    long_description_content_type="text/markdown",
    project_urls={
        'Source': 'https://github.com/momentohq/client_protos',
    },
)
