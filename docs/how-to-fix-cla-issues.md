# How-to fix CLA issues

Fix by:

* giving the "Integrators" team write permission to the repo

Verify by:

* redelivering webhooks from the "Webhooks" section of the Settings for the repository

Make sure you're not looking at the webhoook for the old domain name (http://typesafe.com/contribute/cla/_github-hook), which can be deleted.

Recommend also to add 2 branch protection rules matching `main` and `[0-9].*.x` with:

* requiring the `typesafe-cla-validator` check
