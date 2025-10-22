#!/bin/bash
# Terminal Verification Script for The Hack: Ghost Protocol
# Usage: bash scripts/verify-terminal.sh

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     Terminal Verification for The Hack: Ghost Protocol       ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check terminal type
echo -e "${CYAN}1. Terminal Type${NC}"
echo "   TERM: $TERM"
if [ -z "$TERM" ]; then
    echo -e "   ${RED}✗ TERM variable not set${NC}"
else
    echo -e "   ${GREEN}✓ TERM variable set${NC}"
fi
echo ""

# Check terminal size
echo -e "${CYAN}2. Terminal Size${NC}"
cols=$(tput cols 2>/dev/null || echo "0")
rows=$(tput lines 2>/dev/null || echo "0")
echo "   Current size: ${cols}x${rows}"
echo "   Minimum required: 80x24"
echo "   Recommended: 100x30"

if [ "$cols" -ge 100 ] && [ "$rows" -ge 30 ]; then
    echo -e "   ${GREEN}✓ Optimal size${NC}"
elif [ "$cols" -ge 80 ] && [ "$rows" -ge 24 ]; then
    echo -e "   ${YELLOW}⚠ Adequate size (consider expanding)${NC}"
else
    echo -e "   ${RED}✗ Terminal too small${NC}"
fi
echo ""

# Check color support
echo -e "${CYAN}3. Color Support${NC}"
colors=$(tput colors 2>/dev/null || echo "0")
echo "   Colors supported: $colors"

if [ "$colors" -ge 256 ]; then
    echo -e "   ${GREEN}✓ Excellent color support (256+)${NC}"
elif [ "$colors" -ge 16 ]; then
    echo -e "   ${YELLOW}⚠ Basic color support (16)${NC}"
else
    echo -e "   ${RED}✗ Limited color support${NC}"
fi
echo ""

# Check UTF-8 encoding
echo -e "${CYAN}4. UTF-8 Encoding${NC}"
echo "   LANG: $LANG"
echo "   LC_ALL: $LC_ALL"

if [[ "$LANG" == *"UTF-8"* ]] || [[ "$LC_ALL" == *"UTF-8"* ]]; then
    echo -e "   ${GREEN}✓ UTF-8 encoding enabled${NC}"
else
    echo -e "   ${RED}✗ UTF-8 not detected${NC}"
    echo -e "   ${YELLOW}   Add to ~/.bashrc or ~/.zshrc:${NC}"
    echo "   export LANG=en_US.UTF-8"
    echo "   export LC_ALL=en_US.UTF-8"
fi
echo ""

# Test box-drawing characters
echo -e "${CYAN}5. Unicode Box-Drawing Test${NC}"
echo "   ╔═══════════════════╗"
echo "   ║  Unicode Test     ║"
echo "   ╠═══════════════════╣"
echo "   ║  ► Special chars  ║"
echo "   ╚═══════════════════╝"
echo ""
echo "   If the above box looks correct:"
echo -e "   ${GREEN}✓ Unicode rendering OK${NC}"
echo ""

# Test ANSI colors
echo -e "${CYAN}6. ANSI Color Test${NC}"
echo -e "   ${RED}Red${NC} ${GREEN}Green${NC} ${YELLOW}Yellow${NC} ${BLUE}Blue${NC} \033[35mMagenta\033[0m ${CYAN}Cyan${NC}"
echo -e "   \033[91mBright Red\033[0m \033[92mBright Green\033[0m \033[93mBright Yellow\033[0m"
echo ""
echo "   If colors display correctly:"
echo -e "   ${GREEN}✓ ANSI color codes working${NC}"
echo ""

# Check for required tools
echo -e "${CYAN}7. Required Tools${NC}"

if command -v cargo &> /dev/null; then
    echo -e "   ${GREEN}✓ cargo found${NC} ($(cargo --version))"
else
    echo -e "   ${RED}✗ cargo not found${NC}"
fi

if command -v rustc &> /dev/null; then
    echo -e "   ${GREEN}✓ rustc found${NC} ($(rustc --version))"
else
    echo -e "   ${RED}✗ rustc not found${NC}"
fi
echo ""

# Performance check
echo -e "${CYAN}8. Performance Check${NC}"
if [ -n "$COLORTERM" ]; then
    echo "   COLORTERM: $COLORTERM"
    echo -e "   ${GREEN}✓ Truecolor support advertised${NC}"
else
    echo "   COLORTERM: not set"
    echo -e "   ${YELLOW}⚠ Truecolor may not be available${NC}"
fi
echo ""

# Summary
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                    Verification Summary                       ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Overall assessment
warnings=0
errors=0

[ "$cols" -lt 80 ] || [ "$rows" -lt 24 ] && ((errors++))
[ "$colors" -lt 16 ] && ((errors++))
[[ "$LANG" != *"UTF-8"* ]] && [[ "$LC_ALL" != *"UTF-8"* ]] && ((errors++))
! command -v cargo &> /dev/null && ((errors++))

[ "$cols" -lt 100 ] || [ "$rows" -lt 30 ] && [ "$cols" -ge 80 ] && ((warnings++))
[ "$colors" -lt 256 ] && [ "$colors" -ge 16 ] && ((warnings++))

if [ $errors -eq 0 ]; then
    if [ $warnings -eq 0 ]; then
        echo -e "${GREEN}✓ All checks passed! Terminal is optimally configured.${NC}"
        echo ""
        echo "You're ready to play The Hack: Ghost Protocol!"
        echo "Run: cargo run"
    else
        echo -e "${YELLOW}⚠ Terminal is adequate but could be improved.${NC}"
        echo ""
        echo "Consider the recommendations above for the best experience."
        echo "You can still play: cargo run"
    fi
else
    echo -e "${RED}✗ Some critical checks failed.${NC}"
    echo ""
    echo "Please address the issues above before playing."
    echo "See docs/TERMINAL_SETUP.md for configuration help."
fi

echo ""
echo "For detailed configuration, see: docs/TERMINAL_SETUP.md"
