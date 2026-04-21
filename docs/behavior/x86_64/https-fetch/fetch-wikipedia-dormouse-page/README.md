# ✅ Scenario: Fetch Wikipedia Dormouse page

> Last run: 2026-04-21 05:17:38

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 11160ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2204ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 en.wikipedia.org" on the serial console | ✅ | 2389ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 611ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3168ms | - [📜](./05/serial.log) - |
| 6 | Then the serial output should contain "Dormouse" | ✅ | 0ms | - [📜](./06/serial.log) - |
| 7 | And the serial output should contain "Gliridae" | ✅ | 4622ms | - [📜](./07/serial.log) - |
| 8 | And the serial output should contain "nocturnal" | ✅ | 2881ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[kernel:mem:init] enter
[kernel:mem:init] phys_memory_map ok
[kernel:mem:init] modules ok
[kernel:mem:init] phys_to_virt_offset ok
[kernel:mem:init] memory map logging done
[kernel:mem:init] boot_frame_alloc init ok
[kernel:mem:init] frame allocator build ok
[kernel:mem:init] frame allocator log ok
[kernel:mem:init] FRAME_ALLOCATOR init ok
[kernel:mem:init] tasking init ok
[32770782033] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[kernel:global_alloc] enter
[kernel:global_alloc] set expand hook
[kernel:global_alloc] expand hook ok
[kernel:global_alloc] kernel_heap lock begin
[kernel:global_alloc] kernel_heap lock ok
[kernel:global_alloc] reserve_region begin
[kernel:global_alloc] reserve_region ok
[kernel:global_alloc] inner allocator init begin
[kernel:global_alloc] inner allocator init ok
[kernel:global_alloc] heap top store ok
[kernel:global_alloc] init done
[kernel:devfs] set_boot_fb begin
[kernel:devfs] set_boot_fb ok
[kernel:devfs] register begin
[kernel:devfs] register ok
[kernel:entropy] seed begin
[kernel:entropy] fill_entropy done
[kernel:entropy] add_sample(timer) ok
[kernel:entropy] mark_seeded(timer) ok
[kernel:entropy] seed done
[35612808033] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[35618134299] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[35721272565] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[36455320143] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[36588359610] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
[1;32m
        .-.
       /   \        [1;36mTHING-OS[1;32m
      |     |       [0;36m"People, places, things."[1;32m
       \   /        
        `-'        
       /   \        v0.1  ���  
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[39138295713] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[39261795936] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[39746135253] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet — retrying
ping -c 1 en.wikipedia.org
[?25l[48460574019] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
PING en.wikipedia.org (198.35.26.224) 56 bytes of data
64 bytes from 198.35.26.224: icmp_seq=1 time=664ms
VFS RPC: op=Stat returned error 2

--- en.wikipedia.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 664/664/664 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/en.wikipedia.org/wiki/Dormouse
[60926900562] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[60936323349] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[60964747965] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[60967086609] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[60971551740] [[32mINFO [0m] [http] [CPU1] http: connect host=en.wikipedia.org port=443
[60979924137] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[?25l[61426126377] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[61447910865] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[61562700243] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[61677546282] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect en.wikipedia.org 443
[62988046638] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[63000604392] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=5)...
[63160053594] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[63169772490] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to en.wikipedia.org
[63183135477] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with en.wikipedia.org
[63879433662] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=5)...
[64726115718] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[67753172388] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for en.wikipedia.org
[67759728465] [[32mINFO [0m] [http] [CPU1] http: background task: sending 144 byte request
[67763513631] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 144 bytes (total=144)
[67767258042] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[69820089042] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[73519870512] [[32mINFO [0m] [http] [CPU1] http: background task: read 14724 bytes from TLS
[73548604074] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 14724 bytes to foreground
[73574646354] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http header read revents=0x0001
[73586505597] [[32mINFO [0m] [http] [CPU3] http: received 4096 bytes from background TLS thread
[73610740500] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=1/120)
[73626663627] [[32mINFO [0m] [http] [CPU3] http: waiting for http header read readiness (fd=3)...
[73656423324] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http header read revents=0x0001
[73676233257] [[32mINFO [0m] [http] [CPU3] http: received 4096 bytes from background TLS thread
[73688157048] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=4636
<!DOCTYPE html>
<html class="client-nojs vector-feature-language-in-header-enabled vector-feature-language-in-main-menu-disabled vector-feature-language-in-main-page-header-disabled vector-feature-page-tools-pinned-disabled vector-feature-toc-pinned-clientpref-1 vector-feature-main-menu-pinned-disabled vector-feature-limited-width-clientpref-1 vector-feature-limited-width-content-enabled vector-feature-custom-font-size-clientpref-1 vector-feature-appearance-pinned-clientpref-1 skin-theme-clientpref-day vector-sticky-header-enabled vector-toc-available skin-theme-clientpref-thumb-standard" lang="en" dir="ltr">
<head>
<meta charset="UTF-8">
<title>Dormouse - Wikipedia</title>
<script>(function(){var className="client-js vector-feature-language-in-header-enabled vector-feature-language-in-main-menu-disabled vector-feature-language-in-main-page-header-disabled vector-feature-page-tools-pinned-disabled vector-feature-toc-pinned-clientpref-1 vector-feature-main-menu-pinned-disabled vector-feature-limited-width-clientpref-1 vector-feature-limited-width-content-enabled vector-feature-custom-font-size-clientpref-1 vector-feature-appearance-pinned-clientpref-1 skin-theme-clientpref-day vector-sticky-header-enabled vector-toc-available skin-theme-clientpref-thumb-standard";var cookie=document.cookie.match(/(?:^|; )enwikimwclientpreferences=([^;]+)/);if(cookie){cookie[1].split('%2C').forEach(function(pref){className=className.replace(new RegExp('(^| )'+pref.replace(/-clientpref-\w+$|[^\w-]+/g,'')+'-clientpref-\\w+( |$)'),'$1'+pref+'$2');});}document.documentElement.className=className;}());RLCONF={"wgBreakFrames":false,"wgSeparatorTransformTable":["",""],"wgDigitTransformTable":["",""],"wgDefaultDateFormat":"dmy","wgMonthNames":["","January","February","March","April","May","June","July","August","September","October","November","December"],"wgRequestId":"42973d49-322b-406f-8a3f-7712b0c3ce81","wgCanonicalNamespace":"","wgCanonicalSpecialPageName":false,"wgNamespaceNumber":0,"wgPageName":"Dormouse","wgTitle":"Dormouse","wgCurRevisionId":1348410573,"wgRevisionId":1348410573,"wgArticleId":438703,"wgIsArticle":true,"wgIsRedirect":false,"wgAction":"view","wgUserName":null,"wgUserGroups":["*"],"wgCategories":["Articles with short description","Short description is different from Wikidata","Articles with 'species' microformats","Articles containing Middle English (1100-1500)-language text","Articles containing Old Norse-language text","Articles containing Anglo-Norman-language text","Articles containing Latin-language text","Articles containing Sanskrit-language text","Articles containing Ancient Greek (to 1453)-language text","Commons link from Wikidata","Articles with German-language sources (de)","Dormice","Sciuromorpha","Natural Monuments of Japan","Extant Eocene first appearances"],"wgPageViewLanguage":"en","wgPageContentLanguage":"en","wgPageContentModel":"wikitext","wgRelevantPageName":"Dormouse","wgRelevantArticleId":438703,"wgTempUserName":null,"wgIsProbablyEditable":true,"wgRelevantPageIsProbablyEditable":true,"wgRestrictionEdit":[],"wgRestrictionMove":[],"wgNoticeProject":"wikipedia","wgFlaggedRevsParams":{"tags":{"status":{"levels":1}}},"wgConfirmEditCaptchaNeededForGenericEdit":"hcaptcha","wgConfirmEditHCaptchaVisualEditorOnLoadIntegrationEnabled":false,"wgConfirmEditHCaptchaSiteKey":"5d0c670e-a5f4-4258-ad16-1f42792c9c62","wgMediaViewerOnClick":true,"wgMediaViewerEnabledByDefault":true,"wgPopupsFlags":0,"wgVisualEditor":{"pageLanguageCode":"en","pageLanguageDir":"ltr","pageVariantFallbacks":"en"},[73878380532] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=3556 len=32768 cached=3556 start=0 eof=false
[73890964620] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[73898021703] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read_chunk revents=0x0001
"wgMFDisplayWikibaseDescriptions":{"search":true,"watchlist":true,"tagline":false,"nearby":true},"wgWMESchemaEditAttemptStepOversample":false,"wgWMEPageLength":20000,"wgEditSubmitButtonLabelPublish":true,"wgVisualEditorPageIsDisambiguation":false,"wgULSPosition":"interlanguage","wgULSisCompactLinksEnabled":false,"wgVector2022LanguageInHeader":true,"wgULSisLanguageSelectorEmpty":false,"wgWikibaseItemId":"Q108235","wgCheckUserClientHintsHeadersJsApi":["brands","architecture","bitness","fullVersionList","mobile","model","platform","platformVersion"],"GEHomepageSuggestedEditsEnableTopics":true,"wgGESuggestedEditsTaskTypes":{"taskTypes":["copyedit","link-recommendation"],"unavailableTaskTypes":[]},"wgGETopicsMatchModeEnabled":false,"wgGELevelingUpEnabledForUser":false,"wgTestKitchenUserExperiments":{"overrides":[],"enrolled":[],"assigned":[],"subject_ids":[]}};
RLSTATE={"ext.globalCssJs.user.styles":"ready","site.styles":"ready","user.styles":"ready","ext.globalCssJs.user":"ready","user":"ready","user.options":"loading","ext.wikimediamessages.styles":"ready","ext.cite.styles":"ready","skins.vector.search.codex.styles":"ready","skins.vector.styles":"ready","skins.vector.icons":"ready","jquery.makeCollapsible.styles":"ready","ext.visualEditor.desktopArticleTarget.noscript":"ready","ext.uls.interlanguage":"ready","wikibase.client.init":"ready","ext.wikimediaBadges":"ready"};RLPAGEMODULES=["ext.parsermigration.survey","ext.cite.ux-enhancements","mediawiki.page.media","site","mediawiki.page.ready","jquery.makeCollapsible","mediawiki.toc","skins.vector.js","ext.centralNotice.geoIP","ext.centralNotice.startUp","ext.gadget.ReferenceTooltips","ext.gadget.switcher","ext.urlShortener.toolbar","ext.centralauth.centralautologin","mmv.bootstrap","ext.popups","ext.visualEditor.desktopArticleTarget.init","ext.echo.centralauth","ext.eventLogging","ext.wikimediaEvents","ext.navigationTiming","ext.uls.interface","ext.cx.eventlogging.campaigns","ext.cx.uls.quick.actions","wikibase.client.vector-2022","wikibase.databox.fromWikidata","ext.checkUser.clientHints","ext.quicksurveys.init","ext.growthExperiments.SuggestedEditSession","ext.testKitchen"];</script>
<script>(RLQ=window.RLQ||[]).push(function(){mw.loader.impl(function(){return["user.options@12s5i",function($,jQuery,require,module){mw.user.tokens.set({"patrolToken":"+\\","watchToken":"+\\","csrfToken":"+\\"});
}];});});</script>
<link rel="stylesheet" href="/w/load.php?lang=en&amp;modules=ext.cite.styles%7Cext.uls.interlanguage%7Cext.visualEditor.desktopArticleTarget.noscript%7Cext.wikimediaBadges%7Cext.wikimediamessages.styles%7Cjquery.makeCollapsible.styles%7Cskins.vector.icons%2Cstyles%7Cskins.vector.search.codex.styles%7Cwikibase.client.init&amp;only=styles&amp;skin=vector-2022">
<script async="" src="/w/load.php?lang=en&amp;modules=startup&amp;only=scripts&amp;raw=1&amp;skin=vector-2022"></script>
<meta name="ResourceLoaderDynamicStyles" content="">
<link rel="stylesheet" href="/w/load.php?lang=en&amp;modules=site.styles&amp;only=styles&amp;skin=vector-2022">
<meta name="generator" content="MediaWiki 1.46.0-wmf.24">
<meta name="referrer" content="origin">
<meta name="referrer" content="origin-when-cross-origin">
<meta name="robots" content="max-image-preview:standard">
<meta name="format-detection" content="telephone=no">
<meta property="og:image" content="https://upload.wikimedia.org/wikipedia/commons/4/43/Graphiurus_spec_-murinus-1.jpg">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="800">
<meta name="viewport" content="width=1120">
<meta property="og:title" content="Dormouse - Wikipedia">
<meta property="og:type" content="website">
<link rel="preconnect" href="//upload.wikimedia.org">
<link rel="alternate" type="application/x-wiki" title="Edit this page" href="/w/index.php?title=Dormouse&amp;action=edit">
<link rel="apple-touch-icon" href="/static/apple-touch/wikipedia.png">
<link rel="icon" href="/static/favicon/wikipedia.ico">
<link rel="search" type="application/opensearchdescription+xml" href="/w/rest.php/v1/search" title="Wikipedia (en)">
<link rel="EditURI" type="application/rsd+xml" href="//en.wikipedia.org/w/api.php?action=rsd">
<link rel="canonical" href="https://en.wikipedia.org/wiki/Dormouse">
<link rel="license" href="https://creativecommons.org/licenses/by-sa/4.0/deed.en">
<link rel="alternate" type="application/atom+xml" title="Wikipedia Atom feed" href="/w/index.php?title=Special:RecentChanges&amp;feed=atom">
<link rel="dns-prefetch" href="//meta.wikimedia.org" />
<link rel="dns-prefetch" href="auth.wikimedia.org">
</head>
<body class="skin--responsive skin-vector skin-vector-search-vue mediawiki ltr sitedir-ltr mw-hide-empty-elt ns-0 ns-subject mw-editable page-Dormouse rootpage-Dormouse skin-vector-2022 action-view">
<div id="mw-aria-live-region" class="mw-aria-live-region" aria-live="polite"></div><a class="mw-jump-link" href="#bodyContent">Jump to content</a>
<div class="vector-header-container">
	<header class="vector-header mw-header no-font-mode-scale">
		<div class="vector-header-start">
			<nav class="vector-main-menu-landmark" aria-label="Site">
				
<div id="vector-main-menu-dropdown" class="vector-dropdown vector-main-menu-dropdown vector-button-flush-left vector-button-flush-right"  title="Main menu" >
	<input type="checkbox" id="vector-main-menu-dropdown-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-vector-main-menu-dropdown" class="vector-dropdown-checkbox "  aria-label="Main menu"  >
	<label id="vector-main-menu-dropdown-label" for="vector-main-menu-dropdown-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only " aria-hidden="true"  ><span class="vector-icon mw-ui-icon-menu mw-ui-icon-wikimedia-menu"></span>

<span class="vector-dropdown-label-text">Main menu</span>
	</label>
	<div class="vector-dropdown-content">


				<div id="vector-main-menu-unpinned-container" class="vector-unpinned-container">
		
<div id="vector-main-menu" class="vector-main-menu vector-pinnable-element">
	<div
	class="vector-pinnable-header vector-main-menu-pinnable-header vector-pinnable-header-unpinned"
	data-feature-name="main-menu-pinned"
	data-pinnable-element-id="vector-main-menu"
	data-pinned-container-id="vector-main-menu-pinned-container"
	data-unpinned-container-id="vector-main-menu-unpinned-container"
>
	<div class="vector-pinnable-header-label">Main menu</div>
	<button class="vector-pinna[74163427140] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=10088 len=32768 cached=10088 start=0 eof=false
[74171875503] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[74332247649] [[32mINFO [0m] [http] [CPU1] http: background task: read 1180 bytes from TLS
[74360901087] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 1180 bytes to foreground
[74392350219] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read_chunk revents=0x0001
ble-header-toggle-button vector-pinnable-header-pin-button" data-event-name="pinnable-header.vector-main-menu.pin">move to sidebar</button>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-unpin-button" data-event-name="pinnable-header.vector-main-menu.unpin">hide</button>
</div>

	
<div id="p-navigation" class="vector-menu mw-portlet mw-portlet-navigation"  >
	<div class="vector-menu-heading">
		Navigation
	</div>
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="n-mainpage-description" class="mw-list-item"><a href="/wiki/Main_Page" title="Visit the main page [z]" accesskey="z"><span>Main page</span></a></li><li id="n-contents" class="mw-list-item"><a href="/wiki/Wikipedia:Contents" title="Guides to browsing Wikipedia"><span>Contents</span></a></li><li id="n-currentevents" class="mw-list-item"><a href="/wiki/Portal:Current_events" title="Articles related to current events"><span>Current events</span></a></li><li id="n-randompage" class="mw-list-item"><a href="/wiki/Special:Random" title="Visit a randomly selected article [x]" accesskey="x"><span>Random article</span></a></li><li id="n-aboutsi[74562254085] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=11268 len=32768 cached=11268 start=0 eof=false
[74572451943] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[77737142439] [[32mINFO [0m] [http] [CPU1] http: background task: read 15204 bytes from TLS
[77774180385] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 15204 bytes to foreground
[77790616497] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read_chunk revents=0x0001
te" class="mw-list-item"><a href="/wiki/Wikipedia:About" title="Learn about Wikipedia and how it works"><span>About Wikipedia</span></a></li><li id="n-contactpage" class="mw-list-item"><a href="//en.wikipedia.org/wiki/Wikipedia:Contact_us" title="How to contact Wikipedia"><span>Contact us</span></a></li>
		</ul>
		
	</div>
</div>

	
<div id="p-interaction" class="vector-menu mw-portlet mw-portlet-interaction"  >
	<div class="vector-menu-heading">
		Contribute
	</div>
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="n-help" class="mw-list-item"><a href="/wiki/Help:Contents" title="Guidance on how to use and edit Wikipedia"><span>Help</span></a></li><li id="n-introduction" class="mw-list-item"><a href="/wiki/Help:Introduction" title="Learn how to edit Wikipedia"><span>Learn to edit</span></a></li><li id="n-portal" class="mw-list-item"><a href="/wiki/Wikipedia:Community_portal" title="The hub for editors"><span>Community portal</span></a></li><li id="n-recentchanges" class="mw-list-item"><a href="/wiki/Special:RecentChanges" title="A list of recent changes to Wikipedia [r]" accesskey="r"><span>Recent changes</span></a></li><li id="n-upload" class="mw-list-item"><a href="/wiki/Wikipedia:File_upload_wizard" title="Add images or other media for use on Wikipedia"><span>Upload file</span></a></li><li id="n-specialpages" class="mw-list-item"><a href="/wiki/Special:SpecialPages" title="A list of all special pages [q]" accesskey="q"><span>Special pages</span></a></li>
		</ul>
		
	</div>
</div>

</div>

				</div>

	</div>
</div>

		</nav>
			
<a href="/wiki/Main_Page" class="mw-logo">
	<img class="mw-logo-icon" src="/static/images/icons/enwiki-25.svg" alt="" aria-hidden="true" height="50" width="50">
	<span class="mw-logo-container skin-invert">
		<img class="mw-logo-wordmark" alt="Wikipedia" src="/static/images/mobile/copyright/wikipedia-wordmark-en-25.svg" style="width: 8.75em; height: 1.375em;">
		<img class="mw-logo-tagline" alt="The Free Encyclopedia" src="/static/images/mobile/copyright/wikipedia-tagline-en-25.svg" width="140" height="11" style="width: 8.75em; height: 0.6875em;">
	</span>
</a>

		</div>
		<div class="vector-header-end">
			
<div id="p-search" role="search" class="vector-search-box-vue  vector-search-box-collapses vector-search-box-show-thumbnail vector-search-box-auto-expand-width vector-search-box">
	<a href="/wiki/Special:Search" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only search-toggle" title="Search Wikipedia [f]" accesskey="f"><span class="vector-icon mw-ui-icon-search mw-ui-icon-wikimedia-search"></span>

<span>Search</span>
	</a>
	<div class="vector-typeahead-search-container">
		<div class="cdx-typeahead-search cdx-typeahead-search--show-thumbnail cdx-typeahead-search--auto-expand-width">
			<form action="/w/index.php" id="searchform" class="cdx-search-input cdx-search-input--has-end-button">
				<div id="simpleSearch" class="cdx-search-input__input-wrapper"  data-search-loc="header-moved">
					<div class="cdx-text-input cdx-text-input--has-start-icon">
						<input
							class="cdx-text-input__input mw-searchInput" autocomplete="off"
							 type="search" name="search" placeholder="Search Wikipedia" aria-label="Search Wikipedia" autocapitalize="none" spellcheck="false" title="Search Wikipedia [f]" accesskey="f" id="searchInput"
							>
						<span class="cdx-text-input__icon cdx-text-input__start-icon"></span>
					</div>
					<input type="hidden" name="title" value="Special:Search">
				</div>
				<button class="cdx-button cdx-search-input__end-button">Search</button>
			</form>
		</div>
	</div>
</div>

			<nav class="vector-user-links vector-user-links-wide" aria-label="Personal tools">
	<div class="vector-user-links-main">
	
<div id="p-vector-user-menu-preferences" class="vector-menu mw-portlet emptyPortlet"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			
		</ul>
		
	</div>
</div>

	
<div id="p-vector-user-menu-userpage" class="vector-menu mw-portlet emptyPortlet"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			
		</ul>
		
	</div>
</div>

	<nav class="vector-appearance-landmark" aria-label="Appearance">
		
<div id="vector-appearance-dropdown" class="vector-dropdown "  title="Change the appearance of the page&#039;s font size, width, and color" >
	<input type="checkbox" id="vector-appearance-dropdown-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-vector-appearance-dropdown" class="vector-dropdown-checkbox "  aria-label="Appearance"  >
	<label id="vector-appearance-dropdown-label" for="vector-appearance-dropdown-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only " aria-hidden="true"  ><span class="vector-icon mw-ui-icon-appearance mw-ui-icon-wikimedia-appearance"></span>

<span class="vector-dropdown-label-text">Appearance</span>
	</label>
	<div class="vector-dropdown-content">


			<div id="vector-appearance-unpinned-container" class="vector-unpinned-container">
				
			</div>
		
	</div>
</div>

	</nav>
	
<div id="p-vector-user-menu-notifications" class="vector-menu mw-portlet emptyPortlet"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			
		</ul>
		
	</div>
</div>

	
<div id="p-vector-user-menu-overflow" class="vector-menu mw-portlet"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			<li id="pt-sitesupport-2" class="mw-list-item user-links-collapsible-item"><a data-mw-interface  href="https://donate.wikimedia.org/?wmf_source=donate&amp;wmf_medium=sidebar&amp;wmf_campaign=en.wikipedia.org&amp;uselang=en" class=" cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet"><span class="vector-icon mw-ui-icon-heart mw-ui-icon-wikimedia-heart"></span>

<span>Donate</span></a>
</li>
<li id="pt-createaccount-2" class="mw-list-item user-links-collapsible-item"><a data-mw-interface  href="/w/index.php?title=Special:CreateAccount&amp;returnto=Dormouse" title="You are encouraged to create an account and log in; however, it is not mandatory" class=""><span>Create account</span></a>
</li>
<li id="pt-login-2" class="mw-list-item user-links-collapsible-item"><a data-mw-interface  href="/w/index.php?title=Special:UserLogin&amp;returnto=Dormouse" title="You&#039;re encouraged to log in; however, it&#039;s not mandatory. [o]" accesskey="o" class=""><span>Log in</span></a>
</li>

			
		</ul>
		
	</div>
</div>

	</div>
	
<div id="vector-user-links-dropdown" class="vector-dropdown vector-user-menu vector-button-flush-right vector-user-menu-logged-out user-links-collapsible-item"  title="Log in and more options" >
	<input type="checkbox" id="vector-user-links-dropdown-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-vector-user-links-dropdown" class="vector-dropdown-checkbox "  aria-label="Personal tools"  >
	<label id="vector-user-links-dropdown-label" for="vector-user-links-dropdown-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only " aria-hidden="true"  ><span class="vector-icon mw-ui-icon-ellipsis mw-ui-icon-wikimedia-ellipsis"></span>

<span class="vector-dropdown-label-text">Personal tools</span>
	</label>
	<div class="vector-dropdown-content">


		
<div id="p-personal" class="vector-menu mw-portlet mw-portlet-personal user-links-collapsible-item"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			<li id="pt-sitesupport" class="mw-list-item user-links-collapsible-item"><a data-mw-interface  href="https://donate.wikimedia.org/?wmf_source=donate&amp;wmf_medium=sidebar&amp;wmf_campaign=en.wikipedia.org&amp;uselang=en" class=""><span class="vector-icon mw-ui-icon-heart mw-ui-icon-wikimedia-heart"></span>

<span>Donate</span></a>
</li>
<li id="pt-createaccount" class="mw-list-item user-links-collapsible-item"><a data-mw-interface  href="/w/index.php?title=Special:CreateAccount&amp;returnto=Dormouse" title="You are encouraged to create an account and log in; however, it is not mandatory" class=""><span class="vector-icon mw-ui-icon-userAdd mw-ui-icon-wikimedia-userAdd"></span>

<span>Create account</span></a>
</li>
<li id="pt-login" class="mw-list-item user-links-collapsible-item"><a data-mw-interface  href="/w/index.php?title=Special:UserLogin&amp;returnto=Dormouse" title="You&#039;re encouraged to log in; however, it&#039;s not mandatory. [o]" accesskey="o" class=""><span class="vector-icon mw-ui-icon-logIn mw-ui-icon-wikimedia-logIn"></span>

<span>Log in</span></a>
</li>

			
		</ul>
		
	</div>
</div>

	
	</div>
</div>

</nav>

		</div>
	</header>
</div>
<div class="mw-page-container">
	<div class="mw-page-container-inner">
		<div class="vector-sitenotice-container">
			<div id="siteNotice"><!-- CentralNotice --></div>
		</div>
		<div class="vector-column-start">
			<div class="vector-main-menu-container">
		<div id="mw-navigation">
			<nav id="mw-panel" class="vector-main-menu-landmark" aria-label="Site">
				<div id="vector-main-menu-pinned-container" class="vector-pinned-container">
				
				</div>
		</nav>
		</div>
	</div>
	<div class="vector-sticky-pinned-container">
				<nav id="mw-panel-toc" aria-label="Contents" data-event-name="ui.sidebar-toc" class="mw-table-of-contents-container vector-toc-landmark">
					<div id="vector-toc-pinned-container" class="vector-pinned-container">
					<div id="vector-toc" class="vector-toc vector-pinnable-element">
	<div
	class="vector-pinnable-header vector-toc-pinnable-header vector-pinnable-header-pinned"
	data-feature-name="toc-pinned"
	data-pinnable-element-id="vector-toc"
	data-pinned-container-id="vector-toc-pinned-container"
	data-unpinned-container-id="vector-toc-unpinned-container"
>
	<h2 class="vector-pinnable-header-label">Contents</h2>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-pin-button" data-event-name="pinnable-header.vector-toc.pin">move to sidebar</button>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-unpin-button" data-event-name="pinnable-header.vector-toc.unpin">hide</button>
</div>


	<ul class="vector-toc-contents" id="mw-panel-toc-list">
		<li id="toc-mw-content-text"
			class="vector-toc-list-item vector-toc-level-1">
			<a href="#" class="vector-toc-link">
				<div class="vector-toc-text">(Top)</div>
			</a>
		</li>
		<li id="toc-Etymology"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#Etymology">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">1</span>
				<span>Etymology</span>
			</div>
		</a>
		
		<ul id="toc-Etymology-sublist" class="vector-toc-list">
		</ul>
	</li>
	<li id="toc-Characteristics"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#Characteristics">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">2</span>
				<span>Characteristics</span>
			</div>
		</a>
		
			<button aria-controls="toc-Characteristics-sublist" class="cdx-button cdx-button--weight-quiet cdx-button--icon-only vector-toc-toggle">
				<span class="vector-icon mw-ui-icon-wikimedia-expand"></span>
				<span>Toggle Characteristics subsection</span>
			</button>
		
		<ul id="toc-Characteristics-sublist" class="vector-toc-list">
			<li id="toc-Hibernation"
			class="vector-toc-list-item vector-toc-level-2">
			<a class="vector-toc-link" href="#Hibernation">
				<div class="vector-toc-text">
					<span class="vector-toc-numb">2.1</span>
					<span>Hibernation</span>
				</div>
			</a>
			
			<ul id="toc-Hibernation-sublist" class="vector-toc-list">
			</ul>
		</li>
	</ul>
	</li>
	<li id="toc-Relationship_with_humans"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#Relationship_with_humans">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">3</span>
				<span>Relationship with humans</span>
			</div>
		</a>
		
		<ul id="toc-Relationship_with_humans-sublist" class="vector-toc-list">
		</ul>
	</li>
	<li id="toc-Evolution"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#Evolution">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">4</span>
				<span>Evolution</span>
			</div>
		</a>
		
		<ul id="toc-Evolution-sublist" class="vector-toc-list">
		</ul>
	</li>
	<li id="toc-Classification"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#Classification">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">5</span>
				<span>Classification</span>
			</div>
		</a>
		
			<button aria-controls="toc-Classification-sublist" class="cdx-button cdx-button--weight-quiet cdx-button--icon-only vector-toc-toggle">
				<span class="vector-icon mw-ui-icon-wikimedia-expand"></span>
				<span>Toggle Classification subsection</span>
			</button>
		
		<ul id="toc-Classification-sublist" class="vector-toc-list">
			<li id="toc-Fossil_genera"
			class="vector-toc-list-item vector-toc-level-2">
			<a class="vector-toc-link" href="#Fossil_genera">
				<div class="vector-toc-text">
					<span class="vector-toc-numb">5.1</span>
					<span>Fossil genera</span>
				</div>
			</a>
			
			<ul id="toc-Fossil_genera-sublist" class="vector-toc-list">
			</ul>
		</li>
	</ul>
	</li>
	<li id="toc-References"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#References">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">6</span>
				<span>References</span>
			</div>
		</a>
		
		<ul id="toc-References-sublist" class="vector-toc-list">
		</ul>
	</li>
	<li id="toc-Further_reading"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#Further_reading">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">7</span>
				<span>Further reading</span>
			</div>
		</a>
		
		<ul id="toc-Further_reading-sublist" class="vector-toc-list">
		</ul>
	</li>
	<li id="toc-External_links"
		class="vector-toc-list-item vector-toc-level-1 vector-toc-list-item-expanded">
		<a class="vector-toc-link" href="#External_links">
			<div class="vector-toc-text">
				<span class="vector-toc-numb">8</span>
				<span>External links</span>
			</div>
		</a>
		
		<ul id="toc-External_links-sublist" class="vector-toc-list">
		</ul>
	</li>
</ul>
</div>

					</div>
		</nav>
			</div>
		</div>
		<div class="mw-content-container">
			<main id="content" class="mw-body">
				<header class="mw-body-header vector-page-titlebar no-font-mode-scale">
					<nav aria-label="Contents" class="vector-toc-landmark">
						
<div id="vector-page-titlebar-toc" class="vector-dropdown vector-page-titlebar-toc vector-button-flush-le[78424959525] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=26472 len=32768 cached=26472 start=0 eof=false
[78433439370] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[79104133911] [[32mINFO [0m] [http] [CPU1] http: background task: read 6296 bytes from TLS
[79133597367] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 6296 bytes to foreground
[79139369034] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read_chunk revents=0x0001
ft"  title="Table of Contents" >
	<input type="checkbox" id="vector-page-titlebar-toc-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-vector-page-titlebar-toc" class="vector-dropdown-checkbox "  aria-label="Toggle the table of contents"  >
	<label id="vector-page-titlebar-toc-label" for="vector-page-titlebar-toc-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only " aria-hidden="true"  ><span class="vector-icon mw-ui-icon-listBullet mw-ui-icon-wikimedia-listBullet"></span>

<span class="vector-dropdown-label-text">Toggle the table of contents</span>
	</label>
	<div class="vector-dropdown-content">


							<div id="vector-page-titlebar-toc-unpinned-container" class="vector-unpinned-container">
			</div>
		
	</div>
</div>

					</nav>
					<h1 id="firstHeading" class="firstHeading mw-first-heading"><span lang="en" dir="ltr"><span class="mw-page-title-main">Dormouse</span></span></h1>
							
<div id="p-lang-btn" class="vector-dropdown mw-portlet mw-portlet-lang"  >
	<input type="checkbox" id="p-lang-btn-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-p-lang-btn" class="vector-dropdown-checkbox mw-interlanguage-selector" aria-label="Go to an article in another language. Available in 70 languages"   >
	<label id="p-lang-btn-label" for="p-lang-btn-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--action-progressive mw-portlet-lang-heading-70" aria-hidden="true"  ><span class="vector-icon mw-ui-icon-language-progressive mw-ui-icon-wikimedia-language-progressive"></span>

<span class="vector-dropdown-label-text">70 languages</span>
	</label>
	<div class="vector-dropdown-content">

		<div class="vector-menu-content">
			
			<ul class="vector-menu-content-list">
				
				<li class="interlanguage-link interwiki-ar mw-list-item"><a href="https://ar.wikipedia.org/wiki/%D8%B2%D8%BA%D8%A8%D9%8A%D8%A9" title="زغبية �� Arabic" lang="ar" hreflang="ar" data-title="��غبية" data-language-autonym="ا��ع��بية" data-language-local-name="Arabic" class="interlanguage-link-target"><span>العربية</span></a></li><li class="interlanguage-link interwiki-arz mw-list-item"><a href="https://arz.wikipedia.org/wiki/%D8%B2%D8%BA%D8%A8%D9%8A%D9%87" title="زغبيه – Egyptian Arabic" lang="arz" hreflang="arz" data-title="زغبيه" data-language-autonym="مصرى" data-language-local-name="Egyptian Arabic" class="interlanguage-link-target"><span>مصرى</span></a></li><li class="interlanguage-link interwiki-ast mw-list-item"><a href="https://ast.wikipedia.org/wiki/Gliridae" title="Gliridae – Asturian" lang="ast" hreflang="ast" data-title="Gliridae" data-language-autonym="Asturianu" data-language-local-name="Asturian" class="interlanguage-link-target"><span>Asturianu</span></a></li><li class="interlanguage-link interwiki-avk mw-list-item"><a href="https://avk.wikipedia.org/wiki/Aspakol_(Gliridae)" title="Aspakol (Gliridae) ��� Kotava" lang="avk" hreflang="avk" data-title="Aspakol (Gliridae)" data-language-autonym="Kotava" data-language-local-name="Kotava" class="interlanguage-link-target"><span>Kotava</span></a></li><li class="interlanguage-link interwiki-az mw-list-item"><a href="https://az.wikipedia.org/wiki/S%C3%BCleysinl%C9%99r" title="Süleysinlər – Azerbaijani" lang="az" hreflang="az" data-title="Süleysinlər" data-language-autonym="Azərbaycanca" data-language-local-name="Azerbaijani" class="interlanguage-link-target"><span>Azərbaycanca</span></a></li><li class="interlanguage-link interwiki-ba mw-list-item"><a href="https://ba.wikipedia.org/wiki/%D0%99%D0%BE%D2%A1%D0%BB%D0%B0%D1%81%D1%82%D0%B0%D1%80" title="Йоҡ��ас��ар – Bashkir" lang="ba" hreflang="ba" data-title="Йо��ла��тар" data-language-autonym="Баш��ор��са" data-language-local-name="Bashkir" class="interlanguage-link-target"><span>Башҡортса</span></a></li><li class="interlanguage-link interwiki-be-x-old mw-list-item"><a href="https://be-tarask.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%B5%D0%B2%D1%8B%D1%8F" title="Соневыя – Belarusian (Taraškievica orthography)" lang="be-tarask" hreflang="be-tarask" data-title="������евыя" data-language-autonym="Белару��кая (��а��ашке��і��а)" data-language-local-name="Belarusian (Taraškievica orthography)" class="interlanguage-link-target"><span>Б��л��р��ск��я (т��рашк��в��ца)</span></a></li><li class="interlanguage-link interwiki-be mw-list-item"><a href="https://be.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%B5%D0%B2%D1%8B%D1%8F" title="Сон��выя �� Belarusian" lang="be" hreflang="be" data-title="Сон��в��я" data-language-autonym="Белару��к����" data-language-local-name="Belarusian" class="interlanguage-link-target"><span>Б��лару��кая</span></a></li><li class="interlanguage-link interwiki-bg mw-list-item"><a href="https://bg.wikipedia.org/wiki/%D0%A1%D1%8A%D0%BD%D0%BB%D0%B8%D0%B2%D1%86%D0%BE%D0%B2%D0%B8" title="С��нливцови – Bulgarian" lang="bg" hreflang="bg" data-title="Сънли��ц��ви" data-language-autonym="Б��лг��рски" data-language-local-name="Bulgarian" class="interlanguage-link-target"><span>Бъ��гар��к��</span></a></li><li class="interlanguage-link interwiki-br mw-list-item"><a href="https://br.wikipedia.org/wiki/Glirideged" title="Glirideged �� Breton" lang="br" hreflang="br" data-title="Glirideged" data-language-autonym="Brezhoneg" data-language-local-name="Breton" class="interlanguage-link-target"><span>Brezhoneg</span></a></li><li class="interlanguage-link interwiki-ca mw-list-item"><a href="https://ca.wikipedia.org/wiki/Lirons" title="Lirons – Catalan" lang="ca" hreflang="ca" data-title="Lirons" data-language-autonym="Catal��" data-language-local-name="Catalan" class="interlanguage-link-target"><span>Català</span></a></li><li class="interlanguage-link interwiki-ceb mw-list-item"><a href="https://ceb.wikipedia.org/wiki/Gliridae" title="Gliridae – Cebuano" lang="ceb" hreflang="ceb" data-title="Gliridae" data-language-autonym="Cebuano" data-language-local-name="Cebuano" class="interlanguage-link-target">[79388263350] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=32768 len=32768 cached=32768 start=0 eof=false
[79393591332] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[81772945551] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[81803740392] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
[81812230764] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read_chunk revents=0x0001
<span>Cebuano</span></a></li><li class="interlanguage-link interwiki-co mw-list-item"><a href="https://co.wikipedia.org/wiki/Gliridae" title="Gliridae – Corsican" lang="co" hreflang="co" data-title="Gliridae" data-language-autonym="Corsu" data-language-local-name="Corsican" class="interlanguage-link-target"><span>Corsu</span></a></li><li class="interlanguage-link interwiki-cs mw-list-item"><a href="https://cs.wikipedia.org/wiki/Plchovit%C3%AD" title="Plchovití �� Czech" lang="cs" hreflang="cs" data-title="Plchovití" data-language-autonym="��eština" data-language-local-name="Czech" class="interlanguage-link-target"><span>Če��tina</span></a></li><li class="interlanguage-link interwiki-da mw-list-item"><a href="https://da.wikipedia.org/wiki/Syvsovere" title="Syvsovere – Danish" lang="da" hreflang="da" data-title="Syvsovere" data-language-autonym="Dansk" data-language-local-name="Danish" class="interlanguage-link-target"><span>Dansk</span></a></li><li class="interlanguage-link interwiki-de mw-list-item"><a href="https://de.wikipedia.org/wiki/Bilche" title="Bilche – German" lang="de" hreflang="de" data-title="Bilche" data-language-autonym="Deutsch" data-language-local-name="German" class="interlanguage-link-target"><span>Deutsch</span></a></li><li class="interlanguage-link interwiki-eo mw-list-item"><a href="https://eo.wikipedia.org/wiki/Gliredoj" title="Gliredoj �� Esperanto" lang="eo" hreflang="eo" data-title="Gliredoj" data-language-autonym="Esperanto" data-language-local-name="Esperanto" class="interlanguage-link-target"><span>Esperanto</span></a></li><li class="interlanguage-link interwiki-es mw-list-item"><a href="https://es.wikipedia.org/wiki/Gliridae" title="Gliridae – Spanish" lang="es" hreflang="es" data-title="Gliridae" data-language-autonym="Español" data-language-local-name="Spanish" class="interlanguage-link-target"><span>Español</span></a></li><li class="interlanguage-link interwiki-et mw-list-item"><a href="https://et.wikipedia.org/wiki/Unilased" title="Unilased ��� Estonian" lang="et" hreflang="et" data-title="Unilased" data-language-autonym="Eesti" data-language-local-name="Estonian" class="interlanguage-link-target"><span>Eesti</span></a></li><li class="interlanguage-link interwiki-eu mw-list-item"><a href="https://eu.wikipedia.org/wiki/Muxar" title="Muxar ��� Basque" lang="eu" hreflang="eu" data-title="Muxar" data-language-autonym="Euskara" data-language-local-name="Basque" class="interlanguage-link-target"><span>Euskara</span></a></li><li class="interlanguage-link interwiki-fa mw-list-item"><a href="https://fa.wikipedia.org/wiki/%D9%85%D9%88%D8%B4_%D8%B2%D9%85%D8%B3%D8%AA%D8%A7%D9%86%E2%80%8C%D8%AE%D9%88%D8%A7%D8%A8" title="موش ز��ستا��‌��واب ��� Persian" lang="fa" hreflang="fa" data-title="موش زم��تان��خ��ا��" data-language-autonym="فارسی" data-language-local-name="Persian" class="interlanguage-link-target"><span>فارسی</span></a></li><li class="interlanguage-link interwiki-fi mw-list-item"><a href="https://fi.wikipedia.org/wiki/Unikeot" title="Unikeot – Finnish" lang="fi" hreflang="fi" data-title="Unikeot" data-language-autonym="Suomi" data-language-local-name="Finnish" class="interlanguage-link-target"><span>Suomi</span></a></li><li class="interlanguage-link interwiki-fr mw-list-item"><a href="https://fr.wikipedia.org/wiki/Gliridae" title="Gliridae – French" lang="fr" hreflang="fr" data-title="Gliridae" data-language-autonym="Français" data-language-local-name="French" class="interlanguage-link-target"><span>Français</span></a></li><li class="interlanguage-link interwiki-frr mw-list-item"><a href="https://frr.wikipedia.org/wiki/Sliapm%C3%BCsen" title="Sliapmüsen – Northern Frisian" lang="frr" hreflang="frr" data-title="Sliapmüsen" data-language-autonym="Nordfriisk" data-language-local-name="Northern Frisian" class="interlanguage-link-target"><span>Nordfriisk</span></a></li><li class="interlanguage-link interwiki-fy mw-list-item"><a href="https://fy.wikipedia.org/wiki/Sliepm%C3%BBzen" title="Sliepmûzen – Western Frisian" lang="fy" hreflang="fy" data-title="Sliepmûzen" data-language-autonym="Frysk" data-language-local-name="Western Frisian" class="interlanguage-link-target"><span>Frysk</span></a></li><li class="interlanguage-link interwiki-ga mw-list-item"><a href="https://ga.wikipedia.org/wiki/Codlam%C3%A1n" title="Codlamán – Irish" lang="ga" hreflang="ga" data-title="Codlam��n" data-language-autonym="Gaeilge" data-language-local-name="Irish" class="interlanguage-link-target"><span>Gaeilge</span></a></li><li class="interlanguage-link interwiki-gl mw-list-item"><a href="https://gl.wikipedia.org/wiki/Gl%C3%ADridos" title="Gl��ridos – Galician" lang="gl" hreflang="gl" data-title="Gl��ridos" data-language-autonym="Galego" data-language-local-name="Galician" class="interlanguage-link-target"><span>Galego</span></a></li><li class="interlanguage-link interwiki-he mw-list-item"><a href="https://he.wikipedia.org/wiki/%D7%A0%D7%9E%D7%A0%D7%9E%D7%A0%D7%99%D7%99%D7%9D" title="נ��נ��ניים – Hebrew" lang="he" hreflang="he" data-title="נמנמני��ם" data-language-autonym="עב��ית" data-language-local-name="Hebrew" class="interlanguage-link-target"><span>��בר��ת</span></a></li><li class="interlanguage-link interwiki-hu mw-list-item"><a href="https://hu.wikipedia.org/wiki/Pelef%C3%A9l%C3%A9k" title="Pelefélék – Hungarian" lang="hu" hreflang="hu" data-title="Pelef��l��k" data-language-autonym="Magyar" data-language-local-name="Hungarian" class="interlanguage-link-target"><span>Magyar</span></a></li><li class="interlanguage-link interwiki-id mw-list-item"><a href="https://id.wikipedia.org/wiki/Tikus_penidur" title="Tikus penidur ��� Indonesian" lang="id" hreflang="id" data-title="Tikus penidur" data-language-autonym="Bahasa Indonesia" data-language-local-name="Indonesian" class="interlanguage-link-target"><span>Bahasa Indonesia</span></a></li><li class="interlanguage-link interwiki-inh mw-list-item"><a href="https://inh.wikipedia.org/wiki/%D0%A2%D0%B0%D1%80%D1%81%D0%B0%D0%BB%D0%B0%D1%88" title="Т��рсала�� – Ingush" lang="inh" hreflang="inh" data-title="Та��са��аш" data-language-autonym="ГӀалгӀай" data-language-local-name="Ingush" class="interlanguage-link-target"><span>��Ӏ��л��Ӏ��й</span></a></li><li class="interlanguage-link interwiki-it mw-list-item"><a href="https://it.wikipedia.org/wiki/Gliridae" title="Gliridae – Italian" lang="it" hreflang="it" data-title="Gliridae" data-language-autonym="Italiano" data-language-local-name="Italian" class="interlanguage-link-target"><span>Italiano</span></a></li><li class="interlanguage-link interwiki-ja mw-list-item"><a href="https://ja.wikipedia.org/wiki/%E3%83%A4%E3%83%9E%E3%83%8D%E7%A7%91" title="�����ネ�� – Japanese" lang="ja" hreflang="ja" data-title="ヤマ��科" data-language-autonym="日��語" data-language-local-name="Japanese" class="interlanguage-link-target"><span>日���語</span></a></li><li class="interlanguage-link interwiki-ka mw-list-item"><a href="https://ka.wikipedia.org/wiki/%E1%83%AB%E1%83%98%E1%83%9A%E1%83%92%E1%83%A3%E1%83%93%E1%83%90%E1%83%A1%E1%83%94%E1%83%91%E1%83%A0%E1%83%9C%E1%83%98" title="ძ���ლგ��დ���სე��რ���ი – Georgian" lang="ka" hreflang="ka" data-title="ძ���ლგ��დ��ს���ბრნი" data-language-autonym="��ა���თუ��ი" data-language-local-name="Georgian" class="interlanguage-link-target"><span>��არ��უ��ი</span></a></li><li class="interlanguage-link interwiki-kab mw-list-item"><a href="https://kab.wikipedia.org/wiki/Acebcal" title="Acebcal �� Kabyle" lang="kab" hreflang="kab" data-title="Acebcal" data-language-autonym="Taqbaylit" data-language-local-name="Kabyle" class="interlanguage-link-target"><span>Taqbaylit</span></a></li><li class="interlanguage-link interwiki-kk mw-list-item"><a href="https://kk.wikipedia.org/wiki/%D2%9A%D0%B0%D1%80%D0%B0%D2%9B%D0%B0%D1%81_%D1%82%D2%B1%D2%9B%D1%8B%D0%BC%D0%B4%D0%B0%D1%81%D1%8B" title="Қара����с ����қымдасы �� Kazakh" lang="kk" hreflang="kk" data-title="��арақ��с ��ұқ��м��асы" data-language-autonym="����������ш��" data-language-local-name="Kazakh" class="interlanguage-link-target"><span>Қаз��қш��</span></a></li><li class="interlanguage-link interwiki-ko mw-list-item"><a href="https://ko.wikipedia.org/wiki/%EA%B2%A8%EC%9A%B8%EC%9E%A0%EC%A5%90%EB%A5%98" title="�����������류 ��� Korean" lang="ko" hreflang="ko" data-title="겨울��������" data-language-autonym="���국어" data-language-local-name="Korean" class="interlanguage-link-target"><span>한��어</span></a></li><li class="interlanguage-link interwiki-la mw-list-item"><a href="https://la.wikipedia.org/wiki/Gliridae" title="Gliridae – Latin" lang="la" hreflang="la" data-title="Gliridae" data-language-autonym="Latina" data-language-local-name="Latin" class="interlanguage-link-target"><span>Latina</span></a></li><li class="interlanguage-link interwiki-lb mw-list-item"><a href="https://lb.wikipedia.org/wiki/Schl%C3%A9ifer" title="Schléifer – Luxembourgish" lang="lb" hreflang="lb" data-title="Schléifer" data-language-autonym="Lëtzebuergesch" data-language-local-name="Luxembourgish" class="interlanguage-link-target"><span>Lëtzebuergesch</span></a></li><li class="interlanguage-link interwiki-lfn mw-list-item"><a href="https://lfn.wikipedia.org/wiki/Liron" title="Liron �� Lingua Franca Nova" lang="lfn" hreflang="lfn" data-title="Liron" data-language-autonym="Lingua Franca Nova" data-language-local-name="Lingua Franca Nova" class="interlanguage-link-target"><span>Lingua Franca Nova</span></a></li><li class="interlanguage-link interwiki-lt mw-list-item"><a href="https://lt.wikipedia.org/wiki/Miegapeliniai" title="Miegapeliniai – Lithuanian" lang="lt" hreflang="lt" data-title="Miegapeliniai" data-language-autonym="Lietuvių" data-language-local-name="Lithuanian" class="interlanguage-link-target"><span>Lietuvių</span></a></li><li class="interlanguage-link interwiki-lv mw-list-item"><a href="https://lv.wikipedia.org/wiki/Susuri" title="Susuri – Latvian" lang="lv" hreflang="lv" data-title="Susuri" data-language-autonym="Latvie��u" data-language-local-name="Latvian" class="interlanguage-link-target"><span>Latviešu</span></a></li><li class="interlanguage-link interwiki-mk mw-list-item"><a href="https://mk.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%BB%D0%B8%D0%B2%D1%86%D0%B8" title="Со��ли��ц�� – Macedonian" lang="mk" hreflang="mk" data-title="��онли��ц��" data-language-autonym="Ма��едон��ки" data-language-local-name="Macedonian" class="interlanguage-link-target"><span>Ма��ед��нски</span></a></li><li class="interlanguage-link interwiki-mrj mw-list-item"><a href="https://mrj.wikipedia.org/wiki/%D0%A3%D1%80%D0%B3%D0%B0%D0%BB%D1%8F_%D0%B9%D0%B8%D1%88%D0%B2%D0%BB%D3%93" title="������аля ������������ – Western Mari" lang="mrj" hreflang="mrj" data-title="��ргаля ��ишвлӓ" data-language-autonym="К��рык мары" data-language-local-name="Western Mari" class="interlanguage-link-target"><span>Кырык ма��ы</span></a></li><li class="interlanguage-link interwiki-ms mw-list-item"><a href="https://ms.wikipedia.org/wiki/Tikus_tidur" title="Tikus tidur – Malay" lang="ms" hreflang="ms" data-title="Tikus tidur" data-language-autonym="Bahasa Melayu" data-language-local-name="Malay" class="interlanguage-link-target"><span>Bahasa Melayu</span></a></li><li class="interlanguage-link interwiki-nds mw-list-item"><a href="https://nds.wikipedia.org/wiki/Slaapm%C3%BC%C3%BCs" title="Slaapmüüs – Low German" lang="nds" hreflang="nds" data-title="Slaapmüüs" data-language-autonym="Plattdüütsch" data-language-local-name="Low German" class="interlanguage-link-target"><span>Plattd����tsch</span></a></li><li class="interlanguage-link interwiki-nl mw-list-item"><a href="https://nl.wikipedia.org/wiki/Slaapmuizen" title="Slaapmuizen ��� Dutch" lang="nl" hreflang="nl" data-title="Slaapmuizen" data-language-autonym="Nederlands" data-language-local-name="Dutch" class="interlanguage-link-target"><span>Nederlands</span></a></li><li class="interlanguage-link interwiki-no mw-list-item"><a href="https://no.wikipedia.org/wiki/Syvsovere" title="Syvsovere ��� Norwegian Bokmål" lang="nb" hreflang="nb" data-title="Syvsovere" data-language-autonym="Norsk bokmål" data-language-local-name="Norwegian Bokmål" class="interlanguage-link-target"><span>Norsk bokmål</span></a></li><li class="interlanguage-link interwiki-nv mw-list-item"><a href="https://nv.wikipedia.org/wiki/Y%C3%A9igo_a%C5%82hoshii" title="Yéigo ałhoshii �� Navajo" lang="nv" hreflang="nv" data-title="Y��igo ałhoshii" data-language-autonym="Diné bizaad" data-language-local-name="Navajo" class="interlanguage-link-target"><span>Diné bizaad</span></a></li><li class="interlanguage-link interwiki-pl mw-list-item"><a href="https://pl.wikipedia.org/wiki/Popielicowate" title="Popielicowate – Polish" lang="pl" hreflang="pl" data-title="Popielicowate" data-language-autonym="Polski" data-language-local-name="Polish" class="interlanguage-link-target"><span>Polski</span></a></li><li class="interlanguage-link interwiki-pt mw-list-item"><a href="https://pt.wikipedia.org/wiki/Gliridae" title="Gliridae – Portuguese" lang="pt" hreflang="pt" data-title="Gliridae" data-language-autonym="Português" data-language-local-name="Portuguese" class="interlanguage-link-target"><span>Português</span></a></li><li class="interlanguage-link interwiki-ro mw-list-item"><a href="https://ro.wikipedia.org/wiki/P%C3%A2r%C8%99" title="Pârș – Romanian" lang="ro" hreflang="ro" data-title="Pârș" data-language-autonym="Română" data-language-local-name="Romanian" class="interlanguage-link-target"><span>Română</span></a></li><li class="interlanguage-link interwiki-ru mw-list-item"><a href="https://ru.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%B5%D0%B2%D1%8B%D0%B5" title="Соневые – Russian" lang="ru" hreflang="ru" data-title="Соневые" data-language-autonym="Русский" data-language-local-name="Russian" class="interlanguage-link-target"><span>Русский</span></a></li><li class="interlanguage-link interwiki-simple mw-list-item"><a href="https://simple.wikipedia.org/wiki/Dormouse" title="Dormouse – Simple English" lang="en-simple" hreflang="en-simple" data-title="Dormouse" data-language-autonym="Simple English" data-language-local-name="Simple English" class="interlanguage-link-target"><span>Simple English</span></a></li><li class="interlanguage-link interwiki-sl mw-list-item"><a href="https://sl.wikipedia.org/wiki/Polhi" title="Polhi ��� Slovenian" lang="sl" hreflang="sl" data-title="Polhi" data-language-autonym="Slovenščina" data-language-local-name="Slovenian" class="interlanguage-link-target"><span>Slovenščina</span></a></li><li class="interlanguage-link interwiki-sr mw-list-item"><a href="https://sr.wikipedia.org/wiki/%D0%9F%D1%83%D1%85%D0%BE%D0%B2%D0%B8" title="П��хови – Serbian" lang="sr" hreflang="sr" data-title="Пу��о��и" data-language-autonym="Ср��ск�� / srpski" data-language-local-name="Serbian" class="interlanguage-link-target"><span>С��п��ки / srpski</span></a></li><li class="interlanguage-link interwiki-sv badge-Q17559452 badge-recommendedarticle mw-list-item" title="recommended article"><a href="https://sv.wikipedia.org/wiki/Sovm%C3%B6ss" title="Sovmöss – Swedish" lang="sv" hreflang="sv" data-title="Sovm��ss" data-language-autonym="Svenska" data-language-local-name="Swedish" class="interlanguage-link-target"><span>Svenska</span></a></li><li class="interlanguage-link interwiki-sw mw-list-item"><a href="https://sw.wikipedia.org/wiki/Panya-miti" title="Panya-miti – Swahili" lang="sw" hreflang="sw" data-title="Panya-miti" data-language-autonym="Kiswahili" data-language-local-name="Swahili" class="interlanguage-link-target"><span>Kiswahili</span></a></li><li class="interlanguage-link interwiki-tl badge-Q70893996 mw-list-item" title=""><a href="https://tl.wikipedia.org/wiki/Gliridae" title="Gliridae – Tagalog" lang="tl" hreflang="tl" data-title="Gliridae" data-language-autonym="Tagalog" data-language-local-name="Tagalog" class="interlanguage-link-target"><span>Tagalog</span></a></li><li class="interlanguage-link interwiki-tr mw-list-item"><a href="https://tr.wikipedia.org/wiki/Gliridae" title="Gli[82269102960] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=49152 len=32768 cached=49152 start=0 eof=false
[82280410707] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[85338845361] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[85386417996] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
[85392595794] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read_chunk revents=0x0001
ridae ��� Turkish" lang="tr" hreflang="tr" data-title="Gliridae" data-language-autonym="Türkçe" data-language-local-name="Turkish" class="interlanguage-link-target"><span>Türkçe</span></a></li><li class="interlanguage-link interwiki-tt mw-list-item"><a href="https://tt.wikipedia.org/wiki/%D0%99%D0%BE%D0%BA%D0%BB%D0%B0%D1%87%D0%BB%D0%B0%D1%80" title="Йоклачл��р �� Tatar" lang="tt" hreflang="tt" data-title="Й��к��ачлар" data-language-autonym="Тат��рча / tatar��a" data-language-local-name="Tatar" class="interlanguage-link-target"><span>Татарча / tatarça</span></a></li><li class="interlanguage-link interwiki-uk mw-list-item"><a href="https://uk.wikipedia.org/wiki/%D0%92%D0%BE%D0%B2%D1%87%D0%BA%D0%BE%D0%B2%D1%96" title="��о��ч��ов�� – Ukrainian" lang="uk" hreflang="uk" data-title="Вов��к��в��" data-language-autonym="Ук��а��нськ��" data-language-local-name="Ukrainian" class="interlanguage-link-target"><span>����раї��ська</span></a></li><li class="interlanguage-link interwiki-uz mw-list-item"><a href="https://uz.wikipedia.org/wiki/Olmaxon" title="Olmaxon ��� Uzbek" lang="uz" hreflang="uz" data-title="Olmaxon" data-language-autonym="O��zbekcha / ўзбекча" data-language-local-name="Uzbek" class="interlanguage-link-target"><span>Oʻzbekcha / ��збек��а</span></a></li><li class="interlanguage-link interwiki-vi mw-list-item"><a href="https://vi.wikipedia.org/wiki/H%E1%BB%8D_Chu%E1%BB%99t_s%C3%B3c" title="H��� Chu��t sóc – Vietnamese" lang="vi" hreflang="vi" data-title="Họ Chu��t sóc" data-language-autonym="Tiếng Việt" data-language-local-name="Vietnamese" class="interlanguage-link-target"><span>Tiếng Vi���t</span></a></li><li class="interlanguage-link interwiki-vls mw-list-item"><a href="https://vls.wikipedia.org/wiki/Slapmuyzn" title="Slapmuyzn ��� West Flemish" lang="vls" hreflang="vls" data-title="Slapmuyzn" data-language-autonym="West-Vlams" data-language-local-name="West Flemish" class="interlanguage-link-target"><span>West-Vlams</span></a></li><li class="interlanguage-link interwiki-wa mw-list-item"><a href="https://wa.wikipedia.org/wiki/Sodoirmant" title="Sodoirmant �� Walloon" lang="wa" hreflang="wa" data-title="Sodoirmant" data-language-autonym="Walon" data-language-local-name="Walloon" class="interlanguage-link-target"><span>Walon</span></a></li><li class="interlanguage-link interwiki-war mw-list-item"><a href="https://war.wikipedia.org/wiki/Gliridae" title="Gliridae �� Waray" lang="war" hreflang="war" data-title="Gliridae" data-language-autonym="Winaray" data-language-local-name="Waray" class="interlanguage-link-target"><span>Winaray</span></a></li><li class="interlanguage-link interwiki-wuu mw-list-item"><a href="https://wuu.wikipedia.org/wiki/%E7%9D%A1%E9%BC%A0%E7%A7%91" title="睡鼠科 – Wu" lang="wuu" hreflang="wuu" data-title="睡�����" data-language-autonym="吴��" data-language-local-name="Wu" class="interlanguage-link-target"><span>吴��</span></a></li><li class="interlanguage-link interwiki-zh-yue mw-list-item"><a href="https://zh-yue.wikipedia.org/wiki/%E7%9D%A1%E9%BC%A0%E7%A7%91" title="睡鼠��� �� Cantonese" lang="yue" hreflang="yue" data-title="�����科" data-language-autonym="���語" data-language-local-name="Cantonese" class="interlanguage-link-target"><span>粵語</span></a></li><li class="interlanguage-link interwiki-zh mw-list-item"><a href="https://zh.wikipedia.org/wiki/%E7%9D%A1%E9%BC%A0%E7%A7%91" title="睡鼠科 – Chinese" lang="zh" hreflang="zh" data-title="��鼠���" data-language-autonym="�����" data-language-local-name="Chinese" class="interlanguage-link-target"><span>中文</span></a></li>
			</ul>
			<div class="after-portlet after-portlet-lang"><span class="wb-langlinks-edit wb-langlinks-link"><a href="https://www.wikidata.org/wiki/Special:EntityPage/Q108235#sitelinks-wikipedia" title="Edit interlanguage links" class="wbc-editpage">Edit links</a></span></div>
		</div>

	</div>
</div>
</header>
				<div class="vector-page-toolbar vector-feature-custom-font-size-clientpref--excluded">
					<div class="vector-page-toolbar-container">
						<div id="left-navigation">
							<nav aria-label="Namespaces">
								
<div id="p-associated-pages" class="vector-menu vector-menu-tabs mw-portlet mw-portlet-associated-pages"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="ca-nstab-main" class="selected vector-tab-noicon mw-list-item"><a href="/wiki/Dormouse" title="View the content page [c]" accesskey="c"><span>Article</span></a></li><li id="ca-talk" class="vector-tab-noicon mw-list-item"><a href="/wiki/Talk:Dormouse" rel="discussion" title="Discuss improvements to the content page [t]" accesskey="t"><span>Talk</span></a></li>
		</ul>
		
	</div>
</div>

								
<div id="vector-variants-dropdown" class="vector-dropdown emptyPortlet"  >
	<input type="checkbox" id="vector-variants-dropdown-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-vector-variants-dropdown" class="vector-dropdown-checkbox " aria-label="Change language variant"   >
	<label id="vector-variants-dropdown-label" for="vector-variants-dropdown-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet" aria-hidden="true"  ><span class="vector-dropdown-label-text">English</span>
	</label>
	<div class="vector-dropdown-content">


					
<div id="p-variants" class="vector-menu mw-portlet mw-portlet-variants emptyPortlet"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			
		</ul>
		
	</div>
</div>

				
	</div>
</div>

							</nav>
						</div>
						<div id="right-navigation" class="vector-collapsible">
							<nav aria-label="Views">
								
<div id="p-views" class="vector-menu vector-menu-tabs mw-portlet mw-portlet-views"  >
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="ca-view" class="selected vector-tab-noicon mw-list-item"><a href="/wiki/Dormouse"><span>Read</span></a></li><li id="ca-edit" class="vector-tab-noicon mw-list-item"><a href="/w/index.php?title=Dormouse&amp;action=edit" title="Edit this page [e]" accesskey="e"><span>Edit</span></a></li><li id="ca-history" class="vector-tab-noicon mw-list-item"><a href="/w/index.php?title=Dormouse&amp;action=history" title="Past revisions of this page [h]" accesskey="h"><span>View history</span></a></li>
		</ul>
		
	</div>
</div>

							</nav>
				
							<nav class="vector-page-tools-landmark" aria-label="Page tools">
								
<div id="vector-page-tools-dropdown" class="vector-dropdown vector-page-tools-dropdown"  >
	<input type="checkbox" id="vector-page-tools-dropdown-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-vector-page-tools-dropdown" class="vector-dropdown-checkbox "  aria-label="Tools"  >
	<label id="vector-page-tools-dropdown-label" for="vector-page-tools-dropdown-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet" aria-hidden="true"  ><span class="vector-dropdown-label-text">Tools</span>
	</label>
	<div class="vector-dropdown-content">


									<div id="vector-page-tools-unpinned-container" class="vector-unpinned-container">
						
<div id="vector-page-tools" class="vector-page-tools vector-pinnable-element">
	<div
	class="vector-pinnable-header vector-page-tools-pinnable-header vector-pinnable-header-unpinned"
	data-feature-name="page-tools-pinned"
	data-pinnable-element-id="vector-page-tools"
	data-pinned-container-id="vector-page-tools-pinned-container"
	data-unpinned-container-id="vector-page-tools-unpinned-container"
>
	<div class="vector-pinnable-header-label">Tools</div>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-pin-button" data-event-name="pinnable-header.vector-page-tools.pin">move to sidebar</button>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-unpin-button" data-event-name="pinnable-header.vector-page-tools.unpin">hide</button>
</div>

	
<div id="p-cactions" class="vector-menu mw-portlet mw-portlet-cactions emptyPortlet vector-has-collapsible-items"  title="More options" >
	<div class="vector-menu-heading">
		Actions
	</div>
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="ca-more-view" class="selected vector-more-collapsible-item mw-list-item"><a href="/wiki/Dormouse"><span>Read</span></a></li><li id="ca-more-edit" class="vector-more-collapsible-item mw-list-item"><a href="/w/index.php?title=Dormouse&amp;action=edit" title="Edit this page [e]" accesskey="e"><span>Edit</span></a></li><li id="ca-more-history" class="vector-more-collapsible-item mw-list-item"><a href="/w/index.php?title=Dormouse&amp;action=history"><span>View history</span></a></li>
		</ul>
		
	</div>
</div>

<div id="p-tb" class="vector-menu mw-portlet mw-portlet-tb"  >
	<div class="vector-menu-heading">
		General
	</div>
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="t-whatlinkshere" class="mw-list-item"><a href="/wiki/Special:WhatLinksHere/Dormouse" title="List of all English Wikipedia pages containing links to this page [j]" accesskey="j"><span>What links here</span></a></li><li id="t-recentchangeslinked" class="mw-list-item"><a href="/wiki/Special:RecentChangesLinked/Dormouse" rel="nofollow" title="Recent changes in pages linked from this page [k]" accesskey="k"><span>Related changes</span></a></li><li id="t-upload" class="mw-list-item"><a href="//en.wikipedia.org/wiki/Wikipedia:File_Upload_Wizard" title="Upload files [u]" accesskey="u"><span>Upload file</span></a></li><li id="t-permalink" class="mw-list-item"><a href="/w/index.php?title=Dormouse&amp;oldid=1348410573" title="Permanent link to this revision of this page"><span>Permanent link</span></a></li><li id="t-info" class="mw-list-item"><a href="/w/index.php?title=Dormouse&amp;action=info" title="More information about this page"><span>Page information</span></a></li><li id="t-cite" class="mw-list-item"><a href="/w/index.php?title=Special:CiteThisPage&amp;page=Dormouse&amp;id=1348410573&amp;wpFormIdentifier=titleform" title="Information on how to cite this page"><span>Cite this page</span></a></li><li id="t-urlshortener" class="mw-list-item"><a href="/w/index.php?title=Special:UrlShortener&amp;url=https%3A%2F%2Fen.wikipedia.org%2Fwiki%2FDormouse"><span>Get shortened URL</span></a></li>
		</ul>
		
	</div>
</div>

<div id="p-coll-print_export" class="vector-menu mw-portlet mw-portlet-coll-print_export"  >
	<div class="vector-menu-heading">
		Print/export
	</div>
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="coll-download-as-rl" class="mw-list-item"><a href="/w/index.php?title=Special:DownloadAsPdf&amp;page=Dormouse&amp;action=show-download-screen" title="Download this page as a PDF file"><span>Download as PDF</span></a></li><li id="t-print" class="mw-list-item"><a href="/w/index.php?title=Dormouse&amp;printable=yes" title="Printable version of this page [p]" accesskey="p"><span>Printable version</span></a></li>
		</ul>
		
	</div>
</div>

<div id="p-wikibase-otherprojects" class="vector-menu mw-portlet mw-portlet-wikibase-otherprojects"  >
	<div class="vector-menu-heading">
		In other projects
	</div>
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li class="wb-otherproject-link wb-otherproject-commons mw-list-item"><a href="https://commons.wikimedia.org/wiki/Gliridae" hreflang="en"><span>Wikimedia Commons</span></a></li><li class="wb-otherproject-link wb-otherproject-species mw-list-item"><a href="https://species.wikimedia.org/wiki/Gliridae" hreflang="en"><span>Wikispecies</span></a></li><li id="t-wikibase" class="wb-otherproject-link wb-otherproject-wikibase-dataitem mw-list-item"><a href="https://www.wikidata.org/wiki/Special:EntityPage/Q108235" title="Structured data on this page hosted by Wikidata [g]" accesskey="g"><span>Wikidata item</span></a></li>
		</ul>
		
	</div>
</div>

</div>

									</div>
				
	</div>
</div>

							</nav>
						</div>
					</div>
				</div>
				<div class="vector-column-end no-font-mode-scale">
					<div class="vector-sticky-pinned-container">
						<div class="wp25eastereggs-vector-sitenotice-landmark"></div>
						<nav class="vector-page-tools-landmark" aria-label="Page tools">
							<div id="vector-page-tools-pinned-container" class="vector-pinned-container">
				
							</div>
		</nav>
						<nav class="vector-appearance-landmark" aria-label="Appearance">
							<div id="vector-appearance-pinned-container" class="vector-pinned-container">
				<div id="vector-appearance" class="vector-appearance vector-pinnable-element">
	<div
	class="vector-pinnable-header vector-appearance-pinnable-header vector-pinnable-header-pinned"
	data-feature-name="appearance-pinned"
	data-pinnable-element-id="vector-appearance"
	data-pinned-container-id="vector-appearance-pinned-container"
	data-unpinned-container-id="vector-appearance-unpinned-container"
>
	<div class="vector-pinnable-header-label">Appearance</div>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-pin-button" data-event-name="pinnable-header.vector-appearance.pin">move to sidebar</button>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-unpin-button" data-event-name="pinnable-header.vector-appearance.unpin">hide</button>
</div>


</div>

							</div>
		</nav>
					</div>
				</div>
				<div id="bodyContent" class="vector-body" aria-labelledby="firstHeading" data-mw-ve-target-container>
					<div class="vector-body-before-content">
							<div class="mw-indicators">
		</div>

						<div id="siteSub" class="noprint">From Wikipedia, the free encyclopedia</div>
					</div>
					<div id="contentSub"><div id="mw-content-subtitle"></div></div>
					
					
					<div id="mw-content-text" class="mw-body-content"><div class="mw-subjectpageheader">
</div><div class="mw-content-ltr mw-parser-output" lang="en" dir="ltr"><div class="shortdescription nomobile noexcerpt noprint searchaux" style="display:none">Family of rodents</div>
<style data-mw-deduplicate="TemplateStyles:r1320445320">.mw-parser-output .hatnote{font-style:italic}.mw-parser-output div.hatnote{padding-left:1.6em;margin-bottom:0.5em}.mw-parser-output .hatnote i{font-style:normal}.mw-parser-output .hatnote+link+.hatnote{margin-top:-0.5em}@media print{body.ns-0 .mw-parser-output .hatnote{display:none!important}}</style><div role="note" class="hatnote navigation-not-searchable">"Door mouse" redirects here. For the film, see <a href="/wiki/Door_Mouse" title="Door Mouse">Door Mouse</a>.</div>
<link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1320445320" /><div role="note" class="hatnote navigation-not-searchable">For Lewis Carroll's fictional character, see <a href="/wiki/The_Dormouse" class="mw-redirect" title="The Dormouse">The Dormouse</a>.</div>
<p class="mw-empty-elt">
</p>
<table class="infobox biota" style="text-align: left; width: 200px; font-size: 100%">

<tbody><tr>
<th colspan="2" style="color:inherit; text-align: center; background-color: rgb(235,235,210)">Dormice<br /><div style="font-size: 85%;">Temporal range: <span class="noprint"><span style="display:inline-block;"></span><span style="display:inline-block;">Early Eocene ��� Recent</span> <span style="display:inline-block;"></span><div id="Timeline-row" style="margin: 4px auto 0; clear:both; width:280px; padding:0px; height:18px; overflow:visible; white-space:nowrap; border:1px #666; border-style:solid none; position:relative; z-index:0; font-size:97%;">
<div style="position:absolute; height:100%; left:0px; width:47.901538461538px; text-align:center; color:inherit; background-color:rgb(254,217,106); background-image: linear-gradient(to right, rgba(255,255,255,1), rgba(254,217,106,1) 75%, rgba(254,217,106,1));"><a href="/wiki/Precambrian" title="Precambrian">Pre���</a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(127,160,86); left:47.901538461538px; width:22.378461538462px"><a href="/wiki/Cambrian" title="Cambrian"><span style="color:white;">Ꞓ</span></a></div>
<div style="position:absolute; height:100%; t[85836309834] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=65536 len=32768 cached=65536 start=0 eof=false
[85843677051] [[32mINFO [0m] [http] [CPU3] http: waiting for http read_chunk readiness (fd=3)...
[88474020756] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[88498075710] [[32mINFO [0m] [http] [CPU3] http: wait_fd_ready complete for http read_chunk revents=0x0001
[88503227142] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
ext-align:center; color:inherit; background-color:rgb(0,146,112); left:70.28px; width:18.846153846154px"><a href="/wiki/Ordovician" title="Ordovician"><span style="color:white;">O</span></a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(179,225,182); left:89.126153846154px; width:10.114461538462px"><a href="/wiki/Silurian" title="Silurian">S</a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(203,140,55); left:99.240615384615px; width:26.173538461538px"><a href="/wiki/Devonian" title="Devonian">D</a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(103,165,153); left:125.41415384615px; width:25.828923076923px"><a href="/wiki/Carboniferous" title="Carboniferous"><span style="color:white;">C</span></a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(240,64,40); left:151.24307692308px; width:20.245292307692px"><a href="/wiki/Permian" title="Permian"><span style="color:white;">P</span></a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(129,43,146); left:171.48836923077px; width:21.754707692308px"><a href="/wiki/Triassic" title="Triassic"><span style="color:white;">T</span></a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(52,178,201); left:193.24307692308px; width:25.113846153846px"><a href="/wiki/Jurassic" title="Jurassic"><span style="color:white;">J</span></a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(127,198,78); left:218.35692307692px; width:33.212307692308px"><a href="/wiki/Cretaceous" title="Cretaceous">K</a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(253,154,82); left:251.56923076923px; width:18.505846153846px"><a href="/wiki/Paleogene" title="Paleogene">Pg</a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(255,230,25); left:270.07507692308px; width:8.8135384615385px"><a href="/wiki/Neogene" title="Neogene">N</a></div>
<div id="end-border" style="color:inherit; position:absolute; height:100%; background-color:#666; width:1px; left:279px"></div><div style="margin:0 auto; line-height:0; clear:both; width:280px; padding:0px; height:8px; overflow:visible; color:inherit; background-color:transparent; position:relative; top:-4px; z-index:100;"><div style="position:absolute; height:8px; left:255.87692307692px; width:24.123076923077px; color:inherit; background-color:#360; opacity:0.42;"></div>
<div style="position:absolute; height:8px; left:255.87692307692px; width:24.123076923077px; color:inherit; background-color:#360; opacity:1;"></div>
<div style="position:absolute; height:6px; top:1px; left:256.87692307692px; width:22.123076923077px; color:inherit; background-color:#6c3;"></div>
</div>
</div></span></div>
</th></tr>
<tr>
<td colspan="2" style="text-align: center"><span class="mw-default-size" typeof="mw:File/Frameless"><a href="/wiki/File:Graphiurus_spec_-murinus-1.jpg" class="mw-file-description"><img src="//upload.wikimedia.org/wikipedia/commons/thumb/4/43/Graphiurus_spec_-murinus-1.jpg/250px-Graphiurus_spec_-murinus-1.jpg" decoding="async" width="250" height="167" class="mw-file-element" srcset="//upload.wikimedia.org/wikipedia/commons/thumb/4/43/Graphiurus_spec_-murinus-1.jpg/500px-Graphiurus_spec_-murinus-1.jpg 2x" data-file-width="800" data-file-height="533" /></a></span>
</td></tr>
<tr>
<td colspan="2" style="text-align: center; font-size: 88%">African dormouse, <i><a href="/wiki/Graphiurus" title="Graphiurus">Graphiurus</a></i> sp.
</td></tr>






<tr>
<th colspan="2" style="color:inherit; min-width:15em; text-align: center; background-color: rgb(235,235,210)"><a href="/wiki/Taxonomy_(biology)" title="Taxonomy (biology)">Scientific classification</a> <span class="plainlinks taxobox-edit-taxonomy skin-invert" style="font-size:smaller; float:right; padding-right:0.4em; margin-left:-3em;"><span typeof="mw:File"><a href="/wiki/Template:Taxonomy/Gliridae" title="Edit this classification"><img alt="Edit this classification" src="//upload.wikimedia.org/wikipedia/commons/thumb/8/8a/OOjs_UI_icon_edit-ltr.svg/20px-OOjs_UI_icon_edit-ltr.svg.png" decoding="async" width="15" height="15" class="mw-file-element" srcset="//upload.wikimedia.org/wikipedia/commons/thumb/8/8a/OOjs_UI_icon_edit-ltr.svg/40px-OOjs_UI_icon_edit-ltr.svg.png 2x" data-file-width="20" data-file-height="20" /></a></span></span>
</th></tr>
<tr>
<td>Kingdom:
</td>
<td><a href="/wiki/Animal" title="Animal">Animalia</a>
</td></tr>
<tr class="taxonrow">
<td>Phylum:
</td>
<td><a href="/wiki/Chordate" title="Chordate">Chordata</a>
</td></tr>
<tr class="taxonrow">
<td>Class:
</td>
<td><a href="/wiki/Mammal" title="Mammal">Mammalia</a>
</td></tr>
<tr class="taxonrow">
<td>Order:
</td>
<td><a href="/wiki/Rodent" title="Rodent">Rodentia</a>
</td></tr>
<tr class="taxonrow">
<td>Suborder:
</td>
<td><a href="/wiki/Sciuromorpha" title="Sciuromorpha">Sciuromorpha</a>
</td></tr>
<tr class="taxonrow">
<td>Family:
</td>
<td><a class="mw-selflink selflink">Gliridae</a><br /><small><a href="/wiki/Lockhart_Muirhead" title="Lockhart Muirhead">Muirhead</a> in <a href="/wiki/David_Brewster" title="David Brewster">Brewster</a>, 1819<sup id="cite&#95;ref-1" class="reference"><a href="#cite_note-1"><span class="cite-bracket">&#91;</span>1<span class="cite-bracket">&#93;</span></a></sup></small>
</td></tr>





































































































<tr>
<th colspan="2" style="color:inherit; text-align: center; background-color: rgb(235,235,210)"><a href="/wiki/Type_genus" title="Type genus">Type genus</a>
</th></tr>
<tr>
<td colspan="2" style="text-align: center"><i><a href="/wiki/Edible_dormouse" class="mw-redirect" title="Edible dormouse">Glis</a></i><br /><div style="font-size: 85%;"><a href="/wiki/Mathurin_Jacques_Brisson" title="Mathurin Jacques Brisson">Brisson</a>, 1762</div>
</td></tr>












<tr>
<th colspan="2" style="color:inherit; text-align: center; background-color: rgb(235,235,210)">Subfamilies and genera
</th></tr>
<tr>
<td colspan="2" style="text-align: left">
<p><a href="/wiki/Graphiurinae" class="mw-redirect" title="Graphiurinae">Graphiurinae</a>
</p>
<ul><li><i><a href="/wiki/Graphiurus" title="Graphiurus">Graphiurus</a></i></li></ul>
<p><a href="/wiki/Leithiinae" title="Leithiinae">Leithiinae</a>
</p>
<ul><li><i><a href="/wiki/Chaetocauda" class="mw-redirect" title="Chaetocauda">Chaetocauda</a></i></li>
<li><i><a href="/wiki/Dryomys" title="Dryomys">Dryomys</a></i></li>
<li><i><a href="/wiki/Eliomys" title="Eliomys">Eliomys</a></i></li>
<li>†<i><a href="/wiki/Hypnomys" title="Hypnomys">Hypnomys</a></i></li>
<li>†<i><a href="/wiki/Leithia" title="Leithia">Leithia</a></i></li>
<li><i><a href="/wiki/Muscardinus" class="mw-redirect" title="Muscardinus">Muscardinus</a></i></li>
<li><i><a href="/wiki/Myomimus" title="Myomimus">Myomimus</a></i></li>
<li><i><a href="/wiki/Selevinia" class="mw-redirect" title="Selevinia">Selevinia</a></i></li></ul>
<p><a href="/wiki/Glirinae" title="Glirinae">Glirinae</a>
</p>
<ul><li><i><a href="/wiki/Glirulus" title="Glirulus">Glirulus</a></i></li>
<li><i><a href="/wiki/Edible_dormouse" class="mw-redirect" title="Edible dormouse">Glis</a></i></li></ul>
</td></tr>



<tr>
<td colspan="2" style="text-align: center"><span class="mw-default-size" typeof="mw:File/Frameless"><a href="/wiki/File:Gliridae_distribution.png" class="mw-file-description"><img src="//upload.wikimedia.org/wikipedia/commons/thumb/e/ef/Gliridae_distribution.png/250px-Gliridae_distribution.png" decoding="async" width="250" height="180" class="mw-file-element" srcset="//upload.wikimedia.org/wikipedia/commons/thumb/e/ef/Gliridae_distribution.png/500px-Gliridae_distribution.png 2x" data-file-width="1965" data-file-height="1412" /></a></span>
</td></tr>
<tr>
<td colspan="2" style="text-align: center; font-size: 88%">Gliridae distribution
</td></tr>












</tbody></table><style data-mw-deduplicate="TemplateStyles:r1289143401">@media(max-width:640px){body:not(.skin-minerva) .mw-parser-output .infobox{width:100%!important}body:not(.skin-minerva) .mw-parser-output .infobox th{width:50%}}@media screen{html.skin-theme-clientpref-night .mw-parser-output .infobox.biota tr{background:transparent!important}html.skin-theme-clientpref-night .mw-parser-output .infobox.biota img{background:transparent}}@media screen and (prefers-color-scheme:dark){html.skin-theme-clientpref-os .mw-parser-output .infobox.biota tr{background:transparent!important}html.skin-theme-clientpref-os .mw-parser-output .infobox.biota img{background:white}}.mw-parser-output .infobox.biota .taxobox-edit-taxonomy img{background:transparent!important}body.skin-vector .mw-parser-output table.biota.infobox{margin-top:0.5em}body.skin--responsive .mw-parser-output table.biota.infobox tr.taxonrow2 td{padding:2px 10px}</style>
<p>A <b>dormouse</b> is a <a href="/wiki/Rodent" title="Rodent">rodent</a> of the <a href="/wiki/Family_(biology)" title="Family (biology)">family</a> <b>Gliridae</b> (this family is also variously called <b>Myoxidae</b> or <b>Muscardinidae</b> by different taxonomists). Dormice are nocturnal animals found in Africa, Asia, and Europe. They are named for their long dormant <a href="/wiki/Hibernation" title="Hibernation">hibernation</a> period of six months or longer.<sup id="cite&#95;ref-The&#95;Mammal&#95;Society&#95;2-0" class="reference"><a href="#cite_note-The_Mammal_Society-2"><span class="cite-bracket">&#91;</span>2<span class="cite-bracket">&#93;</span></a></sup> There are nine genera and 28 living species of dormice, with half of living species belonging to the African genus <i><a href="/wiki/Graphiurus" title="Graphiurus">Graphiurus</a>.</i><sup id="cite&#95;ref-Lu-2021&#95;3-0" class="reference"><a href="#cite_note-Lu-2021-3"><span class="cite-bracket">&#91;</span>3<span class="cite-bracket">&#93;</span></a></sup>
</p>
<meta property="mw:PageProp/toc" />
<div class="mw-heading mw-heading2"><h2 id="Etymology">Etymology</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=1" title="Edit section: Etymology"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<p>The word dormouse comes from <a href="/wiki/Middle_English" title="Middle English">Middle English</a> <span title="Middle English (1100-1500)-language text"><i lang="enm">dormous</i></span>, of uncertain origin, possibly from a dialectal element <i>*dor-</i>, from <a href="/wiki/Old_Norse" title="Old Norse">Old Norse</a> <span title="Old Norse-language text"><i lang="non">dár</i></span> <span class="gloss-quot">'</span><span class="gloss-text">benumbed</span><span class="gloss-quot">'</span> and Middle English <span title="Middle English (1100-1500)-language text"><i lang="enm">mous</i></span> <span class="gloss-quot">'</span><span class="gloss-text">mouse</span><span class="gloss-quot">'</span>.
</p><p>The word is sometimes conjectured to come from an <a href="/wiki/Anglo-Norman_language" title="Anglo-Norman language">Anglo-Norman</a> derivative of <span title="Anglo-Norman-language text"><i lang="xno">dormir</i></span> <span class="gloss-quot">'</span><span class="gloss-text">to sleep</span><span class="gloss-quot">'</span>, with the second element mistaken for <i>mouse</i>, but no such Anglo-Norman term is known to have existed.<sup id="cite&#95;ref-4" class="reference"><a href="#cite_note-4"><span class="cite-bracket">&#91;</span>4<span class="cite-bracket">&#93;</span></a></sup><sup id="cite&#95;ref-5" class="reference"><a href="#cite_note-5"><span class="cite-bracket">&#91;</span>5<span class="cite-bracket">&#93;</span></a></sup>
</p><p>The Latin noun <span title="Latin-language text"><i lang="la">gl��s</i></span>, which is the origin of the scientific name, descends from the <a href="/wiki/Proto-Indo-European" class="mw-redirect" title="Proto-Indo-European">Proto-Indo-Eur
```
</details>
