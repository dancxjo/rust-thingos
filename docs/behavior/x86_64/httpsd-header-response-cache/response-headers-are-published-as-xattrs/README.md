# ❌ Scenario: Response headers are published as xattrs

> Last run: 2026-04-21 14:25:49

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is booted | ✅ | 8211ms | - [📜](./01/serial.log) - |
| 2 | When I wait for the shell prompt | ✅ | 2205ms | - [📜](./02/serial.log) - |
| 3 | And I type "ping -c 1 en.wikipedia.org" on the serial console | ✅ | 2382ms | - [📜](./03/serial.log) - |
| 4 | And I wait for the serial output to contain "1 packets transmitted, 1 received" | ✅ | 304ms | - [📜](./04/serial.log) - |
| 5 | And I type "cat /https/en.wikipedia.org/wiki/Dormouse > /dev/null" on the serial console | ✅ | 3769ms | - [📜](./05/serial.log) - |
| 6 | And I wait for 2 seconds | ✅ | 2000ms | - [📜](./06/serial.log) - |
| 7 | And I type "attr_list /https/en.wikipedia.org/wiki/Dormouse" on the serial console | ✅ | 3471ms | - [📜](./07/serial.log) - |
| 8 | Then the serial output should contain "user.http.status_code" within 30s | ❌ | 31065ms | - [📜](./08/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[24407569461] [[32mINFO [0m] [kernel] [CPU0] Initializing global allocator...
[26259842235] [[32mINFO [0m] [kernel] [CPU0] Initializing SIMD...
[26262812961] [[32mINFO [0m] [kernel] [CPU0] Initializing tasking...
[26320562631] [[32mINFO [0m] [kernel::sched] [CPU0] Scheduler initialized
[26879717997] [[32mINFO [0m] [kernel] [CPU0] Entering scheduler loop.
[27010669422] [[32mINFO [0m] [sprout] [CPU0] SPROUT: v0.4.1 [REBUILT] starting (Supervisor Mode)...
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
[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;95mBOOT[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25h[28549558224] [[32mINFO [0m] [netd] [CPU2] NETD: Starting network service...
[28580773254] [[32mINFO [0m] [netd] [CPU2] NETD: Waiting for virtio_netd VFS provider at /dev/net/virtio*...
[28946331414] [[32mINFO [0m] [iso9660d] [CPU1] iso9660d: no ISO9660 filesystem found yet ��� retrying
ping -c 1 en.wikipedia[37730321112] [[32mINFO [0m] [netd] [CPU2] NETD: Network ready
.org
[?25lPING en.wikipedia.org (198.35.26.224) 56 bytes of data
64 bytes from 198.35.26.224: icmp_seq=1 time=1323ms

--- en.wikipedia.org ping statistics ---
1 packets transmitted, 1 received, 0% packet loss
rtt min/avg/max = 1323/1323/1323 ms
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hcat /https/en.wikipedia.org/wiki/Dormouse > /dev/null
[?25l[52085260458] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=0 len=32768 cached=0 start=0 eof=false
[52088006091] [[32mINFO [0m] [httpsd] [CPU3] httpsd: opening upstream stream for handle=2 https://en.wikipedia.org/wiki/Dormouse
[52104615882] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=0/120)
[52105928622] [[32mINFO [0m] [http] [CPU1] http: connect host=en.wikipedia.org port=443
[52107757845] [[32mINFO [0m] [http] [CPU1] http: opening /net/tcp/new
[56537888913] [[32mINFO [0m] [http] [CPU1] http: allocated tcp socket id=2
[56540229735] [[32mINFO [0m] [http] [CPU1] http: opening ctl path /net/tcp/2/ctl
[56614782015] [[32mINFO [0m] [http] [CPU1] http: opening data path /net/tcp/2/data
[56687780226] [[32mINFO [0m] [http] [CPU1] http: issuing connect command: connect en.wikipedia.org 443
[57100579899] [[32mINFO [0m] [http] [CPU1] http: waiting for socket readiness...
[57103684605] [[32mINFO [0m] [http] [CPU1] http: waiting for http connect socket readiness (fd=4)...
[58141220379] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http connect socket revents=0x0004
[58144115403] [[32mINFO [0m] [http] [CPU1] http: background task: TCP connected to en.wikipedia.org
[58146603075] [[32mINFO [0m] [http] [CPU1] http: background task: starting TLS handshake with en.wikipedia.org
[58588768227] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[59247675201] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
[60478125312] [[32mINFO [0m] [http] [CPU1] http: background task: TLS handshake complete for en.wikipedia.org
[60480013968] [[32mINFO [0m] [http] [CPU1] http: background task: sending 144 byte request
[60482164941] [[32mINFO [0m] [http] [CPU1] http: background task: wrote 144 bytes (total=144)
[60483714225] [[32mINFO [0m] [http] [CPU1] http: background task: flushing TLS stream
[61016841798] [[32mINFO [0m] [http] [CPU1] http: background task: entering read loop
[61098239367] [[32mINFO [0m] [http] [CPU1] http: waiting for http read readiness (fd=4)...
[61346785104] [[32mINFO [0m] [http] [CPU1] http: wait_fd_ready complete for http read revents=0x0001
attr_list /[63776118747] [[32mINFO [0m] [http] [CPU1] http: background task: read 14724 bytes from TLS
[63785499987] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 14724 bytes to foreground
[63906807921] [[32mINFO [0m] [http] [CPU3] http: received 4096 bytes from background TLS thread
[63909601536] [[32mINFO [0m] [http] [CPU3] http: waiting for header data from port (attempt=1/120)
[63913391553] [[32mINFO [0m] [http] [CPU3] http: received 4096 bytes from background TLS thread
[63915564174] [[32mINFO [0m] [http] [CPU3] http: headers complete body_start=4636
h[63917524572] [[32mINFO [0m] [http] [CPU3] http: response header: HTTP/1.1 200 OK
[63919004358] [[32mINFO [0m] [http] [CPU3] http: response header: date: Tue, 21 Apr 2026 03:31:16 GMT
[63920559978] [[32mINFO [0m] [http] [CPU3] http: response header: server: mw-web.codfw.main-67b9b4586d-pbrsp
[63922144836] [[32mINFO [0m] [http] [CPU3] http: response header: x-content-type-options: nosniff
[63923643069] [[32mINFO [0m] [http] [CPU3] http: response header: content-language: en
[63925211757] [[32mINFO [0m] [http] [CPU3] http: response header: accept-ch:
[63926524464] [[32mINFO [0m] [http] [CPU3] http: response header: last-modified: Mon, 20 Apr 2026 16:04:02 GMT
[63928401801] [[32mINFO [0m] [http] [CPU3] http: response header: content-type: text/html; charset=UTF-8
[63930839214] [[32mINFO [0m] [http] [CPU3] http: response header: age: 64490
[63932872047] [[32mINFO [0m] [http] [CPU3] http: response header: accept-ranges: bytes
[63936364932] [[32mINFO [0m] [http] [CPU3] http: response header: x-cache: cp4037 miss, cp4037 hit/5
[63937987311] [[32mINFO [0m] [http] [CPU3] http: response header: x-cache-status: hit-front
[63939465381] [[32mINFO [0m] [http] [CPU3] http: response header: server-timing: cache;desc="hit-front", host;desc="cp4037"
[63941202567] [[32mINFO [0m] [http] [CPU3] http: response header: strict-transport-security: max-age=106384710; includeSubDomains; preload
[63943105347] [[32mINFO [0m] [http] [CPU3] http: response header: report-to: { "group": "wm_nel", "max_age": 604800, "endpoints": [{ "url": "https://intake-logging.wikimedia.org/v1/events?stream=w3c.reportingapi.network_error&schema_uri=/w3c/reportingapi/network_error/1.0.0" }] }
[63946630374] [[32mINFO [0m] [http] [CPU3] http: response header: nel: { "report_to": "wm_nel", "max_age": 604800, "failure_fraction": 0.05, "success_fraction": 0.0}
[63948850251] [[32mINFO [0m] [http] [CPU3] http: response header: set-cookie: WMF-Last-Access=21-Apr-2026;Path=/;HttpOnly;secure;Expires=Sat, 23 May 2026 12:00:00 GMT
[63951147975] [[32mINFO [0m] [http] [CPU3] http: response header: set-cookie: WMF-Last-Access-Global=21-Apr-2026;Path=/;Domain=.wikipedia.org;HttpOnly;secure;Expires=Sat, 23 May 2026 12:00:00 GMT
[63953987295] [[32mINFO [0m] [http] [CPU3] http: response header: set-cookie: WMF-DP=918;Path=/;HttpOnly;secure;Expires=Wed, 22 Apr 2026 00:00:00 GMT
[63956017059] [[32mINFO [0m] [http] [CPU3] http: response header: x-client-ip: 76.104.142.245
[63957434937] [[32mINFO [0m] [http] [CPU3] http: response header: cache-control: private, s-maxage=0, max-age=0, must-revalidate, no-transform
[63959691114] [[32mINFO [0m] [http] [CPU3] http: response header: content-security-policy: default-src 'unsafe-eval' 'unsafe-inline' 'self' data: blob: *.wikimedia.org *.wikipedia.org *.wikinews.org *.wiktionary.org *.wikibooks.org *.wikiversity.org *.wikisource.org wikisource.org *.wikiquote
[63963334050] [[32mINFO [0m] [user.print] [CPU3] .org *.wikidata.org wikidata.org *.wikifunctions.org wikifunctions.org *.wikivoyage.org *.mediawiki.org mediawiki.org wikimedia.org *.wmflabs.org *.wmcloud.org *.toolforge.org wss://*.toolforge.org *.jsdelivr.net unpkg.com cdnjs.cloudflare.com raw.githubus
[63966785751] [[32mINFO [0m] [user.print] [CPU3] ercontent.com *.github.com code.jquery.com cdn.mathjax.org use.typekit.net fonts.cdnfonts.com use.fontawesome.com i.ytimg.com rsms.me doi.org localhost https://localhost:* http://localhost:* wss://localhost:* ws://localhost:* *.google.com *.gstatic.com *.g
[63971197191] [[32mINFO [0m] [user.print] [CPU3] oogleapis.com *.translate.yandex.net yastatic.net ya.ru radically.github.io cdn.sammdot.ca cdn.fontshare.com viaf.org publicai-proxy.alaexis.workers.dev iiif.archive.org api.flickr.com live.staticflickr.com api.anthropic.com api.openai.com api.publicai.co
[63974812770] [[32mINFO [0m] [user.print] [CPU3] catalogo.pusc.it parsifal.urbe.it opac.sbn.it overpass-api.de api.openrouteservice.org archive.org *.openstreetmap.org *.waymarkedtrails.org *.thunderforest.com registry.ipe.wiki analytics.ipe.wiki qlever.dev app.goacoustic.com wikipedia-archive.ourworldin
[63978626646] [[32mINFO [0m] [user.print] [CPU3] data.org api.inaturalist.org inaturalist-open-data.s3.amazonaws.com validator.w3.org db.onlinewebfonts.com fontlibrary.org; object-src 'none'; report-uri /w/api.php?action=cspreport&format=json
[63981789729] [[32mINFO [0m] [http] [CPU3] http: response header: content-security-policy-report-only: script-src 'unsafe-eval' blob: 'self' meta.wikimedia.org *.wikimedia.org *.wikipedia.org *.wikinews.org *.wiktionary.org *.wikibooks.org *.wikiversity.org *.wikisource.org wikisource.org *.w
[63985456524] [[32mINFO [0m] [user.print] [CPU3] ikiquote.org *.wikidata.org *.wikifunctions.org *.wikivoyage.org *.mediawiki.org 'unsafe-inline' auth.wikimedia.org; default-src 'self' data: blob: upload.wikimedia.org https://commons.wikimedia.org meta.wikimedia.org *.wikimedia.org *.wikipedia.org *.wiki
[63989115300] [[32mINFO [0m] [user.print] [CPU3] news.org *.wiktionary.org *.wikibooks.org *.wikiversity.org *.wikisource.org wikisource.org *.wikiquote.org *.wikidata.org *.wikifunctions.org *.wikivoyage.org *.mediawiki.org wikimedia.org en.wikibooks.org en.wikinews.org en.wikiquote.org en.wikisource.or
[63992592378] [[32mINFO [0m] [user.print] [CPU3] g en.wikiversity.org en.wikivoyage.org en.wiktionary.org www.mediawiki.org api.wikimedia.org commons.wikimedia.org foundation.wikimedia.org incubator.wikimedia.org species.wikimedia.org wikimania.wikimedia.org www.wikidata.org www.wikifunctions.org auth.wi
[63996439683] [[32mINFO [0m] [user.print] [CPU3] kimedia.org; style-src 'self' data: blob: upload.wikimedia.org https://commons.wikimedia.org meta.wikimedia.org *.wikimedia.org *.wikipedia.org *.wikinews.org *.wiktionary.org *.wikibooks.org *.wikiversity.org *.wikisource.org wikisource.org *.wikiquote.or
[64000178352] [[32mINFO [0m] [user.print] [CPU3] g *.wikidata.org *.wikifunctions.org *.wikivoyage.org *.mediawiki.org wikimedia.org 'unsafe-inline'; object-src 'none'; report-uri /w/api.php?action=cspreport&format=json&reportonly=1
[64003760370] [[32mINFO [0m] [http] [CPU3] http: response header: vary: Accept-Encoding,X-Subdomain,Cookie,Authorization,User-Agent
[64005630381] [[32mINFO [0m] [http] [CPU3] http: response header: set-cookie: GeoIP=US:WA:Seattle:47.54:-122.35:v4; Path=/; secure; Domain=.wikipedia.org
[64007553192] [[32mINFO [0m] [http] [CPU3] http: response header: set-cookie: NetworkProbeLimit=0.001;Path=/;Secure;SameSite=None;Max-Age=3600
[64009518012] [[32mINFO [0m] [http] [CPU3] http: response header: set-cookie: WMF-Uniq=adSDmGMrl23bjje3-_cAfwNJAAAAAFvdtBakcIex_6cu8qlfsPLcT1LWpFXdsyFO;Domain=.wikipedia.org;Path=/;HttpOnly;secure;SameSite=None;Expires=Wed, 21 Apr 2027 00:00:00 GMT
[64012573053] [[32mINFO [0m] [http] [CPU3] http: response header: content-length: 202560
[64014051948] [[32mINFO [0m] [http] [CPU3] http: response header: x-request-id: dd71f3cf-9735-4a4a-ac6a-05a6d9f80028
[64015783095] [[32mINFO [0m] [http] [CPU3] http: response header: x-analytics:
[64017074781] [[32mINFO [0m] [http] [CPU3] http: response header: connection: close
[64018654623] [[32mINFO [0m] [http] [CPU3] http: response header:
<!DOCTYPE html>
<html class="client-nojs vector-feature-language-in-header-enabled vector-feature-language-in-main-menu-disabled vector-feature-language-in-main-page-header-disabled vector-feature-page-tools-pinned-disabled vector-feature-toc-pinned-clientpref-1 vector-feature-main-menu-pinned-disabled vector-feature-limited-width-clientpref-1 vector-feature-limited-width-content-enabled vector-feature-custom-font-size-clientpref-1 vector-feature-appearance-pinned-clientpref-1 skin-theme-clientpref-day vector-sticky-header-enabled vector-toc-available skin-theme-clientpref-thumb-standard" lang="en" dir="ltr">
<head>
<meta charset="UTF-8">
<title>Dormouse - Wikipedia</title>
<script>(function(){var className="client-js vector-feature-language-in-header-enabled vector-feature-language-in-main-menu-disabled vector-feature-language-in-main-page-header-disabled vector-feature-page-tools-pinned-disabled vector-feature-toc-pinned-clientpref-1 vector-feature-main-menu-pinned-disabled vector-feature-limited-width-clientpref-1 vector-feature-limited-width-content-enabled vector-feature-custom-font-size-clientpref-1 vector-feature-appearance-pinned-clientpref-1 skin-theme-clientpref-day vector-sticky-header-enabled vector-toc-available skin-theme-clientpref-thumb-standard";var cookie=document.cookie.match(/(?:^|; )enwikimwclientpreferences=([^;]+)/);if(cookie){cookie[1].split('%2C').forEach(function(pref){className=className.replace(new RegExp('(^| )'+pref.replace(/-clientpref-\w+$|[^\w-]+/g,'')+'-clientpref-\\w+( |$)'),'$1'+pref+'$2');});}document.documentElement.className=className;}());RLCONF={"wgBreakFrames":false,"wgSeparatorTransformTable":["",""],"wgDigitTransformTable":["",""],"wgDefaultDateFormat":"dmy","wgMonthNames":["","January","February","March","April","May","June","July","August","September","October","November","December"],"wgRequestId":"42973d49-322b-406f-8a3f-7712b0c3ce81","wgCanonicalNamespace":"","wgCanonicalSpecialPageName":false,"wgNamespaceNumber":0,"wgPageName":"Dormouse","wgTitle":"Dormouse","wgCurRevisionId":1348410573,"wgRevisionId":1348410573,"wgArticleId":438703,"wgIsArticle":true,"wgIsRedirect":false,"wgAction":"view","wgUserName":null,"wgUserGroups":["*"],"wgCategories":["Articles with short description","Short description is different from Wikidata","Articles with 'species' microformats","Articles containing Middle English (1100-1500)-language text","Articles containing Old Norse-language text","Articles containing Anglo-Norman-language text","Articles containing Latin-language text","Articles containing Sanskrit-language text","Articles containing Ancient Greek (to 1453)-language text","Commons link from Wikidata","Articles with German-language sources (de)","Dormice","Sciuromorpha","Natural Monuments of Japan","Extant Eocene first appeartances"],"wgPageViewLanguage":"en","wgPageContentLanguage":"en","wgPageContentModel":"wikitext","wgRelevantPageName":"Dormouse","wgRelevantArticleId":438703,"wgTempUserName":null,"wgIsProbablyEditable":true,"wgRelevantPageIsProbablyEditable":true,"wgRestrictionEdit":[],"wgRestrictionMove":[],"wgNoticeProject":"wikipedia","wgFlaggedRevsParams":{"tags":{"status":{"levels":1}}},"wgConfirmEditCaptchaNeededForGenericEdit":"hcaptcha","wgConfirmEditHCaptchaVisualEditorOnLoadIntegrationEnabled":false,"wgConfirmEditHCaptchaSiteKey":"5d0c670e-a5f4-4258-ad16-1f42792c9c62","wgMediaViewerOnClick":true,"wgMediaViewerEnabledByDefault":true,"wgPopupsFlags":0,"wgVisualEditor":{"pageLanguageCode":"en","pageLanguageDir":"ltr","pageVariantFallbacks":"en"},[64086909876] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=3556 len=32768 cached=3556 start=0 eof=false
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
	<button class="vector-pinna[64196460240] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=10088 len=32768 cached=10088 start=0 eof=false
t[64313851041] [[32mINFO [0m] [http] [CPU1] http: background task: read 1180 bytes from TLS
[64316567205] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 1180 bytes to foreground
ble-header-toggle-button vector-pinnable-header-pin-button" data-event-name="pinnable-header.vector-main-menu.pin">move to sidebar</button>
	<button class="vector-pinnable-header-toggle-button vector-pinnable-header-unpin-button" data-event-name="pinnable-header.vector-main-menu.unpin">hide</button>
</div>

	
<div id="p-navigation" class="vector-menu mw-portlet mw-portlet-navigation"  >
	<div class="vector-menu-heading">
		Navigation
	</div>
	<div class="vector-menu-content">
		
		<ul class="vector-menu-content-list">
			
			<li id="n-mainpage-description" class="mw-list-item"><a href="/wiki/Main_Page" title="Visit the main page [z]" accesskey="z"><span>Main page</span></a></li><li id="n-contents" class="mw-list-item"><a href="/wiki/Wikipedia:Contents" title="Guides to browsing Wikipedia"><span>Contents</span></a></li><li id="n-currentevents" class="mw-list-item"><a href="/wiki/Portal:Current_events" title="Articles related to current events"><span>Current events</span></a></li><li id="n-randompage" class="mw-list-item"><a href="/wiki/Special:Random" title="Visit a randomly selected article [x]" accesskey="x"><span>Random article</span></a></li><li id="n-aboutsi[64355688342] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=11268 len=32768 cached=11268 start=0 eof=false
ps/en.wiki[66037418865] [[32mINFO [0m] [http] [CPU1] http: background task: read 15204 bytes from TLS
[66048391629] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 15204 bytes to foreground
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

			<navp class="vector-user-links vector-user-links-wide" aria-label="Personal tools">
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
						
<div id="vector-page-titlebar-toc" class="vector-dropdown vector-page-titlebar-toc vector-button-flush-le[66249668100] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=26472 len=32768 cached=26472 start=0 eof=false
edi[66733883733] [[32mINFO [0m] [http] [CPU1] http: background task: read 6296 bytes from TLS
[66738784398] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 6296 bytes to foreground
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
				
				<li class="interlanguage-link interwiki-ar mw-list-item"><a href="https://ar.wikipedia.org/wiki/%D8%B2%D8%BA%D8%A8%D9%8A%D8%A9" title="��غب��ة ��� Arabic" lang="ar" hreflang="ar" data-title="زغ��ية" data-language-autonym="العربية" data-language-local-name="Arabic" class="interlanguage-link-target"><span>العرب����</span></a></li><li class="interlanguage-link interwiki-arz mw-list-item"><a href="https://arz.wikipedia.org/wiki/%D8%B2%D8%BA%D8%A8%D9%8A%D9%87" title="زغ��ي�� – Egyptian Arabic" lang="arz" hreflang="arz" data-title="��غ��ي��" data-language-autonym="مصر��" data-language-local-name="Egyptian Arabic" class="interlanguage-link-target"><span>مصر��</span></a></li><li class="interlanguage-link inaterwiki-ast mw-list-item"><a href="https://ast.wikipedia.org/wiki/Gliridae" title="Gliridae – Asturian" lang="ast" hreflang="ast" data-title="Gliridae" data-language-autonym="Asturianu" data-language-local-name="Asturian" class="interlanguage-link-target"><span>Asturianu</span></a></li><li class="interlanguage-link interwiki-avk mw-list-item"><a href="https://avk.wikipedia.org/wiki/Aspakol_(Gliridae)" title="Aspakol (Gliridae) �� Kotava" lang="avk" hreflang="avk" data-title="Aspakol (Gliridae)" data-language-autonym="Kotava" data-language-local-name="Kotava" class="interlanguage-link-target"><span>Kotava</span></a></li><li class="interlanguage-link interwiki-az mw-list-item"><a href="https://az.wikipedia.org/wiki/S%C3%BCleysinl%C9%99r" title="Süleysinlər – Azerbaijani" lang="az" hreflang="az" data-title="S��leysinl��r" data-language-autonym="Az��rbaycanca" data-language-local-name="Azerbaijani" class="interlanguage-link-target"><span>Az��rbaycanca</span></a></li><li class="interlanguage-link interwiki-ba mw-list-item"><a href="https://ba.wikipedia.org/wiki/%D0%99%D0%BE%D2%A1%D0%BB%D0%B0%D1%81%D1%82%D0%B0%D1%80" title="Йо��л��с��ар ��� Bashkir" lang="ba" hreflang="ba" data-title="Йоҡл��стар" data-language-autonym="��а��ҡортса" data-language-local-name="Bashkir" class="interlanguage-link-target"><span>Башҡ��рт��а</span></a></li><li class="interlanguage-link interwiki-be-x-old mw-list-item"><a href="https://be-tarask.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%B5%D0%B2%D1%8B%D1%8F" title="Соневыя �� Belarusian (Taraškievica orthography)" lang="be-tarask" hreflang="be-tarask" data-title="Со��евыя" data-language-autonym="Б��ларуская (та��а��кевіц��)" data-language-local-name="Belarusian (Taraškievica orthography)" class="interlanguage-link-target"><span>Б��ла��у��к��я (та��а��к����іца)</span></a></li><li class="interlanguage-link interwiki-be mw-list-item"><a href="https://be.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%B5%D0%B2%D1%8B%D1%8F" title="��о��ев��я ��� Belarusian" lang="be" hreflang="be" data-title="С��невыя" data-language-autonym="��е��ар��ск��я" data-language-local-name="Belarusian" class="interlanguage-link-target"><span>��еларус��ая</span></a></li><li class="interlanguage-link interwiki-bg mw-list-item"><a href="https://bg.wikipedia.org/wiki/%D0%A1%D1%8A%D0%BD%D0%BB%D0%B8%D0%B2%D1%86%D0%BE%D0%B2%D0%B8" title="��ън����вц��ви ��� Bulgarian" lang="bg" hreflang="bg" data-title="С��н��и����о��и" data-language-autonym="Бъл��а��с��и" data-language-local-name="Bulgarian" class="interlanguage-link-target"><span>Бълг��р��к��</span></a></li><li class="interlanguage-link interwiki-br mw-list-item"><a href="https://br.wikipedia.org/wiki/Glirideged" title="Glirideged ��� Breton" lang="br" hreflang="br" data-title="Glirideged" data-language-autonym="Brezhoneg" data-language-local-name="Breton" class="interlanguage-link-target"><span>Brezhoneg</span></a></li><li class="interlanguage-link interwiki-ca mw-list-item"><a href="https://ca.wikipedia.org/wiki/Lirons" title="Lirons – Catalan" lang="ca" hreflang="ca" data-title="Lirons" data-language-autonym="Català" data-language-local-name="Catalan" class="interlanguage-link-target"><span>Catal��</span></a></li><li class="interlanguage-link interwiki-ceb mw-list-item"><a href="https://ceb.wikipedia.org/wiki/Gliridae" title="Gliridae ��� Cebuano" lang="ceb" hreflang="ceb" data-title="Gliridae" data-language-autonym="Cebuano" data-language-local-name="Cebuano" class="interlanguage-link-target">[66832402659] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=32768 len=32768 cached=32768 start=0 eof=false
.org/wiki/D[68733081912] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[68743323231] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
<span>Cebuano</span></a></li><li class="interlanguage-link interwiki-co mw-list-item"><a href="https://co.wikipedia.org/wiki/Gliridae" title="Gliridae – Corsican" lang="co" hreflang="co" data-title="Gliridae" data-language-autonym="Corsu" data-language-local-name="Corsican" class="interlanguage-link-target"><span>Corsu</span></a></li><li class="interlanguage-link interwiki-cs mw-list-item"><a href="https://cs.wikipedia.org/wiki/Plchovit%C3%AD" title="Plchovit�� ��� Czech" lang="cs" hreflang="cs" data-title="Plchovití" data-language-autonym="Čeština" data-language-local-name="Czech" class="interlanguage-link-target"><span>Če��tina</span></a></li><li class="interlanguage-link interwiki-da mw-list-item"><a href="https://da.wikipedia.org/wiki/Syvsovere" title="Syvsovere ��� Danish" lang="da" hreflang="da" data-title="Syvsovere" data-language-autonym="Dansk" data-language-local-name="Danish" class="interlanguage-link-target"><span>Dansk</span></a></li><li class="interlanguage-link interwiki-de mw-list-item"><a href="https://de.wikipedia.org/wiki/Bilche" title="Bilche – German" lang="de" hreflang="de" data-title="Bilche" data-language-autonym="Deutsch" data-language-local-name="German" class="interlanguage-link-target"><span>Deutsch</span></a></li><li class="interlanguage-link interwiki-eo mw-list-item"><a href="https://eo.wikipedia.org/wiki/Gliredoj" title="Gliredoj �� Esperanto" lang="eo" hreflang="eo" data-title="Gliredoj" data-language-autonym="Esperanto" data-language-local-name="Esperanto" class="interlanguage-link-target"><span>Esperanto</span></a></li><li class="interlanguage-link interwiki-es mw-list-item"><a href="https://es.wikipedia.org/wiki/Gliridae" title="Gliridae – Spanish" lang="es" hreflang="es" data-title="Gliridae" data-language-autonym="Español" data-language-local-name="Spanish" class="interlanguage-link-target"><span>Espa��ol</span></a></li><li class="interlanguage-link interwiki-et mw-list-item"><a href="https://et.wikipedia.org/wiki/Unilased" title="Unilased – Estonian" lang="et" hreflang="et" data-title="Unilased" data-language-autonym="Eesti" data-language-local-name="Estonian" class="interlanguage-link-target"><span>Eesti</span></a></li><li class="interlanguage-link interwiki-eu mw-list-item"><a href="https://eu.wikipedia.org/wiki/Muxar" title="Muxar – Basque" lang="eu" hreflang="eu" data-title="Muxar" data-language-autonym="Euskara" data-language-local-name="Basque" class="interlanguage-link-target"><span>Euskara</span></a></li><li class="interlanguage-link interwiki-fa mw-list-item"><a href="https://fa.wikipedia.org/wiki/%D9%85%D9%88%D8%B4_%D8%B2%D9%85%D8%B3%D8%AA%D8%A7%D9%86%E2%80%8C%D8%AE%D9%88%D8%A7%D8%A8" title="موش زم��تان��خو��ب – Persian" lang="fa" hreflang="fa" data-title="م��ش ��مس��ان���خواب" data-language-autonym="ف��رسی" data-language-local-name="Persian" class="interlanguage-link-target"><span>فارسی</span></a></li><li class="interlanguage-link interwiki-fi mw-list-item"><a href="https://fi.wikipedia.org/wiki/Unikeot" title="Unikeot – Finnish" lang="fi" hreflang="fi" data-title="Unikeot" data-language-autonym="Suomi" data-language-local-name="Finnish" class="interlanguage-link-target"><span>Suomi</span></a></li><li class="interlanguage-link interwiki-fr mw-list-item"><a href="https://fr.wikipedia.org/wiki/Gliridae" title="Gliridae ��� French" lang="fr" hreflang="fr" data-title="Gliridae" data-language-autonym="Français" data-language-local-name="French" class="interlanguage-link-target"><span>Français</span></a></li><li class="inteorlanguage-link interwiki-frr mw-list-item"><a href="https://frr.wikipedia.org/wiki/Sliapm%C3%BCsen" title="Sliapmüsen – Northern Frisian" lang="frr" hreflang="frr" data-title="Sliapm��sen" data-language-autonym="Nordfriisk" data-language-local-name="Northern Frisian" class="interlanguage-link-target"><span>Nordfriisk</span></a></li><li class="interlanguage-link interwiki-fy mw-list-item"><a href="https://fy.wikipedia.org/wiki/Sliepm%C3%BBzen" title="Sliepmûzen – Western Frisian" lang="fy" hreflang="fy" data-title="Sliepmûzen" data-language-autonym="Frysk" data-language-local-name="Western Frisian" class="interlanguage-link-target"><span>Frysk</span></a></li><li class="interlanguage-link interwiki-ga mw-list-item"><a href="https://ga.wikipedia.org/wiki/Codlam%C3%A1n" title="Codlamán ��� Irish" lang="ga" hreflang="ga" data-title="Codlam��n" data-language-autonym="Gaeilge" data-language-local-name="Irish" class="interlanguage-link-target"><span>Gaeilge</span></a></li><li class="interlanguage-link interwiki-gl mw-list-item"><a href="https://gl.wikipedia.org/wiki/Gl%C3%ADridos" title="Glíridos ��� Galician" lang="gl" hreflang="gl" data-title="Gl��ridos" data-language-autonym="Galego" data-language-local-name="Galician" class="interlanguage-link-target"><span>Galego</span></a></li><li class="interlanguage-link interwiki-he mw-list-item"><a href="https://he.wikipedia.org/wiki/%D7%A0%D7%9E%D7%A0%D7%9E%D7%A0%D7%99%D7%99%D7%9D" title="נמנ��נ��י�� �� Hebrew" lang="he" hreflang="he" data-title="������מני��ם" data-language-autonym="��ברית" data-language-local-name="Hebrew" class="interlanguage-link-target"><span>����������</span></a></li><li class="interlanguage-link interwiki-hu mw-list-item"><a href="https://hu.wikipedia.org/wiki/Pelef%C3%A9l%C3%A9k" title="Pelefélék �� Hungarian" lang="hu" hreflang="hu" data-title="Pelef��l��k" data-language-autonym="Magyar" data-language-local-name="Hungarian" class="interlanguage-link-target"><span>Magyar</span></a></li><li class="interlanguage-link interwiki-id mw-list-item"><a href="https://id.wikipedia.org/wiki/Tikus_penidur" title="Tikus penidur ��� Indonesian" lang="id" hreflang="id" data-title="Tikus penidur" data-language-autonym="Bahasa Indonesia" data-language-local-name="Indonesian" class="interlanguage-link-target"><span>Bahasa Indonesia</span></a></li><li class="interlanguage-link interwiki-inh mw-list-item"><a href="https://inh.wikipedia.org/wiki/%D0%A2%D0%B0%D1%80%D1%81%D0%B0%D0%BB%D0%B0%D1%88" title="Т��рсалаш – Ingush" lang="inh" hreflang="inh" data-title="Т��рсал��ш" data-language-autonym="ГӀалгӀ��й" data-language-local-name="Ingush" class="interlanguage-link-target"><span>Г��а��гӀай</span></a></li><li class="interlanguage-link interwiki-it mw-list-item"><a href="https://it.wikipedia.org/wiki/Gliridae" title="Gliridae �� Italian" lang="it" hreflang="it" data-title="Gliridae" data-language-autonym="Italiano" data-language-local-name="Italian" class="interlanguage-link-target"><span>Italiano</span></a></li><li class="interlanguage-link interwiki-ja mw-list-item"><a href="https://ja.wikipedia.org/wiki/%E3%83%A4%E3%83%9E%E3%83%8D%E7%A7%91" title="ヤ�������� �� Japanese" lang="ja" hreflang="ja" data-title="���������" data-language-autonym="日������" data-language-local-name="Japanese" class="interlanguage-link-target"><span>日本語</span></a></li><li class="interlanguage-link interwiki-ka mw-list-item"><a href="https://ka.wikipedia.org/wiki/%E1%83%AB%E1%83%98%E1%83%9A%E1%83%92%E1%83%A3%E1%83%93%E1%83%90%E1%83%A1%E1%83%94%E1%83%91%E1%83%A0%E1%83%9C%E1%83%98" title="ძ���ლ���უ���ა���ე���რნ�� ��� Georgian" lang="ka" hreflang="ka" data-title="�����ლ���უდ�����ე��რ���ი" data-language-autonym="���ა���თ���ლი" data-language-local-name="Georgian" class="interlanguage-link-target"><span>ქ�����თ��ლ���</span></a></li><li class="interlanguage-link interwiki-kab mw-list-item"><a href="https://kab.wikipedia.org/wiki/Acebcal" title="Acebcal – Kabyle" lang="kab" hreflang="kab" data-title="Acebcal" data-language-autonym="Taqbaylit" data-language-local-name="Kabyle" class="interlanguage-link-target"><span>Taqbaylit</span></a></li><li class="interlanguage-link interwiki-kk mw-list-item"><a href="https://kk.wikipedia.org/wiki/%D2%9A%D0%B0%D1%80%D0%B0%D2%9B%D0%B0%D1%81_%D1%82%D2%B1%D2%9B%D1%8B%D0%BC%D0%B4%D0%B0%D1%81%D1%8B" title="Қар��қас ��ұ��ы��д��с�� �� Kazakh" lang="kk" hreflang="kk" data-title="Қарақа�� т��қы��д��сы" data-language-autonym="Қазақш��" data-language-local-name="Kazakh" class="interlanguage-link-target"><span>��азақша</span></a></li><li class="interlanguage-link interwiki-ko mw-list-item"><a href="https://ko.wikipedia.org/wiki/%EA%B2%A8%EC%9A%B8%EC%9E%A0%EC%A5%90%EB%A5%98" title="���울���쥐�� ��� Korean" lang="ko" hreflang="ko" data-title="���울잠��류" data-language-autonym="��국���" data-language-local-name="Korean" class="interlanguage-link-target"><span>��국어</span></a></li><li class="interlanguage-link interwiki-la mw-list-item"><a href="https://la.wikipedia.org/wiki/Gliridae" title="Gliridae – Latin" lang="la" hreflang="la" data-title="Gliridae" data-language-autonym="Latina" data-language-local-name="Latin" class="interlanguage-link-target"><span>Latina</span></a></li><li class="interlanguage-link interwiki-lb mw-list-item"><a href="https://lb.wikipedia.org/wiki/Schl%C3%A9ifer" title="Schléifer – Luxembourgish" lang="lb" hreflang="lb" data-title="Schléifer" data-language-autonym="Lëtzebuergesch" data-language-local-name="Luxembourgish" class="interlanguage-link-target"><span>Lëtzebuergesch</span></a></li><li class="interlanguage-link interwiki-lfn mw-list-item"><a href="https://lfn.wikipedia.org/wiki/Liron" title="Liron �� Lingua Franca Nova" lang="lfn" hreflang="lfn" data-title="Liron" data-language-autonym="Lingua Franca Nova" data-language-local-name="Lingua Franca Nova" class="interlanguage-link-target"><span>Lingua Franca Nova</span></a></li><li class="interlanguage-link interwiki-lt mw-list-item"><a href="https://lt.wikipedia.org/wiki/Miegapeliniai" title="Miegapeliniai – Lithuanian" lang="lt" hreflang="lt" data-title="Miegapeliniai" data-language-autonym="Lietuvių" data-language-local-name="Lithuanian" class="interlanguage-link-target"><span>Lietuvių</span></a></li><li class="interlanguage-link interwiki-lv mw-list-item"><a href="https://lv.wikipedia.org/wiki/Susuri" title="Susuri – Latvian" lang="lv" hreflang="lv" data-title="Susuri" data-language-autonym="Latviešu" data-language-local-name="Latvian" class="interlanguage-link-target"><span>Latviešu</span></a></li><li class="interlanguage-link interwiki-mk mw-list-item"><a href="https://mk.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%BB%D0%B8%D0%B2%D1%86%D0%B8" title="С��нл��вц�� – Macedonian" lang="mk" hreflang="mk" data-title="��он��ивци" data-language-autonym="М��кедон��ки" data-language-local-name="Macedonian" class="interlanguage-link-target"><span>Македо��ски</span></a></li><li class="interlanguage-link interwiki-mrj mw-list-item"><a href="https://mrj.wikipedia.org/wiki/%D0%A3%D1%80%D0%B3%D0%B0%D0%BB%D1%8F_%D0%B9%D0%B8%D1%88%D0%B2%D0%BB%D3%93" title="У��галя ��ишвлӓ – Western Mari" lang="mrj" hreflang="mrj" data-title="��ргал�� йишвлӓ" data-language-autonym="К��рык м��ры" data-language-local-name="Western Mari" class="interlanguage-link-target"><span>К��рык мары</span></a></li><li class="interlanguage-link interwiki-ms mw-list-item"><a href="https://ms.wikipedia.org/wiki/Tikus_tidur" title="Tikus tidur – Malay" lang="ms" hreflang="ms" data-title="Tikus tidur" data-language-autonym="Bahasa Melayu" data-language-local-name="Malay" class="interlanguage-link-target"><span>Bahasa Melayu</span></a></li><li class="interlanguage-link interwiki-nds mw-list-item"><a href="https://nds.wikipedia.org/wiki/Slaapm%C3%BC%C3%BCs" title="Slaapmüüs �� Low German" lang="nds" hreflang="nds" data-title="Slaapmüüs" data-language-autonym="Plattdü��tsch" data-language-local-name="Low German" class="interlanguage-link-target"><span>Plattdüütsch</span></a></li><li class="interlanguage-link interwiki-nl mw-list-item"><a href="https://nl.wikipedia.org/wiki/Slaapmuizen" title="Slaapmuizen – Dutch" lang="nl" hreflang="nl" data-title="Slaapmuizen" data-language-autonym="Nederlands" data-language-local-name="Dutch" class="interlanguage-link-target"><span>Nederlands</span></a></li><li class="interlanguage-link interwiki-no mw-list-item"><a href="https://no.wikipedia.org/wiki/Syvsovere" title="Syvsovere ��� Norwegian Bokmål" lang="nb" hreflang="nb" data-title="Syvsovere" data-language-autonym="Norsk bokm��l" data-language-local-name="Norwegian Bokmål" class="interlanguage-link-target"><span>Norsk bokm��l</span></a></li><li class="interlanguage-link interwiki-nv mw-list-item"><a href="https://nv.wikipedia.org/wiki/Y%C3%A9igo_a%C5%82hoshii" title="Y��igo a��hoshii �� Navajo" lang="nv" hreflang="nv" data-title="Yéigo ałhoshii" data-language-autonym="Din�� bizaad" data-language-local-name="Navajo" class="interlanguage-link-target"><span>Din�� bizaad</span></a></li><li class="interlanguage-link interwiki-pl mw-list-item"><a href="https://pl.wikipedia.org/wiki/Popielicowate" title="Popielicowate ��� Polish" lang="pl" hreflang="pl" data-title="Popielicowate" data-language-autonym="Polski" data-language-local-name="Polish" class="interlanguage-link-target"><span>Polski</span></a></li><li class="interlanguage-link interwiki-pt mw-list-item"><a href="https://pt.wikipedia.org/wiki/Gliridae" title="Gliridae – Portuguese" lang="pt" hreflang="pt" data-title="Gliridae" data-language-autonym="Português" data-language-local-name="Portuguese" class="interlanguage-link-target"><span>Portugu��s</span></a></li><li class="interlanguage-link interwiki-ro mw-list-item"><a href="https://ro.wikipedia.org/wiki/P%C3%A2r%C8%99" title="Pârș – Romanian" lang="ro" hreflang="ro" data-title="Pârș" data-language-autonym="Română" data-language-local-name="Romanian" class="interlanguage-link-target"><span>Română</span></a></li><li class="interlanguage-link interwiki-ru mw-list-item"><a href="https://ru.wikipedia.org/wiki/%D0%A1%D0%BE%D0%BD%D0%B5%D0%B2%D1%8B%D0%B5" title="��о����в��е – Russian" lang="ru" hreflang="ru" data-title="��он��вые" data-language-autonym="��ус��кий" data-language-local-name="Russian" class="interlanguage-link-target"><span>Р��с��кий</span></a></li><li class="interlanguage-link interwiki-simple mw-list-item"><a href="https://simple.wikipedia.org/wiki/Dormouse" title="Dormouse �� Simple English" lang="en-simple" hreflang="en-simple" data-title="Dormouse" data-language-autonym="Simple English" data-language-local-name="Simple English" class="interlanguage-link-target"><span>Simple English</span></a></li><li class="interlanguage-link interwiki-sl mw-list-item"><a href="https://sl.wikipedia.org/wiki/Polhi" title="Polhi �� Slovenian" lang="sl" hreflang="sl" data-title="Polhi" data-language-autonym="Slovenščina" data-language-local-name="Slovenian" class="interlanguage-link-target"><span>Sloven��čina</span></a></li><li class="interlanguage-link interwiki-sr mw-list-item"><a href="https://sr.wikipedia.org/wiki/%D0%9F%D1%83%D1%85%D0%BE%D0%B2%D0%B8" title="П��хови – Serbian" lang="sr" hreflang="sr" data-title="П��хови" data-language-autonym="С��пс��и / srpski" data-language-local-name="Serbian" class="interlanguage-link-target"><span>Српск�� / srpski</span></a></li><li class="interlanguage-link interwiki-sv badge-Q17559452 badge-recommendedarticle mw-list-item" title="recommended article"><a href="https://sv.wikipedia.org/wiki/Sovm%C3%B6ss" title="Sovm��ss – Swedish" lang="sv" hreflang="sv" data-title="Sovmöss" data-language-autonym="Svenska" data-language-local-name="Swedish" class="interlanguage-link-target"><span>Svenska</span></a></li><li class="interlanguage-link interwiki-sw mw-list-item"><a href="https://sw.wikipedia.org/wiki/Panya-miti" title="Panya-miti – Swahili" lang="sw" hreflang="sw" data-title="Panya-miti" data-language-autonym="Kiswahili" data-language-local-name="Swahili" class="interlanguage-link-target"><span>Kiswahili</span></a></li><li class="interlanguage-link interwiki-tl badge-Q70893996 mw-list-item" title=""><a href="https://tl.wikipedia.org/wiki/Gliridae" title="Gliridae – Tagalog" lang="tl" hreflang="tl" data-title="Gliridae" data-language-autonym="Tagalog" data-language-local-name="Tagalog" class="interlanguage-link-target"><span>Tagalog</span></a></li><li class="interlanguage-link interwiki-tr mw-list-item"><a href="https://tr.wikipedia.org/wiki/Gliridae" title="Gli[68957193921] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=49152 len=32768 cached=49152 start=0 eof=false
rmouse
[70466107206] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[70475996580] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
ridae – Turkish" lang="tr" hreflang="tr" data-title="Gliridae" data-language-autonym="T��rkçe" data-language-local-name="Turkish" class="interlanguage-link-target"><span>Türkçe</span></a></li><li class="interlanguage-link interwiki-tt mw-list-item"><a href="https://tt.wikipedia.org/wiki/%D0%99%D0%BE%D0%BA%D0%BB%D0%B0%D1%87%D0%BB%D0%B0%D1%80" title="Йоклачлар – Tatar" lang="tt" hreflang="tt" data-title="Йо��ла��ла��" data-language-autonym="Т��т��р��а / tatarça" data-language-local-name="Tatar" class="interlanguage-link-target"><span>Тат��рча / tatarça</span></a></li><li class="interlanguage-link interwiki-uk mw-list-item"><a href="https://uk.wikipedia.org/wiki/%D0%92%D0%BE%D0%B2%D1%87%D0%BA%D0%BE%D0%B2%D1%96" title="Вовч��ові – Ukrainian" lang="uk" hreflang="uk" data-title="Вов��к��ві" data-language-autonym="У��раїнськ��" data-language-local-name="Ukrainian" class="interlanguage-link-target"><span>Українська</span></a></li><li class="interlanguage-link interwiki-uz mw-list-item"><a href="https://uz.wikipedia.org/wiki/Olmaxon" title="Olmaxon �� Uzbek" lang="uz" hreflang="uz" data-title="Olmaxon" data-language-autonym="O��zbekcha / ўз��е��ч��" data-language-local-name="Uzbek" class="interlanguage-link-target"><span>Oʻzbekcha / ўзбекча</span></a></li><li class="interlanguage-link interwiki-vi mw-list-item"><a href="https://vi.wikipedia.org/wiki/H%E1%BB%8D_Chu%E1%BB%99t_s%C3%B3c" title="Họ Chuột sóc – Vietnamese" lang="vi" hreflang="vi" data-title="H��� Chuột sóc" data-language-autonym="Tiếng Vi���t" data-language-local-name="Vietnamese" class="interlanguage-link-target"><span>Tiếng Việt</span></a></li><li class="interlanguage-link interwiki-vls mw-list-item"><a href="https://vls.wikipedia.org/wiki/Slapmuyzn" title="Slapmuyzn ��� West Flemish" lang="vls" hreflang="vls" data-title="Slapmuyzn" data-language-autonym="West-Vlams" data-language-local-name="West Flemish" class="interlanguage-link-target"><span>West-Vlams</span></a></li><li class="interlanguage-link interwiki-wa mw-list-item"><a href="https://wa.wikipedia.org/wiki/Sodoirmant" title="Sodoirmant – Walloon" lang="wa" hreflang="wa" data-title="Sodoirmant" data-language-autonym="Walon" data-language-local-name="Walloon" class="interlanguage-link-target"><span>Walon</span></a></li><li class="interlanguage-link interwiki-war mw-list-item"><a href="https://war.wikipedia.org/wiki/Gliridae" title="Gliridae ��� Waray" lang="war" hreflang="war" data-title="Gliridae" data-language-autonym="Winaray" data-language-local-name="Waray" class="interlanguage-link-target"><span>Winaray</span></a></li><li class="interlanguage-link interwiki-wuu mw-list-item"><a href="https://wuu.wikipedia.org/wiki/%E7%9D%A1%E9%BC%A0%E7%A7%91" title="��鼠科 – Wu" lang="wuu" hreflang="wuu" data-title="睡鼠科" data-language-autonym="���语" data-language-local-name="Wu" class="interlanguage-link-target"><span>��语</span></a></li><li class="interlanguage-link interwiki-zh-yue mw-list-item"><a href="https://zh-yue.wikipedia.org/wiki/%E7%9D%A1%E9%BC%A0%E7%A7%91" title="��鼠�� – Cantonese" lang="yue" hreflang="yue" data-title="睡��科" data-language-autonym="粵語" data-language-local-name="Cantonese" class="interlanguage-link-target"><span>粵語</span></a></li><li class="interlanguage-link interwiki-zh mw-list-item"><a href="https://zh.wikipedia.org/wiki/%E7%9D%A1%E9%BC%A0%E7%A7%91" title="���鼠科 ��� Chinese" lang="zh" hreflang="zh" data-title="睡��科" data-language-autonym="中文" data-language-local-name="Chinese" class="interlanguage-link-target"><span>中���</span></a></li>
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
<th colspan="2" style="color:inherit; text-align: center; background-color: rgb(235,235,210)">Dormice<br /><div style="font-size: 85%;">Temporal range: <span class="noprint"><span style="display:inline-block;"></span><span style="display:inline-block;">Early Eocene – Recent</span> <span style="display:inline-block;"></span><div id="Timeline-row" style="margin: 4px auto 0; clear:both; width:280px; padding:0px; height:18px; overflow:visible; white-space:nowrap; border:1px #666; border-style:solid none; position:relative; z-index:0; font-size:97%;">
<div style="position:absolute; height:100%; left:0px; width:47.901538461538px; text-align:center; color:inherit; background-color:rgb(254,217,106); background-image: linear-gradient(to right, rgba(255,255,255,1), rgba(254,217,106,1) 75%, rgba(254,217,106,1));"><a href="/wiki/Precambrian" title="Precambrian">PreꞒ</a></div>
<div style="position:absolute; height:100%; text-align:center; color:inherit; background-color:rgb(127,160,86); left:47.901538461538px; width:22.378461538462px"><a href="/wiki/Cambrian" title="Cambrian"><span style="color:white;">Ꞓ</span></a></div>
<div style="position:absolute; height:100%; t[70674437031] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=65536 len=32768 cached=65536 start=0 eof=false
[72291021726] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[72301142760] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
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
<li>���<i><a href="/wiki/Hypnomys" title="Hypnomys">Hypnomys</a></i></li>
<li>��<i><a href="/wiki/Leithia" title="Leithia">Leithia</a></i></li>
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
</p><p>The Latin noun <span title="Latin-language text"><i lang="la">glīs</i></span>, which is the origin of the scientific name, descends from the <a href="/wiki/Proto-Indo-European" class="mw-redirect" title="Proto-Indo-European">Proto-Indo-European</a> noun <i>*gl��h₁éys</i> <span class="gloss-quot">'</span><span class="gloss-text">weasel, mouse</span><span class="gloss-quot">'</span>, and is related to <a href="/wiki/Sanskrit" title="Sanskrit">Sanskrit</a> <span title="Sanskrit-language text"><span lang="sa">गिरि</span></span> (<span title="Sanskrit-language romanization"><i lang="sa-Latn">gir��</i></span>) <span class="gloss-quot">'</span><span class="gloss-text">mouse</span><span class="gloss-quot">'</span> and <a href="/wiki/Ancient_Greek" title="Ancient Greek">Ancient Greek</a> <span title="Ancient Greek (to 1453)-language text"><span lang="grc">γαλέη</span></span> (<span title="Ancient Greek (to 1453)-language romanization"><i lang="grc-Latn">galé��</i></span>) <span class="gloss-quot">'</span><span class="gloss-text">weasel</span><span class="gloss-quot">'</span>.
</p>
<div class="mw-heading mw-heading2"><h2 id="Characteristics">Characteristics</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=2" title="Edit section: Characteristics"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<p>Dormice are small rodents, with body lengths between 6 and 19&#160;cm (2.4 and 7.5&#160;in), and weight between 15 and 180&#160;g (0.53 and 6.35&#160;oz).<sup id="cite&#95;ref-6" class="reference"><a href="#cite_note-6"><span class="cite-bracket">&#91;</span>6<span class="cite-bracket">&#93;</span></a></sup> They are generally <a href="/wiki/Mouse" title="Mouse">mouse</a>-like in appearance, but with <a href="/wiki/Fur" title="Fur">furred</a> <a href="/wiki/Tail" title="Tail">tails</a>. They are largely <a href="/wiki/Arboreal" class="mw-redirect" title="Arboreal">arboreal</a>, agile, and well-adapted to climbing. Most species are <a href="/wiki/Nocturnal" class="mw-redirect" title="Nocturnal">nocturnal</a>. Dormice have an excellent sense of <a href="/wiki/Hearing_(sense)" class="mw-redirect" title="Hearing (sense)">hearing</a> and signal each other with a variety of vocalisations.<sup id="cite&#95;ref-EoM&#95;7-0" class="reference"><a href="#cite_note-EoM-7"><span class="cite-bracket">&#91;</span>7<span class="cite-bracket">&#93;</span></a></sup>
</p><p>Dormice are <a href="/wiki/Omnivore" title="Omnivore">omnivorous</a>, and typically feed on berries, flowers, fruits, insects, and nuts. They are unique among rodents in that they lack a <a href="/wiki/Cecum" title="Cecum">cecum</a>, a part of the gut used in other species to ferment vegetable matter. Their <a href="/wiki/Dentition" title="Dentition">dental formula</a> is similar to that of <a href="/wiki/Squirrel" title="Squirrel">squirrels</a>, although they often lack <a href="/wiki/Premolar" title="Premolar">premolars</a>:
</p>
<table class="wikitable" style="text-align: center">
<tbody><tr>
<th><a href="/wiki/Dentition" title="Dentition">Dentition</a>
</th></tr>
<tr>
<td>1.0.0��1.3
</td></tr>
<tr>
<td>1.0.0��1.3
</td></tr></tbody></table>
<p>Dormice breed once (or, occasionally, twice) each year, producing litters with an average of four young after a <a href="/wiki/Gestation" title="Gestation">gestation</a> period of 22��24 days. They can live for as long as five years. The young are born hairless and helpless, and their eyes do not open until about 18 days after birth. They typically become sexually mature after the end of their first hibernation. Dormice live in small family groups, with home ranges that vary widely between species and depend on the availability of food.<sup id="cite&#95;ref-EoM&#95;7-1" class="reference"><a href="#cite_note-EoM-7"><span class="cite-bracket">&#91;</span>7<span class="cite-bracket">&#93;</span></a></sup>
</p>
<div class="mw-heading mw-heading3"><h3 id="Hibernation">Hibernation</h3><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=3" title="Edit section: Hibernation"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span[72533346501] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=81920 len=32768 cached=81920 start=0 eof=false
[74328784101] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[74340465111] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
></div>
<figure class="mw-default-size" typeof="mw:File/Thumb"><a href="/wiki/File:Muscardinus_avellanarius_-_1700-1880_little_dormouse,_sleeping_in_the_winter_nest.jpg" class="mw-file-description"><img src="//upload.wikimedia.org/wikipedia/commons/thumb/5/51/Muscardinus_avellanarius_-_1700-1880_little_dormouse%2C_sleeping_in_the_winter_nest.jpg/250px-Muscardinus_avellanarius_-_1700-1880_little_dormouse%2C_sleeping_in_the_winter_nest.jpg" decoding="async" width="250" height="272" class="mw-file-element" srcset="//upload.wikimedia.org/wikipedia/commons/thumb/5/51/Muscardinus_avellanarius_-_1700-1880_little_dormouse%2C_sleeping_in_the_winter_nest.jpg/500px-Muscardinus_avellanarius_-_1700-1880_little_dormouse%2C_sleeping_in_the_winter_nest.jpg 2x" data-file-width="1012" data-file-height="1102" /></a><figcaption>The little dormouse, sleeping in the winter nest.</figcaption></figure>
<p>One of the most notable characteristics of those dormice that live in <a href="/wiki/Temperate" class="mw-redirect" title="Temperate">temperate</a> zones is hibernation. They can hibernate six months out of the year, or even longer if the weather does not become warm enough, sometimes waking for brief periods to eat food they had previously stored nearby. During the summer, they accumulate fat in their bodies to nourish them through the hibernation period.<sup id="cite&#95;ref-EoM&#95;7-2" class="reference"><a href="#cite_note-EoM-7"><span class="cite-bracket">&#91;</span>7<span class="cite-bracket">&#93;</span></a></sup>
</p>
<div class="mw-heading mw-heading2"><h2 id="Relationship_with_humans">Relationship with humans</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=4" title="Edit section: Relationship with humans"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<p>The <a href="/wiki/European_edible_dormouse" title="European edible dormouse">European edible dormouse</a> (<i>Glis glis</i>) was considered a <a href="/wiki/Ancient_Roman_cuisine" title="Ancient Roman cuisine">delicacy in ancient Rome</a>, either as a savoury appetizer or as a dessert (dipped in honey and poppy seeds). The Romans used a special kind of enclosure, a <a href="/wiki/Glirarium" title="Glirarium">glirarium</a>, to raise and fatten dormice for the table.<sup id="cite&#95;ref-EoM&#95;7-3" class="reference"><a href="#cite_note-EoM-7"><span class="cite-bracket">&#91;</span>7<span class="cite-bracket">&#93;</span></a></sup> It is still considered a delicacy in <a href="/wiki/Slovenia" title="Slovenia">Slovenia</a> and in several places in <a href="/wiki/Croatia" title="Croatia">Croatia</a>, namely <a href="/wiki/Lika" title="Lika">Lika</a>, and the islands of <a href="/wiki/Hvar" title="Hvar">Hvar</a> and <a href="/wiki/Bra%C4%8D" title="Bra��">Brač</a>.<sup id="cite&#95;ref-8" class="reference"><a href="#cite_note-8"><span class="cite-bracket">&#91;</span>8<span class="cite-bracket">&#93;</span></a></sup><sup id="cite&#95;ref-9" class="reference"><a href="#cite_note-9"><span class="cite-bracket">&#91;</span>9<span class="cite-bracket">&#93;</span></a></sup> Dormouse fat was believed by the <a href="/wiki/Elizabethan_era" title="Elizabethan era">Elizabethans</a> to induce sleep since the animal put on fat before hibernating.<sup id="cite&#95;ref-10" class="reference"><a href="#cite_note-10"><span class="cite-bracket">&#91;</span>10<span class="cite-bracket">&#93;</span></a></sup>
</p><p>In more recent years,<sup id="cite&#95;ref-11" class="reference"><a href="#cite_note-11"><span class="cite-bracket">&#91;</span>11<span class="cite-bracket">&#93;</span></a></sup> dormice have begun to enter the pet trade; however, they are uncommon as pets and are considered an <a href="/wiki/Exotic_pet" title="Exotic pet">exotic pet</a>. The <a href="/wiki/Woodland_dormouse" title="Woodland dormouse">woodland dormouse</a> (<i>Graphiurus murinus)</i> is the most commonly seen species in the pet trade.<sup id="cite&#95;ref-12" class="reference"><a href="#cite_note-12"><span class="cite-bracket">&#91;</span>12<span class="cite-bracket">&#93;</span></a></sup> <a href="/wiki/Asian_garden_dormouse" title="Asian garden dormouse">Asian garden dormice</a> (<i>Eliomys melanurus</i>) are also occasionally kept as pets.<sup id="cite&#95;ref-13" class="reference"><a href="#cite_note-13"><span class="cite-bracket">&#91;</span>13<span class="cite-bracket">&#93;</span></a></sup>
</p>
<div class="mw-heading mw-heading2"><h2 id="Evolution">Evolution</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=5" title="Edit section: Evolution"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<p>Dormice likely originated in Europe, with the earliest dormouse genus <i><a href="/wiki/Eogliravus" title="Eogliravus">Eogliravus</a></i> being known from the Early Eocene (around 48-41 million years ago) of France. Dormice were relatively uniform in the Eocene but considerably diversified during the <a href="/wiki/Oligocene" title="Oligocene">Oligocene</a> (34-23 million years ago). Their ability to hibernate may have emerged during this period. They reached an apex of diversity during the late Early <a href="/wiki/Miocene" title="Miocene">Miocene</a> (around 17 million years ago<sup id="cite&#95;ref-Li-2023&#95;14-0" class="reference"><a href="#cite_note-Li-2023-14"><span class="cite-bracket">&#91;</span>14<span class="cite-bracket">&#93;</span></a></sup>) when there were 18 genera and 36 species of dormice in Europe alone during this period.<sup id="cite&#95;ref-Lu-2021&#95;3-1" class="reference"><a href="#cite_note-Lu-2021-3"><span class="cite-bracket">&#91;</span>3<span class="cite-bracket">&#93;</span></a></sup> During this time span, dormice represented the dominant group of rodents in Europe.<sup id="cite&#95;ref-Li-2023&#95;14-1" class="reference"><a href="#cite_note-Li-2023-14"><span class="cite-bracket">&#91;</span>14<span class="cite-bracket">&#93;</span></a></sup>
</p><p>The earliest Asian dormice are known from the early Miocene, and the Miocene saw the emergence of several of the modern genera of living dormice. The diversity of dormice saw continual decline until the middle <a href="/wiki/Pliocene" title="Pliocene">Pliocene</a>, when there was again a period of speciation, mostly driven by the diversification of the African <i><a href="/wiki/Graphiurus" title="Graphiurus">Graphiurus</a></i>, which first appeared during the Pliocene, while the diversity of European dormice remained relatively low compared to their Miocene peak.<sup id="cite&#95;ref-Lu-2021&#95;3-2" class="reference"><a href="#cite_note-Lu-2021-3"><span class="cite-bracket">&#91;</span>3<span class="cite-bracket">&#93;</span></a></sup>
</p><p>Several dormouse lineages experienced <a href="/wiki/Island_gigantism" title="Island gigantism">insular gigantism</a> after being isolated on islands in the Mediterranean during the Pliocene and <a href="/wiki/Pleistocene" title="Pleistocene">Pleistocene</a>, the largest being the rabbit-sized <i><a href="/wiki/Leithia" title="Leithia">Leithia</a></i> of Sicily and Malta.<sup id="cite&#95;ref-Hennekam-2020&#95;15-0" class="reference"><a href="#cite_note-Hennekam-2020-15"><span class="cite-bracket">&#91;</span>15<span class="cite-bracket">&#93;</span></a></sup>
</p>
<div class="mw-heading mw-heading2"><h2 id="Classification">Classification</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=6" title="Edit section: Classification"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1320445320" /><div role="note" class="hatnote navigation-not-searchable">Further information: <a href="/wiki/List_of_glirids" title="List of glirids">List of glirids</a></div>
<p>The family consists of 29 extant species, in three subfamilies and (arguably) nine genera:
</p><p>
Cladogram of most living and recently extinct dormice genera based on <a href="/wiki/Mitochondrial_DNA" title="Mitochondrial DNA">mitochondrial DNA</a> after Petrova et al. 2024:<sup id="cite&#95;ref-16" class="reference"><a href="#cite_note-16"><span class="cite-bracket">&#91;</span>16<span class="cite-bracket">&#93;</span></a></sup></p><div class="clade"><style data-mw-deduplicate="TemplateStyles:r1344960044">body.skin-vector-2022 .mw-parser-output div.clade,body.skin-minerva .mw-parser-output div.clade{overflow-x:auto;overflow-y:hidden}body.skin-minerva .mw-parser-output div.clade p{font-size:inherit}.mw-parser-output table.clade{border-spacing:0;margin:0;font-size:100%;line-height:100%;border-collapse:separate;width:auto;display:table}.mw-parser-output table.clade table.clade{width:100%;line-height:inherit}.mw-parser-output table.clade td.clade-label{min-width:0.7em;width:0.7em;padding:0.1em 0.25em;vertical-align:bottom;text-align:center;border-left:1px solid;border-bottom:1px solid;white-space:nowrap}.mw-parser-output table.clade td.clade-label::before,.mw-parser-output table.clade td.clade-slabel::before{content:"\2060 "}.mw-parser-output table.clade td.clade-fixed-width{overflow:hidden;text-overflow:ellipsis}.mw-parser-output table.clade td.clade-fixed-width:hover{overflow:visible}.mw-parser-output table.clade td.clade-label.first{border-left:none;border-right:none}.mw-parser-output table.clade td.clade-label.reverse{border-left:none;border-right:1px solid}.mw-parser-output table.clade td.clade-slabel{padding:0.1em 0.25em;vertical-align:top;text-align:center;border-left:1px solid;white-space:nowrap}.mw-parser-output table.clade td.clade-slabel:hover{overflow:visible}.mw-parser-output table.clade td.clade-slabel.last{border-left:none;border-right:none}.mw-parser-output table.clade td.clade-slabel.reverse{border-left:none;border-right:1px solid}.mw-parser-output table.clade td.clade-bar{vertical-align:middle;text-align:left;padding:0 0.5em;position:relative}.mw-parser-output table.clade td.clade-bar.reverse{text-align:right;position:relative}.mw-parser-output table.clade td.clade-leaf{border:0;padding:0;text-align:left}.mw-parser-output table.clade td.clade-leaf p{padding-right:5px;padding-left:2px}.mw-parser-output table.clade td.clade-leafR{border:0;padding:0;text-align:right}.mw-parser-output table.clade td.clade-leafR p{padding-left:5px;padding-right:2px}.mw-parser-output table.clade td.clade-leaf.reverse{text-align:right}.mw-parser-output table.clade td.clade-leaf.reverse p{padding-left:5px;padding-right:2px}.mw-parser-output table.clade:hover span.linkA{background-color:yellow}.mw-parser-output table.clade:hover span.linkB{background-color:green}</style>
<table class="clade">


<tbody><tr>
<td class="clade-label first"><a href="/wiki/Gliridae" class="mw-redirect" title="Gliridae">Gliridae</a>&#160;(dormice)
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">Graphiurinae
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Graphiurus" title="Graphiurus">Graphiurus</a></i> (African dormice)
</p>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label"><a href="/wiki/Glirinae" title="Glirinae">Glirinae</a>
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Glirulus" title="Glirulus">Glirulus</a></i> (Japanese dormouse)
</p>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label">
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Glis_(genus)" title="Glis (genus)">Glis</a></i> (edible dormice)
</p>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label"><a href="/wiki/Leithiinae" title="Leithiinae">Leithiinae</a>
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Muscardinus" class="mw-redirect" title="Muscardinus">Muscardinus</a></i> (hazel dormouse)
</p>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label">
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Myomimus" title="Myomimus">Myomimus</a></i> (mouse-tailed dormice)
</p>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label">
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Selevinia" class="mw-redirect" title="Selevinia">Selevinia</a></i> (desert dormouse)
</p>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label">
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Dryomys" title="Dryomys">Dryomys</a></i> (woolly and forest dormice)
</p>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label">
</td>
<td rowspan="2" class="clade-leaf">
<div><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1344960044" />
<table class="clade">


<tbody><tr>
<td class="clade-label first">
</td>
<td rowspan="2" class="clade-leaf">
<p><i><a href="/wiki/Eliomys" title="Eliomys">Eliomys</a></i> (garden dormice)
</p>
</td></tr>
<tr>
<td class="clade-slabel">
</td></tr>
<tr>
<td class="clade-label">
</td>
<td rowspan="2" class="clade-leaf">
<p><abbr title="Extinct" aria-label="Extinct" style="border: none; text-decoration: none; cursor: inherit; font-weight: normal; font-style: normal;">†</abbr><i><a href="/wiki/Hypnomys" title="Hypnomys">Hypnomys</a></i> (Balearic dormice)
</p>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div>
</td></tr>
<tr>
<td class="clade-slabel last">
</td></tr></tbody></table></div><p><b>Family Gliridae</b> ��� Dormice
</p><ul><li><b>Subfamily <a href="/wiki/Glirinae" title="Glirinae">Glirinae</a></b>
<ul><li>Genus <i><a href="/wiki/Glirulus" title="Glirulus">Glirulus</a></i>
<ul><li><a href="/wiki/Japanese_dormouse" title="Japanese dormouse">Japanese dormouse</a>, <i>Glirulus japonicus</i></li></ul></li>
<li>Genus <i><a href="/wiki/Glis_(genus)" title="Glis (genus)">Glis</a></i>
<ul><li><a href="/wiki/European_edible_dormouse" title="European edible dormouse">European edible dormouse</a>, <i>Glis glis</i></li>
<li><a href="/wiki/Iranian_edible_dormouse" title="Iranian edible dormouse">Iranian edible dormouse</a>, <i>Glis persicus</i></li></ul></li></ul></li>
<li><b>Subfamily <a href="/wiki/Graphiurinae" class="mw-redirect" title="Graphiurinae">Graphiurinae</a></b>
<ul><li>Genus <i><a href="/wiki/Graphiurus" title="Graph[74551621617] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=98304 len=32768 cached=98304 start=0 eof=false
[76342101363] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[76352218305] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
iurus">Graphiurus</a></i>, African dormice
<ul><li><a href="/wiki/Angolan_African_dormouse" title="Angolan African dormouse">Angolan African dormouse</a>, <i>Graphiurus angolensis</i></li>
<li><a href="/wiki/Christy%27s_dormouse" title="Christy&#39;s dormouse">Christy's dormouse</a>, <i>Graphiurus christyi</i></li>
<li><a href="/wiki/Walter_Verheyen%27s_African_dormouse" title="Walter Verheyen&#39;s African dormouse">Walter Verheyen's African dormouse</a>, <i>Graphiurus walterverheyeni</i> <sup id="cite&#95;ref-17" class="reference"><a href="#cite_note-17"><span class="cite-bracket">&#91;</span>17<span class="cite-bracket">&#93;</span></a></sup></li>
<li><a href="/wiki/Jentink%27s_dormouse" title="Jentink&#39;s dormouse">Jentink's dormouse</a>, <i>Graphiurus crassicaudatus</i></li>
<li><a href="/wiki/Johnston%27s_African_dormouse" title="Johnston&#39;s African dormouse">Johnston's African dormouse</a>, <i>Graphiurus johnstoni</i></li>
<li><a href="/wiki/Kellen%27s_dormouse" title="Kellen&#39;s dormouse">Kellen's dormouse</a>, <i>Graphiurus kelleni</i></li>
<li><a href="/wiki/Lorrain_dormouse" title="Lorrain dormouse">Lorrain dormouse</a>, <i>Graphiurus lorraineus</i></li>
<li><a href="/wiki/Monard%27s_dormouse" title="Monard&#39;s dormouse">Monard's dormouse</a>, <i>Graphiurus monardi</i></li>
<li><a href="/wiki/Nagtglas%27s_African_dormouse" title="Nagtglas&#39;s African dormouse">Nagtglas's African dormouse</a>, <i>Graphiurus nagtglasii</i></li>
<li><a href="/wiki/Rock_dormouse" title="Rock dormouse">Rock dormouse</a>, <i>Graphiurus platyops</i></li>
<li><a href="/wiki/Silent_dormouse" title="Silent dormouse">Silent dormouse</a>, <i>Graphiurus surdus</i></li>
<li><a href="/wiki/Small-eared_dormouse" title="Small-eared dormouse">Small-eared dormouse</a>, <i>Graphiurus microtis</i></li>
<li><a href="/wiki/Spectacled_dormouse" title="Spectacled dormouse">Spectacled dormouse</a>, <i>Graphiurus ocularis</i></li>
<li><a href="/wiki/Stone_dormouse" title="Stone dormouse">Stone dormouse</a>, <i>Graphiurus rupicola</i></li>
<li><a href="/wiki/Woodland_dormouse" title="Woodland dormouse">Woodland dormouse</a>, <i>Graphiurus murinus</i></li></ul></li></ul></li>
<li><b>Subfamily <a href="/wiki/Leithiinae" title="Leithiinae">Leithiinae</a></b>
<ul><li>Genus <i><a href="/wiki/Chaetocauda" class="mw-redirect" title="Chaetocauda">Chaetocauda</a></i>
<ul><li><a href="/wiki/Chinese_dormouse" title="Chinese dormouse">Chinese dormouse</a>, <i>Chaetocauda sichuanensis</i></li></ul></li>
<li>Genus <i><a href="/wiki/Dryomys" title="Dryomys">Dryomys</a></i>
<ul><li><a href="/wiki/Balochistan_forest_dormouse" title="Balochistan forest dormouse">Balochistan forest dormouse</a>, <i>Dryomys niethammeri</i></li>
<li><a href="/wiki/Forest_dormouse" title="Forest dormouse">Forest dormouse</a>, <i>Dryomys nitedula</i></li>
<li><a href="/wiki/Woolly_dormouse" title="Woolly dormouse">Woolly dormouse</a>, <i>Dryomys laniger</i></li></ul></li>
<li>Genus <i><a href="/wiki/Eliomys" title="Eliomys">Eliomys</a></i>, garden dormice
<ul><li><a href="/wiki/Asian_garden_dormouse" title="Asian garden dormouse">Asian garden dormouse</a>, <i>Eliomys melanurus</i></li>
<li><a href="/wiki/Garden_dormouse" title="Garden dormouse">Garden dormouse</a>, <i>Eliomys quercinus</i><figure typeof="mw:File/Thumb"><a href="/wiki/File:Dormouse.jpeg" class="mw-file-description"><img src="//upload.wikimedia.org/wikipedia/commons/0/02/Dormouse.jpeg" decoding="async" width="200" height="178" class="mw-file-element" data-file-width="200" data-file-height="178" /></a><figcaption></figcaption></figure></li>
<li><a href="/wiki/Maghreb_garden_dormouse" title="Maghreb garden dormouse">Maghreb garden dormouse</a>, <i>Eliomys munbyanus</i></li></ul></li>
<li>Genus <i><a href="/wiki/Hypnomys" title="Hypnomys">Hypnomys</a></i>† (Balearic dormouse)
<ul><li><a href="/wiki/Majorcan_giant_dormouse" class="mw-redirect" title="Majorcan giant dormouse">Majorcan giant dormouse</a>, <i>Hypnomys morphaeus</i>��</li>
<li><a href="/wiki/Minorcan_giant_dormouse" class="mw-redirect" title="Minorcan giant dormouse">Minorcan giant dormouse</a>, <i>Hypnomys mahonensis</i>��</li></ul></li>
<li>Genus  <i><a href="/wiki/Leithia" title="Leithia">Leithia</a></i>��
<ul><li><i>Leithia cartei</i>��</li>
<li>Maltese giant dormouse, <i>Leithia melitensis</i>���</li></ul></li>
<li>Genus <i><a href="/wiki/Muscardinus" class="mw-redirect" title="Muscardinus">Muscardinus</a></i>
<ul><li><a href="/wiki/Hazel_dormouse" title="Hazel dormouse">Hazel dormouse</a>, <i>Muscardinus avellanarius</i></li></ul></li>
<li>Genus <i><a href="/wiki/Myomimus" title="Myomimus">Myomimus</a></i>, mouse-tailed dormice
<ul><li><a href="/wiki/Masked_mouse-tailed_dormouse" title="Masked mouse-tailed dormouse">Masked mouse-tailed dormouse</a>, <i>Myomimus personatus</i></li>
<li><a href="/wiki/Roach%27s_mouse-tailed_dormouse" title="Roach&#39;s mouse-tailed dormouse">Roach's mouse-tailed dormouse</a>, <i>Myomimus roachi</i></li>
<li><a href="/wiki/Setzer%27s_mouse-tailed_dormouse" title="Setzer&#39;s mouse-tailed dormouse">Setzer's mouse-tailed dormouse</a>, <i>Myomimus setzeri</i></li></ul></li>
<li>Genus <i><a href="/wiki/Selevinia" class="mw-redirect" title="Selevinia">Selevinia</a></i>
<ul><li><a href="/wiki/Desert_dormouse" title="Desert dormouse">Desert dormouse</a>, <i>Selevinia betpakdalaensis</i></li></ul></li>
<li class="mw-empty-elt"></li></ul></li></ul>
<p>† indicates an extinct species.
</p>
<div class="mw-heading mw-heading3"><h3 id="Fossil_genera">Fossil genera</h3><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=7" title="Edit section: Fossil genera"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<ul><li><i><a href="/wiki/Eogliravus" title="Eogliravus">Eogliravus</a></i> Hartenberger, 1971 - Eocene</li>
<li><i><a href="/w/index.php?title=Bransatoglis&amp;action=edit&amp;redlink=1" class="new" title="Bransatoglis (page does not exist)">Bransatoglis</a></i> Hugueney, 1967 - Oligocene<sup id="cite&#95;ref-Freudenthal2007&#95;18-0" class="reference"><a href="#cite_note-Freudenthal2007-18"><span class="cite-bracket">&#91;</span>18<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Butseloglis&amp;action=edit&amp;redlink=1" class="new" title="Butseloglis (page does not exist)">Butseloglis</a></i> Vianey-Liaud, 2003 - Oligocene<sup id="cite&#95;ref-Freudenthal2007&#95;18-1" class="reference"><a href="#cite_note-Freudenthal2007-18"><span class="cite-bracket">&#91;</span>18<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Glamys&amp;action=edit&amp;redlink=1" class="new" title="Glamys (page does not exist)">Glamys</a></i> Vianey-Liaud, 1989 - Oligocene</li>
<li><i><a href="/w/index.php?title=Moissenetia&amp;action=edit&amp;redlink=1" class="new" title="Moissenetia (page does not exist)">Moissenetia</a></i> Hugueney &amp; Adrover, 1995 - Oligocene<sup id="cite&#95;ref-Lu2021&#95;19-0" class="reference"><a href="#cite_note-Lu2021-19"><span class="cite-bracket">&#91;</span>19<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Oligodyromys&amp;action=edit&amp;redlink=1" class="new" title="Oligodyromys (page does not exist)">Oligodyromys</a></i> Bahlo, 1975 - Oligocene<sup id="cite&#95;ref-Freudenthal2007&#95;18-2" class="reference"><a href="#cite_note-Freudenthal2007-18"><span class="cite-bracket">&#91;</span>18<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Paraglis&amp;action=edit&amp;redlink=1" class="new" title="Paraglis (page does not exist)">Paraglis</a></i> Baudelot, 1970 - Oligocene<sup id="cite&#95;ref-Freudenthal2007&#95;18-3" class="reference"><a href="#cite_note-Freudenthal2007-18"><span class="cite-bracket">&#91;</span>18<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Glirudinus&amp;action=edit&amp;redlink=1" class="new" title="Glirudinus (page does not exist)">Glirudinus</a></i> de Bruijn, 1966 - Oligocene to Miocene<sup id="cite&#95;ref-Li2023&#95;20-0" class="reference"><a href="#cite_note-Li2023-20"><span class="cite-bracket">&#91;</span>20<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Microdyromys&amp;action=edit&amp;redlink=1" class="new" title="Microdyromys (page does not exist)">Microdyromys</a></i> de Bruijn, 1966 - Oligocene to Miocene<sup id="cite&#95;ref-GarciaParedes2010&#95;21-0" class="reference"><a href="#cite_note-GarciaParedes2010-21"><span class="cite-bracket">&#91;</span>21<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Peridyromys&amp;action=edit&amp;redlink=1" class="new" title="Peridyromys (page does not exist)">Peridyromys</a></i> Stehlin &amp; Schaub, 1951 - Oligocene to Miocene<sup id="cite&#95;ref-Dalmasso2022&#95;22-0" class="reference"><a href="#cite_note-Dalmasso2022-22"><span class="cite-bracket">&#91;</span>22<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Vasseuromys&amp;action=edit&amp;redlink=1" class="new" title="Vasseuromys (page does not exist)">Vasseuromys</a></i> Baudelot &amp; de Bonis, 1966 - Oligocene to Miocene<sup id="cite&#95;ref-Sinitsa2018&#95;23-0" class="reference"><a href="#cite_note-Sinitsa2018-23"><span class="cite-bracket">&#91;</span>23<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Carbomys&amp;action=edit&amp;redlink=1" class="new" title="Carbomys (page does not exist)">Carbomys</a></i> Mein &amp; Adrover, 1982 - Miocene<sup id="cite&#95;ref-Lu2021&#95;19-1" class="reference"><a href="#cite_note-Lu2021-19"><span class="cite-bracket">&#91;</span>19<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Graphiurops&amp;action=edit&amp;redlink=1" class="new" title="Graphiurops (page does not exist)">Graphiurops</a></i> Bachmayer &amp; Wilson, 1980 - Miocene<sup id="cite&#95;ref-Lu2021&#95;19-2" class="reference"><a href="#cite_note-Lu2021-19"><span class="cite-bracket">&#91;</span>19<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Miodyromys&amp;action=edit&amp;redlink=1" class="new" title="Miodyromys (page does not exist)">Miodyromys</a></i> Kretzoi, 1943 - Early Miocene<sup id="cite&#95;ref-Lu2021&#95;19-3" class="reference"><a href="#cite_note-Lu2021-19"><span class="cite-bracket">&#91;</span>19<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Praearmantomys&amp;action=edit&amp;redlink=1" class="new" title="Praearmantomys (page does not exist)">Praearmantomys</a></i> de Bruijn, 1966 - Early Miocene<sup id="cite&#95;ref-RuizSanchez2012&#95;24-0" class="reference"><a href="#cite_note-RuizSanchez2012-24"><span class="cite-bracket">&#91;</span>24<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Pseudodryomys&amp;action=edit&amp;redlink=1" class="new" title="Pseudodryomys (page does not exist)">Pseudodryomys</a></i> de Bruijn, 1966 - Early Miocene<sup id="cite&#95;ref-Dalmasso2022&#95;22-1" class="reference"><a href="#cite_note-Dalmasso2022-22"><span class="cite-bracket">&#91;</span>22<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Simplomys&amp;action=edit&amp;redlink=1" class="new" title="Simplomys (page does not exist)">Simplomys</a></i> García-Paredes <i>et al.</i>, 2009 - Early Miocene<sup id="cite&#95;ref-Lu2021&#95;19-4" class="reference"><a href="#cite_note-Lu2021-19"><span class="cite-bracket">&#91;</span>19<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/wiki/Seorsumuscardinus" title="Seorsumuscardinus">Seorsumuscardinus</a></i> de Bruijn 1998 - Early Miocene</li>
<li><i><a href="/w/index.php?title=Armantomys&amp;action=edit&amp;redlink=1" class="new" title="Armantomys (page does not exist)">Armantomys</a></i> de Bruijn, 1966 - Early to Middle Miocene<sup id="cite&#95;ref-RuizSanchez2012&#95;24-1" class="reference"><a href="#cite_note-RuizSanchez2012-24"><span class="cite-bracket">&#91;</span>24<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Prodryomys&amp;action=edit&amp;redlink=1" class="new" title="Prodryomys (page does not exist)">Prodryomys</a></i> Mayr, 1979 - Early to Middle Miocene<sup id="cite&#95;ref-Lu2021&#95;19-5" class="reference"><a href="#cite_note-Lu2021-19"><span class="cite-bracket">&#91;</span>19<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Tempestia&amp;action=edit&amp;redlink=1" class="new" title="Tempestia (page does not exist)">Tempestia</a></i> van de Weerd, 1976 - Middle Miocene<sup id="cite&#95;ref-Dalmasso2022&#95;22-2" class="reference"><a href="#cite_note-Dalmasso2022-22"><span class="cite-bracket">&#91;</span>22<span class="cite-bracket">&#93;</span></a></sup></li>
<li><i><a href="/w/index.php?title=Ramys&amp;action=edit&amp;redlink=1" class="new" title="Ramys (page does not exist)">Ramys</a></i> García-Moreno &amp; Lopez-Martínez,1986 - Late Miocene<sup id="cite&#95;ref-Lu2021&#95;19-6" class="reference"><a href="#cite_note-Lu2021-19"><span class="cite-bracket">&#91;</span>19<span class="cite-bracket">&#93;</span></a></sup></li></ul>
<div class="mw-heading mw-heading2"><h2 id="References">References</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=8" title="Edit section: References"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<style data-mw-deduplicate="TemplateStyles:r1327269900">.mw-parser-output .reflist-columns-2{column-width:30em}.mw-parser-output .reflist-columns-3{column-width:25em}body.skin-vector-2022 .mw-parser-output .reflist-columns-2{column-width:27em}body.skin-vector-2022 .mw-parser-output .reflist-columns-3{column-width:22.5em}.mw-parser-output .references[data-mw-group=upper-alpha]{list-style-type:upper-alpha}.mw-parser-output .references[data-mw-group=upper-roman]{list-style-type:upper-roman}.mw-parser-output .references[data-mw-group=lower-alpha]{list-style-type:lower-alpha}.mw-parser-output .references[data-mw-group=lower-greek]{list-style-type:lower-greek}.mw-parser-output .references[data-mw-group=lower-roman]{list-style-type:lower-roman}.mw-parser-output div.reflist-liststyle-upper-alpha .references{list-style-type:upper-alpha}.mw-parser-output div.reflist-liststyle-upper-roman .references{list-style-type:upper-roman}.mw-parser-output div.reflist-liststyle-lower-alpha .references{list-style-type:lower-alpha}.mw-parser-output div.reflist-liststyle-lower-greek .references{list-style-type:lower-greek}.mw-parser-output div.reflist-liststyle-lower-roman .references{list-style-type:lower-roman}</style><div>
<div class="mw-references-wrap mw-references-columns"><ol class="references">
<li id="cite&#95;note-1"><span class="mw-cite-backlink"><b><a href="#cite_ref-1">^</a></b></span> <span class="reference-text">Davis Brewster, ed. <i><a href="/wiki/Edinburgh_Encyclop%C3%A6dia" title="Edinburgh Encyclopædia">Edinburgh Encyclopædia</a></i>, 1819.</span>
</li>
<li id="cite&#95;note-The&#95;Mammal&#95;Society-2"><span class="mw-cite-backlink"><b><a href="#cite_ref-The_Mammal_Society_2-0">^</a></b></span> <span class="reference-text"><style data-mw-deduplicate="TemplateStyles:r1333433106">.mw-parser-output cite.citation{font-style:inherit;word-wrap:break-word}.mw-parser-output .citation q{quotes:"\"""\"""'""'"}.mw-parser-output .citation:target{background-color:rgba(0,127,255,0.133)}.mw-parser-output .id-lock-free.id-lock-free a{background:url("//upload.wikimedia.org/wikipedia/commons/6/65/Lock-green.svg")right 0.1em center/9px no-repeat}.mw-parser-output .id-lock-limited.id-lock-limited a,.mw-parser-output .id-lock-registration.id-lock-registration a{background:url("//upload.wikimedia.org/wikipedia/commons/d/d6/Lock-gray-alt-2.svg")right 0.1em center/9px no-repeat}.mw-parser-output .id-lock-subscription.id-lock-subscription a{background:url("//upload.wikimedia.org/wikipedia/commons/a/aa/Lock-red-alt-2.svg")right 0.1em center/9px no-repeat}.mw-parser-outpu[76599707745] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=114688 len=32768 cached=114688 start=0 eof=false
[78184926237] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[78195541413] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
t .cs1-ws-icon a{background:url("//upload.wikimedia.org/wikipedia/commons/4/4c/Wikisource-logo.svg")right 0.1em center/12px no-repeat}body:not(.skin-timeless):not(.skin-minerva) .mw-parser-output .id-lock-free a,body:not(.skin-timeless):not(.skin-minerva) .mw-parser-output .id-lock-limited a,body:not(.skin-timeless):not(.skin-minerva) .mw-parser-output .id-lock-registration a,body:not(.skin-timeless):not(.skin-minerva) .mw-parser-output .id-lock-subscription a,body:not(.skin-timeless):not(.skin-minerva) .mw-parser-output .cs1-ws-icon a{background-size:contain;padding:0 1em 0 0}.mw-parser-output .cs1-code{color:inherit;background:inherit;border:none;padding:inherit}.mw-parser-output .cs1-hidden-error{display:none;color:var(--color-error,#bf3c2c)}.mw-parser-output .cs1-visible-error{color:var(--color-error,#bf3c2c)}.mw-parser-output .cs1-maint{display:none;color:#085;margin-left:0.3em}.mw-parser-output .cs1-kern-left{padding-left:0.2em}.mw-parser-output .cs1-kern-right{padding-right:0.2em}.mw-parser-output .citation .mw-selflink{font-weight:inherit}@media screen{.mw-parser-output .cs1-format{font-size:95%}html.skin-theme-clientpref-night .mw-parser-output .cs1-maint{color:#18911f}}@media screen and (prefers-color-scheme:dark){html.skin-theme-clientpref-os .mw-parser-output .cs1-maint{color:#18911f}}</style><cite class="citation web cs1"><a rel="nofollow" class="external text" href="https://web.archive.org/web/20180308231407/http://www.mammal.org.uk/discover-mammals/species-dormouse/">"Species �� Dormouse"</a>. <i>The Mammal Society</i>. Archived from <a rel="nofollow" class="external text" href="http://www.mammal.org.uk/discover-mammals/species-dormouse/">the original</a> on March 8, 2018<span class="reference-accessdate">. Retrieved <span class="nowrap">March 8,</span> 2018</span>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=unknown&amp;rft.jtitle=The+Mammal+Society&amp;rft.atitle=Species+%E2%80%93+Dormouse&amp;rft&#95;id=http%3A%2F%2Fwww.mammal.org.uk%2Fdiscover-mammals%2Fspecies-dormouse%2F&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Lu-2021-3"><span class="mw-cite-backlink">^ <a href="#cite_ref-Lu-2021_3-0"><sup><i><b>a</b></i></sup></a> <a href="#cite_ref-Lu-2021_3-1"><sup><i><b>b</b></i></sup></a> <a href="#cite_ref-Lu-2021_3-2"><sup><i><b>c</b></i></sup></a></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFLuCosteurHugueneyMaridet2021" class="citation journal cs1">Lu, Xiaoyu; Costeur, Loïc; Hugueney, Marguerite; Maridet, Olivier (2021-02-01). <a rel="nofollow" class="external text" href="https://www.tandfonline.com/doi/full/10.1080/14772019.2021.1888814">"New data on early Oligocene dormice (Rodentia, Gliridae) from southern Europe: phylogeny and diversification of the family"</a>. <i>Journal of Systematic Palaeontology</i>. <b>19</b> (3): <span class="nowrap">169���</span>189. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1080%2F14772019.2021.1888814">10.1080/14772019.2021.1888814</a>. <a href="/wiki/ISSN_(identifier)" class="mw-redirect" title="ISSN (identifier)">ISSN</a>&#160;<a rel="nofollow" class="external text" href="https://search.worldcat.org/issn/1477-2019">1477-2019</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Journal+of+Systematic+Palaeontology&amp;rft.atitle=New+data+on+early+Oligocene+dormice+%28Rodentia%2C+Gliridae%29+from+southern+Europe%3A+phylogeny+and+diversification+of+the+family&amp;rft.volume=19&amp;rft.issue=3&amp;rft.pages=169-189&amp;rft.date=2021-02-01&amp;rft&#95;id=info%3Adoi%2F10.1080%2F14772019.2021.1888814&amp;rft.issn=1477-2019&amp;rft.aulast=Lu&amp;rft.aufirst=Xiaoyu&amp;rft.au=Costeur%2C+Lo%C3%AFc&amp;rft.au=Hugueney%2C+Marguerite&amp;rft.au=Maridet%2C+Olivier&amp;rft&#95;id=https%3A%2F%2Fwww.tandfonline.com%2Fdoi%2Ffull%2F10.1080%2F14772019.2021.1888814&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-4"><span class="mw-cite-backlink"><b><a href="#cite_ref-4">^</a></b></span> <span class="reference-text">Random House Dictionary, dormouse.</span>
</li>
<li id="cite&#95;note-5"><span class="mw-cite-backlink"><b><a href="#cite_ref-5">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFWedgwood1855" class="citation journal cs1"><a href="/wiki/Hensleigh_Wedgwood" title="Hensleigh Wedgwood">Wedgwood, Hensleigh</a> (1855). <a rel="nofollow" class="external text" href="https://babel.hathitrust.org/cgi/pt?id=uc1.b3924121;view=1up;seq=78">"On false etymologies"</a>. <i>Transactions of the Philological Society</i> (6): 66.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Transactions+of+the+Philological+Society&amp;rft.atitle=On+false+etymologies&amp;rft.issue=6&amp;rft.pages=66&amp;rft.date=1855&amp;rft.aulast=Wedgwood&amp;rft.aufirst=Hensleigh&amp;rft&#95;id=https%3A%2F%2Fbabel.hathitrust.org%2Fcgi%2Fpt%3Fid%3Duc1.b3924121%3Bview%3D1up%3Bseq%3D78&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-6"><span class="mw-cite-backlink"><b><a href="#cite_ref-6">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFJuškaitis2001" class="citation journal cs1">Juškaitis, R. (2001). <a rel="nofollow" class="external text" href="https://ptes.org/wp-content/uploads/2014/06/Juskaitis-2001-Weight-changes-of-common-dormouse.pdf">"Weight changes of the common dormouse (Muscardinus avellanarius L.) during the year in Lithuania"</a> <span class="cs1-format">(PDF)</span>. <i>Trakya University Journal of Scientific Research</i>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Trakya+University+Journal+of+Scientific+Research&amp;rft.atitle=Weight+changes+of+the+common+dormouse+%28Muscardinus+avellanarius+L.%29+during+the+year+in+Lithuania&amp;rft.date=2001&amp;rft.aulast=Ju%C5%A1kaitis&amp;rft.aufirst=R.&amp;rft&#95;id=https%3A%2F%2Fptes.org%2Fwp-content%2Fuploads%2F2014%2F06%2FJuskaitis-2001-Weight-changes-of-common-dormouse.pdf&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-EoM-7"><span class="mw-cite-backlink">^ <a href="#cite_ref-EoM_7-0"><sup><i><b>a</b></i></sup></a> <a href="#cite_ref-EoM_7-1"><sup><i><b>b</b></i></sup></a> <a href="#cite_ref-EoM_7-2"><sup><i><b>c</b></i></sup></a> <a href="#cite_ref-EoM_7-3"><sup><i><b>d</b></i></sup></a></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFBaudoin,&#95;Claude1984" class="citation book cs1">Baudoin, Claude (1984). Macdonald, D. (ed.). <span class="id-lock-registration" title="Free registration required"><a rel="nofollow" class="external text" href="https://archive.org/details/encyclopediaofma00mals_0/page/678"><i>The Encyclopedia of Mammals</i></a></span>. New York: Facts on File. pp.&#160;<a rel="nofollow" class="external text" href="https://archive.org/details/encyclopediaofma00mals_0/page/678">678–680</a>. <a href="/wiki/ISBN_(identifier)" class="mw-redirect" title="ISBN (identifier)">ISBN</a>&#160;<a href="/wiki/Special:BookSources/978-0-87196-871-5" title="Special:BookSources/978-0-87196-871-5"><bdi>978-0-87196-871-5</bdi></a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Abook&amp;rft.genre=book&amp;rft.btitle=The+Encyclopedia+of+Mammals&amp;rft.place=New+York&amp;rft.pages=678-680&amp;rft.pub=Facts+on+File&amp;rft.date=1984&amp;rft.isbn=978-0-87196-871-5&amp;rft.au=Baudoin%2C+Claude&amp;rft&#95;id=https%3A%2F%2Farchive.org%2Fdetails%2Fencyclopediaofma00mals&#95;0%2Fpage%2F678&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-8"><span class="mw-cite-backlink"><b><a href="#cite_ref-8">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFFreedman2008" class="citation web cs1">Freedman, Paul (March 6, 2008). <a rel="nofollow" class="external text" href="https://web.archive.org/web/20080311011527/http://www.gourmet.com/food/2008/03/dormouse">"Meals that Time Forgot"</a>. <i>Gourmet.com</i>. Archived from <a rel="nofollow" class="external text" href="http://www.gourmet.com/food/2008/03/dormouse">the original</a> on March 11, 2008<span class="reference-accessdate">. Retrieved <span class="nowrap">February 13,</span> 2017</span>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=unknown&amp;rft.jtitle=Gourmet.com&amp;rft.atitle=Meals+that+Time+Forgot&amp;rft.date=2008-03-06&amp;rft.aulast=Freedman&amp;rft.aufirst=Paul&amp;rft&#95;id=http%3A%2F%2Fwww.gourmet.com%2Ffood%2F2008%2F03%2Fdormouse&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-9"><span class="mw-cite-backlink"><b><a href="#cite_ref-9">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFKolumbić" class="citation web cs1">Kolumbić, Igor. <a rel="nofollow" class="external text" href="https://web.archive.org/web/20210307001427/http://www.otok-hvar.com/en/news/fifth-puhijada-dol-hvar-695">"Fifth Puhijada"</a>. <i>otok-hvar.com</i>. Hvar: Offero Prima d.o.o. Archived from <a rel="nofollow" class="external text" href="http://www.otok-hvar.com/en/news/fifth-puhijada-dol-hvar-695">the original</a> on March 7, 2021<span class="reference-accessdate">. Retrieved <span class="nowrap">February 13,</span> 2017</span>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=unknown&amp;rft.jtitle=otok-hvar.com&amp;rft.atitle=Fifth+Puhijada&amp;rft.aulast=Kolumbi%C4%87&amp;rft.aufirst=Igor&amp;rft&#95;id=http%3A%2F%2Fwww.otok-hvar.com%2Fen%2Fnews%2Ffifth-puhijada-dol-hvar-695&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-10"><span class="mw-cite-backlink"><b><a href="#cite_ref-10">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite class="citation news cs1"><a rel="nofollow" class="external text" href="https://news.bbc.co.uk/2/hi/uk_news/magazine/7967968.stm">"10 ways to get a really good sleep"</a>. BBC. 27 March 2009<span class="reference-accessdate">. Retrieved <span class="nowrap">February 13,</span> 2017</span>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.atitle=10+ways+to+get+a+really+good+sleep&amp;rft.date=2009-03-27&amp;rft&#95;id=https%3A%2F%2Fnews.bbc.co.uk%2F2%2Fhi%2Fuk&#95;news%2Fmagazine%2F7967968.stm&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-11"><span class="mw-cite-backlink"><b><a href="#cite_ref-11">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite class="citation web cs1"><a rel="nofollow" class="external text" href="http://www.oocities.org/efexotics/africandormouse.html">"www.oocities.org/efexotics/africandormouse.html"</a>. 2009. <q>As far as I know, my own pet shop in Cambridgeshire was the first pet shop in Britain to regularly stock the species (this was as recently as the 1990s).</q></cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Abook&amp;rft.genre=unknown&amp;rft.btitle=www.oocities.org%2Fefexotics%2Fafricandormouse.html&amp;rft.date=2009&amp;rft&#95;id=http%3A%2F%2Fwww.oocities.org%2Fefexotics%2Fafricandormouse.html&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-12"><span class="mw-cite-backlink"><b><a href="#cite_ref-12">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite class="citation web cs1"><a rel="nofollow" class="external text" href="https://crittery.co.uk/species-list/african-pygmy-dormice">"Crittery Exotics"</a>. <i>crittery.co.uk</i>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=unknown&amp;rft.jtitle=crittery.co.uk&amp;rft.atitle=Crittery+Exotics&amp;rft&#95;id=https%3A%2F%2Fcrittery.co.uk%2Fspecies-list%2Fafrican-pygmy-dormice&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-13"><span class="mw-cite-backlink"><b><a href="#cite_ref-13">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite class="citation web cs1"><a rel="nofollow" class="external text" href="https://crittery.co.uk/species-list/asian-garden-dormice">"Crittery Exotics"</a>. <i>crittery.co.uk</i>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=unknown&amp;rft.jtitle=crittery.co.uk&amp;rft.atitle=Crittery+Exotics&amp;rft&#95;id=https%3A%2F%2Fcrittery.co.uk%2Fspecies-list%2Fasian-garden-dormice&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Li-2023-14"><span class="mw-cite-backlink">^ <a href="#cite_ref-Li-2023_14-0"><sup><i><b>a</b></i></sup></a> <a href="#cite_ref-Li-2023_14-1"><sup><i><b>b</b></i></sup></a></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFLiMörs2023" class="citation journal cs1">Li, Zhaoyu; Mörs, Thomas (June 2023). <a rel="nofollow" class="external text" href="https://linkinghub.elsevier.com/retrieve/pii/S0016699523000414">"Dormice (Rodentia, Gliridae) from the Middle Miocene of Hambach 6C, Northwest Germany"</a>. <i>Geobios</i>. <b>78</b>: <span class="nowrap">15–</span>31. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1016%2Fj.geobios.2023.05.002">10.1016/j.geobios.2023.05.002</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Geobios&amp;rft.atitle=Dormice+%28Rodentia%2C+Gliridae%29+from+the+Middle+Miocene+of+Hambach+6C%2C+Northwest+Germany&amp;rft.volume=78&amp;rft.pages=15-31&amp;rft.date=2023-06&amp;rft&#95;id=info%3Adoi%2F10.1016%2Fj.geobios.2023.05.002&amp;rft.aulast=Li&amp;rft.aufirst=Zhaoyu&amp;rft.au=M%C3%B6rs%2C+Thomas&amp;rft&#95;id=https%3A%2F%2Flinkinghub.elsevier.com%2Fretrieve%2Fpii%2FS0016699523000414&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Hennekam-2020-15"><span class="mw-cite-backlink"><b><a href="#cite_ref-Hennekam-2020_15-0">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFHennekamBensonHerridgeJeffery2020" class="citation journal cs1">Hennekam, Jesse J.; Benson, Roger B. J.; Herridge, Victoria L.; Jeffery, Nathan; Torres-Roig, Enric; Alcover, Josep Antoni; Cox, Philip G. (2020-11-11). <a rel="nofollow" class="external text" href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7735280">"Morphological divergence in giant fossil dormice"</a>. <i>Proceedings of the Royal Society B: Biological Sciences</i>. <b>287</b> (1938) [78436020597] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=131072 len=32768 cached=131072 start=0 eof=false
[80161241820] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[80170936857] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
20202085. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1098%2Frspb.2020.2085">10.1098/rspb.2020.2085</a>. <a href="/wiki/ISSN_(identifier)" class="mw-redirect" title="ISSN (identifier)">ISSN</a>&#160;<a rel="nofollow" class="external text" href="https://search.worldcat.org/issn/0962-8452">0962-8452</a>. <a href="/wiki/PMC_(identifier)" class="mw-redirect" title="PMC (identifier)">PMC</a>&#160;<span class="id-lock-free" title="Freely accessible"><a rel="nofollow" class="external text" href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7735280">7735280</a></span>. <a href="/wiki/PMID_(identifier)" class="mw-redirect" title="PMID (identifier)">PMID</a>&#160;<a rel="nofollow" class="external text" href="https://pubmed.ncbi.nlm.nih.gov/33143584">33143584</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Proceedings+of+the+Royal+Society+B%3A+Biological+Sciences&amp;rft.atitle=Morphological+divergence+in+giant+fossil+dormice&amp;rft.volume=287&amp;rft.issue=1938&amp;rft.artnum=20202085&amp;rft.date=2020-11-11&amp;rft&#95;id=https%3A%2F%2Fwww.ncbi.nlm.nih.gov%2Fpmc%2Farticles%2FPMC7735280%23id-name%3DPMC&amp;rft.issn=0962-8452&amp;rft&#95;id=info%3Apmid%2F33143584&amp;rft&#95;id=info%3Adoi%2F10.1098%2Frspb.2020.2085&amp;rft.aulast=Hennekam&amp;rft.aufirst=Jesse+J.&amp;rft.au=Benson%2C+Roger+B.+J.&amp;rft.au=Herridge%2C+Victoria+L.&amp;rft.au=Jeffery%2C+Nathan&amp;rft.au=Torres-Roig%2C+Enric&amp;rft.au=Alcover%2C+Josep+Antoni&amp;rft.au=Cox%2C+Philip+G.&amp;rft&#95;id=https%3A%2F%2Fwww.ncbi.nlm.nih.gov%2Fpmc%2Farticles%2FPMC7735280&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-16"><span class="mw-cite-backlink"><b><a href="#cite_ref-16">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFPetrovaPanitsinaBodrovAbramson2024" class="citation journal cs1">Petrova, Tatyana V.; Panitsina, Valentina A.; Bodrov, Semyon Yu.; Abramson, Natalia I. (2024-09-27). <a rel="nofollow" class="external text" href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC11436627">"The mitochondrial genome of the critically endangered enigmatic Kazakhstani endemic Selevinia betpakdalaensis (Rodentia: Gliridae) and its phylogenetic relationships with other dormouse species"</a>. <i>Scientific Reports</i>. <b>14</b> (1). <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1038%2Fs41598-024-73703-2">10.1038/s41598-024-73703-2</a>. <a href="/wiki/ISSN_(identifier)" class="mw-redirect" title="ISSN (identifier)">ISSN</a>&#160;<a rel="nofollow" class="external text" href="https://search.worldcat.org/issn/2045-2322">2045-2322</a>. <a href="/wiki/PMC_(identifier)" class="mw-redirect" title="PMC (identifier)">PMC</a>&#160;<span class="id-lock-free" title="Freely accessible"><a rel="nofollow" class="external text" href="https://www.ncbi.nlm.nih.gov/pmc/articles/PMC11436627">11436627</a></span>. <a href="/wiki/PMID_(identifier)" class="mw-redirect" title="PMID (identifier)">PMID</a>&#160;<a rel="nofollow" class="external text" href="https://pubmed.ncbi.nlm.nih.gov/39333293">39333293</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Scientific+Reports&amp;rft.atitle=The+mitochondrial+genome+of+the+critically+endangered+enigmatic+Kazakhstani+endemic+Selevinia+betpakdalaensis+%28Rodentia%3A+Gliridae%29+and+its+phylogenetic+relationships+with+other+dormouse+species&amp;rft.volume=14&amp;rft.issue=1&amp;rft.date=2024-09-27&amp;rft&#95;id=https%3A%2F%2Fwww.ncbi.nlm.nih.gov%2Fpmc%2Farticles%2FPMC11436627%23id-name%3DPMC&amp;rft.issn=2045-2322&amp;rft&#95;id=info%3Apmid%2F39333293&amp;rft&#95;id=info%3Adoi%2F10.1038%2Fs41598-024-73703-2&amp;rft.aulast=Petrova&amp;rft.aufirst=Tatyana+V.&amp;rft.au=Panitsina%2C+Valentina+A.&amp;rft.au=Bodrov%2C+Semyon+Yu.&amp;rft.au=Abramson%2C+Natalia+I.&amp;rft&#95;id=https%3A%2F%2Fwww.ncbi.nlm.nih.gov%2Fpmc%2Farticles%2FPMC11436627&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-17"><span class="mw-cite-backlink"><b><a href="#cite_ref-17">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFHolden,&#95;Mary&#95;EllenLevine,&#95;Rebecca&#95;S2009" class="citation journal cs1">Holden, Mary Ellen; Levine, Rebecca S (2009). "Chapter 9. Systematic Revision of Sub-Saharan African Dormice (Rodentia: Gliridae: <i>Graphiurus</i>) Part II: Description of a New Species of <i>Graphiurus</i> from the Central Congo Basin, Including Morphological and Ecological Niche Comparisons with <i>G. crassicaudatus</i> and <i>G. lorraineus</i>". <i>Bulletin of the American Museum of Natural History</i>. <b>331</b>: <span class="nowrap">314���</span>355. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1206%2F582-9.1">10.1206/582-9.1</a>. <a href="/wiki/S2CID_(identifier)" class="mw-redirect" title="S2CID (identifier)">S2CID</a>&#160;<a rel="nofollow" class="external text" href="https://api.semanticscholar.org/CorpusID:85409018">85409018</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Bulletin+of+the+American+Museum+of+Natural+History&amp;rft.atitle=Chapter+9.+Systematic+Revision+of+Sub-Saharan+African+Dormice+%28Rodentia%3A+Gliridae%3A+Graphiurus%29+Part+II%3A+Description+of+a+New+Species+of+Graphiurus+from+the+Central+Congo+Basin%2C+Including+Morphological+and+Ecological+Niche+Comparisons+with+G.+crassicaudatus+and+G.+lorraineus&amp;rft.volume=331&amp;rft.pages=314-355&amp;rft.date=2009&amp;rft&#95;id=info%3Adoi%2F10.1206%2F582-9.1&amp;rft&#95;id=https%3A%2F%2Fapi.semanticscholar.org%2FCorpusID%3A85409018%23id-name%3DS2CID&amp;rft.au=Holden%2C+Mary+Ellen&amp;rft.au=Levine%2C+Rebecca+S&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Freudenthal2007-18"><span class="mw-cite-backlink">^ <a href="#cite_ref-Freudenthal2007_18-0"><sup><i><b>a</b></i></sup></a> <a href="#cite_ref-Freudenthal2007_18-1"><sup><i><b>b</b></i></sup></a> <a href="#cite_ref-Freudenthal2007_18-2"><sup><i><b>c</b></i></sup></a> <a href="#cite_ref-Freudenthal2007_18-3"><sup><i><b>d</b></i></sup></a></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFFreudenthalMartín-Suárez2007" class="citation journal cs1">Freudenthal, M.; Mart��n-Su��rez, E. (2007). <a rel="nofollow" class="external text" href="https://repository.naturalis.nl/pub/314199">"Revision of the subfamily Bransatoglirinae (Gliridae, Rodentia, Mammalia)"</a>. <i>Scripta Geologica</i>. <b>135</b>: <span class="nowrap">241��</span>274<span class="reference-accessdate">. Retrieved <span class="nowrap">22 January</span> 2026</span>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Scripta+Geologica&amp;rft.atitle=Revision+of+the+subfamily+Bransatoglirinae+%28Gliridae%2C+Rodentia%2C+Mammalia%29&amp;rft.volume=135&amp;rft.pages=241-274&amp;rft.date=2007&amp;rft.aulast=Freudenthal&amp;rft.aufirst=M.&amp;rft.au=Mart%C3%ADn-Su%C3%A1rez%2C+E.&amp;rft&#95;id=https%3A%2F%2Frepository.naturalis.nl%2Fpub%2F314199&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Lu2021-19"><span class="mw-cite-backlink">^ <a href="#cite_ref-Lu2021_19-0"><sup><i><b>a</b></i></sup></a> <a href="#cite_ref-Lu2021_19-1"><sup><i><b>b</b></i></sup></a> <a href="#cite_ref-Lu2021_19-2"><sup><i><b>c</b></i></sup></a> <a href="#cite_ref-Lu2021_19-3"><sup><i><b>d</b></i></sup></a> <a href="#cite_ref-Lu2021_19-4"><sup><i><b>e</b></i></sup></a> <a href="#cite_ref-Lu2021_19-5"><sup><i><b>f</b></i></sup></a> <a href="#cite_ref-Lu2021_19-6"><sup><i><b>g</b></i></sup></a></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFLuCosteur2021" class="citation journal cs1">Lu, X.; Costeur, L.; et&#160;al. (February 2021). "New data on early Oligocene dormice (Rodentia, Gliridae) from southern Europe: phylogeny and diversification of the family". <i>Journal of Systematic Palaeontology</i>. <b>19</b> (3): <span class="nowrap">169���</span>189. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1080%2F14772019.2021.1888814">10.1080/14772019.2021.1888814</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Journal+of+Systematic+Palaeontology&amp;rft.atitle=New+data+on+early+Oligocene+dormice+%28Rodentia%2C+Gliridae%29+from+southern+Europe%3A+phylogeny+and+diversification+of+the+family&amp;rft.volume=19&amp;rft.issue=3&amp;rft.pages=169-189&amp;rft.date=2021-02&amp;rft&#95;id=info%3Adoi%2F10.1080%2F14772019.2021.1888814&amp;rft.aulast=Lu&amp;rft.aufirst=X.&amp;rft.au=Costeur%2C+L.&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Li2023-20"><span class="mw-cite-backlink"><b><a href="#cite_ref-Li2023_20-0">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFLiMörs2023" class="citation journal cs1">Li, Z.; Mörs, T. (June 2023). "Dormice (Rodentia, Gliridae) from the Middle Miocene of Hambach 6C, Northwest Germany". <i>Geobios</i>. <b>78</b>: <span class="nowrap">15���</span>31. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1016%2Fj.geobios.2023.05.002">10.1016/j.geobios.2023.05.002</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Geobios&amp;rft.atitle=Dormice+%28Rodentia%2C+Gliridae%29+from+the+Middle+Miocene+of+Hambach+6C%2C+Northwest+Germany&amp;rft.volume=78&amp;rft.pages=15-31&amp;rft.date=2023-06&amp;rft&#95;id=info%3Adoi%2F10.1016%2Fj.geobios.2023.05.002&amp;rft.aulast=Li&amp;rft.aufirst=Z.&amp;rft.au=M%C3%B6rs%2C+T.&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-GarciaParedes2010-21"><span class="mw-cite-backlink"><b><a href="#cite_ref-GarciaParedes2010_21-0">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFGarc��a-PeredesPeláez-Campomanes��ngeles&#95;Álvarez-Sierra2010" class="citation journal cs1">Garc��a-Peredes, I.; Peláez-Campomanes, P.; Ángeles ��lvarez-Sierra, M. (September 2010). "<i>Microdyromys remmerti</i>, sp. nov., a new Gliridae (Rodentia, Mammalia) from the Aragonian type area (Miocene, Calatayud-Montalbán basin, Spain)". <i>Journal of Vertebrate Paleontology</i>. <b>30</b> (5): <span class="nowrap">1594���</span>1609. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1080%2F02724634.2010.501453">10.1080/02724634.2010.501453</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Journal+of+Vertebrate+Paleontology&amp;rft.atitle=Microdyromys+remmerti%2C+sp.+nov.%2C+a+new+Gliridae+%28Rodentia%2C+Mammalia%29+from+the+Aragonian+type+area+%28Miocene%2C+Calatayud-Montalb%C3%A1n+basin%2C+Spain%29&amp;rft.volume=30&amp;rft.issue=5&amp;rft.pages=1594-1609&amp;rft.date=2010-09&amp;rft&#95;id=info%3Adoi%2F10.1080%2F02724634.2010.501453&amp;rft.aulast=Garc%C3%ADa-Peredes&amp;rft.aufirst=I.&amp;rft.au=Pel%C3%A1ez-Campomanes%2C+P.&amp;rft.au=%C3%81ngeles+%C3%81lvarez-Sierra%2C+M.&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Dalmasso2022-22"><span class="mw-cite-backlink">^ <a href="#cite_ref-Dalmasso2022_22-0"><sup><i><b>a</b></i></sup></a> <a href="#cite_ref-Dalmasso2022_22-1"><sup><i><b>b</b></i></sup></a> <a href="#cite_ref-Dalmasso2022_22-2"><sup><i><b>c</b></i></sup></a></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFDalmassoPaláez-CampomanesLópez-Antoñanzas2022" class="citation journal cs1">Dalmasso, A.; Pal��ez-Campomanes, P.; López-Antoñanzas, R. (August 2022). "Relative performance of Bayesian morphological clock and parsimony methods for phylogenetic reconstructions: Insights from the case of Myomiminae and Dryomyinae glirid rodents". <i>Cladistics</i>. <b>38</b> (6): <span class="nowrap">702���</span>710. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1111%2Fcla.12516">10.1111/cla.12516</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Cladistics&amp;rft.atitle=Relative+performance+of+Bayesian+morphological+clock+and+parsimony+methods+for+phylogenetic+reconstructions%3A+Insights+from+the+case+of+Myomiminae+and+Dryomyinae+glirid+rodents&amp;rft.volume=38&amp;rft.issue=6&amp;rft.pages=702-710&amp;rft.date=2022-08&amp;rft&#95;id=info%3Adoi%2F10.1111%2Fcla.12516&amp;rft.aulast=Dalmasso&amp;rft.aufirst=A.&amp;rft.au=Pal%C3%A1ez-Campomanes%2C+P.&amp;rft.au=L%C3%B3pez-Anto%C3%B1anzas%2C+R.&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-Sinitsa2018-23"><span class="mw-cite-backlink"><b><a href="#cite_ref-Sinitsa2018_23-0">^</a></b></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFSinitsaNesin2018" class="citation journal cs1">Sinitsa, M.V.; Nesin, V.A. (March 2018). "Systematics and phylogeny of Vasseuromys (Mammalia, Rodentia, Gliridae) with a description of a new species from the late Miocene of eastern Europe". <i>Palaeontology</i>. <b>61</b> (5): <span class="nowrap">679–</span>701. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.1111%2Fpala.12359">10.1111/pala.12359</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Palaeontology&amp;rft.atitle=Systematics+and+phylogeny+of+Vasseuromys+%28Mammalia%2C+Rodentia%2C+Gliridae%29+with+a+description+of+a+new+species+from+the+late+Miocene+of+eastern+Europe&amp;rft.volume=61&amp;rft.issue=5&amp;rft.pages=679-701&amp;rft.date=2018-03&amp;rft&#95;id=info%3Adoi%2F10.1111%2Fpala.12359&amp;rft.aulast=Sinitsa&amp;rft.aufirst=M.V.&amp;rft.au=Nesin%2C+V.A.&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
<li id="cite&#95;note-RuizSanchez2012-24"><span class="mw-cite-backlink">^ <a href="#cite_ref-RuizSanchez2012_24-0"><sup><i><b>a</b></i></sup></a> <a href="#cite_ref-RuizSanchez2012_24-1"><sup><i><b>b</b></i></sup></a></span> <span class="reference-text"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFRuiz-SánchezMurelaga2012" class="citation journal cs1">Ruiz-Sánchez, F.J.; Murelaga, X.; et&#160;al. (September 2012). "Hypsodont Myomiminae (Gliridae, Rodentia) from five new localities in the Lower Miocene Tudela Formation (Bardenas Reales, Ebro Basin, Spain) and their bearing on the age[80446987335] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=147456 len=32768 cached=131072 start=16384 eof=false
[82232724189] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[82243557165] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
 of the Agenian-Ramblian boundary". <i>Geodiversitas</i>. <b>34</b> (3): <span class="nowrap">645–</span>663. <a href="/wiki/Doi_(identifier)" class="mw-redirect" title="Doi (identifier)">doi</a>:<a rel="nofollow" class="external text" href="https://doi.org/10.5252%2Fg2012n3a10">10.5252/g2012n3a10</a>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=Geodiversitas&amp;rft.atitle=Hypsodont+Myomiminae+%28Gliridae%2C+Rodentia%29+from+five+new+localities+in+the+Lower+Miocene+Tudela+Formation+%28Bardenas+Reales%2C+Ebro+Basin%2C+Spain%29+and+their+bearing+on+the+age+of+the+Agenian-Ramblian+boundary&amp;rft.volume=34&amp;rft.issue=3&amp;rft.pages=645-663&amp;rft.date=2012-09&amp;rft&#95;id=info%3Adoi%2F10.5252%2Fg2012n3a10&amp;rft.aulast=Ruiz-S%C3%A1nchez&amp;rft.aufirst=F.J.&amp;rft.au=Murelaga%2C+X.&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></span>
</li>
</ol></div></div>
<div class="mw-heading mw-heading2"><h2 id="Further_reading">Further reading</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=9" title="Edit section: Further reading"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<ul><li><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite id="CITEREFHolden,&#95;M.&#95;E.2005" class="citation book cs1">Holden, M. E. (2005). "Family Gliridae". In Wilson, D. E.; Reeder, D. M. (eds.). <i>Mammal Species of the World a Taxonomic and Geographic Reference</i>. Baltimore: Johns Hopkins University Press. pp.&#160;<span class="nowrap">819��</span>841.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Abook&amp;rft.genre=bookitem&amp;rft.atitle=Family+Gliridae&amp;rft.btitle=Mammal+Species+of+the+World+a+Taxonomic+and+Geographic+Reference&amp;rft.place=Baltimore&amp;rft.pages=819-841&amp;rft.pub=Johns+Hopkins+University+Press&amp;rft.date=2005&amp;rft.au=Holden%2C+M.+E.&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></li></ul>
<div class="mw-heading mw-heading2"><h2 id="External_links">External links</h2><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Dormouse&amp;action=edit&amp;section=10" title="Edit section: External links"><span>edit</span></a><span class="mw-editsection-bracket">]</span></span></div>
<style data-mw-deduplicate="TemplateStyles:r1314755338">.mw-parser-output .side-box{margin:4px 0;box-sizing:border-box;border:1px solid #aaa;font-size:88%;line-height:1.25em;background-color:var(--background-color-interactive-subtle,#f8f9fa);color:inherit;display:flow-root}.mw-parser-output .infobox .side-box{font-size:100%}.mw-parser-output .side-box-abovebelow,.mw-parser-output .side-box-text{padding:0.25em 0.9em}.mw-parser-output .side-box-image{padding:2px 0 2px 0.9em;text-align:center}.mw-parser-output .side-box-imageright{padding:2px 0.9em 2px 0;text-align:center}@media(min-width:500px){.mw-parser-output .side-box-flex{display:flex;align-items:center}.mw-parser-output .side-box-text{flex:1;min-width:0}}@media(min-width:640px){.mw-parser-output .side-box{width:238px}.mw-parser-output .side-box-right{clear:right;float:right;margin-left:1em}.mw-parser-output .side-box-left{margin-right:1em}}</style><style data-mw-deduplicate="TemplateStyles:r1311551236">@media print{body.ns-0 .mw-parser-output .sistersitebox{display:none!important}}@media screen{html.skin-theme-clientpref-night .mw-parser-output .sistersitebox img[src*="Wiktionary-logo-en-v2.svg"]{filter:invert(1)brightness(55%)contrast(250%)hue-rotate(180deg)}}@media screen and (prefers-color-scheme:dark){html.skin-theme-clientpref-os .mw-parser-output .sistersitebox img[src*="Wiktionary-logo-en-v2.svg"]{filter:invert(1)brightness(55%)contrast(250%)hue-rotate(180deg)}}</style><div class="side-box side-box-right plainlinks sistersitebox"><style data-mw-deduplicate="TemplateStyles:r1126788409">.mw-parser-output .plainlist ol,.mw-parser-output .plainlist ul{line-height:inherit;list-style:none;margin:0;padding:0}.mw-parser-output .plainlist ol li,.mw-parser-output .plainlist ul li{margin-bottom:0}</style>
<div class="side-box-flex">
<div class="side-box-image"><span class="noviewer" typeof="mw:File"><a href="/wiki/File:Commons-logo.svg" class="mw-file-description"><img alt="Wikimedia Commons logo" src="//upload.wikimedia.org/wikipedia/en/thumb/4/4a/Commons-logo.svg/40px-Commons-logo.svg.png" decoding="async" width="30" height="40" class="mw-file-element" srcset="//upload.wikimedia.org/wikipedia/en/thumb/4/4a/Commons-logo.svg/60px-Commons-logo.svg.png 2x" data-file-width="1024" data-file-height="1376" /></a></span></div>
<div class="side-box-text plainlist">Wikimedia Commons has media related to <a href="https://commons.wikimedia.org/wiki/Gliridae" class="extiw" title="commons:Gliridae"><span style="font-style:italic; font-weight:bold;">Gliridae</span></a>.</div></div>
</div>
<ul><li><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite class="citation web cs1"><a rel="nofollow" class="external text" href="http://www.the-piedpiper.co.uk/th1k.htm">"Dormice"</a>. <i>The PiedPiper</i>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=unknown&amp;rft.jtitle=The+PiedPiper&amp;rft.atitle=Dormice&amp;rft&#95;id=http%3A%2F%2Fwww.the-piedpiper.co.uk%2Fth1k.htm&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></li>
<li><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333433106" /><cite class="citation news cs1"><a rel="nofollow" class="external text" href="https://www.bbc.co.uk/wales/nature/sites/species/mammals/dormouse.shtml">"Dormouse"</a>. <i>BBC Wales Nature</i>.</cite><span title="ctx&#95;ver=Z39.88-2004&amp;rft&#95;val&#95;fmt=info%3Aofi%2Ffmt%3Akev%3Amtx%3Ajournal&amp;rft.genre=article&amp;rft.jtitle=BBC+Wales+Nature&amp;rft.atitle=Dormouse&amp;rft&#95;id=https%3A%2F%2Fwww.bbc.co.uk%2Fwales%2Fnature%2Fsites%2Fspecies%2Fmammals%2Fdormouse.shtml&amp;rfr&#95;id=info%3Asid%2Fen.wikipedia.org%3ADormouse" class="Z3988"></span></li>
<li><a rel="nofollow" class="external text" href="https://web.archive.org/web/20040122040057/http://www.glirarium.org/">Glirarium.org</a> <span class="languageicon">(in English and German)</span></li></ul>
<div class="navbox-styles"><style data-mw-deduplicate="TemplateStyles:r1333133064">.mw-parser-output .hlist dl,.mw-parser-output .hlist ol,.mw-parser-output .hlist ul{margin:0;padding:0}.mw-parser-output .hlist dd,.mw-parser-output .hlist dt,.mw-parser-output .hlist li{margin:0;display:inline}.mw-parser-output .hlist.inline,.mw-parser-output .hlist.inline dl,.mw-parser-output .hlist.inline ol,.mw-parser-output .hlist.inline ul,.mw-parser-output .hlist dl dl,.mw-parser-output .hlist dl ol,.mw-parser-output .hlist dl ul,.mw-parser-output .hlist ol dl,.mw-parser-output .hlist ol ol,.mw-parser-output .hlist ol ul,.mw-parser-output .hlist ul dl,.mw-parser-output .hlist ul ol,.mw-parser-output .hlist ul ul{display:inline}.mw-parser-output .hlist .mw-empty-li{display:none}.mw-parser-output .hlist dt::after{content:": "}.mw-parser-output .hlist dd::after,.mw-parser-output .hlist li::after{content:"\a0 · ";font-weight:bold}.mw-parser-output .hlist dd:last-child::after,.mw-parser-output .hlist dt:last-child::after,.mw-parser-output .hlist li:last-child::after{content:none}.mw-parser-output .hlist dd dd:first-child::before,.mw-parser-output .hlist dd dt:first-child::before,.mw-parser-output .hlist dd li:first-child::before,.mw-parser-output .hlist dt dd:first-child::before,.mw-parser-output .hlist dt dt:first-child::before,.mw-parser-output .hlist dt li:first-child::before,.mw-parser-output .hlist li dd:first-child::before,.mw-parser-output .hlist li dt:first-child::before,.mw-parser-output .hlist li li:first-child::before{content:" (";font-weight:normal}.mw-parser-output .hlist dd dd:last-child::after,.mw-parser-output .hlist dd dt:last-child::after,.mw-parser-output .hlist dd li:last-child::after,.mw-parser-output .hlist dt dd:last-child::after,.mw-parser-output .hlist dt dt:last-child::after,.mw-parser-output .hlist dt li:last-child::after,.mw-parser-output .hlist li dd:last-child::after,.mw-parser-output .hlist li dt:last-child::after,.mw-parser-output .hlist li li:last-child::after{content:")";font-weight:normal}.mw-parser-output .hlist ol{counter-reset:listitem}.mw-parser-output .hlist ol>li{counter-increment:listitem}.mw-parser-output .hlist ol>li::before{content:" "counter(listitem)"\a0 "}.mw-parser-output .hlist dd ol>li:first-child::before,.mw-parser-output .hlist dt ol>li:first-child::before,.mw-parser-output .hlist li ol>li:first-child::before{content:" ("counter(listitem)"\a0 "}</style><style data-mw-deduplicate="TemplateStyles:r1314944253">.mw-parser-output .navbox{box-sizing:border-box;border:1px solid #a2a9b1;width:100%;clear:both;font-size:88%;text-align:center;padding:1px;margin:1em auto 0}.mw-parser-output .navbox .navbox{margin-top:0}.mw-parser-output .navbox+.navbox,.mw-parser-output .navbox+.navbox-styles+.navbox{margin-top:-1px}.mw-parser-output .navbox-inner,.mw-parser-output .navbox-subgroup{width:100%}.mw-parser-output .navbox-group,.mw-parser-output .navbox-title,.mw-parser-output .navbox-abovebelow{padding:0.25em 1em;line-height:1.5em;text-align:center}.mw-parser-output .navbox-group{white-space:nowrap;text-align:right}.mw-parser-output .navbox,.mw-parser-output .navbox-subgroup{background-color:#fdfdfd;color:inherit}.mw-parser-output .navbox-list{line-height:1.5em;border-color:#fdfdfd}.mw-parser-output .navbox-list-with-group{text-align:left;border-left-width:2px;border-left-style:solid}.mw-parser-output tr+tr>.navbox-abovebelow,.mw-parser-output tr+tr>.navbox-group,.mw-parser-output tr+tr>.navbox-image,.mw-parser-output tr+tr>.navbox-list{border-top:2px solid #fdfdfd}.mw-parser-output .navbox-title{background-color:#ccf;color:inherit}.mw-parser-output .navbox-abovebelow,.mw-parser-output .navbox-group,.mw-parser-output .navbox-subgroup .navbox-title{background-color:#ddf;color:inherit}.mw-parser-output .navbox-subgroup .navbox-group,.mw-parser-output .navbox-subgroup .navbox-abovebelow{background-color:#e6e6ff;color:inherit}.mw-parser-output .navbox-even{background-color:#f7f7f7;color:inherit}.mw-parser-output .navbox-odd{background-color:transparent;color:inherit}.mw-parser-output .navbox .hlist td dl,.mw-parser-output .navbox .hlist td ol,.mw-parser-output .navbox .hlist td ul,.mw-parser-output .navbox td.hlist dl,.mw-parser-output .navbox td.hlist ol,.mw-parser-output .navbox td.hlist ul{padding:0.125em 0}.mw-parser-output .navbox .navbar{display:block;font-size:100%}.mw-parser-output .navbox-title .navbar{float:left;text-align:left;margin-right:0.5em}body.skin--responsive .mw-parser-output .navbox-image img{max-width:none!important}@media print{body.ns-0 .mw-parser-output .navbox{display:none!important}}</style><style data-mw-deduplicate="TemplateStyles:r886047488">.mw-parser-output .nobold{font-weight:normal}</style><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886047488" /><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886047488" /><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886047488" /><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886047488" /></div><div role="navigation" class="navbox" aria-labelledby="Extant&#95;families&#95;in&#95;order&#95;Rodentia3221" style="padding:3px"><table class="nowraplinks hlist mw-collapsible autocollapse navbox-inner" style="border-spacing:0;background:transparent;color:inherit"><tbody><tr><th scope="col" class="navbox-title" colspan="2"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333133064" /><style data-mw-deduplicate="TemplateStyles:r1239400231">.mw-parser-output .navbar{display:inline;font-size:88%;font-weight:normal}.mw-parser-output .navbar-collapse{float:left;text-align:left}.mw-parser-output .navbar-boxtext{word-spacing:0}.mw-parser-output .navbar ul{display:inline-block;white-space:nowrap;line-height:inherit}.mw-parser-output .navbar-brackets::before{margin-right:-0.125em;content:"[ "}.mw-parser-output .navbar-brackets::after{margin-left:-0.125em;content:" ]"}.mw-parser-output .navbar li{word-spacing:-0.125em}.mw-parser-output .navbar a>span,.mw-parser-output .navbar a>abbr{text-decoration:inherit}.mw-parser-output .navbar-mini abbr{font-variant:small-caps;border-bottom:none;text-decoration:none;cursor:inherit}.mw-parser-output .navbar-ct-full{font-size:114%;margin:0 7em}.mw-parser-output .navbar-ct-mini{font-size:114%;margin:0 4em}html.skin-theme-clientpref-night .mw-parser-output .navbar li a abbr{color:var(--color-base)!important}@media(prefers-color-scheme:dark){html.skin-theme-clientpref-os .mw-parser-output .navbar li a abbr{color:var(--color-base)!important}}@media print{.mw-parser-output .navbar{display:none!important}}</style><div class="navbar plainlinks hlist navbar-mini"><ul><li class="nv-view"><a href="/wiki/Template:Rodents" title="Template:Rodents"><abbr title="View this template">v</abbr></a></li><li class="nv-talk"><a href="/wiki/Template_talk:Rodents" title="Template talk:Rodents"><abbr title="Discuss this template">t</abbr></a></li><li class="nv-edit"><a href="/wiki/Special:EditPage/Template:Rodents" title="Special:EditPage/Template:Rodents"><abbr title="Edit this template">e</abbr></a></li></ul></div><div id="Extant&#95;families&#95;in&#95;order&#95;Rodentia3221" style="font-size:114%;margin:0 4em">Extant families in order <a href="/wiki/Rodent" title="Rodent">Rodentia</a></div></th></tr><tr><td class="navbox-abovebelow" colspan="2"><div>
<ul><li>Kingdom: <a href="/wiki/Animal" title="Animal">Animalia</a></li>
<li>Phylum: <a href="/wiki/Chordate" title="Chordate">Chordata</a></li>
<li>Class: <a href="/wiki/Mammal" title="Mammal">Mammalia</a></li>
<li>Infraclass: <a href="/wiki/Eutheria" title="Eutheria">Eutheria</a></li>
<li>Superorder: <a href="/wiki/Euarchontoglires" title="Euarchontoglires">Euarchontoglires</a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Sciuromorpha" title="Sciuromorpha">Sciuromorpha</a><br /><small><span style="color:#696969"><span class="nobold">("Squirrel-like")</span></span></small></th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Mountain_beaver" title="Mountain beaver">Aplodontiidae <small>(Mountain beaver)</small></a></li>
<li><a class="mw-selflink selflink">Gliridae <small>(Dormice)</small></a></li>
<li><a href="/wiki/Squirrel" title="Squirrel">Sciuridae <small>(Squirrels, chipmunks, marmots, susliks and prairie dogs)</small></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Castorimorpha" title="Castorimorpha">Castorimorpha</a><br /><small><span style="color:#696969"><span class="nobold">("Beaver-like")</span></span></small></th><td class="navbox-list-with-group navbox-list navbox-even" style="width:100%;padding:0"><div style="padding:0 0.25em">
<dl><dt><a href="/wiki/Castoridae" title="Castoridae">Castoroidea</a></dt>
<dd></dd>
<dd><a href="/wiki/Castoridae" title="Castoridae">Castoridae <small>(Beavers)</small></a></dd></dl>
<dl><dt><a href="/wiki/Geomyoidea" title="Geomyoidea">Geomyoidea</a></dt>
<dd></dd>
<dd><a href="/wiki/Gopher" title="Gopher">Geomyidae <small>(Pocket gophers)</small></a></dd>
<dd><a href="/wiki/Heteromyidae" title="Heteromyidae">Heteromyidae <small>(Kangaroo rats and mice, pocket mice)</small></a></dd></dl>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Myomorpha" title="Myomorpha">Myomorpha</a><br /><small><span style="color:#696969"><span class="nobold">("Mouse-like")</span></span></small></th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em">
<dl><dt><a href="/wiki/Dipodidae" class="mw-redirect" title="Dipodidae">Dipodoidea</a></dt>
<dd></dd>
<dd><a hre[82512261843] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=163840 len=32768 cached=131072 start=32768 eof=false
[84580145595] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[84590116215] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
f="/wiki/Dipodidae" class="mw-redirect" title="Dipodidae">Dipodidae <small>(Jerboas, jumping mice and birch mice)</small></a></dd></dl>
<dl><dt><a href="/wiki/Muroidea" title="Muroidea">Muroidea</a></dt>
<dd></dd>
<dd><a href="/wiki/Platacanthomyidae" title="Platacanthomyidae">Platacanthomyidae <small>(Oriental dormice)</small></a></dd>
<dd><a href="/wiki/Spalacidae" title="Spalacidae">Spalacidae <small>(Zokors, bamboo rats, mole rats, blind mole rats)</small></a></dd>
<dd><a href="/wiki/Mouse-like_hamster" title="Mouse-like hamster">Calomyscidae <small>(Mouse-like hamsters)</small></a></dd>
<dd><a href="/wiki/Nesomyidae" title="Nesomyidae">Nesomyidae <small>(Malagasy rats and relatives)</small></a></dd>
<dd><a href="/wiki/Cricetidae" title="Cricetidae">Cricetidae <small>(Hamsters and relatives)</small></a></dd>
<dd><a href="/wiki/Muridae" title="Muridae">Muridae <small>(House mouse and relatives)</small></a></dd></dl>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Anomaluromorpha" title="Anomaluromorpha">Anomaluromorpha</a><br /><small><span style="color:#696969"><span class="nobold">("Anomalure-like")</span></span></small></th><td class="navbox-list-with-group navbox-list navbox-even" style="width:100%;padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Anomalure" title="Anomalure">Anomaluridae <small>(Anomalures)</small></a></li>
<li><a href="/wiki/Pedetidae" title="Pedetidae">Pedetidae <small>(Springhares)</small></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Hystricomorpha" title="Hystricomorpha">Hystricomorpha</a><br /><small><span style="color:#696969"><span class="nobold">("Porcupine-like")</span></span></small></th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Gundi" title="Gundi">Ctenodactylidae <small>(Gundis)</small></a></li>
<li><a href="/wiki/Diatomyidae" title="Diatomyidae">Diatomyidae <small>(Laotian rock rat)</small></a></li>
<li><a href="/wiki/Old_World_porcupine" title="Old World porcupine">Hystricidae <small>(Old World porcupines)</small></a></li></ul>
<dl><dt><a href="/wiki/Phiomorpha" title="Phiomorpha">Phiomorpha</a></dt>
<dd></dd>
<dd><a href="/wiki/Blesmol" title="Blesmol">Bathyergidae <small>(Blesmols)</small></a></dd>
<dd><a href="/wiki/Dassie_rat" title="Dassie rat">Petromuridae <small>(Dassie rat)</small></a></dd>
<dd><a href="/wiki/Cane_rat" title="Cane rat">Thryonomyidae <small>(Cane rats)</small></a></dd></dl>
<dl><dt><a href="/wiki/Caviomorpha" title="Caviomorpha">Caviomorpha <small>(New World hystricognaths)</small></a></dt>
<dd></dd>
<dd><a href="/wiki/New_World_porcupine" title="New World porcupine">Erethizontidae <small>(New World porcupines)</small></a></dd>
<dd><a href="/wiki/Caviidae" title="Caviidae">Caviidae <small>(Cavies)</small></a></dd>
<dd><a href="/wiki/Paca" title="Paca">Cuniculidae <small>(Pacas)</small></a></dd>
<dd><a href="/wiki/Dasyproctidae" title="Dasyproctidae">Dasyproctidae <small>(Agoutis and acouchis)</small></a></dd>
<dd><a href="/wiki/Dinomyidae" title="Dinomyidae">Dinomyidae <small>(Pacarana)</small></a></dd>
<dd><a href="/wiki/Tuco-tuco" title="Tuco-tuco">Ctenomyidae <small>(Tuco-tucos)</small></a></dd>
<dd><a href="/wiki/Echimyidae" title="Echimyidae">Echimyidae <small>(Spiny rats, coypus, hutias)</small></a></dd>
<dd><a href="/wiki/Octodontidae" title="Octodontidae">Octodontidae <small>(Degus and relatives)</small></a></dd>
<dd><a href="/wiki/Chinchilla_rat" title="Chinchilla rat">Abrocomidae <small>(Chinchilla rats)</small></a></dd>
<dd><a href="/wiki/Chinchillidae" title="Chinchillidae">Chinchillidae <small>(Chinchillas and viscachas)</small></a></dd></dl>
</div></td></tr></tbody></table></div>
<div class="navbox-styles"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333133064" /><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1314944253" /></div><div role="navigation" class="navbox" aria-labelledby="Extant&#95;species&#95;of&#95;family&#95;Gliridae&#95;(Dormice)5123" style="padding:3px"><table class="nowraplinks hlist mw-collapsible mw-collapsed navbox-inner" style="border-spacing:0;background:transparent;color:inherit"><tbody><tr><th scope="col" class="navbox-title" colspan="2"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333133064" /><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1239400231" /><div class="navbar plainlinks hlist navbar-mini"><ul><li class="nv-view"><a href="/wiki/Template:Gliridae_nav" title="Template:Gliridae nav"><abbr title="View this template">v</abbr></a></li><li class="nv-talk"><a href="/wiki/Template_talk:Gliridae_nav" title="Template talk:Gliridae nav"><abbr title="Discuss this template">t</abbr></a></li><li class="nv-edit"><a href="/wiki/Special:EditPage/Template:Gliridae_nav" title="Special:EditPage/Template:Gliridae nav"><abbr title="Edit this template">e</abbr></a></li></ul></div><div id="Extant&#95;species&#95;of&#95;family&#95;Gliridae&#95;(Dormice)5123" style="font-size:114%;margin:0 4em">Extant species of family <a class="mw-selflink selflink">Gliridae <small>(Dormice)</small></a></div></th></tr><tr><td class="navbox-abovebelow" colspan="2"><div>
<ul><li>Kingdom <a href="/wiki/Animal" title="Animal">Animalia</a></li>
<li>Phylum <a href="/wiki/Chordate" title="Chordate">Chordata</a></li>
<li>Class <a href="/wiki/Mammal" title="Mammal">Mammalia</a></li>
<li>Order <a href="/wiki/Rodent" title="Rodent">Rodentia</a></li>
<li>Suborder <a href="/wiki/Sciuromorpha" title="Sciuromorpha">Sciuromorpha</a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Graphiurus" title="Graphiurus">Graphiurinae</a></th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em"></div><table class="nowraplinks navbox-subgroup" style="border-spacing:0"><tbody><tr><th id="Graphiurus(African&#95;dormice)1295" scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Graphiurus" title="Graphiurus">Graphiurus</a></i><br /><small><span style="color:#696969"><span class="nobold">(African dormice)</span></span></small></th><td class="navbox-list-with-group navbox-list navbox-odd" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Angolan_African_dormouse" title="Angolan African dormouse">Angolan African dormouse <i>(Graphiurus angolensis)</i></a></li>
<li><a href="/wiki/Christy%27s_dormouse" title="Christy&#39;s dormouse">Christy's dormouse <i>(Graphiurus christyi)</i></a></li>
<li><a href="/wiki/Jentink%27s_dormouse" title="Jentink&#39;s dormouse">Jentink's dormouse <i>(Graphiurus crassicaudatus)</i></a></li>
<li><a href="/wiki/Johnston%27s_African_dormouse" title="Johnston&#39;s African dormouse">Johnston's African dormouse <i>(Graphiurus johnstoni)</i></a></li>
<li><a href="/wiki/Kellen%27s_dormouse" title="Kellen&#39;s dormouse">Kellen's dormouse <i>(Graphiurus kelleni)</i></a></li>
<li><a href="/wiki/Lorrain_dormouse" title="Lorrain dormouse">Lorrain dormouse <i>(Graphiurus lorraineus)</i></a></li>
<li><a href="/wiki/Small-eared_dormouse" title="Small-eared dormouse">Small-eared dormouse <i>(Graphiurus microtis)</i></a></li>
<li><a href="/wiki/Monard%27s_dormouse" title="Monard&#39;s dormouse">Monard's dormouse <i>(Graphiurus monardi)</i></a></li>
<li><a href="/wiki/Woodland_dormouse" title="Woodland dormouse">Woodland dormouse <i>(Graphiurus murinus)</i></a></li>
<li><a href="/wiki/Nagtglas%27s_African_dormouse" title="Nagtglas&#39;s African dormouse">Nagtglas's African dormouse <i>(Graphiurus nagtglasii)</i></a></li>
<li><a href="/wiki/Spectacled_dormouse" title="Spectacled dormouse">Spectacled dormouse/Namtap <i>(Graphiurus ocularis)</i></a></li>
<li><a href="/wiki/Rock_dormouse" title="Rock dormouse">Rock dormouse <i>(Graphiurus platyops)</i></a></li>
<li><a href="/wiki/Stone_dormouse" title="Stone dormouse">Stone dormouse <i>(Graphiurus rupicola)</i></a></li>
<li><a href="/wiki/Silent_dormouse" title="Silent dormouse">Silent dormouse <i>(Graphiurus surdus)</i></a></li>
<li><a href="/wiki/Walter_Verheyen%27s_African_dormouse" title="Walter Verheyen&#39;s African dormouse">Walter Verheyen's African dormouse <i>(Graphiurus walterverheyeni)</i></a></li></ul>
</div></td></tr></tbody></table><div></div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Leithiinae" title="Leithiinae">Leithiinae</a></th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em"></div><table class="nowraplinks navbox-subgroup" style="border-spacing:0"><tbody><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Chinese_dormouse" title="Chinese dormouse">Chaetocauda</a></i></th><td class="navbox-list-with-group navbox-list navbox-even" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Chinese_dormouse" title="Chinese dormouse">Chinese dormouse <i>(Chaetocauda sichuanensis)</i></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Forest_dormouse" title="Forest dormouse">Dryomys</a></i><br /><small><span style="color:#696969"><span class="nobold">(Forest dormice)</span></span></small></th><td class="navbox-list-with-group navbox-list navbox-odd" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Woolly_dormouse" title="Woolly dormouse">Woolly dormouse <i>(Dryomys laniger)</i></a></li>
<li><a href="/wiki/Balochistan_forest_dormouse" title="Balochistan forest dormouse">Balochistan forest dormouse <i>(Dryomys niethammeri)</i></a></li>
<li><a href="/wiki/Forest_dormouse" title="Forest dormouse">Forest dormouse <i>(Dryomys nitedula)</i></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Eliomys" title="Eliomys">Eliomys</a></i></th><td class="navbox-list-with-group navbox-list navbox-even" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Asian_garden_dormouse" title="Asian garden dormouse">Asian garden dormouse <i>(Eliomys melanurus)</i></a></li>
<li><a href="/wiki/Maghreb_garden_dormouse" title="Maghreb garden dormouse">Maghreb garden dormouse <i>(Eliomys munbyanus)</i></a></li>
<li><a href="/wiki/Garden_dormouse" title="Garden dormouse">Garden dormouse <i>(Eliomys quercinus)</i></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Hazel_dormouse" title="Hazel dormouse">Muscardinus</a></i></th><td class="navbox-list-with-group navbox-list navbox-odd" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Hazel_dormouse" title="Hazel dormouse">Hazel dormouse <i>(Muscardinus avellanarius)</i></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Myomimus" title="Myomimus">Myomimus</a></i></th><td class="navbox-list-with-group navbox-list navbox-even" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Masked_mouse-tailed_dormouse" title="Masked mouse-tailed dormouse">Masked mouse-tailed dormouse <i>(Myomimus personatus)</i></a></li>
<li><a href="/wiki/Roach%27s_mouse-tailed_dormouse" title="Roach&#39;s mouse-tailed dormouse">Roach's mouse-tailed dormouse <i>(Myomimus roachi)</i></a></li>
<li><a href="/wiki/Setzer%27s_mouse-tailed_dormouse" title="Setzer&#39;s mouse-tailed dormouse">Setzer's mouse-tailed dormouse <i>(Myomimus setzeri)</i></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Desert_dormouse" title="Desert dormouse">Selevinia</a></i></th><td class="navbox-list-with-group navbox-list navbox-odd" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Desert_dormouse" title="Desert dormouse">Desert dormouse <i>(Selevinia betpakdalaensis)</i></a></li></ul>
</div></td></tr></tbody></table><div></div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%"><a href="/wiki/Glirinae" title="Glirinae">Glirinae</a></th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em"></div><table class="nowraplinks navbox-subgroup" style="border-spacing:0"><tbody><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Japanese_dormouse" title="Japanese dormouse">Glirulus</a></i></th><td class="navbox-list-with-group navbox-list navbox-even" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/Japanese_dormouse" title="Japanese dormouse">Japanese dormouse <i>(Glirulus japonicus)</i></a></li></ul>
</div></td></tr><tr><th scope="row" class="navbox-group" style="width:9em"><i><a href="/wiki/Glis_(genus)" title="Glis (genus)">Glis</a></i></th><td class="navbox-list-with-group navbox-list navbox-odd" style="padding:0"><div style="padding:0 0.25em">
<ul><li><a href="/wiki/European_edible_dormouse" title="European edible dormouse">European edible dormouse <i>(Glis glis)</i></a></li>
<li><a href="/wiki/Iranian_edible_dormouse" title="Iranian edible dormouse">Iranian edible dormouse <i>(Glis persicus)</i></a></li></ul>
</div></td></tr></tbody></table><div></div></td></tr><tr><td class="navbox-abovebelow" colspan="2"><div><b><a href="/wiki/Category:Dormice" title="Category:Dormice">Category</a></b></div></td></tr></tbody></table></div>
<div class="navbox-styles"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333133064" /><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1314944253" /></div><div role="navigation" class="navbox" aria-labelledby="Taxon&#95;identifiers2744" style="padding:3px"><table class="nowraplinks hlist navbox-inner" style="border-spacing:0;background:transparent;color:inherit"><tbody><tr><th scope="col" class="navbox-title" colspan="2"><div id="Taxon&#95;identifiers2744" style="font-size:114%;margin:0 4em"><a href="/wiki/Help:Taxon_identifiers" title="Help:Taxon identifiers">Taxon identifiers</a></div></th></tr><tr><th scope="row" class="navbox-group" style="width:1%;text-align: left;"><i>Gliridae</i></th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em">
<ul><li><span style="white-space:nowrap;"><a href="/wiki/Wikidata" title="Wikidata">Wikidata</a>: <span class="uid"><span class="external"><a href="https://www.wikidata.org/wiki/Q108235" class="extiw" title="wikidata:Q108235">Q108235</a></span></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Wikispecies" title="Wikispecies">Wikispecies</a>: <span class="uid"><span class="external"><a href="https://species.wikimedia.org/wiki/Gliridae" class="extiw" title="wikispecies:Gliridae">Gliridae</a></span></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Animal_Diversity_Web" title="Animal Diversity Web">ADW</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://animaldiversity.org/accounts/Gliridae/">Gliridae</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Barcode_of_Life_Data_System" title="Barcode of Life Data System">BOLD</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://bench.boldsystems.org/index.php/TaxBrowser_TaxonPage?taxid=142622">142622</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Catalogue_of_Life" title="Catalogue of Life">CoL</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://www.catalogueoflife.org/data/taxon/623HW">623HW</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Encyclopedia_of_Life" title="Encyclopedia of Life">EoL</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://eol.org/pages/47051528">47051528</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/EPPO_Code" title="EPPO Code">EPPO</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://gd.eppo.int/taxon/1GLISF">1GLISF</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Global_Biodiversity_Information_Facility" title="Global Biodiversity Information Facility">GBI[84848885319] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=180224 len=32768 cached=131072 start=49152 eof=false
[86744442510] [[32mINFO [0m] [http] [CPU1] http: background task: read 16384 bytes from TLS
[86754274926] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 16384 bytes to foreground
F</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://www.gbif.org/species/3240562">3240562</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/INaturalist" title="INaturalist">iNaturalist</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://inaturalist.org/taxa/71385">71385</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Interim_Register_of_Marine_and_Nonmarine_Genera" title="Interim Register of Marine and Nonmarine Genera">IRMNG</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://www.irmng.org/aphia.php?p=taxdetails&amp;id=104259">104259</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Integrated_Taxonomic_Information_System" title="Integrated Taxonomic Information System">ITIS</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://www.itis.gov/servlet/SingleRpt/SingleRpt?search_topic=TSN&amp;search_value=951282">951282</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Mammal_Species_of_the_World" title="Mammal Species of the World">MSW</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://www.departments.bucknell.edu/biology/resources/msw3/browse.asp?s=y&amp;id=12500001">12500001</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/National_Biodiversity_Network" title="National Biodiversity Network">NBN</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://species.nbnatlas.org/species/NHMSYS0000376184">NHMSYS0000376184</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/National_Center_for_Biotechnology_Information" title="National Center for Biotechnology Information">NCBI</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://www.ncbi.nlm.nih.gov/Taxonomy/Browser/wwwtax.cgi?mode=Info&amp;id=30650">30650</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Open_Tree_of_Life" title="Open Tree of Life">Open Tree of Life</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://tree.opentreeoflife.org/taxonomy/browse?id=425409">425409</a></span></span></li>
<li><span style="white-space:nowrap;"><a href="/wiki/Paleobiology_Database" title="Paleobiology Database">Paleobiology Database</a>: <span class="uid"><a rel="nofollow" class="external text" href="https://paleobiodb.org/classic/basicTaxonInfo?taxon_no=41578">41578</a></span></span></li></ul>
</div></td></tr></tbody></table></div>
<div class="navbox-styles"><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1333133064" /><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r1314944253" /></div><div role="navigation" class="navbox authority-control" aria-labelledby="Authority&#95;control&#95;databases&#95;frameless&amp;#124;text-top&amp;#124;10px&amp;#124;alt=Edit&#95;this&#95;at&#95;Wikidata&amp;#124;link=https&amp;#58;//www.wikidata.org/wiki/Q108235#identifiers&amp;#124;class=noprint&amp;#124;Edit&#95;this&#95;at&#95;Wikidata775" style="padding:3px"><table class="nowraplinks hlist mw-collapsible autocollapse navbox-inner" style="border-spacing:0;background:transparent;color:inherit"><tbody><tr><th scope="col" class="navbox-title" colspan="2"><div id="Authority&#95;control&#95;databases&#95;frameless&amp;#124;text-top&amp;#124;10px&amp;#124;alt=Edit&#95;this&#95;at&#95;Wikidata&amp;#124;link=https&amp;#58;//www.wikidata.org/wiki/Q108235#identifiers&amp;#124;class=noprint&amp;#124;Edit&#95;this&#95;at&#95;Wikidata775" style="font-size:114%;margin:0 4em"><a href="/wiki/Help:Authority_control" title="Help:Authority control">Authority control databases</a> <span class="mw-valign-text-top noprint" typeof="mw:File/Frameless"><a href="https://www.wikidata.org/wiki/Q108235#identifiers" title="Edit this at Wikidata"><img alt="Edit this at Wikidata" src="//upload.wikimedia.org/wikipedia/en/thumb/8/8a/OOjs_UI_icon_edit-ltr-progressive.svg/20px-OOjs_UI_icon_edit-ltr-progressive.svg.png" decoding="async" width="10" height="10" class="mw-file-element" data-file-width="20" data-file-height="20" /></a></span></div></th></tr><tr><th scope="row" class="navbox-group" style="width:1%">National</th><td class="navbox-list-with-group navbox-list navbox-odd" style="width:100%;padding:0"><div style="padding:0 0.25em"><ul><li><span class="uid"><a rel="nofollow" class="external text" href="https://id.loc.gov/authorities/sh85039104">United States</a></span></li><li><span class="uid"><a rel="nofollow" class="external text" href="https://kopkatalogs.lv/F?func=direct&amp;local_base=lnc10&amp;doc_number=000344498&amp;P_CON_LNG=ENG">Latvia</a></span></li><li><span class="uid"><a rel="nofollow" class="external text" href="https://www.nli.org.il/en/authorities/987007560320705171">Israel</a></span></li></ul></div></td></tr><tr><th scope="row" class="navbox-group" style="width:1%">Other</th><td class="navbox-list-with-group navbox-list navbox-even" style="width:100%;padding:0"><div style="padding:0 0.25em"><ul><li><span class="uid"><a rel="nofollow" class="external text" href="https://lux.collections.yale.edu/view/concept/2bb50abc-a04c-4f9f-af07-92429924b775">Yale LUX</a></span></li></ul></div></td></tr></tbody></table></div>
<!-- 
NewPP limit report
Parsed by mw‐web.codfw.main‐648f759785���bcz4p
Cached time: 20260420161336
Cache expiry: 2592000
Cache expiry source: Module:Citation/CS1 (os.date(%Y))
Reduced expiry: false
Complications: [vary��revision��sha1, prevent‐selective‐update, show‐toc]
CPU time usage: 1.002 seconds
Real time usage: 1.166 seconds
Preprocessor visited node count: 19447/1000000
Revision size: 20515/2097152 bytes
Post���expand include size: 196335/2097152 bytes
Template argument size: 26181/2097152 bytes
Highest expansion depth: 24/100
Expensive parser function count: 20/500
Unstrip recursion depth: 1/20
Unstrip post���expand size: 146776/5000000 bytes
Lua time usage: 0.702/10.000 seconds
Lua memory usage: 21939997/52428800 bytes
Number of Wikibase entities loaded: 16/500
-->
<!--
Transclusion expansion time report (%,ms,calls,template)
100.00% 1034.731      1 -total
 29.43%  304.531      1 Template:Automatic_taxobox
 18.69%  193.342      1 Template:Reflist
 13.39%  138.596      9 Template:Clade
 12.28%  127.047      1 Template:Taxonbar
  9.41%   97.377      7 Template:Lang
  8.95%   92.621      7 Template:Cite_web
  8.70%   89.975      1 Template:Fossil_range
  7.57%   78.345      5 Template:Navbox
  7.20%   74.495      1 Template:Phanerozoic_280px
-->

<!-- Render ID e270ba8a-3cd3-11f1-b208-ab127f3188dd -->

<!-- Saved in parser cache with key enwiki:pcache:438703:|#|:idhash:canonical and timestamp 20260420161336 and revision id 1348410573. Rendering was triggered because: page_view
 -->
</div><noscript><img src="https://en.wikipedia.org/wiki/Special:CentralAutoLogin/start?useformat=desktop&amp;type=1x1&amp;usesul3=1" alt="" width="1" height="1" style="border: none; position: absolute;"></noscript>
<div class="printfooter" data-nosnippet="">Retrieved from "<a dir="ltr" href="https://en.wikipedia.org/w/index.php?title=Dormouse&amp;oldid=1348410573">https://en.wikipedia.org/w/index.php?title=Dormouse&amp;oldid=1348410573</a>"</div></div>
					<div id="catlinks" class="catlinks" data-mw-interface=""><div id="mw-normal-catlinks" class="mw-normal-catlinks"><a href="/wiki/Help:Category" title="Help:Category">Categories</a>: <ul><li><a href="/wiki/Category:Dormice" title="Category:Dormice">Dormice</a></li><li><a href="/wiki/Category:Sciuromorpha" title="Category:Sciuromorpha">Sciuromorpha</a></li><li><a href="/wiki/Category:Natural_Monuments_of_Japan" title="Category:Natural Monuments of Japan">Natural Monuments of Japan</a></li><li><a href="/wiki/Category:Extant_Eocene_first_appearances" title="Category:Extant Eocene first appearances">Extant Eocene first appearances</a></li></ul></div><div id="mw-hidden-catlinks" class="mw-hidden-catlinks mw-hidden-cats-hidden">Hidden categories: <ul><li><a href="/wiki/Category:Articles_with_short_description" title="Category:Articles with short description">Articles with short description</a></li><li><a href="/wiki/Category:Short_description_is_different_from_Wikidata" title="Category:Short description is different from Wikidata">Short description is different from Wikidata</a></li><li><a href="/wiki/Category:Articles_with_%27species%27_microformats" title="Category:Articles with &#039;species&#039; microformats">Articles with &#039;species&#039; microformats</a></li><li><a href="/wiki/Category:Articles_containing_Middle_English_(1100-1500)-language_text" title="Category:Articles containing Middle English (1100-1500)-language text">Articles containing Middle English (1100-1500)-language text</a></li><li><a href="/wiki/Category:Articles_containing_Old_Norse-language_text" title="Category:Articles containing Old Norse-language text">Articles containing Old Norse-language text</a></li><li><a href="/wiki/Category:Articles_containing_Anglo-Norman-language_text" title="Category:Articles containing Anglo-Norman-language text">Articles containing Anglo-Norman-language text</a></li><li><a href="/wiki/Category:Articles_containing_Latin-language_text" title="Category:Articles containing Latin-language text">Articles containing Latin-language text</a></li><li><a href="/wiki/Category:Articles_containing_Sanskrit-language_text" title="Category:Articles containing Sanskrit-language text">Articles containing Sanskrit-language text</a></li><li><a href="/wiki/Category:Articles_containing_Ancient_Greek_(to_1453)-language_text" title="Category:Articles containing Ancient Greek (to 1453)-language text">Articles containing Ancient Greek (to 1453)-language text</a></li><li><a href="/wiki/Category:Commons_link_from_Wikidata" title="Category:Commons link from Wikidata">Commons link from Wikidata</a></li><li><a href="/wiki/Category:Articles_with_German-language_sources_(de)" title="Category:Articles with German-language sources (de)">Articles with German-language sources (de)</a></li></ul></div></div>
				</div>
			</main>
			
		</div>
		<div class="mw-footer-container">
			
<footer id="footer" class="mw-footer" >
	<ul id="footer-info">
	<li id="footer-info-lastmod"> This page was last edited on 12 April 2026, at 14:26<span class="anonymous-show">&#160;(UTC)</span>.</li>
	<li id="footer-info-copyright">Text is available under the <a href="/wiki/Wikipedia:Text_of_the_Creative_Commons_Attribution-ShareAlike_4.0_International_License" title="Wikipedia:Text of the Creative Commons Attribution-ShareAlike 4.0 International License">Creative Commons Attribution-ShareAlike 4.0 License</a>;
additional terms may apply. By using this site, you agree to the <a href="https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Terms_of_Use" class="extiw" title="foundation:Special:MyLanguage/Policy:Terms of Use">Terms of Use</a> and <a href="https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Privacy_policy" class="extiw" title="foundation:Special:MyLanguage/Policy:Privacy policy">Privacy Policy</a>. Wikipedia® is a registered trademark of the <a rel="nofollow" class="external text" href="https://wikimediafoundation.org/">Wikimedia Foundation, Inc.</a>, a non-profit organization.</li>
</ul>

	<ul id="footer-places">
	<li id="footer-places-privacy"><a href="https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Privacy_policy">Privacy policy</a></li>
	<li id="footer-places-about"><a href="/wiki/Wikipedia:About">About Wikipedia</a></li>
	<li id="footer-places-disclaimers"><a href="/wiki/Wikipedia:General_disclaimer">Disclaimers</a></li>
	<li id="footer-places-contact"><a href="//en.wikipedia.org/wiki/Wikipedia:Contact_us">Contact Wikipedia</a></li>
	<li id="footer-places-legal-safety-contacts"><a href="https://foundation.wikimedia.org/wiki/Special:MyLanguage/Legal:Wikimedia_Foundation_Legal_and_Safety_Contact_Information">Legal &amp; safety contacts</a></li>
	<li id="footer-places-wm-codeofconduct"><a href="https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Universal_Code_of_Conduct">Code of Conduct</a></li>
	<li id="footer-places-developers"><a href="https://developer.wikimedia.org">Developers</a></li>
	<li id="footer-places-statslink"><a href="https://stats.wikimedia.org/#/en.wikipedia.org">Statistics</a></li>
	<li id="footer-places-cookiestatement"><a href="https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Cookie_statement">Cookie statement</a></li>
	<li id="footer-places-mobileview"><a href="//en.wikipedia.org/w/index.php?title=Dormouse&amp;mobileaction=toggle_view_mobile" class="noprint stopMobileRedirectToggle">Mobile view</a></li>
</ul>

	<ul id="footer-icons" class="noprint">
	<li id="footer-copyrightico"><a href="https://www.wikimedia.org/" class="cdx-button cdx-button--fake-button cdx-button--size-large cdx-button--fake-button--enabled"><picture><source media="(min-width: 500px)" srcset="/static/images/footer/wikimedia-button.svg" width="84" height="29"><img src="/static/images/footer/wikimedia.svg" width="25" height="25" alt="Wikimedia Foundation" lang="en" loading="lazy"></picture></a></li>
	<li id="footer-poweredbyico"><a href="https://www.mediawiki.org/" class="cdx-button cdx-button--fake-button cdx-button--size-large cdx-button--fake-button--enabled"><picture><source media="(min-width: 500px)" srcset="/w/resources/assets/poweredby_mediawiki.svg" width="88" height="31"><img src="/w/resources/assets/mediawiki_compact.svg" alt="Powered by MediaWiki" lang="en" width="25" height="25" loading="lazy"></picture></a></li>
</ul>

</footer>

		</div>
	</div> 
</div> 
<div class="vector-header-container vector-sticky-header-container no-font-mode-scale">
	<div id="vector-sticky-header" class="vector-sticky-header">
		<div class="vector-sticky-header-start">
			<div class="vector-sticky-header-icon-start vector-button-flush-left" aria-hidden="true">
				<button class="cdx-button cdx-button--weight-quiet cdx-button--icon-only vector-sticky-header-search-toggle" tabindex="-1" data-event-name="ui.vector-sticky-search-form.icon"><span class="vector-icon mw-ui-icon-search mw-ui-icon-wikimedia-search"></span>

<span>Search</span>
			</button>
		</div>
			
		<div role="search" class="vector-search-box-vue  vector-search-box-show-thumbnail vector-search-box">
			<div class="vector-typeahead-search-container">
				<div class="cdx-typeahead-search cdx-typeahead-search--show-thumbnail">
					<form action="/w/index.php" id="vector-sticky-search-form" class="cdx-search-input cdx-search-input--has-end-button">
						<div  class="cdx-search-input__input-wrapper"  data-search-loc="header-moved">
							<div class="cdx-text-input cdx-text-input--has-start-icon">
								<input
									class="cdx-text-input__input mw-searchInput" autocomplete="off"
									
									type="search" name="search" placeholder="Search Wikipedia">
								<span class="cdx-text-input__icon cdx-text-input__start-icon"></span>
							</div>
							<input type="hidden" name="title" value="Special:Search">
						</div>
						<button class="cdx-button cdx-search-input__end-button">Search</button>
					</form>
				</div>
			</div>
		</div>
		<div class="vector-sticky-header-context-bar">
				<nav aria-label="Contents" class="vector-toc-landmark">
						
					<div id="vector-sticky-header-toc" class="vector-dropdown mw-portlet mw-portlet-sticky-header-toc vector-sticky-header-toc vector-button-flush-left"  >
						<input type="checkbox" id="vector-sticky-header-toc-checkbox" role="button" aria-haspopup="true" data-event-name="ui.dropdown-vector-sticky-header-toc" class="vector-dropdown-checkbox "  aria-label="Toggle the table of contents"  >
						<label id="vector-sticky-header-toc-label" for="vector-sticky-header-toc-checkbox" class="vector-dropdown-label cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only " aria-hidden="true"  ><span class="vector-icon mw-ui-icon-listBullet mw-ui-icon-wikimedia-listBullet"></span>

<span class="vector-dropdown-label-text">Toggle the table of contents</span>
						</label>
						<div class="vector-dropdown-content">
					
						<div id="vector-sticky-header-toc-unpinned-container" class="vector-unpinned-container">
						</div>
					
			[86986255257] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=196608 len=32768 cached=131072 start=65536 eof=false
[87095951514] [[32mINFO [0m] [http] [CPU1] http: background task: read 5952 bytes from TLS
[87102397800] [[32mINFO [0m] [http] [CPU1] http: background task: forwarded 5952 bytes to foreground
			</div>
					</div>
			</nav>
				<div class="vector-sticky-header-context-bar-primary" aria-hidden="true" ><span lang="en" dir="ltr"><span class="mw-page-title-main">Dormouse</span></span></div>
			</div>
		</div>
		<div class="vector-sticky-header-end" aria-hidden="true">
			<div class="vector-sticky-header-icons">
				<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only" id="ca-talk-sticky-header" tabindex="-1" data-event-name="talk-sticky-header"><span class="vector-icon mw-ui-icon-speechBubbles mw-ui-icon-wikimedia-speechBubbles"></span>

<span></span>
			</a>
			<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only" id="ca-subject-sticky-header" tabindex="-1" data-event-name="subject-sticky-header"><span class="vector-icon mw-ui-icon-article mw-ui-icon-wikimedia-article"></span>

<span></span>
			</a>
			<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only" id="ca-history-sticky-header" tabindex="-1" data-event-name="history-sticky-header"><span class="vector-icon mw-ui-icon-wikimedia-history mw-ui-icon-wikimedia-wikimedia-history"></span>

<span></span>
			</a>
			<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only mw-watchlink" id="ca-watchstar-sticky-header" tabindex="-1" data-event-name="watch-sticky-header"><span class="vector-icon mw-ui-icon-wikimedia-star mw-ui-icon-wikimedia-wikimedia-star"></span>

<span></span>
			</a>
			<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only" id="ca-edit-sticky-header" tabindex="-1" data-event-name="wikitext-edit-sticky-header"><span class="vector-icon mw-ui-icon-wikimedia-wikiText mw-ui-icon-wikimedia-wikimedia-wikiText"></span>

<span></span>
			</a>
			<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only" id="ca-ve-edit-sticky-header" tabindex="-1" data-event-name="ve-edit-sticky-header"><span class="vector-icon mw-ui-icon-wikimedia-edit mw-ui-icon-wikimedia-wikimedia-edit"></span>

<span></span>
			</a>
			<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--icon-only" id="ca-viewsource-sticky-header" tabindex="-1" data-event-name="ve-edit-protected-sticky-header"><span class="vector-icon mw-ui-icon-wikimedia-editLock mw-ui-icon-wikimedia-wikimedia-editLock"></span>

<span></span>
			</a>
		</div>
			<div class="vector-sticky-header-buttons">
				<button class="cdx-button cdx-button--weight-quiet mw-interlanguage-selector" id="p-lang-btn-sticky-header" tabindex="-1" data-event-name="ui.dropdown-p-lang-btn-sticky-header"><span class="vector-icon mw-ui-icon-wikimedia-language mw-ui-icon-wikimedia-wikimedia-language"></span>

<span>70 languages</span>
			</button>
			<a href="#" class="cdx-button cdx-button--fake-button cdx-button--fake-button--enabled cdx-button--weight-quiet cdx-button--action-progressive" id="ca-addsection-sticky-header" tabindex="-1" data-event-name="addsection-sticky-header"><span class="vector-icon mw-ui-icon-speechBubbleAdd-progressive mw-ui-icon-wikimedia-speechBubbleAdd-progressive"></span>

<span>Add topic</span>
			</a>
		</div>
			<div class="vector-sticky-header-icon-end">
				<div class="vector-user-links">
				</div>
			</div>
		</div>
	</div>
</div>
<div class="mw-portlet mw-portlet-dock-bottom emptyPortlet" id="p-dock-bottom">
	<ul>
		
	</ul>
</div>
<script>(RLQ=window.RLQ||[]).push(function(){mw.config.set({"wgHostname":"mw-web.codfw.main-67b9b4586d-pbrsp","wgBackendResponseTime":136,"wgPageParseReport":{"limitreport":{"cputime":"1.002","walltime":"1.166","ppvisitednodes":{"value":19447,"limit":1000000},"revisionsize":{"value":20515,"limit":2097152},"postexpandincludesize":{"value":196335,"limit":2097152},"templateargumentsize":{"value":26181,"limit":2097152},"expansiondepth":{"value":24,"limit":100},"expensivefunctioncount":{"value":20,"limit":500},"unstrip-depth":{"value":1,"limit":20},"unstrip-size":{"value":146776,"limit":5000000},"entityaccesscount":{"value":16,"limit":500},"timingprofile":["100.00% 1034.731      1 -total"," 29.43%  304.531      1 Template:Automatic_taxobox"," 18.69%  193.342      1 Template:Reflist"," 13.39%  138.596      9 Template:Clade"," 12.28%  127.047      1 Template:Taxonbar","  9.41%   97.377      7 Template:Lang","  8.95%   92.621      7 Template:Cite_web","  8.70%   89.975      1 Template:Fossil_range","  7.57%   78.345      5 Template:Navbox","  7.20%   74.495      1 Template:Phanerozoic_280px"]},"scribunto":{"limitreport-timeusage":{"value":"0.702","limit":"10.000"},"limitreport-memusage":{"value":21939997,"limit":52428800}},"cachereport":{"origin":"mw-web.codfw.main-648f759785-bcz4p","timestamp":"20260420161336","ttl":2592000,"transientcontent":false,"expiry-source":"Module:Citation/CS1 (os.date(%Y))"}}});});</script>
<script type="application/ld+json">{"@context":"https:\/\/schema.org","@type":"Article","name":"Dormouse","url":"https:\/\/en.wikipedia.org\/wiki\/Dormouse","sameAs":"http:\/\/www.wikidata.org\/entity\/Q108235","mainEntity":"http:\/\/www.wikidata.org\/entity\/Q108235","author":{"@type":"Organization","name":"Contributors to Wikimedia projects"},"publisher":{"@type":"Organization","name":"Wikimedia Foundation, Inc.","logo":{"@type":"ImageObject","url":"https:\/\/www.wikimedia.org\/static\/images\/wmf-hor-googpub.png"}},"datePublished":"2004-01-22T06:51:12Z","dateModified":"2026-04-12T14:26:20Z","image":"https:\/\/upload.wikimedia.org\/wikipedia\/commons\/4\/43\/Graphiurus_spec_-murinus-1.jpg","headline":"family of mammals, the dormice"}</script>
</body>
</html>[87207538935] [[32mINFO [0m] [httpsd] [CPU3] httpsd: read handle=2 url=https://en.wikipedia.org/wiki/Dormouse offset=202560 len=32768 cached=131072 start=71488 eof=false
[87434850591] [[32mINFO [0m] [httpsd] [CPU3] httpsd: upstream EOF for handle=2 cached=131072
[?25h[1;95mTHING[0m[1;96m-OS[0m [2;94m[[0m[1;92mOK[0m[2;94m][0m [1;93m/[0m [1;96m>[0m [?25hattr_list /https/en.wikipedia.org/wiki/Dormouse
[?25lattr_list: failed: EIO
[?25h
```
</details>
