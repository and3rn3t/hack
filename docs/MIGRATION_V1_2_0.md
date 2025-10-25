# Migration Guide: Version 1.2.0

**Date**: October 25, 2025
**Version**: 1.1.0 → 1.2.0
**Migration Type**: Backward Compatible (Automatic)

---

## 🎯 What's New in v1.2.0

Version 1.2.0 introduces major enhancements to user experience and challenge variety:

### ✨ **Key Features**

- **Advanced Command System** with user aliases and enhanced tab completion
- **Comprehensive Settings** for difficulty, hints, themes, and accessibility
- **Multiple Save Slots** with export/import functionality
- **Challenge Difficulty Variants** (Beginner, Standard, Advanced, Expert)
- **Dynamic Challenge Generation** with practice mode
- **Progress Analytics** tracking performance and learning patterns

---

## 🔄 Automatic Migration

### **Save File Compatibility**

✅ **No action required** - Your existing save files will automatically migrate to v1.2.0.

When you first run v1.2.0:

1. Your existing `game_save.json` will be loaded normally
2. New fields will be initialized with default values:
   - User preferences set to defaults (Adaptive difficulty, Moderate hints, etc.)
   - Progress analytics will start tracking from this point forward
   - No existing progress will be lost

### **What Gets Migrated**

- ✅ Player name and statistics
- ✅ Completed challenges and experience
- ✅ Current level and sanity
- ✅ Achievement progress
- ✅ Tutorial completion status

### **What's New (Default Values)**

- 🆕 Difficulty scaling: **Adaptive** (adjusts based on your performance)
- 🆕 Hint verbosity: **Moderate** (standard detailed hints)
- 🆕 Color theme: **Horror** (your current theme)
- 🆕 Font size: **Medium** (standard size)
- 🆕 Audio enabled: **true** (atmospheric effects on)
- 🆕 Animation speed: **Normal** (standard animation speed)

---

## 🎮 New Features Guide

### **1. Settings Menu**

Access with the new `settings` command from the main menu:

```
> settings
```

**Available Options:**

- **Difficulty Scaling**: Choose how challenge difficulty adapts to your performance
- **Hint Verbosity**: Control how detailed hints are
- **Color Theme**: Same as before, now accessible from settings
- **Font Size**: Accessibility option for terminal display
- **Audio**: Toggle atmospheric sound effects
- **Animations**: Control animation speed for accessibility

### **2. User Aliases**

Create shortcuts for frequently used commands:

```
> alias
```

**Examples:**

- `s` → `stats` (quick statistics)
- `h` → `help` (quick help)
- `t` → `theme` (quick theme change)

### **3. Challenge Difficulty Variants**

When selecting challenges, you may see a difficulty selection menu:

**Difficulty Levels:**

- **Beginner**: Extra guidance and tutorials (50% XP, 50% sanity cost)
- **Standard**: Default difficulty (100% XP, 100% sanity cost)
- **Advanced**: Fewer hints, time pressure (150% XP, 120% sanity cost)
- **Expert**: Minimal help, complex scenarios (200% XP, 150% sanity cost)

**Automatic Selection:**

- **Adaptive**: Game automatically chooses based on your performance
- **Static**: Always uses Standard difficulty
- **Custom**: Always prompts for selection

### **4. Practice Mode**

Access with the new `practice` command:

```
> practice
```

- Practice with randomly generated challenges
- Challenges don't affect main story progress
- Earn bonus XP for skill improvement
- Perfect for learning without pressure

### **5. Enhanced Save System**

Access with the improved `save` command:

**New Save Options:**

- **Multiple Slots**: Save to slots 0-4 for different playthroughs
- **Export**: Backup your save data as JSON
- **Import**: Restore from backed-up save data
- **Metadata**: View save file details and timestamps

---

## 📈 Performance & Analytics

### **New Tracking (Privacy-First)**

All analytics are stored locally and never transmitted:

- **Challenge Attempts**: Track how many times you've tried each challenge
- **Hint Usage**: Monitor learning patterns and hint effectiveness
- **Completion Times**: Measure improvement over time
- **Learning Streaks**: Track consecutive successful challenges
- **Playtime**: Total time spent in the game

### **Adaptive Difficulty Algorithm**

The new adaptive system considers:

- Your overall completion rate (%)
- Current sanity levels
- Number of attempts per challenge
- Current game level
- Historical performance patterns

---

## 🛠️ Technical Changes

### **File System Changes**

- `game_save.json` - Enhanced with new fields (backward compatible)
- `save_slot_0.json` to `save_slot_4.json` - New multi-slot saves
- All existing files remain untouched

### **Command Changes**

**New Commands:**

- `settings` - Access preferences menu
- `alias` - Manage command shortcuts
- `practice` - Enter practice mode

**Enhanced Commands:**

- `save` - Now opens save slot management
- `stats` - Shows enhanced analytics
- `help` - Updated with new features

### **Completion System**

Tab completion now supports:

- Settings menu navigation
- Alias management commands
- Save slot selection
- Difficulty level selection

---

## 🔧 Troubleshooting

### **Save File Issues**

If you encounter save file problems:

1. **Backup your saves** before troubleshooting
2. **Check file permissions** - ensure the game can write to its directory
3. **Corrupted save**: Use the import/export feature to restore from backup
4. **Multiple versions**: Each save slot is independent, try different slots

### **Performance Issues**

New analytics tracking is lightweight, but if you experience issues:

1. **Analytics can be reset** in settings (future update)
2. **Save files are larger** but still under 50KB typically
3. **Cache clearing** - restart the game to refresh challenge cache

### **Compatibility Issues**

If you experience unexpected behavior:

1. **Reset preferences** to defaults in settings menu
2. **Clear aliases** if command resolution has issues
3. **Use standard difficulty** if adaptive difficulty causes problems

---

## 📚 Learning the New Features

### **Recommended First Steps**

1. **Explore Settings**: Run `settings` to familiarize yourself with new options
2. **Try Adaptive Difficulty**: Let the game adjust challenge difficulty for you
3. **Create Useful Aliases**: Set up shortcuts for your most-used commands
4. **Practice Mode**: Try `practice` for skill-building without story pressure
5. **Multiple Saves**: Use different save slots for different difficulty preferences

### **Advanced Features**

- **Export Saves**: Backup your progress before trying new settings
- **Expert Mode**: Challenge yourself with expert difficulty variants
- **Analytics Review**: Check your learning patterns in enhanced statistics

---

## 🎯 Migration Checklist

**Before Upgrading:**

- [ ] **Backup**: Export your current save (optional, auto-migration works)
- [ ] **Note Settings**: Remember your preferred color theme

**After Upgrading:**

- [ ] **Verify Save**: Check that your progress loaded correctly
- [ ] **Explore Settings**: Customize your new preferences
- [ ] **Test Aliases**: Create a few command shortcuts
- [ ] **Try Practice**: Experience dynamic challenges
- [ ] **Check Analytics**: Review your new performance data

**Optional Customization:**

- [ ] **Difficulty Preference**: Set your preferred scaling method
- [ ] **Accessibility**: Adjust font size and animation speed if needed
- [ ] **Save Organization**: Use multiple slots for different playthroughs

---

## 🔮 What's Next

v1.2.0 sets the foundation for future features:

- **v2.0.0**: Multiplayer and community features
- **v2.5.0**: AI mentor system and adaptive learning
- **v3.0.0**: Cross-platform sync and mobile apps

Your v1.2.0 save files and preferences will be forward-compatible with these future versions.

---

## 🆘 Getting Help

**Need assistance with migration?**

- Check the [documentation](../README.md) for detailed usage guides
- Review [CHANGELOG.md](../CHANGELOG.md) for complete feature list
- Open an issue on [GitHub](https://github.com/and3rn3t/hack/issues) for technical problems

**Feature Questions:**

- All new features have in-game help tooltips
- Use `help` command for updated guidance
- Settings menus include descriptions for all options

---

**Welcome to The Hack: Ghost Protocol v1.2.0!** 🎮👻

_Your journey into advanced cybersecurity education continues with enhanced personalization and challenge variety._
