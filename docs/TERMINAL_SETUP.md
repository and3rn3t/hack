# Terminal Configuration for The Hack: Ghost Protocol

## Overview

This game requires a terminal with ANSI color support and Unicode character rendering. This guide helps you configure your terminal for the best experience.

## Minimum Requirements

- **ANSI Color Support**: 256-color or TrueColor support
- **Unicode Support**: UTF-8 encoding
- **Minimum Size**: 80 columns × 24 rows (recommended: 100 × 30)
- **Font**: Monospace font with good box-drawing character support

## Platform-Specific Configuration

### Windows

#### Windows Terminal (Recommended ⭐)

Windows Terminal provides the best experience with full Unicode and color support.

**Installation:**

```powershell
# Via winget
winget install Microsoft.WindowsTerminal

# Via Microsoft Store
# Search for "Windows Terminal"
```

**Recommended Settings (settings.json):**

```json
{
  "profiles": {
    "defaults": {
      "colorScheme": "One Half Dark",
      "font": {
        "face": "Cascadia Code",
        "size": 11
      },
      "useAcrylic": false,
      "opacity": 100,
      "cursorShape": "bar",
      "antialiasingMode": "cleartype"
    }
  },
  "schemes": [
    {
      "name": "Hack Horror Theme",
      "background": "#0C0C0C",
      "foreground": "#CCCCCC",
      "black": "#0C0C0C",
      "blue": "#0037DA",
      "cyan": "#3A96DD",
      "green": "#13A10E",
      "purple": "#881798",
      "red": "#C50F1F",
      "white": "#CCCCCC",
      "yellow": "#C19C00",
      "brightBlack": "#767676",
      "brightBlue": "#3B78FF",
      "brightCyan": "#61D6D6",
      "brightGreen": "#16C60C",
      "brightPurple": "#B4009E",
      "brightRed": "#E74856",
      "brightWhite": "#F2F2F2",
      "brightYellow": "#F9F1A5"
    }
  ]
}
```

**PowerShell Profile Configuration:**
Add to `$PROFILE` (create with `notepad $PROFILE`):

```powershell
# Set UTF-8 encoding
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::InputEncoding = [System.Text.Encoding]::UTF8

# Set window title
$Host.UI.RawUI.WindowTitle = "The Hack: Ghost Protocol"

# Optional: Set minimum window size
$pshost = Get-Host
$pswindow = $pshost.UI.RawUI
$newsize = $pswindow.WindowSize
$newsize.Width = 100
$newsize.Height = 30
$pswindow.WindowSize = $newsize
```

**Quick Setup Script (PowerShell):**

```powershell
# Save as setup-terminal.ps1
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
Write-Host "Terminal configured for The Hack: Ghost Protocol" -ForegroundColor Green
Write-Host "Window Size: $($Host.UI.RawUI.WindowSize.Width)x$($Host.UI.RawUI.WindowSize.Height)"
Write-Host "Encoding: UTF-8"
Write-Host "Ready to hack! Run: cargo run" -ForegroundColor Cyan
```

#### PowerShell 7+ (Alternative)

```powershell
# Install PowerShell 7
winget install Microsoft.PowerShell

# Set UTF-8 in profile
Add-Content $PROFILE "`n[Console]::OutputEncoding = [System.Text.Encoding]::UTF8"
```

#### Command Prompt (Not Recommended)

- Limited color support
- Poor Unicode rendering
- Use Windows Terminal or PowerShell instead

### Linux

#### GNOME Terminal (Recommended ⭐)

**Installation:**

```bash
sudo apt install gnome-terminal  # Debian/Ubuntu
sudo dnf install gnome-terminal  # Fedora
```

**Configuration:**

1. Open Preferences (Ctrl+,)
2. Create new profile: "Hack Horror"
3. Set colors:
   - Background: #0C0C0C
   - Foreground: #CCCCCC
   - Use built-in scheme or customize
4. Set font: Monospace 11
5. Set initial terminal size: 100 × 30

**Profile Script (.bashrc or .zshrc):**

```bash
# Add to ~/.bashrc or ~/.zshrc

# Ensure UTF-8
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Alias for the game
alias hack='cd ~/projects/hack && cargo run'

# Optional: Set title
function hack() {
    echo -ne "\033]0;The Hack: Ghost Protocol\007"
    cd ~/projects/hack && cargo run
}
```

#### Alacritty (High Performance, Recommended for Advanced Users)

**Installation:**

```bash
sudo apt install alacritty  # Debian/Ubuntu
```

**Configuration (~/.config/alacritty/alacritty.yml):**

```yaml
# Optimized for The Hack: Ghost Protocol

window:
  dimensions:
    columns: 100
    lines: 30
  padding:
    x: 2
    y: 2
  opacity: 1.0

font:
  normal:
    family: "Fira Code"
    style: Regular
  size: 11.0

colors:
  primary:
    background: '#0C0C0C'
    foreground: '#CCCCCC'
  normal:
    black:   '#0C0C0C'
    red:     '#C50F1F'
    green:   '#13A10E'
    yellow:  '#C19C00'
    blue:    '#0037DA'
    magenta: '#881798'
    cyan:    '#3A96DD'
    white:   '#CCCCCC'
  bright:
    black:   '#767676'
    red:     '#E74856'
    green:   '#16C60C'
    yellow:  '#F9F1A5'
    blue:    '#3B78FF'
    magenta: '#B4009E'
    cyan:    '#61D6D6'
    white:   '#F2F2F2'

# Performance
scrolling:
  history: 10000
  multiplier: 3

# Remove delay for escape sequences
keyboard:
  bindings:
    - { key: Escape, action: ToggleViMode }
```

#### Kitty (GPU-Accelerated)

**Configuration (~/.config/kitty/kitty.conf):**

```conf
# The Hack: Ghost Protocol Theme

font_family      Fira Code
font_size        11.0

background #0C0C0C
foreground #CCCCCC

# Colors
color0  #0C0C0C
color1  #C50F1F
color2  #13A10E
color3  #C19C00
color4  #0037DA
color5  #881798
color6  #3A96DD
color7  #CCCCCC
color8  #767676
color9  #E74856
color10 #16C60C
color11 #F9F1A5
color12 #3B78FF
color13 #B4009E
color14 #61D6D6
color15 #F2F2F2

# Window
remember_window_size  yes
initial_window_width  100c
initial_window_height 30c

# Performance
repaint_delay 10
input_delay 3
sync_to_monitor yes
```

### macOS

#### iTerm2 (Recommended ⭐)

**Installation:**

```bash
brew install --cask iterm2
```

**Configuration:**

1. Preferences → Profiles → Create new profile "Hack Horror"
2. Colors:
   - Background: RGB(12, 12, 12)
   - Foreground: RGB(204, 204, 204)
   - Import color preset or set manually
3. Text:
   - Font: Menlo or SF Mono, 11pt
   - Unicode normalization: NFC
4. Window:
   - Columns: 100
   - Rows: 30
5. Terminal:
   - Report terminal type: xterm-256color
   - Character encoding: Unicode (UTF-8)

**iTerm2 Profile JSON (Import):**

```json
{
  "Name": "Hack Horror",
  "Guid": "hack-horror-theme",
  "Background Color": {
    "Red": 0.047,
    "Green": 0.047,
    "Blue": 0.047
  },
  "Foreground Color": {
    "Red": 0.8,
    "Green": 0.8,
    "Blue": 0.8
  },
  "Ansi 0 Color": {
    "Red": 0.047,
    "Green": 0.047,
    "Blue": 0.047
  }
}
```

**Shell Configuration (.zshrc):**

```bash
# Add to ~/.zshrc

# UTF-8
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Terminal type
export TERM=xterm-256color

# Alias
alias hack='cd ~/projects/hack && cargo run'
```

#### Terminal.app (Built-in, Adequate)

1. Preferences → Profiles
2. Duplicate "Basic" profile → Rename to "Hack Horror"
3. Text:
   - Font: SF Mono, 11pt
   - Character encoding: Unicode (UTF-8)
4. Window:
   - Columns: 100, Rows: 30
5. Colors: Adjust to dark theme

## Font Recommendations

### Windows

1. **Cascadia Code** (Best for Windows Terminal)

   ```powershell
   # Pre-installed with Windows Terminal
   ```

2. **JetBrains Mono**

   ```powershell
   choco install jetbrainsmono
   ```

3. **Fira Code**

   ```powershell
   choco install firacode
   ```

### Linux

```bash
# Ubuntu/Debian
sudo apt install fonts-firacode fonts-jetbrains-mono

# Fedora
sudo dnf install fira-code-fonts jetbrains-mono-fonts

# Arch
sudo pacman -S ttf-fira-code ttf-jetbrains-mono
```

### macOS

```bash
# Via Homebrew
brew tap homebrew/cask-fonts
brew install --cask font-fira-code font-jetbrains-mono font-cascadia-code
```

## Verification Script

Create `scripts/verify-terminal.sh`:

```bash
#!/bin/bash
# Verify terminal capabilities for The Hack: Ghost Protocol

echo "=== Terminal Verification ==="
echo ""

# Check terminal type
echo "Terminal Type: $TERM"

# Check size
cols=$(tput cols)
rows=$(tput lines)
echo "Terminal Size: ${cols}x${rows}"

if [ "$cols" -lt 80 ] || [ "$rows" -lt 24 ]; then
    echo "⚠️  WARNING: Terminal too small (minimum 80x24)"
else
    echo "✅ Terminal size OK"
fi

# Check colors
colors=$(tput colors)
echo "Colors Supported: $colors"

if [ "$colors" -ge 256 ]; then
    echo "✅ Color support OK"
else
    echo "⚠️  WARNING: Limited color support"
fi

# Check UTF-8
if [[ "$LANG" == *"UTF-8"* ]]; then
    echo "✅ UTF-8 encoding OK"
else
    echo "⚠️  WARNING: UTF-8 not set"
fi

# Test box drawing
echo ""
echo "Box Drawing Test:"
echo "╔═══════════════════╗"
echo "║  Unicode Test  ║"
echo "╚═══════════════════╝"

# Test colors
echo ""
echo "Color Test:"
echo -e "\033[31mRed\033[0m \033[32mGreen\033[0m \033[33mYellow\033[0m \033[34mBlue\033[0m \033[35mMagenta\033[0m \033[36mCyan\033[0m"

echo ""
echo "=== Verification Complete ==="
```

Create `scripts/verify-terminal.ps1`:

```powershell
# Verify terminal capabilities for The Hack: Ghost Protocol

Write-Host "=== Terminal Verification ===" -ForegroundColor Cyan
Write-Host ""

# Check window size
$size = $Host.UI.RawUI.WindowSize
Write-Host "Terminal Size: $($size.Width)x$($size.Height)"

if ($size.Width -lt 80 -or $size.Height -lt 24) {
    Write-Host "⚠️  WARNING: Terminal too small (minimum 80x24)" -ForegroundColor Yellow
} else {
    Write-Host "✅ Terminal size OK" -ForegroundColor Green
}

# Check encoding
$encoding = [Console]::OutputEncoding
Write-Host "Output Encoding: $($encoding.EncodingName)"

if ($encoding.EncodingName -match "UTF-8") {
    Write-Host "✅ UTF-8 encoding OK" -ForegroundColor Green
} else {
    Write-Host "⚠️  WARNING: UTF-8 not set" -ForegroundColor Yellow
}

# Test box drawing
Write-Host ""
Write-Host "Box Drawing Test:"
Write-Host "╔═══════════════════╗"
Write-Host "║  Unicode Test     ║"
Write-Host "╚═══════════════════╝"

# Test colors
Write-Host ""
Write-Host "Color Test:"
Write-Host "Red " -ForegroundColor Red -NoNewline
Write-Host "Green " -ForegroundColor Green -NoNewline
Write-Host "Yellow " -ForegroundColor Yellow -NoNewline
Write-Host "Blue " -ForegroundColor Blue -NoNewline
Write-Host "Magenta " -ForegroundColor Magenta -NoNewline
Write-Host "Cyan" -ForegroundColor Cyan

Write-Host ""
Write-Host "=== Verification Complete ===" -ForegroundColor Cyan
```

## Performance Optimization

### General Tips

1. **Disable transparency/blur** - Improves rendering performance
2. **Use GPU acceleration** - If available (Alacritty, Kitty, iTerm2)
3. **Increase scrollback** - To 10,000+ lines for better debugging
4. **Disable animations** - Faster screen updates
5. **Use native fonts** - Better rendering performance

### For Development

```bash
# Enable fast terminal output
export TERM=xterm-256color

# Disable terminal bell
setopt NO_BEEP  # Zsh
set bell-style none  # Bash
```

### For the Game

The game uses:

- Alternate screen buffer (no scrollback pollution)
- Direct cursor positioning (fast)
- Minimal ANSI escape sequences
- Efficient color codes

## Troubleshooting

### Colors Not Displaying

**Problem**: Colors appear wrong or as plain text

**Solution**:

```bash
# Linux/macOS
export TERM=xterm-256color

# Windows
# Use Windows Terminal or PowerShell 7+
```

### Box Characters Broken

**Problem**: Unicode box-drawing characters appear as `?` or broken

**Solution**:

1. Install a proper monospace font (see recommendations)
2. Set UTF-8 encoding:

   ```bash
   export LANG=en_US.UTF-8
   export LC_ALL=en_US.UTF-8
   ```

3. Use Windows Terminal on Windows

### Terminal Too Small

**Problem**: Game UI is cut off

**Solution**:

```bash
# Check size
tput cols  # Should be 80+
tput lines # Should be 24+

# Resize terminal or increase font size
```

### Slow Rendering

**Problem**: Screen updates are laggy

**Solution**:

1. Use GPU-accelerated terminal (Alacritty, Kitty)
2. Disable transparency/effects
3. Close other terminal tabs/windows
4. Update terminal emulator

### Input Lag

**Problem**: Keypresses delayed

**Solution**:

1. Reduce input delay in terminal settings
2. Disable terminal multiplexers (tmux/screen) for gaming
3. Use native terminal instead of IDE embedded terminal

## Best Practices

### During Development

- Use integrated terminal in VS Code for quick testing
- Use native terminal for full gameplay experience
- Keep terminal at least 100×30 for best layout

### For Players

- Maximize terminal window
- Use dark theme for horror atmosphere
- Disable distractions (notifications, other windows)
- Good lighting to avoid eye strain

### For Streamers/Recording

- Use 120×40 or larger for better visibility
- Increase font size (13-14pt)
- Use high-contrast color scheme
- Consider recording at 1080p or higher

## Quick Setup Commands

### Windows (PowerShell)

```powershell
# One-line setup
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; cd path\to\hack; cargo run
```

### Linux/macOS (Bash/Zsh)

```bash
# One-line setup
export LANG=en_US.UTF-8 && cd ~/projects/hack && cargo run
```

## Environment Variables

Set these for optimal experience:

```bash
# Linux/macOS
export TERM=xterm-256color
export COLORTERM=truecolor
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Add to ~/.bashrc or ~/.zshrc for persistence
```

```powershell
# Windows (PowerShell profile)
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
[Console]::InputEncoding = [System.Text.Encoding]::UTF8
$env:TERM = "xterm-256color"
```

## Testing Your Configuration

Run the verification script:

```bash
# Linux/macOS
bash scripts/verify-terminal.sh

# Windows
pwsh scripts/verify-terminal.ps1
```

Then test with the game:

```bash
cargo run
```

Look for:

- ✅ Colors display correctly
- ✅ Box-drawing characters render properly
- ✅ No flickering or tearing
- ✅ Smooth screen updates
- ✅ Responsive input

## Additional Resources

- [Crossterm Documentation](https://docs.rs/crossterm/)
- [Windows Terminal Documentation](https://docs.microsoft.com/windows-terminal/)
- [iTerm2 Documentation](https://iterm2.com/documentation.html)
- [ANSI Escape Codes Reference](https://en.wikipedia.org/wiki/ANSI_escape_code)

---

**Ready to hack?** Make sure your terminal is properly configured for the best horror experience! 👻🔒
