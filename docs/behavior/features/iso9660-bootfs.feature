Feature: ISO9660 boot filesystem mount

  Scenario: iso9660d mounts the livedisk filesystem at /media/livedisk
    Given the machine is booted
    Then the log should match pattern "iso9660d: found ISO9660 on device (atapi|ata_|ahci)"
    And the serial output should contain "iso9660d: mounted at /media/livedisk" within 20s
    When I wait for the shell prompt
    And I type "cat /media/livedisk/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: iso9660d serves multiple sequential reads correctly
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "cat /media/livedisk/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"
    When I type "cat /media/livedisk/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: iso9660d handles sequential reads and readdir without response mis-correlation
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "ls /media/livedisk" on the serial console
    Then the latest command output should contain "etc"
    When I type "cat /media/livedisk/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"
    And I type "ls /media/livedisk/etc" on the serial console
    Then the latest command output should contain "hostname"
    When I type "cat /media/livedisk/etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: boot ISO files are accessible at /media/livedisk
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "cd /media/livedisk" on the serial console
    And I type "ls" on the serial console
    Then the latest command output should contain "etc"
    And the latest command output should contain "bin"
    And the latest command output should contain "applications"
    And the latest command output should contain "lib"
    And the latest command output should contain "public"
    And the latest command output should contain "drivers"

  Scenario: boot fruit root is overlaid into the root namespace
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "cat /etc/hostname" on the serial console
    Then the latest command output should contain "thingos"

  Scenario: /etc/roots/boot describes the livedisk root overlay
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "cat /etc/roots/boot" on the serial console
    Then the latest command output should contain "source=/media/livedisk"
    And the latest command output should contain "target=/"
    And the latest command output should contain "flags=before,cor"

  Scenario: runtime root exposes the canonical top-level layout
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "ls /" on the serial console
    Then the command output should contain "bin"
    And the command output should contain "applications"
    And the command output should contain "lib"
    And the command output should contain "etc"
    And the command output should contain "dev"
    And the command output should contain "proc"
    And the command output should contain "sys"
    And the command output should contain "run"
    And the command output should contain "tmp"
    And the command output should contain "media"
    And the command output should contain "drivers"
    And the command output should contain "session"
    And the command output should contain "public"
    And the command output should contain "services"
    And the command output should contain "version"
    And the command output should not contain "boot"
    And the command output should not contain "data"
    And the command output should not contain "drv"
    And the command output should not contain "hosts"
    And the command output should not contain "mnt"
    And the command output should not contain "share"
    And the command output should not contain "srv"
    And the command output should not contain "vol"

  Scenario: /bin contains the traditional command set and services live elsewhere
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "ls /bin" on the serial console
    Then the command output should contain "cat"
    And the command output should contain "cp"
    And the command output should contain "ls"
    And the command output should contain "mkdir"
    And the command output should contain "mv"
    And the command output should contain "rm"
    And the command output should contain "stat"
    And the command output should contain "sh"
    And the command output should contain "tree"
    And the command output should not contain "bloom"
    And the command output should not contain "bristle"
    And the command output should not contain "cambium"
    And the command output should not contain "iso9660d"
    When I type "ls /services" on the serial console
    Then the command output should contain "bloom"
    And the command output should contain "bristle"
    And the command output should contain "cambium"
    And the command output should contain "fatd"
    And the command output should contain "iso9660d"
    And the command output should contain "mdns"
    And the command output should contain "mdnsd"
    And the command output should contain "mesocarp"
    And the command output should contain "netd"

  Scenario: ELF binaries are loaded correctly via the memfd bulk-transfer path
    # Exercises ReadIntoFd: the kernel injects a memfd into iso9660d's fd table
    # and iso9660d writes the entire binary in one vfs_write syscall, removing
    # the O(file_size / 64 KiB) IPC round-trip bottleneck.
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "ps" on the serial console
    Then the latest command output should contain "PID"
    When I type "ps" on the serial console
    Then the latest command output should contain "PID"

  Scenario: Repeated process spawns use the kernel page cache (no redundant IPC reads)
    # Verifies that the second execution of /bin/ps does not re-read the ELF
    # from iso9660d (the kernel VFS page cache serves the cached bytes).
    Given the machine is booted
    When I wait for the shell prompt
    And I wait for the serial output to contain "SPROUT: /etc/roots activation complete"
    And I type "ps" on the serial console
    Then the latest command output should contain "PID"
    And the log should match pattern "SPAWN: page cache miss for"
    When I type "ps" on the serial console
    Then the latest command output should contain "PID"
    And the log should match pattern "SPAWN: page cache hit for"
