# Testing Guide for NML App

This document describes the testing infrastructure for the Nerd's Minecraft Launcher app.

## Frontend Testing (JavaScript/TypeScript)

### Setup

The frontend uses [Vitest](https://vitest.dev/) for unit and integration testing of Vue components and JavaScript utilities.

**Dependencies:**
- `vitest` - Test runner
- `@vitest/ui` - UI interface for test results
- `@vue/test-utils` - Vue component testing utilities
- `happy-dom` - Fast DOM implementation for testing

### Running Tests

```bash
# Run tests once
pnpm test --filter @nml/app-frontend

# Watch mode for development
pnpm --filter @nml/app-frontend test:watch

# Run with UI
pnpm --filter @nml/app-frontend test:ui

# From the root
pnpm test
```

### Test Files

Test files should be placed alongside the code they test with the naming convention:
- `*.test.js` or `*.test.ts` for JavaScript/TypeScript files
- `*.spec.js` or `*.spec.ts` for alternative naming

Example structure:
```
src/helpers/
├── utils.js
└── utils.test.js
```

### Example Tests

See:
- `apps/app-frontend/src/helpers/utils.test.js` - Testing pure functions
- `apps/app-frontend/src/helpers/profile.test.js` - Testing Tauri API wrappers with mocks

### Component Testing (Advanced)

For testing Vue components, use `@vue/test-utils`:

```javascript
import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import MyComponent from './MyComponent.vue'

describe('MyComponent', () => {
  it('renders properly', () => {
    const wrapper = mount(MyComponent, {
      props: {
        title: 'Test Title'
      }
    })
    expect(wrapper.text()).toContain('Test Title')
  })
})
```

**Note**: Component tests may require additional setup for:
- Vue Router
- Pinia stores
- Global plugins (e.g., vintl for internationalization)
- CSS/asset imports

## Backend Testing (Rust)

### Setup

The backend uses Rust's built-in test framework with the `cargo test` command.

### Running Tests

```bash
# Run all tests
pnpm test --filter @nml/app

# Or using cargo directly
cd apps/app
cargo test
```

**Note:** Running Rust tests requires system dependencies (glib-2.0, webkit2gtk, etc.) to be installed. These are typically available on Linux development machines but may not be present in CI environments without GUI support.

### Test Files

Rust tests are written inline using the `#[cfg(test)]` module and `#[test]` attribute:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Example Tests

See:
- `apps/app/src/api/utils.rs` - Tests for OS detection and serialization
- `apps/app/src/error.rs` - Tests for error handling utilities

## Test Organization

- **Unit tests**: Test individual functions in isolation
- **Integration tests**: Test how multiple units work together
- **Component tests**: Test Vue components (frontend only)

## Best Practices

1. **Test pure functions first** - Functions without side effects are easiest to test
2. **Mock external dependencies** - Use mocks for Tauri APIs, external services, etc.
3. **Keep tests simple** - Each test should verify one thing
4. **Use descriptive names** - Test names should clearly describe what they're testing
5. **Run tests before committing** - Ensure all tests pass before pushing changes

## CI/CD Integration

Tests are integrated with Turbo for monorepo-wide test execution:

```bash
# Run all tests across the monorepo
pnpm test

# Run tests for specific packages
pnpm test --filter @nml/app-frontend
pnpm test --filter @nml/app
```

The `turbo.json` configuration ensures tests are run efficiently with proper caching.
