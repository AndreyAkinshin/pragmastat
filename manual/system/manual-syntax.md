# Manual Syntax

The manual uses an extended version of the markdown language.
Since it converts to knitr/rmarkdown document and hugo website,
  it includes additional directives that are preprocessed by the build system.

## Include directives

These directives embed the content of the given file inside the current document.
All the paths should be relative to the repository root.

Format:

```html
<!-- INCLUDE <file-name> -->
```

Example:

```html
<!-- INCLUDE manual/postface.md -->
```

## Image directives

To insert an image, use:

```html
<!-- IMG <image-name> -->
```

Example:

```html
<!-- IMG central-tendency-gaussian-efficiency -->
```

This command references an image by its name in the global image registry.
In `web` version, it will be expanded to light/dark image pair to support a pleasant look of images in both themes.