# NexOS First Boot

After installation, NexOS starts a one-shot first-boot service.

It provides a welcome message and points users to:

- NexOS Settings
- NexOS System Tools
- NexOS Recovery Tools
- Firefox ESR
- Thunar File Manager

The first-boot marker is:

`/var/lib/nexos/firstboot-complete`

The service is safe to run repeatedly: once the marker exists it exits without changing user configuration.
