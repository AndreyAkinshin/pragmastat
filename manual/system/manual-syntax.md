# Manual Syntax

The manual uses an extended version of the markdown language.
Since it converts to knitr/rmarkdown documents and Hugo websites,
  it includes additional directives that are preprocessed by the build system.

## Include directives

These directives embed the content of the given file inside the current document.
All paths should be relative to the repository root.

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
In the `web` version, it expands to a light/dark image pair to support a pleasant look of images in both themes.