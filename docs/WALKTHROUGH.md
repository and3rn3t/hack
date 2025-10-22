# The Hack: Ghost Protocol - Walkthrough Guide

This walkthrough provides solutions and explanations for all challenges. **Spoiler warning!**

## Level 0: The Awakening

### Challenge 1: The First Message
**Encoded Message**: `V2VsY29tZSB0byB0aGUgR2hvc3QgUHJvdG9jb2w=`

**Solution**: `Welcome to the Ghost Protocol`

**Explanation**: This is Base64 encoding. You can decode it using:
- Online tools: base64decode.org
- Linux command: `echo "V2VsY29tZSB0byB0aGUgR2hvc3QgUHJvdG9jb2w=" | base64 -d`
- Python: `import base64; base64.b64decode(b"V2VsY29tZSB0byB0aGUgR2hvc3QgUHJvdG9jb2w=").decode()`

**Learning**: Base64 is commonly used to encode binary data in text format. It's not encryption, just encoding!

---

### Challenge 2: Hidden Files
**Answer**: `ghost_admin_2024`

**Explanation**: On Unix-like systems (Linux, macOS), files starting with a dot (`.`) are hidden. The challenge mentions a file called `.secret_key` that contains the password.

**Learning**: 
- Use `ls -la` to see hidden files
- Many configuration files are hidden (like `.bashrc`, `.gitconfig`)
- This is a common place to store secrets (though not secure!)

---

### Challenge 3: The Open Door
**Answer**: `6666`

**Explanation**: Among standard ports (22=SSH, 80=HTTP, 443=HTTPS, 3306=MySQL), port 6666 stands out as unusual and is referenced as the "devil's port" in the challenge.

**Learning**: 
- Common ports should be memorized for security work
- Unusual ports often indicate custom services or malware
- Port scanning is a key reconnaissance technique

---

## Level 1: Whispers in Code

### Challenge 4: Caesar Cipher
**Encoded**: `JSHZW SURWRFRO FRPSOHWH. WKH DQVZHU LV: FUBSWRJUDSKB`

**Solution**: `CRYPTOGRAPHY`

**Explanation**: Caesar cipher shifts each letter by a fixed amount. This uses ROT3 (shift by 3).
- J → G (back 3)
- S → P (back 3)
- H → E (back 3)

The decoded message reads: "GHOST PROTOCOL COMPLETE. THE ANSWER IS: CRYPTOGRAPHY"

**Learning**: 
- Caesar cipher is one of the oldest encryption methods
- It's vulnerable to frequency analysis
- ROT13 is a special case (shift by 13) commonly used online

---

### Challenge 5: SQL Injection Basics
**Answer**: `' OR '1'='1' --` (or variations like `admin'--`)

**Explanation**: The vulnerable query is:
```sql
SELECT * FROM users WHERE username='[INPUT]' AND password='[PASS]'
```

By injecting `' OR '1'='1' --`, it becomes:
```sql
SELECT * FROM users WHERE username='' OR '1'='1' --' AND password='[PASS]'
```

The `--` comments out everything after, and `'1'='1'` is always true.

**Learning**:
- SQL injection is a critical web vulnerability
- Always use parameterized queries
- Never trust user input
- This is #1 on OWASP Top 10

---

### Challenge 6: Hexadecimal Horror
**Encoded**: `48 45 58 41 44 45 43 49 4D 41 4C`

**Solution**: `HEXADECIMAL`

**Explanation**: Convert hex pairs to ASCII:
- 48 → H
- 45 → E
- 58 → X
- 41 → A
- 44 → D
- 45 → E
- 43 → C
- 49 → I
- 4D → M
- 41 → A
- 4C → L

**Learning**:
- Hexadecimal is base-16 (0-9, A-F)
- Each byte can be represented by 2 hex digits
- Common in memory addresses and binary data

---

## Level 2: Mobile/Web Challenges

### Challenge 7: HTTP Headers
**Encoded**: `X-Ghost-Token: R0hPU1RfVE9LRU4=`

**Solution**: `GHOST_TOKEN`

**Explanation**: The header value is Base64 encoded. Decode it to get the answer.

**Learning**:
- HTTP headers often contain metadata
- Custom headers (X-*) can contain important information
- Headers are visible in browser developer tools
- Sensitive data in headers is a security risk

---

### Challenge 8: Mobile Deep Link
**Deep Link**: `myapp://secret/unlock/MOBILEHACK`

**Solution**: `MOBILEHACK`

**Explanation**: Deep links are URIs that open specific parts of mobile apps. The parameter value "MOBILEHACK" is in the URL structure.

**Learning**:
- Deep links can expose app functionality
- They should be properly validated
- Can be exploited for unauthorized access
- Important for mobile app security testing

---

## Level 3+: Advanced Challenges

### Challenge 9: Buffer Overflow
**Solution**: `OVERFLOW` or `BUFFER OVERFLOW`

**Explanation**: The code shows:
```c
char buffer[8];
strcpy(buffer, user_input);
```

If user_input is longer than 8 characters, it overflows into adjacent memory. This is a classic buffer overflow vulnerability.

**Learning**:
- Buffer overflows can overwrite memory
- Can lead to code execution
- Use safe functions like `strncpy` or bounds checking
- Critical vulnerability in C/C++ programs

---

### Challenge 10: Reverse Engineering
**Given**: `input XOR 0x42 == 0x2D`

**Solution**: `o` (or `111` or `0x6F`)

**Explanation**: 
- XOR is reversible: if A ⊕ B = C, then A ⊕ C = B
- So: input = 0x2D ⊕ 0x42
- 0x2D = 45 (decimal)
- 0x42 = 66 (decimal)
- 45 ⊕ 66 = 111 = 0x6F = 'o' in ASCII

**Learning**:
- XOR is commonly used in encryption
- It's reversible (self-inverse)
- Useful for obfuscation in malware
- Important for reverse engineering

---

### Challenge 11: The Ghost's True Name
**Solution**: Various accepted (like `PROTOCOL`, `PHANTOM`)

**Explanation**: This meta-challenge asks you to look at the first letter of each challenge ID from the challenges you've completed:
- welcome → W
- file_discovery → F
- port_scan → P
- caesar_cipher → C
- sql_injection_basics → S
- hex_decode → H
- http_header → H
- mobile_deeplink → M
- binary_exploit → B
- reverse_engineering → R
- final_protocol → F

**Learning**:
- CTF challenges often have meta-puzzles
- Pay attention to patterns and connections
- Sometimes the journey is the answer

---

## Tips and Strategies

### For Beginners
1. **Start with hints**: Don't struggle - the hints are progressive
2. **Use online tools**: Base64 decoders, hex converters are your friends
3. **Google concepts**: Research terms you don't understand
4. **Take breaks**: If stuck, step away and return fresh

### For Advanced Players
1. **Try without hints**: Challenge yourself first
2. **Understand the WHY**: Don't just solve, understand the vulnerability
3. **Research real exploits**: Look up real-world examples
4. **Think creatively**: Try different approaches

### General Strategy
1. **Read carefully**: Challenges contain clues in the descriptions
2. **Manage sanity**: Do easier challenges first to build XP
3. **Save often**: Use the save command between attempts
4. **Learn from failures**: Each wrong answer teaches something

---

## Real-World Applications

These challenges teach skills used in:
- **Penetration Testing**: Finding vulnerabilities in systems
- **Security Research**: Understanding attack vectors
- **Bug Bounty Hunting**: Finding and reporting security issues
- **Incident Response**: Understanding how attacks work
- **Security Engineering**: Building secure systems

---

## Resources for Learning More

### Online Platforms
- **HackTheBox**: Real hacking challenges
- **TryHackMe**: Guided cybersecurity training
- **OverTheWire**: Classic wargames
- **PicoCTF**: Beginner-friendly CTF

### Books
- "The Web Application Hacker's Handbook"
- "Hacking: The Art of Exploitation"
- "The Shellcoder's Handbook"

### Tools
- **Burp Suite**: Web application testing
- **Wireshark**: Network analysis
- **Ghidra**: Reverse engineering
- **Metasploit**: Exploitation framework

---

## Easter Eggs and Secrets

Throughout the game, watch for:
- **Glitched text**: Sometimes contains hidden messages
- **Sanity drops**: Different messages at different sanity levels
- **Multiple endings**: Your completion affects the ending
- **Hidden achievements**: Discover all secrets for bonus content

---

**Remember**: The best hackers are always learning. Every challenge you solve builds your knowledge and skills!

*"The Ghost Protocol was never about escaping... it was about understanding."*
