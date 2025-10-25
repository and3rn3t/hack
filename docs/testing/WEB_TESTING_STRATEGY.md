# Web Version Testing Strategy

## Overview

The web version of "The Hack: Ghost Protocol" requires comprehensive testing across multiple layers due to its complex architecture involving WebAssembly, JavaScript, browser APIs, and user interactions.

## Testing Architecture

### 1. **WebAssembly Integration Tests** 🦀

- **Location**: `tests/web_integration_tests.rs`
- **Framework**: `wasm-bindgen-test`
- **Purpose**: Test Rust-to-JavaScript bindings and WASM functionality

#### What's Tested

- WebAssembly module loading and initialization
- Challenge serialization to/from JSON
- Game state management and persistence
- Cross-compilation compatibility
- Memory safety in browser environment
- Error handling in WASM boundary

#### Running WASM Tests

```bash
# Run in browser (Chrome headless)
wasm-pack test --chrome --headless --features web

# Run with different browsers
wasm-pack test --firefox --headless --features web
wasm-pack test --safari --features web

# Debug mode with console output
wasm-pack test --chrome --features web
```

### 2. **JavaScript Unit Tests** 🧪

- **Location**: `web/tests/game.test.js`
- **Framework**: Jest with jsdom
- **Purpose**: Test JavaScript game logic and browser interactions

#### What's Tested

- Achievement system logic and persistence
- Theme system functionality
- Audio system initialization and error handling
- Save/load mechanisms with localStorage
- Input validation and sanitization
- Progress sharing and clipboard integration
- Error boundary handling

#### Running Unit Tests

```bash
cd web
npm test                    # Run all unit tests
npm run test:watch          # Watch mode for development
npm run test:coverage       # Generate coverage report
```

### 3. **End-to-End Browser Tests** 🌐

- **Location**: `web/tests/e2e.spec.js`
- **Framework**: Playwright
- **Purpose**: Test complete user workflows in real browsers

#### What's Tested

- Game loading and initialization
- Command execution and responses
- Theme switching and visual changes
- Challenge starting and completion flows
- Save/load functionality
- Mobile responsiveness
- Cross-browser compatibility
- Performance benchmarks
- Visual regression testing

#### Running E2E Tests

```bash
cd web
npm run test:e2e            # Run in all browsers
npx playwright test         # Run with Playwright directly
npx playwright test --ui    # Interactive UI mode
npx playwright show-report  # View test report
```

### 4. **Browser Compatibility Matrix**

| Feature | Chrome | Firefox | Safari | Mobile Chrome | Mobile Safari |
|---------|--------|---------|--------|---------------|---------------|
| WebAssembly | ✅ | ✅ | ✅ | ✅ | ✅ |
| Audio API | ✅ | ✅ | ✅ | ⚠️* | ⚠️* |
| Clipboard API | ✅ | ✅ | ✅ | ❌ | ❌ |
| Local Storage | ✅ | ✅ | ✅ | ✅ | ✅ |
| File API | ✅ | ✅ | ✅ | ✅ | ✅ |
| Terminal Emulation | ✅ | ✅ | ✅ | ✅ | ✅ |

*_Requires user interaction to start audio_

### 5. **Performance Testing** ⚡

#### Metrics Tracked

- **Initial Load Time**: < 5 seconds
- **WebAssembly Compilation**: < 2 seconds
- **Command Response Time**: < 1 second
- **Memory Usage**: < 50MB after 30 minutes
- **Bundle Size**: WASM < 500KB, JS < 200KB

#### Performance Tests

```javascript
// Load time testing
test('should load within acceptable time', async ({ page }) => {
  const startTime = Date.now();
  await page.goto('https://hack.andernet.dev');
  await page.waitForSelector('#gameContainer', { state: 'visible' });
  const loadTime = Date.now() - startTime;
  expect(loadTime).toBeLessThan(5000);
});
```

### 6. **Accessibility Testing** ♿

#### Areas Covered

- **Keyboard Navigation**: All functions accessible via keyboard
- **Screen Reader Compatibility**: Proper ARIA labels
- **High Contrast Themes**: Contrast theme for visibility
- **Color Blind Support**: Multiple theme options
- **Mobile Accessibility**: Touch-friendly interface

### 7. **Security Testing** 🔒

#### What's Tested

- **Input Sanitization**: XSS prevention
- **Command Injection**: Safe command parsing
- **Data Validation**: Challenge ID validation
- **Storage Security**: localStorage data integrity
- **CSP Compliance**: Content Security Policy adherence

## Test Data and Fixtures

### Mock WebAssembly Module

```javascript
// Comprehensive WASM mocking for unit tests
global.mockWasmModule = {
  WebGameEngine: jest.fn(() => ({
    get_challenges_json: () => JSON.stringify(mockChallenges),
    validate_challenge_answer: (id, answer, state) => { /* logic */ }
  }))
};
```

### Test Challenges

```javascript
const mockChallenges = [
  {
    id: "test_base64",
    title: "Base64 Test",
    level: 0,
    category: "Encoding",
    solution: "dGVzdA==", // base64 for "test"
    hints: ["Try encoding 'test' in base64"]
  }
];
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/web-tests.yml
name: Web Tests
on: [push, pull_request]
jobs:
  web-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
      - name: Install dependencies
        run: cd web && npm install
      - name: Run unit tests
        run: cd web && npm test
      - name: Run E2E tests
        run: cd web && npm run test:e2e
      - name: Run WASM tests
        run: wasm-pack test --chrome --headless --features web
```

## Test Coverage Goals

### Target Coverage Levels

- **JavaScript Unit Tests**: > 85%
- **WebAssembly Tests**: > 90%
- **End-to-End Tests**: > 70% of user workflows
- **Browser Compatibility**: 100% of target browsers

### Coverage Reports

```bash
# Generate comprehensive coverage
npm run test:coverage

# View HTML coverage report
open web/tests/coverage/lcov-report/index.html
```

## Running Tests Locally

### Quick Test Suite

```bash
# Run all web tests
./scripts/test-web.ps1 all

# Run specific test types
./scripts/test-web.ps1 unit      # JavaScript unit tests
./scripts/test-web.ps1 e2e       # End-to-end browser tests
./scripts/test-web.ps1 wasm      # WebAssembly tests
./scripts/test-web.ps1 coverage  # Coverage analysis
```

### VS Code Integration

Available tasks in Command Palette (Ctrl+Shift+P):

- `Tasks: Run Task` → `Web: Test All`
- `Tasks: Run Task` → `Web: Unit Tests`
- `Tasks: Run Task` → `Web: End-to-End Tests`
- `Tasks: Run Task` → `Web: WebAssembly Tests`
- `Tasks: Run Task` → `Web: Coverage Report`

## Test Environment Setup

### Prerequisites

```bash
# Install Node.js dependencies
cd web && npm install

# Install Playwright browsers
npx playwright install --with-deps

# Install wasm-pack for WASM testing
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Environment Variables

```bash
# For CI/CD
export CI=true              # Enable CI-specific settings
export BROWSER=chrome       # Default browser for tests
export HEADLESS=true        # Run browsers in headless mode
```

## Debugging Test Failures

### WebAssembly Issues

```bash
# Run with debug output
wasm-pack test --chrome --features web -- --nocapture

# Check WASM compilation
wasm-pack build --dev --target web --features web
```

### JavaScript Issues

```bash
# Run with verbose output
npm test -- --verbose

# Debug specific test
npm test -- --testNamePattern="achievement"
```

### E2E Issues

```bash
# Run with UI for debugging
npx playwright test --ui

# Generate trace files
npx playwright test --trace on

# View test artifacts
npx playwright show-report
```

## Test Maintenance

### Regular Tasks

- **Weekly**: Update browser versions in CI
- **Monthly**: Review and update test coverage goals
- **Per Release**: Add tests for new features
- **Per Bug Fix**: Add regression tests

### Performance Monitoring

- Monitor bundle size changes
- Track load time regressions
- Analyze memory usage patterns
- Review Core Web Vitals metrics

This comprehensive testing strategy ensures the web version maintains high quality, performance, and compatibility across all target platforms and browsers while providing a robust development workflow for continuous improvement.
