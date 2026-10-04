# NexOS Security Defaults

NexOS uses standard Debian security mechanisms.

## Firewall

UFW defaults to:
- deny incoming
- allow outgoing

Inspect with:

```sh
sudo ufw status verbose
```

## AppArmor

AppArmor is installed and enabled where profiles are available:

```sh
sudo aa-status
```

## Security updates

NexOS enables Debian unattended security updates. Major upgrades remain under normal package-management control.

## Recovery

Package repair and initramfs recovery are available through:

```sh
sudo nexos-recovery repair
```

Security controls reduce risk; they are not a guarantee against compromise. Users should keep backups and install updates promptly.
