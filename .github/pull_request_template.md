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

<!-- Typed-first, Unix-second check (required for substantial changes, especially touching kernel/, abi/, bran/, stem/, or userspace/).
     See docs/migration/review-guidelines.md and:
     - docs/architecture/ontology.md
     - docs/architecture/concept-classification.md
     - docs/architecture/unix-projection.md -->

- **What canonical concept owns this meaning?**
  _Which typed-world object (`Thing`, `Place`, `Job`, `Space`, `Authority`, `Group`, `Task`, `Message`, `Presence`) primarily owns this change?_

- **Is this introducing truth or compatibility?**
  _State whether this change adds canonical typed-world truth or a compatibility surface._

- **If compatibility, what is the canonical typed-world concept underneath it?**
  _Describe the underlying canonical concept and why the compatibility form is still needed._

- **Are new names aligned with ontology and classification docs?**
  _Confirm naming aligns with `docs/architecture/ontology.md` and `docs/architecture/concept-classification.md`._
