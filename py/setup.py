import numpy
from setuptools import Extension, setup

# Define the C extensions
extensions = [
    Extension(
        "pragmastat._center_impl_c",
        sources=["src/center_impl_c.c"],
        include_dirs=[numpy.get_include()],
        extra_compile_args=["-O3", "-Wall"],
    ),
    Extension(
        "pragmastat._spread_impl_c",
        sources=["src/spread_impl_c.c"],
        include_dirs=[numpy.get_include()],
        extra_compile_args=["-O3", "-Wall"],
    ),
    Extension(
        "pragmastat._shift_impl_c",
        sources=["src/shift_impl_c.c"],
        include_dirs=[numpy.get_include()],
        extra_compile_args=["-O3", "-Wall"],
    ),
]

setup(
    ext_modules=extensions,
    package_dir={"": "."},
)
