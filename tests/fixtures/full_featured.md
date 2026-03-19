---
title: "Full-Featured Test"
author: "Test Author"
date: "2026-03-19"
theme: default
spacing:
  line_height: 1.7
---

# Full-Featured Document

This document exercises most md2paper features.

## Typography

Normal, **bold**, *italic*, ~~strikethrough~~, `inline code`.

## Code Block

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
```

## Table

| Feature      | Status  | Notes           |
|--------------|---------|-----------------|
| Headings     | ✅      | H1–H6           |
| Bold/Italic  | ✅      | CommonMark      |
| Tables       | ✅      | GFM             |
| Code blocks  | ✅      | With lang hint  |
| Math         | ✅      | Dollar syntax   |
| Images       | ✅      | Relative paths  |

## Math

Euler's identity: $e^{i\pi} + 1 = 0$

$$
\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0}
$$

## Blockquote

> "The best way to predict the future is to invent it."
> — Alan Kay

## Lists

### Unordered

- First item
- Second item
  - Nested A
  - Nested B
- Third item

### Ordered

1. Step one
2. Step two
3. Step three

## Horizontal Rule

---

*End of full-featured test document.*
