# PWA Icons Directory

This directory contains icons for the Progressive Web App (PWA) implementation of The Hack: Ghost Protocol.

## Required Icons

The following icons need to be created for full PWA support:

### Standard Icons

- `icon-72x72.png` - Small icon for Android
- `icon-96x96.png` - Medium icon for Android
- `icon-128x128.png` - Large icon for Android
- `icon-144x144.png` - Windows tile icon
- `icon-152x152.png` - iPad icon
- `icon-192x192.png` - Standard PWA icon
- `icon-384x384.png` - Large PWA icon
- `icon-512x512.png` - Maximum size PWA icon

### Shortcut Icons

- `shortcut-new.png` - New game shortcut
- `shortcut-continue.png` - Continue game shortcut
- `shortcut-stats.png` - Statistics shortcut
- `shortcut-close.png` - Close notification

### Screenshots

- `screenshots/desktop.png` - Desktop interface (1280x720)
- `screenshots/mobile.png` - Mobile interface (360x640)

## Icon Design Guidelines

All icons should:

- Use the horror green theme color (#00ff41)
- Have a black or dark background (#000000)
- Include the ghost/skull motif from the game
- Be high contrast for accessibility
- Support maskable icon requirements (safe area within 80% of canvas)

## Temporary Fallbacks

Until custom icons are created, the app will use:

- Base64 encoded favicon for small sizes
- Generated solid color icons for PWA requirements
- Screenshot placeholders

## Icon Generation

To generate icons from a source image:

```bash
# Using ImageMagick (if available)
convert source-icon.png -resize 192x192 icon-192x192.png
convert source-icon.png -resize 512x512 icon-512x512.png

# Or use online tools:
# - https://realfavicongenerator.net/
# - https://www.pwabuilder.com/imageGenerator
```

These icons enhance the native app experience when installed as a PWA.
