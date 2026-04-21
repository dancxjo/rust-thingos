Feature: mDNS /hosts VFS
  Verify that the mesocarp daemon exposes the local host at `/hosts` and
  advertises the hostname configured in `/etc/hostname`.

  Mesocarp is mounted via `/etc/fstab` (type `mdns`) so `/hosts` should be
  available by the time the shell prompt is reached. Because QEMU's
  default user-mode networking does not bridge multicast to the host LAN,
  this scenario only exercises the local (self) half of the service: the
  daemon must still read `/etc/hostname`, discover our own IPv4, and
  publish a `<hostname>.local` entry in the VFS.

  Scenario: Self hostname is published in /hosts
    Given the machine is booted
    When I wait for the shell prompt
    And I type "cat /etc/hostname" on the serial console
    Then the serial output should contain "thingos"
    When I type "ls /hosts" on the serial console
    And I wait for 2 seconds
    Then the serial output should contain "thingos.local"
    When I type "attr_list /hosts/thingos.local" on the serial console
    And I wait for 1 seconds
    Then the serial output should contain "net.ip.ipv4"
    And the serial output should contain "net.hostname"
    When I type "attr_get /hosts/thingos.local net.hostname" on the serial console
    And I wait for 1 seconds
    Then the serial output should contain "thingos.local"
