<!-- homu-ignore:start -->
<!--
If this PR is related to an unstable feature or an otherwise tracked effort,
please link to the relevant tracking issue here. If you don't know of a related
tracking issue or there are none, feel free to ignore this.

This PR will get automatically assigned to a reviewer. In case you would like
a specific user to review your work, you can assign it to them by using

    r? <reviewer name>
-->
<!-- homu-ignore:end -->

<!-- Typed-first, Unix-second check (required for changes touching kernel/, abi/, bran/, stem/, or userspace/).
     See docs/migration/review-guidelines.md and docs/architecture/unix-projection.md §4 Step 4 for guidance. -->

**Canonical concept**: _What typed-world object (`Thing`, `Place`, `Job`, `Space`, `Authority`, `Group`, `Task`, `Message`, `Presence`) does this change primarily affect?_

**Unix projection**: _What Unix surface is exposed, and why is this surface a projection rather than canonical truth?_

**Typed-world naming check**: _Could this meaning be named in a canonical typed-world way instead? If not, why?_

**Unix term justification**: _Is each Unix term used because compatibility requires it, or because it is familiar?_ 
