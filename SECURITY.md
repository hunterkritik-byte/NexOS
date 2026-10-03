# Security Policy

NexOS is a security-sensitive operating-system project.

## Reporting

Do not disclose an unpatched kernel vulnerability in a public issue. Report security issues privately to the maintainer or through GitHub private vulnerability reporting when available.

Project inquiries and security coordination:

hunterkritik@gmail.com

## Security principles

- Validate data at kernel boundaries.
- Keep unsafe Rust isolated and reviewed.
- Minimize privileged code.
- Do not treat debug assertions as security boundaries.
- Add regression tests for security fixes.
- Never ship known exploitable kernel bugs as a production release.
