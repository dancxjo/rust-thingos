# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-21 15:35:46

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11569ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2204ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 example.com" on the serial console | ✅ | 2133ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ❌ | 301072ms | - [📜](./04/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[33897151596] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[36745478319] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[36752189364] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[36878263950] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[37751886249] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[37883680791] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
vfs: mount::lookup path='/bin'
vfs: mount match prefix='/' rel='bin'
[37906970145] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount calling driver.lookup rel='bin'
[37922973726] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount driver.lookup rel='bin' OK
vfs: mount::lookup path='/run/sprout/shell'
vfs: mount match prefix='/run' rel='sprout/shell'
vfs: mount match prefix='/' rel='run/sprout/shell'
[37996576596] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount calling driver.lookup rel='sprout/shell'
[37999064004] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount driver.lookup rel='sprout/shell' ENOENT
[38001019155] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount calling driver.lookup rel='run/sprout/shell'
[38007178176] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount driver.lookup rel='run/sprout/shell' ENOENT
vfs: mount::lookup path='/etc/default/shell'
vfs: mount match prefix='/' rel='etc/default/shell'
[38011806525] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount calling driver.lookup rel='etc/default/shell'
[38016581988] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount driver.lookup rel='etc/default/shell' ENOENT
vfs: mount::lookup path='/bin/sh'
vfs: mount match prefix='/' rel='bin/sh'
[38022511362] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount calling driver.lookup rel='bin/sh'
[38025619269] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount driver.lookup rel='bin/sh' OK
vfs: mount::lookup path='/dev/console'
vfs: mount match prefix='/dev' rel='console'
vfs: mount match prefix='/' rel='dev/console'
[38031193695] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount calling driver.lookup rel='console'
[38035447692] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount driver.lookup rel='console' OK
vfs: mount::lookup path='/bin/sh'
vfs: mount match prefix='/' rel='bin/sh'
[38050786686] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount calling driver.lookup rel='bin/sh'
[38052863739] [[32mINFO [0m] [kernel::vfs::mount] [CPU0] vfs: mount driver.lookup rel='bin/sh' OK
vfs: mount::lookup path='/run/motd'
vfs: mount match prefix='/run' rel='motd'
vfs: mount match prefix='/' rel='run/motd'
[38176141608] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='motd'
[38179497675] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='motd' ENOENT
[38182844436] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='run/motd'
[38194415754] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='run/motd' ENOENT
vfs: mount::lookup path='/etc/motd'
vfs: mount match prefix='/' rel='etc/motd'
[38205833721] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='etc/motd'
[38213519850] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='etc/motd' OK
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ��  
      |     |       2026-04-16
       \   /
        `-'
[0m
[2m--------------------------------------------------------------[0m
[1m sprout has taken root. the system is awake.[0m

  try:
    [36mls /bin[0m       browse available shoots
    [36mps[0m            observe living processes
    [36mcat /version[0m  inspect the genome

[2m--------------------------------------------------------------[0m
vfs: mount::lookup path='/etc/profile'
vfs: mount match prefix='/' rel='etc/profile'
[38267509203] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='etc/profile'
[38272782108] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='etc/profile' OK
vfs: mount::lookup path='/drivers'
vfs: mount match prefix='/' rel='drivers'
[38301623580] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers'
[38309401383] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers' OK
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hvfs: mount::lookup path='/drivers/ahci_disk'
vfs: mount match prefix='/' rel='drivers/ahci_disk'
[38392453572] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/ahci_disk'
[38398784655] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/ahci_disk' OK
vfs: mount::lookup path='/drivers/chime'
vfs: mount match prefix='/' rel='drivers/chime'
[38875949838] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/chime'
[38881389888] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/chime' OK
vfs: mount::lookup path='/drivers/display_bootfb'
vfs: mount match prefix='/' rel='drivers/display_bootfb'
[39124386741] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/display_bootfb'
[39128744556] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/display_bootfb' OK
vfs: mount::lookup path='/drivers/display_virtio_gpu'
vfs: mount match prefix='/' rel='drivers/display_virtio_gpu'
[39441815160] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/display_virtio_gpu'
[39446718828] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/display_virtio_gpu' OK
vfs: mount::lookup path='/drivers/hdaudio'
vfs: mount match prefix='/' rel='drivers/hdaudio'
[39721379841] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/hdaudio'
[39725049045] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/hdaudio' OK
vfs: mount::lookup path='/drivers/pci_stubd'
vfs: mount match prefix='/' rel='drivers/pci_stubd'
[40062055407] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/pci_stubd'
[40066450611] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/pci_stubd' OK
vfs: mount::lookup path='/drivers/ps2_kbd'
vfs: mount match prefix='/' rel='drivers/ps2_kbd'
[40394917992] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/ps2_kbd'
[40399001049] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/ps2_kbd' OK
vfs: mount::lookup path='/drivers/ps2_mouse'
vfs: mount match prefix='/' rel='drivers/ps2_mouse'
[40738197456] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/ps2_mouse'
[40743388422] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/ps2_mouse' OK
[40934955963] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[41019567732] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
vfs: mount::lookup path='/dev/net/virtio0/rx'
vfs: mount match prefix='/dev' rel='net/virtio0/rx'
vfs: mount match prefix='/' rel='dev/net/virtio0/rx'
[41035715391] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio0/rx'
[41040070335] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio0/rx' ENOENT
[41043597342] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio0/rx'
[41055305148] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio0/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio1/rx'
vfs: mount::lookup path='/drivers/rtc_cmos'
vfs: mount match prefix='/dev' rel='net/virtio1/rx'
vfs: mount match prefix='/' rel='drivers/rtc_cmos'
vfs: mount match prefix='/' rel='dev/net/virtio1/rx'
[41071106076] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/rtc_cmos'
[41074229262] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio1/rx'
[41077174347] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/rtc_cmos' OK
[41080229487] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio1/rx' ENOENT
[41084031648] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio1/rx'
[41095822911] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio1/rx' ENOENT
vfs: mount::lookup path='/services/storage'
vfs: mount::lookup path='/dev/net/virtio2/rx'
vfs: mount match prefix='/services' rel='storage'
vfs: mount match prefix='/dev' rel='net/virtio2/rx'
vfs: mount match prefix='/' rel='services/storage'
vfs: mount match prefix='/' rel='dev/net/virtio2/rx'
[41112401583] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio2/rx'
[41115637068] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='storage'
[41118519255] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio2/rx' ENOENT
[41121314091] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='storage' ENOENT
[41124034545] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio2/rx'
[41128110705] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='services/storage'
[41136459210] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio2/rx' ENOENT
[41139331365] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='services/storage' ENOENT
vfs: mount::lookup path='/dev/net/virtio3/rx'
vfs: mount match prefix='/dev' rel='net/virtio3/rx'
[41146483554] [[32mINFO [0m] [iso9660d] [CPU3] iso9660d: no ISO9660 filesystem found yet — retrying
vfs: mount match prefix='/' rel='dev/net/virtio3/rx'
[41149906281] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio3/rx'
[41153125035] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio3/rx' ENOENT
[41156721969] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio3/rx'
[41166705954] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio3/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio4/rx'
vfs: mount match prefix='/dev' rel='net/virtio4/rx'
vfs: mount match prefix='/' rel='dev/net/virtio4/rx'
[41178234339] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio4/rx'
[41180868927] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio4/rx' ENOENT
[41184053229] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio4/rx'
[41190558255] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio4/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio5/rx'
vfs: mount match prefix='/dev' rel='net/virtio5/rx'
vfs: mount match prefix='/' rel='dev/net/virtio5/rx'
[41200844091] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio5/rx'
[41204623185] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio5/rx' ENOENT
[41208562659] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio5/rx'
[41218392402] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio5/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio6/rx'
vfs: mount match prefix='/dev' rel='net/virtio6/rx'
vfs: mount match prefix='/' rel='dev/net/virtio6/rx'
[41229434895] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio6/rx'
[41232676683] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio6/rx' ENOENT
[41236447230] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio6/rx'
[41248290336] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio6/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio7/rx'
vfs: mount match prefix='/dev' rel='net/virtio7/rx'
vfs: mount match prefix='/' rel='dev/net/virtio7/rx'
[41259826806] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio7/rx'
[41263138455] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio7/rx' ENOENT
[41266515312] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio7/rx'
[41279295585] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio7/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio8/rx'
vfs: mount match prefix='/dev' rel='net/virtio8/rx'
vfs: mount match prefix='/' rel='dev/net/virtio8/rx'
[41292611085] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio8/rx'
[41296060311] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio8/rx' ENOENT
[41301023280] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio8/rx'
[41313257733] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio8/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio9/rx'
vfs: mount match prefix='/dev' rel='net/virtio9/rx'
vfs: mount match prefix='/' rel='dev/net/virtio9/rx'
[41327438493] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio9/rx'
[41331541614] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio9/rx' ENOENT
[41335693311] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio9/rx'
[41347823286] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio9/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio10/rx'
vfs: mount match prefix='/dev' rel='net/virtio10/rx'
vfs: mount match prefix='/' rel='dev/net/virtio10/rx'
[41360450340] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio10/rx'
[41364361071] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio10/rx' ENOENT
[41368345656] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio10/rx'
vfs: mount::lookup path='/drivers/rtl8168d'
vfs: mount match prefix='/' rel='drivers/rtl8168d'
[41377083495] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/rtl8168d'
[41381134575] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/rtl8168d' OK
[41383311288] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio10/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio11/rx'
vfs: mount match prefix='/dev' rel='net/virtio11/rx'
vfs: mount match prefix='/' rel='dev/net/virtio11/rx'
[41395021503] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio11/rx'
[41398514058] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio11/rx' ENOENT
[41402623713] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio11/rx'
[41413350000] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio11/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio12/rx'
vfs: mount match prefix='/dev' rel='net/virtio12/rx'
vfs: mount match prefix='/' rel='dev/net/virtio12/rx'
[41423294979] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio12/rx'
[41426969001] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio12/rx' ENOENT
[41430756906] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio12/rx'
[41443007166] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio12/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio13/rx'
vfs: mount match prefix='/dev' rel='net/virtio13/rx'
vfs: mount match prefix='/' rel='dev/net/virtio13/rx'
[41453177271] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio13/rx'
[41457819216] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio13/rx' ENOENT
[41461777533] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio13/rx'
[41473945755] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio13/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio14/rx'
vfs: mount match prefix='/dev' rel='net/virtio14/rx'
vfs: mount match prefix='/' rel='dev/net/virtio14/rx'
[41485697451] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio14/rx'
[41489583729] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio14/rx' ENOENT
[41493083379] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio14/rx'
[41504560845] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio14/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio15/rx'
vfs: mount match prefix='/dev' rel='net/virtio15/rx'
vfs: mount match prefix='/' rel='dev/net/virtio15/rx'
[41516016366] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio15/rx'
[41519077314] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio15/rx' ENOENT
[41522606070] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio15/rx'
[41534546361] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio15/rx' ENOENT
vfs: mount::lookup path='/drivers/virtio_netd'
vfs: mount match prefix='/' rel='drivers/virtio_netd'
[41630276127] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/virtio_netd'
[41635939488] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/virtio_netd' OK
vfs: mount::lookup path='/drivers/virtio_sound'
vfs: mount::lookup path='/dev/net/virtio0/rx'
vfs: mount match prefix='/' rel='drivers/virtio_sound'
[41911692900] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/virtio_sound'
vfs: mount match prefix='/dev' rel='net/virtio0/rx'
vfs: mount match prefix='/' rel='dev/net/virtio0/rx'
[41918314218] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/virtio_sound' OK
[41921452056] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio0/rx'
[41925068625] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio0/rx' ENOENT
[41928751887] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio0/rx'
[41940949149] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio0/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio1/rx'
vfs: mount match prefix='/dev' rel='net/virtio1/rx'
vfs: mount match prefix='/' rel='dev/net/virtio1/rx'
[41954701470] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio1/rx'
[41958473502] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio1/rx' ENOENT
[41962426407] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio1/rx'
[41975150118] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio1/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio2/rx'
vfs: mount match prefix='/dev' rel='net/virtio2/rx'
vfs: mount match prefix='/' rel='dev/net/virtio2/rx'
[41985656427] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio2/rx'
[41988932304] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio2/rx' ENOENT
[41992262433] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio2/rx'
[41998944240] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio2/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio3/rx'
vfs: mount match prefix='/dev' rel='net/virtio3/rx'
vfs: mount match prefix='/' rel='dev/net/virtio3/rx'
[42009132429] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio3/rx'
[42013685307] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio3/rx' ENOENT
[42017396289] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio3/rx'
[42029653248] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio3/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio4/rx'
vfs: mount match prefix='/dev' rel='net/virtio4/rx'
vfs: mount match prefix='/' rel='dev/net/virtio4/rx'
[42042930270] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio4/rx'
[42046690818] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio4/rx' ENOENT
[42050157039] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio4/rx'
[42059487129] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio4/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio5/rx'
vfs: mount match prefix='/dev' rel='net/virtio5/rx'
vfs: mount match prefix='/' rel='dev/net/virtio5/rx'
[42070355052] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio5/rx'
[42073922814] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio5/rx' ENOENT
[42077555949] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio5/rx'
[42088282995] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio5/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio6/rx'
vfs: mount match prefix='/dev' rel='net/virtio6/rx'
vfs: mount match prefix='/' rel='dev/net/virtio6/rx'
[42099921897] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio6/rx'
[42103535562] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio6/rx' ENOENT
[42106885920] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio6/rx'
[42117576699] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio6/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio7/rx'
vfs: mount match prefix='/dev' rel='net/virtio7/rx'
vfs: mount match prefix='/' rel='dev/net/virtio7/rx'
[42129115545] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio7/rx'
[42133041258] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio7/rx' ENOENT
[42137051319] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio7/rx'
[42144149058] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio7/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio8/rx'
vfs: mount match prefix='/dev' rel='net/virtio8/rx'
vfs: mount match prefix='/' rel='dev/net/virtio8/rx'
[42154996323] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio8/rx'
[42158271177] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio8/rx' ENOENT
[42161521116] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio8/rx'
[42172994457] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio8/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio9/rx'
vfs: mount match prefix='/dev' rel='net/virtio9/rx'
vfs: mount match prefix='/' rel='dev/net/virtio9/rx'
[42183167400] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio9/rx'
[42186768558] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio9/rx' ENOENT
[42193530588] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio9/rx'
[42204015546] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio9/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio10/rx'
vfs: mount match prefix='/dev' rel='net/virtio10/rx'
vfs: mount match prefix='/' rel='dev/net/virtio10/rx'
vfs: mount::lookup path='/sys/devices'
[42217448592] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio10/rx'
vfs: mount match prefix='/sys' rel='devices'
[42222623949] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio10/rx' ENOENT
vfs: mount match prefix='/' rel='sys/devices'
[42226822473] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio10/rx'
[42229660836] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices'
[42236200545] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio10/rx' ENOENT
[42239615451] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices' OK
vfs: mount::lookup path='/dev/net/virtio11/rx'
vfs: mount match prefix='/dev' rel='net/virtio11/rx'
vfs: mount match prefix='/' rel='dev/net/virtio11/rx'
[42248179347] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio11/rx'
[42251708334] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio11/rx' ENOENT
[42256241280] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio11/rx'
[42264761319] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio11/rx' ENOENT
vfs: mount::lookup path='/sys/devices/pci-0000:00:00.0/vendor'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:00.0/vendor'
vfs: mount::lookup path='/dev/net/virtio12/rx'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:00.0/vendor'
vfs: mount match prefix='/dev' rel='net/virtio12/rx'
[42276532353] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:00.0/vendor'
vfs: mount match prefix='/' rel='dev/net/virtio12/rx'
[42282004875] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio12/rx'
[42286528548] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:00.0/vendor' OK
[42289830363] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio12/rx' ENOENT
[42296084127] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio12/rx'
vfs: mount::lookup path='/sys/devices/pci-0000:00:00.0/device'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:00.0/device'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:00.0/device'
[42305291094] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:00.0/device'
[42311418072] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:00.0/device' OK
[42317482152] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio12/rx' ENOENT
vfs: mount::lookup path='/sys/devices/pci-0000:00:00.0/class'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:00.0/class'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:00.0/class'
vfs: mount::lookup path='/dev/net/virtio13/rx'
[42328661133] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:00.0/class'
vfs: mount match prefix='/dev' rel='net/virtio13/rx'
vfs: mount match prefix='/' rel='dev/net/virtio13/rx'
[42334759467] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio13/rx'
[42337122927] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:00.0/class' OK
[42340771110] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio13/rx' ENOENT
[42343530471] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio13/rx'
vfs: mount::lookup path='/sys/devices/pci-0000:00:00.0/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:00.0/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:00.0/status'
[42352510629] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio13/rx' ENOENT
[42355784361] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:00.0/status'
vfs: mount::lookup path='/dev/net/virtio14/rx'
vfs: mount match prefix='/dev' rel='net/virtio14/rx'
[42362126070] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:00.0/status' OK
vfs: mount match prefix='/' rel='dev/net/virtio14/rx'
[42367604235] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio14/rx'
[42371168466] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio14/rx' ENOENT
vfs: mount::lookup path='/sys/devices/pci-0000:00:00.0/kind'
[42375674220] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio14/rx'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:00.0/kind'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:00.0/kind'
[42382540596] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:00.0/kind'
[42385935174] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio14/rx' ENOENT
[42390278139] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:00.0/kind' OK
vfs: mount::lookup path='/dev/net/virtio15/rx'
vfs: mount match prefix='/dev' rel='net/virtio15/rx'
vfs: mount match prefix='/' rel='dev/net/virtio15/rx'
[42398543682] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio15/rx'
[42401699736] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio15/rx' ENOENT
vfs: mount::lookup path='/sys/devices/pci-0000:00:01.0/vendor'
[42405260700] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio15/rx'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:01.0/vendor'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:01.0/vendor'
[42412928646] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:01.0/vendor'
[42416397210] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio15/rx' ENOENT
[42419092881] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:01.0/vendor' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:01.0/device'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:01.0/device'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:01.0/device'
[42433451346] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:01.0/device'
[42439043625] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:01.0/device' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:01.0/class'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:01.0/class'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:01.0/class'
[42452950749] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:01.0/class'
[42458697039] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:01.0/class' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:01.0/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:01.0/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:01.0/status'
[42476237364] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:01.0/status'
[42481733085] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:01.0/status' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:01.0/kind'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:01.0/kind'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:01.0/kind'
[42494686839] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:01.0/kind'
[42499616346] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:01.0/kind' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/vendor'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/vendor'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/vendor'
[42512422194] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/vendor'
[42517668996] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/vendor' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/device'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/device'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/device'
[42529745016] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/device'
[42534981720] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/device' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/class'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/class'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/class'
[42547741962] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/class'
[42552581049] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/class' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/status'
[42564761910] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/status'
[42570369765] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/status' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/kind'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/kind'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/kind'
[42582572175] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/kind'
[42587387667] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/kind' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.0/vendor'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.0/vendor'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.0/vendor'
[42599438673] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.0/vendor'
[42604362735] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.0/vendor' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.0/device'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.0/device'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.0/device'
[42623069478] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.0/device'
[42629168241] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.0/device' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.0/class'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.0/class'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.0/class'
[42642607392] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.0/class'
[42648156540] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.0/class' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.0/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.0/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.0/status'
[42661289550] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.0/status'
[42666725145] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.0/status' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.0/kind'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.0/kind'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.0/kind'
[42680290686] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.0/kind'
[42685601706] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.0/kind' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.2/vendor'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.2/vendor'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.2/vendor'
[42699277038] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.2/vendor'
[42705077976] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.2/vendor' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.2/device'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.2/device'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.2/device'
[42718974342] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.2/device'
[42725158245] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.2/device' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.2/class'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.2/class'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.2/class'
[42738967227] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.2/class'
[42744295242] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.2/class' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.2/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.2/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.2/status'
[42756897282] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.2/status'
[42762924699] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.2/status' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.2/kind'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.2/kind'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.2/kind'
[42775754241] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.2/kind'
[42781638207] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.2/kind' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.3/vendor'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.3/vendor'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.3/vendor'
[42795187644] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.3/vendor'
[42801446721] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.3/vendor' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.3/device'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.3/device'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.3/device'
[42814303290] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.3/device'
[42820560090] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.3/device' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.3/class'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.3/class'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.3/class'
[42829697130] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.3/class'
[42836004849] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.3/class' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.3/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.3/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.3/status'
[42848222967] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.3/status'
[42853091820] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.3/status' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.3/kind'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.3/kind'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.3/kind'
[42864562884] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.3/kind'
[42868972905] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.3/kind' OK
vfs: mount::lookup path='/services/storage'
vfs: mount::lookup path='/dev/net/virtio0/rx'
vfs: mount::lookup path='/sys/devices/isa-0070/vendor'
vfs: mount match prefix='/services' rel='storage'
vfs: mount match prefix='/dev' rel='net/virtio0/rx'
vfs: mount match prefix='/' rel='services/storage'
vfs: mount match prefix='/sys' rel='devices/isa-0070/vendor'
[42905177799] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='storage'
vfs: mount match prefix='/' rel='dev/net/virtio0/rx'
[42908783973] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='storage' ENOENT
vfs: mount match prefix='/' rel='sys/devices/isa-0070/vendor'
[42913429350] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio0/rx'
[42916695789] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='services/storage'
[42920352651] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0070/vendor'
[42923611830] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio0/rx' ENOENT
[42927090525] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio0/rx'
[42932132958] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='services/storage' ENOENT
[42935421375] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0070/vendor' OK
[42938754672] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio0/rx' ENOENT
vfs: mount::lookup path='/sys/devices/isa-0070/device'
vfs: mount::lookup path='/dev/net/virtio1/rx'
vfs: mount match prefix='/sys' rel='devices/isa-0070/device'
vfs: mount match prefix='/dev' rel='net/virtio1/rx'
vfs: mount match prefix='/' rel='sys/devices/isa-0070/device'
vfs: mount match prefix='/' rel='dev/net/virtio1/rx'
[42951234711] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0070/device'
[42954220980] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio1/rx'
[42957053964] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0070/device' OK
[42959713302] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio1/rx' ENOENT
vfs: mount::lookup path='/sys/devices/isa-0070/class'
[42963972249] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio1/rx'
vfs: mount match prefix='/sys' rel='devices/isa-0070/class'
vfs: mount match prefix='/' rel='sys/devices/isa-0070/class'
[42970326861] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0070/class'
[42975185583] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0070/class' OK
[42978177264] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio1/rx' ENOENT
vfs: mount::lookup path='/sys/devices/isa-0070/status'
vfs: mount::lookup path='/dev/net/virtio2/rx'
vfs: mount match prefix='/sys' rel='devices/isa-0070/status'
vfs: mount match prefix='/dev' rel='net/virtio2/rx'
vfs: mount match prefix='/' rel='sys/devices/isa-0070/status'
vfs: mount match prefix='/' rel='dev/net/virtio2/rx'
[42993194409] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0070/status'
[42996758013] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio2/rx'
[42999811668] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0070/status' OK
[43002891921] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio2/rx' ENOENT
[43006074309] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio2/rx'
vfs: mount::lookup path='/sys/devices/isa-0070/kind'
vfs: mount match prefix='/sys' rel='devices/isa-0070/kind'
vfs: mount match prefix='/' rel='sys/devices/isa-0070/kind'
[43014324573] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0070/kind'
[43021555071] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio2/rx' ENOENT
[43024895298] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0070/kind' OK
vfs: mount::lookup path='/dev/net/virtio3/rx'
vfs: mount match prefix='/dev' rel='net/virtio3/rx'
vfs: mount match prefix='/' rel='dev/net/virtio3/rx'
vfs: mount::lookup path='/sys/devices/isa-0060/vendor'
[43033725999] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio3/rx'
vfs: mount match prefix='/sys' rel='devices/isa-0060/vendor'
[43037701740] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio3/rx' ENOENT
vfs: mount match prefix='/' rel='sys/devices/isa-0060/vendor'
[43042393713] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio3/rx'
[43044637152] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0060/vendor'
[43050949293] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio3/rx' ENOENT
[43053942162] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0060/vendor' OK
vfs: mount::lookup path='/dev/net/virtio4/rx'
vfs: mount match prefix='/dev' rel='net/virtio4/rx'
vfs: mount match prefix='/' rel='dev/net/virtio4/rx'
vfs: mount::lookup path='/sys/devices/isa-0060/device'
[43062448869] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio4/rx'
vfs: mount match prefix='/sys' rel='devices/isa-0060/device'
[43065959838] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio4/rx' ENOENT
vfs: mount match prefix='/' rel='sys/devices/isa-0060/device'
[43071104373] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio4/rx'
[43075285308] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0060/device'
[43080518943] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0060/device' OK
[43085828907] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio4/rx' ENOENT
vfs: mount::lookup path='/sys/devices/isa-0060/class'
vfs: mount match prefix='/sys' rel='devices/isa-0060/class'
vfs: mount match prefix='/' rel='sys/devices/isa-0060/class'
vfs: mount::lookup path='/dev/net/virtio5/rx'
[43094702937] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0060/class'
vfs: mount match prefix='/dev' rel='net/virtio5/rx'
vfs: mount match prefix='/' rel='dev/net/virtio5/rx'
[43101337917] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0060/class' OK
[43104654582] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio5/rx'
[43107700911] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio5/rx' ENOENT
vfs: mount::lookup path='/sys/devices/isa-0060/status'
[43112165778] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio5/rx'
vfs: mount match prefix='/sys' rel='devices/isa-0060/status'
vfs: mount match prefix='/' rel='sys/devices/isa-0060/status'
[43119252990] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0060/status'
[43125597273] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0060/status' OK
[43128972975] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio5/rx' ENOENT
vfs: mount::lookup path='/sys/devices/isa-0060/kind'
vfs: mount::lookup path='/dev/net/virtio6/rx'
vfs: mount match prefix='/sys' rel='devices/isa-0060/kind'
vfs: mount match prefix='/dev' rel='net/virtio6/rx'
vfs: mount match prefix='/' rel='sys/devices/isa-0060/kind'
vfs: mount match prefix='/' rel='dev/net/virtio6/rx'
[43142440737] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/isa-0060/kind'
[43145825349] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio6/rx'
[43149628500] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/isa-0060/kind' OK
[43153148346] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio6/rx' ENOENT
[43156664760] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio6/rx'
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/status'
[43170384180] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio6/rx' ENOENT
[43173649233] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/status'
vfs: mount::lookup path='/dev/net/virtio7/rx'
vfs: mount match prefix='/dev' rel='net/virtio7/rx'
[43179784626] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/status' OK
vfs: mount match prefix='/' rel='dev/net/virtio7/rx'
[43184922462] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio7/rx'
[43188582624] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio7/rx' ENOENT
[43191920013] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio7/rx'
[43202087808] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio7/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio8/rx'
vfs: mount match prefix='/dev' rel='net/virtio8/rx'
vfs: mount match prefix='/' rel='dev/net/virtio8/rx'
vfs: mount::lookup path='/drivers/virtio_netd'
[43213952166] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio8/rx'
vfs: mount match prefix='/' rel='drivers/virtio_netd'
[43218624999] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio8/rx' ENOENT
[43221552297] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/virtio_netd'
[43224908199] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio8/rx'
[43228392537] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/virtio_netd' OK
[43234748436] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio8/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio9/rx'
vfs: mount match prefix='/dev' rel='net/virtio9/rx'
vfs: mount match prefix='/' rel='dev/net/virtio9/rx'
[43249132080] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio9/rx'
[43252509564] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio9/rx' ENOENT
[43255891536] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio9/rx'
[43267300098] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio9/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio10/rx'
vfs: mount match prefix='/dev' rel='net/virtio10/rx'
vfs: mount match prefix='/' rel='dev/net/virtio10/rx'
[43277548776] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio10/rx'
[43280935368] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio10/rx' ENOENT
[43284774258] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio10/rx'
[43294706697] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio10/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio11/rx'
vfs: mount match prefix='/dev' rel='net/virtio11/rx'
vfs: mount match prefix='/' rel='dev/net/virtio11/rx'
[43304904489] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio11/rx'
[43308534687] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio11/rx' ENOENT
[43311930123] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio11/rx'
[43321887939] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio11/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio12/rx'
vfs: mount match prefix='/dev' rel='net/virtio12/rx'
vfs: mount match prefix='/' rel='dev/net/virtio12/rx'
[43332794109] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio12/rx'
[43336167468] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio12/rx' ENOENT
[43339690119] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio12/rx'
[43347801453] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio12/rx' ENOENT
vfs: mount::lookup path='/sys/devices/pci-0000:00:1f.2/status'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:1f.2/status'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:1f.2/status'
[43361382108] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices/pci-0000:00:1f.2/status'
[43366635510] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices/pci-0000:00:1f.2/status' OK
vfs: mount::lookup path='/drivers/ahci_disk'
vfs: mount match prefix='/' rel='drivers/ahci_disk'
[43377856665] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='drivers/ahci_disk'
[43383022551] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='drivers/ahci_disk' OK
vfs: mount::lookup path='/proc/11/job_observer'
vfs: mount match prefix='/proc' rel='11/job_observer'
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/virtio/common_bar'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/virtio/common_bar'
vfs: mount match prefix='/' rel='proc/11/job_observer'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/virtio/common_bar'
[43458042177] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='11/job_observer'
[43461434181] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/virtio/common_bar'
[43474453506] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/virtio/common_bar' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/virtio/common_offset'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/virtio/common_offset'
[43508583954] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='11/job_observer' OK
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/virtio/common_offset'
[43514044596] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/virtio/common_offset'
[43521213516] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/virtio/common_offset' OK
vfs: mount::lookup path='/proc/12/job_observer'
vfs: mount match prefix='/proc' rel='12/job_observer'
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/virtio/notify_bar'
vfs: mount match prefix='/' rel='proc/12/job_observer'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/virtio/notify_bar'
[43536880398] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='12/job_observer'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/virtio/notify_bar'
[43542471489] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/virtio/notify_bar'
[43549485639] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='12/job_observer' OK
[43551720828] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/virtio/notify_bar' OK
vfs: mount::lookup path='/proc/self/inbox'
vfs: mount match prefix='/proc' rel='self/inbox'
vfs: mount match prefix='/' rel='proc/self/inbox'
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/virtio/notify_offset'
[43564138365] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='self/inbox'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/virtio/notify_offset'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/virtio/notify_offset'
[43571817036] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='self/inbox' OK
[43574922369] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/virtio/notify_offset'
vfs: mount::lookup path='/sys/devices'
vfs: mount match prefix='/sys' rel='devices'
vfs: mount match prefix='/' rel='sys/devices'
[43583406999] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='devices'
[43586532363] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/virtio/notify_offset' OK
[43590394056] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='devices' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/virtio/notify_multiplier'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/virtio/notify_multiplier'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/virtio/notify_multiplier'
[43602700119] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/virtio/notify_multiplier'
[43609554582] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/virtio/notify_multiplier' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/virtio/device_bar'
vfs: mount::lookup path='/services'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/virtio/device_bar'
vfs: mount match prefix='/services' rel=''
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/virtio/device_bar'
vfs: mount match prefix='/' rel='services'
[43631674383] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/virtio/device_bar'
[43635561618] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel=''
[43639204257] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='' OK
[43642181517] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/virtio/device_bar' OK
vfs: mount::lookup path='/sys/devices/pci-0000:00:02.0/virtio/device_offset'
vfs: mount match prefix='/sys' rel='devices/pci-0000:00:02.0/virtio/device_offset'
vfs: mount match prefix='/' rel='sys/devices/pci-0000:00:02.0/virtio/device_offset'
[43659248127] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='devices/pci-0000:00:02.0/virtio/device_offset'
[43666600923] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='devices/pci-0000:00:02.0/virtio/device_offset' OK
vfs: mount::lookup path='/services/storage/atapi2'
vfs: mount match prefix='/services' rel='storage/atapi2'
vfs: mount match prefix='/' rel='services/storage/atapi2'
[43675100634] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='storage/atapi2'
[43678877484] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='storage/atapi2' ENOENT
[43682505768] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='services/storage/atapi2'
[43694591853] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='services/storage/atapi2' ENOENT
vfs: mount::lookup path='/dev/net/virtio13/rx'
vfs: mount match prefix='/dev' rel='net/virtio13/rx'
vfs: mount match prefix='/' rel='dev/net/virtio13/rx'
[43728912777] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio13/rx'
[43732566834] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio13/rx' ENOENT
[43735895841] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio13/rx'
[43746981927] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio13/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio14/rx'
vfs: mount match prefix='/dev' rel='net/virtio14/rx'
vfs: mount match prefix='/' rel='dev/net/virtio14/rx'
[43759413819] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio14/rx'
[43763052102] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio14/rx' ENOENT
[43766601153] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio14/rx'
[43777696182] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio14/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio15/rx'
vfs: mount match prefix='/dev' rel='net/virtio15/rx'
vfs: mount match prefix='/' rel='dev/net/virtio15/rx'
[43789535526] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio15/rx'
[43793356926] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio15/rx' ENOENT
[43797130707] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio15/rx'
[43808587152] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio15/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio0/rx'
vfs: mount match prefix='/dev/net/virtio0' rel='rx'
vfs: mount match prefix='/dev' rel='net/virtio0/rx'
vfs: mount match prefix='/' rel='dev/net/virtio0/rx'
[44161165587] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='rx'
[44245560612] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='rx' OK
vfs: mount::lookup path='/dev/net/virtio0/tx'
vfs: mount match prefix='/dev/net/virtio0' rel='tx'
vfs: mount match prefix='/dev' rel='net/virtio0/tx'
vfs: mount match prefix='/' rel='dev/net/virtio0/tx'
[44258656530] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='tx'
[44299663485] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='tx' OK
vfs: mount::lookup path='/dev/net/virtio0/events'
vfs: mount match prefix='/dev/net/virtio0' rel='events'
vfs: mount match prefix='/dev' rel='net/virtio0/events'
vfs: mount match prefix='/' rel='dev/net/virtio0/events'
[44310776268] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='events'
[44335837920] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='events' OK
vfs: mount::lookup path='/dev/net/virtio0/mac'
vfs: mount match prefix='/dev/net/virtio0' rel='mac'
vfs: mount match prefix='/dev' rel='net/virtio0/mac'
vfs: mount match prefix='/' rel='dev/net/virtio0/mac'
[44344768512] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='mac'
[44369891181] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='mac' OK
vfs: mount::lookup path='/dev/net/virtio0/mtu'
vfs: mount match prefix='/dev/net/virtio0' rel='mtu'
vfs: mount match prefix='/dev' rel='net/virtio0/mtu'
vfs: mount match prefix='/' rel='dev/net/virtio0/mtu'
[44481451971] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='mtu'
[44512436430] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='mtu' OK
vfs: mount::lookup path='/dev/net/virtio0/status'
vfs: mount match prefix='/dev/net/virtio0' rel='status'
vfs: mount match prefix='/dev' rel='net/virtio0/status'
vfs: mount match prefix='/' rel='dev/net/virtio0/status'
[44624614056] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='status'
[44660071764] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='status' OK
vfs: mount::lookup path='/services/storage'
vfs: mount match prefix='/services' rel='storage'
vfs: mount match prefix='/' rel='services/storage'
[44678406333] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='storage'
[44681828202] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='storage' OK
vfs: mount::lookup path='/services/storage/atapi2'
vfs: mount match prefix='/services' rel='storage/atapi2'
vfs: mount match prefix='/' rel='services/storage/atapi2'
[44694776280] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='storage/atapi2'
[44697229896] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='storage/atapi2' OK
pin[45559828314] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
vfs: mount::lookup path='/dev/net/virtio0/rx'
vfs: mount match prefix='/dev/net/virtio0' rel='rx'
vfs: mount match prefix='/dev' rel='net/virtio0/rx'
vfs: mount match prefix='/' rel='dev/net/virtio0/rx'
[45575070750] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='rx'
[45602975154] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='rx' OK
vfs: mount::lookup path='/dev/net/virtio1/rx'
vfs: mount match prefix='/dev' rel='net/virtio1/rx'
vfs: mount match prefix='/' rel='dev/net/virtio1/rx'
[45637551762] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio1/rx'
[45641076294] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio1/rx' ENOENT
[45644684052] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio1/rx'
[45653982891] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio1/rx' ENOENT
gvfs: mount::lookup path='/dev/net/virtio2/rx'
vfs: mount match prefix='/dev' rel='net/virtio2/rx'
vfs: mount match prefix='/' rel='dev/net/virtio2/rx'
[45664970241] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio2/rx'
[45668764548] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio2/rx' ENOENT
[45672346566] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio2/rx'
[45684235938] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio2/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio3/rx'
vfs: mount match prefix='/dev' rel='net/virtio3/rx'
vfs: mount match prefix='/' rel='dev/net/virtio3/rx'
[45695723964] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio3/rx'
[45699584271] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio3/rx' ENOENT
[45703828038] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio3/rx'
[45716175648] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio3/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio4/rx'
vfs: mount match prefix='/dev' rel='net/virtio4/rx'
vfs: mount match prefix='/' rel='dev/net/virtio4/rx'
[45728474781] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio4/rx'
[45732149925] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio4/rx' ENOENT
[45736557240] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio4/rx'
[45748882872] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio4/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio5/rx'
vfs: mount match prefix='/dev' rel='net/virtio5/rx'
vfs: mount match prefix='/' rel='dev/net/virtio5/rx'
[45761370237] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio5/rx'
[45765067029] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio5/rx' ENOENT
[45768825465] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio5/rx'
[45781520202] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio5/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio6/rx'
vfs: mount match prefix='/dev' rel='net/virtio6/rx'
vfs: mount match prefix='/' rel='dev/net/virtio6/rx'
[45793729905] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio6/rx'
[45797509362] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio6/rx' ENOENT
[45801068346] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio6/rx'
[45815770836] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio6/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio7/rx'
vfs: mount match prefix='/dev' rel='net/virtio7/rx'
vfs: mount match prefix='/' rel='dev/net/virtio7/rx'
 [45830282256] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio7/rx'
[45833315286] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio7/rx' ENOENT
[45838718343] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio7/rx'
[45850553133] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio7/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio8/rx'
vfs: mount match prefix='/dev' rel='net/virtio8/rx'
vfs: mount match prefix='/' rel='dev/net/virtio8/rx'
[45862059375] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio8/rx'
[45865992447] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio8/rx' ENOENT
[45868958520] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio8/rx'
[45876261057] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio8/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio9/rx'
vfs: mount match prefix='/dev' rel='net/virtio9/rx'
vfs: mount match prefix='/' rel='dev/net/virtio9/rx'
[45886080339] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio9/rx'
[45890551608] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio9/rx' ENOENT
[45894019215] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio9/rx'
[45900067422] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio9/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio10/rx'
vfs: mount match prefix='/dev' rel='net/virtio10/rx'
vfs: mount match prefix='/' rel='dev/net/virtio10/rx'
[45908070945] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio10/rx'
[45911915445] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio10/rx' ENOENT
[45915361173] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio10/rx'
[45925121847] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio10/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio11/rx'
vfs: mount match prefix='/dev' rel='net/virtio11/rx'
vfs: mount match prefix='/' rel='dev/net/virtio11/rx'
[45932894964] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio11/rx'
[45935904036] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio11/rx' ENOENT
[45938259444] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio11/rx'
[45948454431] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio11/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio12/rx'
vfs: mount match prefix='/dev' rel='net/virtio12/rx'
vfs: mount match prefix='/' rel='dev/net/virtio12/rx'
[45956786271] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio12/rx'
[45959193357] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio12/rx' ENOENT
[45961571931] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio12/rx'
[45968615517] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio12/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio13/rx'
vfs: mount match prefix='/dev' rel='net/virtio13/rx'
vfs: mount match prefix='/' rel='dev/net/virtio13/rx'
[45979884126] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio13/rx'
[45983587353] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio13/rx' ENOENT
[45987711198] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio13/rx'
-[46000013796] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio13/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio14/rx'
vfs: mount match prefix='/dev' rel='net/virtio14/rx'
vfs: mount match prefix='/' rel='dev/net/virtio14/rx'
[46014285405] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio14/rx'
[46016885013] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio14/rx' ENOENT
[46020635364] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio14/rx'
[46029906582] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio14/rx' ENOENT
vfs: mount::lookup path='/dev/net/virtio15/rx'
vfs: mount match prefix='/dev' rel='net/virtio15/rx'
vfs: mount match prefix='/' rel='dev/net/virtio15/rx'
[46041131202] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net/virtio15/rx'
[46044711768] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net/virtio15/rx' ENOENT
[46048266462] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='dev/net/virtio15/rx'
[46058815242] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='dev/net/virtio15/rx' ENOENT
vfs: mount::lookup path='/dev/net'
vfs: mount match prefix='/dev' rel='net'
vfs: mount match prefix='/' rel='dev/net'
[46074832452] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount calling driver.lookup rel='net'
[46078907754] [[32mINFO [0m] [kernel::vfs::mount] [CPU2] vfs: mount driver.lookup rel='net' OK
c vfs: mount::lookup path='/services/storage'
vfs: mount match prefix='/services' rel='storage'
vfs: mount match prefix='/' rel='services/storage'
[46402383852] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='storage'
[46406105064] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='storage' OK
vfs: mount::lookup path='/services/storage/atapi2'
vfs: mount match prefix='/services' rel='storage/atapi2'
vfs: mount match prefix='/' rel='services/storage/atapi2'
[46421434554] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='storage/atapi2'
[46425138408] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='storage/atapi2' OK
1 example.com
[48712300131] [[32mINFO [0m] [sh] [CPU3] sh: spawning job cmd='ping'
vfs: mount::lookup path='/bin/ping'
vfs: mount match prefix='/' rel='bin/ping'
[48720124728] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='bin/ping'
[48726199764] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='bin/ping' OK
vfs: mount::lookup path='/bin/ping'
vfs: mount match prefix='/' rel='bin/ping'
[48748885779] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount calling driver.lookup rel='bin/ping'
[48754620981] [[32mINFO [0m] [kernel::vfs::mount] [CPU3] vfs: mount driver.lookup rel='bin/ping' OK
[?25lvfs: mount::lookup path='/net/dns/lookup'
vfs: mount match prefix='/net' rel='dns/lookup'
vfs: mount match prefix='/' rel='net/dns/lookup'
[48869214900] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='dns/lookup'
[48990564513] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='dns/lookup' OK
vfs: mount::lookup path='/net/dns/lookup'
vfs: mount match prefix='/net' rel='dns/lookup'
vfs: mount match prefix='/' rel='net/dns/lookup'
[49149014409] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='dns/lookup'
[50529342765] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='dns/lookup' OK
vfs: mount::lookup path='/net/icmp/new'
vfs: mount match prefix='/net' rel='icmp/new'
vfs: mount match prefix='/' rel='net/icmp/new'
[50821979010] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='icmp/new'
[50928494826] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='icmp/new' OK
vfs: mount::lookup path='/net/icmp/1/ctl'
vfs: mount match prefix='/net' rel='icmp/1/ctl'
vfs: mount match prefix='/' rel='net/icmp/1/ctl'
[51190612341] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='icmp/1/ctl'
[51310257306] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='icmp/1/ctl' OK
PING example.com (104.20.23.154) 56 bytes of data
vfs: mount::lookup path='/net/icmp/1/data'
vfs: mount match prefix='/net' rel='icmp/1/data'
vfs: mount match prefix='/' rel='net/icmp/1/data'
[51491294910] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='icmp/1/data'
[51590625339] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount driver.lookup rel='icmp/1/data' OK
vfs: mount::lookup path='/net/icmp/1/data'
vfs: mount match prefix='/net' rel='icmp/1/data'
vfs: mount match prefix='/' rel='net/icmp/1/data'
[51752538189] [[32mINFO [0m] [kernel::vfs::mount] [CPU1] vfs: mount calling driver.lookup rel='icmp/1/data'
[52189874682
```
</details>
