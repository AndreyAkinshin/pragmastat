import numpy
from setuptools import Extension, setup

# Define the C extensions
extensions = [
    Extension(
        "pragmastat._fast_center_c",
        sources=["src/fast_center_c.c"],
        include_dirs=[numpy.get_include()],
        extra_compile_args=["-O3", "-Wall"],
    ),
    Extension(
        "pragmastat._fast_spread_c",
        sources=["src/fast_spread_c.c"],
        include_dirs=[numpy.get_include()],
        extra_compile_args=["-O3", "-Wall"],
    ),
    Extension(
        "pragmastat._fast_shift_c",
        sources=["src/fast_shift_c.c"],
        include_dirs=[numpy.get_include()],
        extra_compile_args=["-O3", "-Wall"],
    ),
]

setup(
    ext_modules=extensions,
    package_dir={"": "."},
)
