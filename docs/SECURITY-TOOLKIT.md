# NexOS Security & Bug-Hunting Toolkit

NexOS provides an optional security toolkit for **authorized security research, bug bounty work, defensive testing, and lab environments**.

## Included

- Nmap — network discovery and auditing
- Wireshark / TShark — packet analysis
- tcpdump — packet capture
- Netcat — network diagnostics
- DNS utilities / whois — DNS and domain diagnostics
- curl / wget — HTTP and API testing
- Git — source/repository work
- Python + virtual environments — scripting and research
- jq / ripgrep — data and source analysis
- GDB / binutils / strace / ltrace — native debugging and analysis
- GHex — binary inspection
- Nikto — web-server assessment
- SQLMap — authorized SQL-injection testing

## Usage policy

Only test systems, applications, accounts and networks where you have explicit authorization. Bug-bounty testing must remain inside the target program's published scope and rules.

The toolkit does not grant permission to attack third-party systems.

## Optional tools

Large or separately licensed applications should be installed through their official repositories/installers rather than embedded into the NexOS base ISO. This keeps the normal NexOS image smaller and lets users receive current versions.

Nmap redistribution has additional licensing considerations for products, so NexOS should use the Debian package/repository model and review the applicable license before shipping it in a redistributed image. See the Nmap licensing documentation before each release.

## Installation

The toolkit package list is kept separate from the normal desktop package list so NexOS can offer:

- NexOS Daily — normal desktop
- NexOS Security Toolkit — authorized research tools

This design avoids making every daily-use installation carry every security tool.
