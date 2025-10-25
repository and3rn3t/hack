# OSINT Challenge Guide

**The Hack: Ghost Protocol - Open Source Intelligence Challenges**

## What is OSINT?

**Open Source Intelligence (OSINT)** is the collection and analysis of information gathered from publicly available sources. In cybersecurity, OSINT is used for:

-   Reconnaissance and information gathering
-   Security assessments and penetration testing
-   Threat intelligence and incident response
-   Digital forensics and investigations
-   Social engineering defenses

## Level 2 OSINT Challenges (10 Total)

### Core OSINT Challenges

#### 1. Digital Footprints (`osint_social_media`)

**Difficulty**: ⭐⭐⭐
**Focus**: GPS coordinates and social media metadata

Learn how photos posted on social media can reveal your exact location through EXIF metadata. Discover why sharing images without removing metadata can compromise your privacy.

**Real-World Use**: Journalists and investigators use this technique to verify locations and authenticate images.

---

#### 2. Domain Investigation (`osint_domain_recon`)

**Difficulty**: ⭐⭐⭐
**Focus**: WHOIS and DNS reconnaissance

Explore how domain registration information and DNS records can reveal organizational details, email providers, and infrastructure information.

**Real-World Use**: Security researchers use WHOIS/DNS data to identify attack infrastructure and track threat actors.

---

#### 3. Electronic Trail (`osint_email_analysis`)

**Difficulty**: ⭐⭐⭐
**Focus**: Email header analysis and phishing detection

Analyze email headers to identify phishing attempts, track originating IPs, and recognize typosquatting attacks.

**Real-World Use**: Security teams analyze email headers to identify phishing campaigns and block malicious senders.

---

#### 4. Location Triangulation (`osint_geolocation`)

**Difficulty**: ⭐⭐⭐⭐
**Focus**: Contextual geolocation from clues

Learn to triangulate locations using contextual clues like landmarks, time zones, language patterns, and local references.

**Real-World Use**: Intelligence analysts use context clues to geolocate images and verify claims when metadata is unavailable.

---

#### 5. Data Breach Analysis (`osint_breach_investigation`)

**Difficulty**: ⭐⭐⭐
**Focus**: Password security and hashing

Understand why proper password storage (salting and strong hashing) is critical after analyzing a data breach scenario.

**Real-World Use**: Forensics teams analyze breaches to understand attack vectors and improve security practices.

---

### New Advanced OSINT Challenges

#### 6. Digital Identity Hunt (`osint_username_recon`)

**Difficulty**: ⭐⭐⭐
**Focus**: Username enumeration across platforms

Master the technique of correlating accounts across multiple platforms using username patterns and email prefixes.

**Tools to Explore**:

-   Sherlock (GitHub username search tool)
-   WhatsMyName (cross-platform username checker)
-   Namechk (username availability checker)

**Real-World Use**: Threat hunters track adversaries across platforms to build comprehensive threat profiles.

---

#### 7. Echoes from the Past (`osint_wayback_machine`)

**Difficulty**: ⭐⭐⭐
**Focus**: Internet Archive and historical web data

Discover how the Wayback Machine preserves deleted content, allowing you to uncover contradictions and recover lost information.

**Tools to Explore**:

-   archive.org Wayback Machine
-   Archive.today (alternative archiving service)
-   Google Cache (recent snapshots)

**Real-World Use**: Investigators use archives to prove claims, recover deleted evidence, and track website changes over time.

---

#### 8. Hidden Data Trails (`osint_metadata_extraction`)

**Difficulty**: ⭐⭐⭐⭐
**Focus**: Document and image metadata extraction

Learn to extract hidden information from files that can reveal sensitive details like authors, creation dates, and internal company information.

**Tools to Explore**:

-   ExifTool (comprehensive metadata viewer)
-   PDF metadata viewers
-   Microsoft Office document properties
-   Image metadata extractors

**Real-World Use**: Forensics analysts extract metadata to authenticate documents, trace origins, and uncover hidden information.

---

#### 9. The Internet Scanner (`osint_shodan_recon`)

**Difficulty**: ⭐⭐⭐⭐⭐
**Focus**: Internet-connected device discovery

Explore Shodan, the "Google for hackers" that indexes internet-connected devices worldwide, revealing exposed systems and vulnerabilities.

**What Shodan Reveals**:

-   Open ports and running services
-   Vulnerable industrial control systems
-   Exposed webcams and IoT devices
-   Misconfigured databases and servers
-   Network infrastructure details

**Real-World Use**: Security teams use Shodan to identify exposed assets in their organization and remediate vulnerabilities before attackers exploit them.

**Important**: Only search for devices you own or have permission to assess. Unauthorized access is illegal.

---

#### 10. Data Dump Discovery (`osint_pastebin_leak`)

**Difficulty**: ⭐⭐⭐
**Focus**: Credential leak monitoring on paste sites

Learn how attackers dump stolen credentials on public text-sharing sites and how to monitor for leaked organizational data.

**Paste Sites to Monitor**:

-   Pastebin.com
-   Ghostbin
-   Hastebin
-   GitHub Gists
-   Rentry.co

**Real-World Use**: Security teams monitor paste sites for credential dumps mentioning their domain to detect breaches early.

---

## OSINT Best Practices

### Legal and Ethical Guidelines

1. **Only investigate information that is publicly available**

    - Don't hack, crack, or bypass security measures
    - Don't social engineer or deceive people
    - Stay within legal boundaries

2. **Respect privacy and data protection laws**

    - GDPR in Europe
    - CCPA in California
    - Local privacy regulations

3. **Use OSINT responsibly**
    - Don't harass or stalk individuals
    - Don't use information for malicious purposes
    - Report vulnerabilities through responsible disclosure

### OSINT Framework

A systematic approach to OSINT investigations:

```
1. Requirements Definition
   └─ What information do you need?

2. Source Identification
   └─ Where can you find this information?

3. Data Collection
   └─ Gather information from identified sources

4. Processing & Integration
   └─ Organize and correlate collected data

5. Analysis
   └─ Draw conclusions from the data

6. Reporting
   └─ Present findings clearly and ethically
```

---

## Recommended OSINT Tools

### Free Tools (Legal and Ethical)

**Search and Discovery**:

-   Google Dorking (advanced search operators)
-   Shodan.io (internet-connected devices)
-   Wayback Machine (archived websites)
-   Have I Been Pwned (breach checking)

**Metadata Analysis**:

-   ExifTool (EXIF data extraction)
-   FOCA (document metadata analyzer)
-   Metagoofil (metadata extraction tool)

**Domain & Network**:

-   WHOIS lookups (domain registration info)
-   DNSDumpster (DNS reconnaissance)
-   SecurityTrails (historical DNS data)
-   Censys (internet-wide scanning)

**Social Media**:

-   Sherlock (username search)
-   Social Searcher (social media monitoring)
-   TweetDeck (Twitter monitoring)

**Username & Email**:

-   Namechk (username availability)
-   Hunter.io (email finder)
-   EmailRep (email reputation)

### OSINT Frameworks

-   **Maltego** (commercial, visual link analysis)
-   **Recon-ng** (command-line reconnaissance)
-   **SpiderFoot** (automated OSINT collection)
-   **theHarvester** (email/subdomain gathering)

---

## Learning Resources

### Books

-   "Open Source Intelligence Techniques" by Michael Bazzell
-   "OSINT Handbook" by i-intelligence
-   "Social Engineering: The Art of Human Hacking" by Christopher Hadnagy

### Courses

-   SANS SEC487: Open-Source Intelligence (OSINT) Gathering and Analysis
-   Udemy: OSINT courses (various instructors)
-   TraceLabs OSINT training

### Communities

-   r/OSINT (Reddit community)
-   OSINT Curious (community and projects)
-   TraceLabs (OSINT for missing persons)
-   Bellingcat (investigative journalism with OSINT)

### Practice

-   TraceLabs CTF (OSINT competitions for good)
-   OSINT exercises and challenges online
-   The Hack: Ghost Protocol (this game!)

---

## Tips for Success

### Challenge-Solving Strategies

1. **Read Carefully**: Challenge descriptions contain all the clues you need
2. **Think Like an Investigator**: What would you Google? What patterns do you see?
3. **Use Hints Wisely**: Hints are progressive - use them if stuck
4. **Learn the Concepts**: Understanding techniques is more important than answers
5. **Stay Ethical**: Apply OSINT skills responsibly in the real world

### Common Mistakes to Avoid

-   Overthinking simple questions
-   Ignoring obvious clues in descriptions
-   Not using available hints when stuck
-   Trying to "hack" instead of investigating
-   Forgetting that OSINT uses publicly available information

---

## Real-World Applications

### Career Paths Using OSINT

-   **Security Analyst**: Monitor threats and investigate incidents
-   **Penetration Tester**: Gather reconnaissance for assessments
-   **Threat Intelligence Analyst**: Track adversaries and campaigns
-   **Digital Forensics Investigator**: Analyze evidence and trace origins
-   **Bug Bounty Hunter**: Reconnaissance for vulnerability research
-   **Journalist/Researcher**: Investigate and verify information
-   **Compliance Analyst**: Monitor for data leaks and breaches

### Industry Use Cases

**Corporate Security**:

-   Monitor for credential leaks
-   Identify exposed assets
-   Investigate security incidents
-   Competitive intelligence

**Law Enforcement**:

-   Digital investigations
-   Missing persons cases
-   Fraud detection
-   Evidence collection

**Journalism**:

-   Source verification
-   Fact-checking
-   Investigative reporting
-   Geolocation verification

---

## Challenge Progression

### Beginner Path (Start Here)

1. Digital Footprints (GPS/EXIF basics)
2. Domain Investigation (WHOIS/DNS)
3. Electronic Trail (Email analysis)

### Intermediate Path

4. Username Enumeration (Cross-platform)
5. Wayback Machine (Historical data)
6. Breach Investigation (Password security)

### Advanced Path

7. Metadata Extraction (File analysis)
8. Pastebin Monitoring (Leak detection)
9. Location Triangulation (Context analysis)

### Expert Path

10. Shodan Reconnaissance (Internet scanning)

---

## Additional Challenge Notes

### Answer Formats

Most OSINT challenges accept multiple answer formats:

-   Case-insensitive
-   Common variations (hyphens, spaces, underscores)
-   Abbreviations when appropriate
-   Alternative terminology

**Examples**:

-   "Wayback Machine" = "Internet Archive" = "Web Archive"
-   "Username Enumeration" = "Username Recon"
-   "Metadata" = "EXIF" = "File Metadata"

### Sanity Management

OSINT challenges cost **10-15 sanity** each. Plan your approach:

-   Read descriptions thoroughly before attempting
-   Use hints if you're stuck after 2-3 attempts
-   Take breaks if frustrated
-   Remember: Learning is the goal, not just completion

---

## Get Started

Ready to become an OSINT expert? Start with Level 2 challenges and work your way through all 10 OSINT challenges. Each one teaches valuable real-world investigation techniques used by security professionals worldwide.

**Remember**: With great OSINT knowledge comes great responsibility. Use these skills ethically and legally!

---

**🔍 "The best intelligence is open source intelligence. The best investigators are curious investigators." - Ghost Protocol**

**Game Commands**:

-   `challenges` - View all available challenges
-   `hint` - Get a progressive hint when stuck
-   `skip` - Skip a challenge if too difficult
-   `stats` - View your progress

Good luck, investigator! 👻🔍
