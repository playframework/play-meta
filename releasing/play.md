## Before the release

Make sure that other teams inside Lightbend are aware of the upcoming release, even if it is a minor/patch one. For example:

1. [Lightbend Telemetry](https://developer.lightbend.com/docs/telemetry/current/home.html) Team
1. [Lightbend Platform](https://www.lightbend.com/lightbend-platform) Team
1. Akka Team

## Track your release

Create a [new Play release issue][].

[new Play release issue]: https://github.com/lightbend/play-lagom-team/issues/new?labels=release&template=play-release.md&title=Release+Play+x.y.z

# Play 2.4.x and onwards

Play 2.4 introduced a much more modular setup for the Play modules and documentation, where modules now span
multiple projects.  Each project needs to be released individually, and does not necessarily need to be
re-released every time Play itself is released.  When cutting a release of Play, you need to make the following
decisions:

* Does this release of Play require releasing the modules that depend on it, eg play-ebean and play-slick.  For
  a typical minor release, it would not.  For a major release, and often for pre releases of major releases, the
  answer is yes.

* Does this release of Play require updating the activator templates?  If it is the latest stable release that
  you are releasing, then yes.  Specifically, if the current stable release is 2.3.8, and you’re releasing
  2.3.9, then you need to publish the templates, but if you’re releasing 2.2.7, then you must not publish the
  templates, since doing so will revert the Play seed templates back to 2.2 when they should be on 2.3.

## Prerequisites

You need an account on `vegemite` with sudo access to the `play` account.  Generally, all actions
should be performed by ssh-ing into `vegemite`, starting a `screen` session, then switching to the `play` user.  By
starting a `screen` session, you ensure that your internet connection does not impact the release.

For tagging some releases (e.g. play-ws) you may need to enter your Github username and password. If you have
2FA enabled for Github (which you should have!) then you may need to generate a [personal access token](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/).

## If something goes wrong

The release process pushes all artifacts to maven central **and** bintray, but doesn’t “promote” them until all
publishing is complete and successful.

If the build failed during or after the promotion of either bintray or maven central artifacts, there is no
going back.  Published artifacts are immutable, they find their way into CDNs, into caches on developer
machines, and if there are two different versions of the same artifact out there, this can cause big problems.
Your only option is to fix the problem, and attempt another release of the next version.  Remember, version
numbers are cheap.

If the build failed during or before the publishing of artifacts, but not after either the bintray or maven
central promotion, you can drop the maven central staging repository and delete bintray version.  This can
either be done through their corresponding web interfaces, or by using the sonatypeDrop and bintrayUnpublish sbt
commands.

## Releasing Play

### Step 0 - release projects that Play depends on (play-json, play-ws, Twirl)

Prepare the branch for each project:

* Look for PRs that should be merged.
* Look at needs-backport issues/PRs (including closed ones).
* Look at issues/PRs tagged milestone version (including closed ones).
* Update any dependencies that are needed.
* Update any upstream projects (e.g make sure new play-ws is using new play-json)

May need to release these projects: play-json, play-ws, twirl

When ready:

```bash
cd deploy
./release --project <project> --branch <branch>
```

### Step 1 - release Play itself

Prepare the branch:

* Look for PRs that should be merged.
* Look at needs-backport issues/PRs (including closed ones).
* Look at issues/PRs tagged milestone version (including closed ones).
* Updated any dependencies that are needed (e.g. Dependencies.scala)

When ready:

```bash
cd deploy
./release --branch <branch>
```

Where branch is the branch of Play that you want to release, eg 2.7.x.  You will be prompted for the version
number and the next version number.  For minor releases, the default should usually be appropriate.

Once Play is released, you need to wait 10 minutes or so for a Maven central sync before you can perform any of
the remaining tasks.

**Verification**: You can check that the artifacts are available at Maven Central under play_\<scalaversion>.
https://repo1.maven.org/maven2/com/typesafe/play/

**Warning**: the play release will create a tag on the repository and push it to GitHub. That will trigger one
or two travis jobs that are granted to fail. The reason of the failure is a circular dependency with Omnidoc.
Play vX.Y.Z uses Omnidoc vX.Y.Z so those two travis jobs triggered by the tag and the changes in version.sbt
will be broken until you release Omnidoc few steps down this document.

**Warning**: If you are releasing a major version of Play which requires a new branch, for example, 2.7.0, you also need to configure MiMa before merging new features at this new branch. Ideally, do this as part of the release process.

### Step 2 - release external modules (play-slick, play-ebean, scalatestplus-play)

This includes modules like play-slick, play-grpc and scalatestplus-play.  Only release these if they need to be
released, generally for minor Play releases, there’s no reason to cut a new release of these, these libraries
are free to have their own release cycle.

**Note**: since we update omnidoc and the Play templates and seeds people reading the docs or starting a new
project will automatically see and use the latest minor versions of all modules, even if we don't patch all
modules directly to update dependencies.

You may need to bump the Play version in the external module, do this, commit, and depending on how major the
version bump is, push directly to the repo or go through a pull request.  Once that is done, to release an
external module:

```
cd deploy
./release --project <project> --branch <branch>
```

Where project is, for example, play-ebean, play-slick,...

Again, you will need to wait 10 minutes or so for a Maven central sync before you can perform any of the
remaining tasks. In the meantime you can see the staged artifacts here on OSS Sonatype. E.g. here is a search
for Play 2.6.3 artifacts:
[https://oss.sonatype.org/#nexus-search;gav~com.typesafe.play\~~2.6.3~~](https://oss.sonatype.org/#nexus-search;gav~com.typesafe.play~~2.6.3~~)

**Verification**: You can check that the artifacts are available at Maven Central under
play-slick_\<scalaversion\>, etc.

https://repo1.maven.org/maven2/com/typesafe/play/play-slick_2.12/
https://repo1.maven.org/maven2/com/typesafe/play/play-ebean_2.12/
https://repo1.maven.org/maven2/org/scalatestplus/play/scalatestplus-play_2.12/

**Verification**: when you run sbt new playframework/play-{scala,java}-seed.g8 it should pick up the new version
on Maven. Try the templates out. You may need to update them (possibly with templatecontrol?) if they don't work
with the new version of Play.

### Step 3 - release omnidoc

**Warning**: this is a compulsory step and the version vX.Y.Z of omnidoc released here must match the version
vX.Y.Z of Play released in step 1 above. There’s a circular dependency.

Omnidoc builds Play’s documentation from all the current versions of Play and its modules.  The first step you
need to take is to update the versions in the omnidoc build file for the branch of Play that you are releasing.
Update the Play version to the version of Play that you just released, and also update any external modules to
their latest version that is compatible with that version of Play.

Here’s an example update to the omnidoc 2.4.x branch.

```diff
$ git diff
diff --git a/project/OmnidocBuild.scala b/project/OmnidocBuild.scala
-  val playVersion       = sys.props.getOrElse("play.version",       "2.4.3")
+  val playVersion       = sys.props.getOrElse("play.version",       "2.4.4")
```

These changes can generally be pushed directly to GitHub.

To release omnidoc:

```
cd deploy
./release --project omnidoc --branch <branch>
```

**Verification**: check that the artifacts are available at Maven Central under play-omnidoc_<scalaversion>. It
may take a few minutes.  https://repo1.maven.org/maven2/com/typesafe/play/

Once that is done, you can update the docs on playframework.com, by running: (use the branch to push to in
https://github.com/playframework/play-generated-docs and the tag id you want to create)

    ./omnidoc/bin/deploy --branch <branch> --tag <tag> \
         /home/play/deploy/play-generated-docs

Verification: check there is a new tag <tag> at https://github.com/playframework/play-generated-docs project. It
should be on top of https://github.com/playframework/play-generated-docs/releases. The website should pick this
tagged version of the generated docs up to 10 minutes later. You can check that then using the following URL
pattern: https://www.playframework.com/documentation/<tag>/Home. For example
https://www.playframework.com/documentation/2.6.15/Home.

### Step 4 - update playframework templates and seeds

The “core” playframework templates describe Play’s overall feature set -- they are maintained by the Play team
and in the github.com/playframework repository.  Historically, there were issues with out of date Activator
templates, and confusion between what was supported and current, and what was third party and out of date.

The sample templates are run through https://github.com/playframework/templatecontrol -- this contains a list of
the example projects and seeds.  Using Template Control

Running templatecontrol will look for version numbers and automatically create a pull request with any needed
upgrades.  It's safe to run, but you will need local forks of all the template projects.

So here's how to check out all of the projects and fork them to your own local github: this assumes you have
https://hub.github.com/ installed. An Ubuntu/Debian PPA is here:
https://launchpad.net/~cpick/+archive/ubuntu/hub.

```bash
#!/bin/bash

declare -a templates=( \
   "play-java-chatroom-example" \
   "play-java-compile-di-example" \
   "play-java-dagger2-example" \
   "play-java-ebean-example" \
   "play-java-fileupload-example" \
   "play-java-forms-example" \
   "play-java-jpa-example" \
   "play-java-rest-api-example" \
   "play-java-seed.g8" \
   "play-java-starter-example" \
   "play-java-streaming-example" \
   "play-java-websocket-example" \
   "play-scala-anorm-example" \
   "play-scala-chatroom-example" \
   "play-scala-compile-di-example" \
   "play-scala-fileupload-example" \
   "play-scala-forms-example" \
   "play-scala-isolated-slick-example" \
   "play-scala-log4j2-example" \
   "play-scala-macwire-di-example" \
   "play-scala-rest-api-example" \
   "play-scala-secure-session-example" \
   "play-scala-seed.g8" \
   "play-scala-slick-example" \
   "play-scala-starter-example" \
   "play-scala-streaming-example" \
   "play-scala-tls-example" \
   "play-scala-websocket-example" \
)

for f in "${templates[@]}"; do
  git clone git@github.com:playframework/$f.git
  (cd $f; hub fork)
done
```

Run the script to create the templates inside the “templates” directory within the templatecontrol folder.

```
git clone https://github.com/playframework/templatecontrol.git
cd templatecontrol
```

Note  that you will need to set up a personal access token for github:

```
export TCONTROL_GITHUB_REMOTE=wsargent
export TCONTROL_GITHUB_USER=wsargent
export TCONTROL_GITHUB_OAUTH=<personal access token>
```

See https://github.com/playframework/templatecontrol#prerequisites for more details.

You will need to update templatecontrol's application.conf (or <play-branch-name>.conf) file with the new Play
version. Configuration files are in the src/main/resources directory:
https://github.com/lightbend/templatecontrol/blob/master/src/main/resources/

Create a PR from the version you want so that your changes are captured. For example:

```
git checkout -b upgrade-2.5.13
vi src/main/resources/<play-branch-name>.conf
```

After updating the .conf files, commit and push your changes, submit a new pull request to templatecontrol
project, merge it and then run templatecontrol:

```
sbt run
```

**Verification**: Check out a local copy of play-scala-starter-example and play-java-starter-example locally and
smoke test them locally:

1. $ git clone https://github.com/<yourname>/play-scala-starter-example.git
1. $ cd play-scala-starter-example
1. $ git fetch origin templatecontrol-2.6.x
1. $ git checkout templatecontrol-2.6.x
1. $ git diff HEAD^
1. $ sbt
1. \> show dependencyClasspath
1. \> test
1. \> run

Then do "sbt run" and check that all the projects build successfully from #play-buzz -- then merge PR and delete
branch for each project (two button clicks).

Once you're done, commit your changes and merge the updated version into the project:

    git commit -am "upgrade section to play 2.5.13"
    git push origin upgrade-2.5.13
    hub pull-request --browse

So that everything is up to date.

**Verification**: Each of the example templates has CI integration into #play-buzz, so creating a new pull
request will kick off a build and you'll be notified of what passes and what fails.

There is an integration through webhook to example-code-service: see
https://github.com/typesafehub/example-code-service/ for what is on their end for packaging ./sbt in a zip file.

### Step 5 - Update Example Code Service

The Example Code Service will need to be updated to point to the new versions.  All the play templates are in
play-templates.conf:

https://github.com/typesafehub/example-code-service/blob/master/example-code-service/conf/play-templates.conf

You will need to have unique names for each template, so rename the current templates so it follows the
templates with the version explicitly appended (see bold bits):

i.e. when you come out with 2.7.0, you rename the current one to

```
{
  display-name = "Play Java using Dagger 2 for Compile Time DI (2.6.x)"
  name = "play-java-dagger2-example-2.6.x"
  url = "git@github.com:playframework/play-java-dagger2-example.git"
  github-repo = "playframework/play-java-dagger2-example"
  branch = "2.6.x"
  keywords = [ "play", "java", "di", "dagger2", "2.6.x"
  summary = "Play Application using Dagger 2 for Compile Time DI"
},
```

Create a pull request with all the updates, test it, run some examples with "sbt run", then submit that to
example-code-service and push.  The deploy should be automatic, but you may want to check with "tooling team"
that you did it right and it is updated.

**Verification**: visit https://developer.lightbend.com/start/?group=play and download a template and check that
it's up-to-date and works properly.

### Step 6 - Update playframework.com

Update `playReleases.json` and `changelog.md` in playframework.com website git repository.

Note that the changelog should be updated describing all milestone/release candidates for the same release
collectively. In addition the `playReleases.json` should be updated with the development releases in place i.e.
you don’t need to publish information on both an M1 and an M2 release.  If the release is for the latest stable
or development version, upgrade the website itself to use that version.

You will also want to check that the downloads page has the right branch tag (i.e. 2.7.x) for the example code
service branches that you just updated.  This is defined in playframework/playframework.com, under the
application.conf setting:

```
examples.playVersions = [ "2.5.x", "2.6.x" ]
```

**Verification**: Run playframework.com locally on your machine and check the /download and /changelog pages.

Check that you have the right version numbers and that kickstartr / example-code-service is up to date.

Commit and push your changes.

**NOTE**: you will need a distinct SSH public key for this.  Talk to James or Ed if you don’t have access.

To set up your public key:
1. Ask someone else from Play team to add your pub key to the website instance.
1. Edit ~/.ssh/config and add the following:

```
 Host www.playframework.com
  User ubuntu
  Hostname 54.173.126.144
```

3. You can now log in: ssh ubuntu@www.playframework.com

ssh into `ubuntu@www.playframework.com`, and run `./deploy.sh`

**Verification**: Check that https://www.playframework.com/download#alternatives contains the new release.

### Step 7 - Announce

* Write a blog post on https://playframework.ghost.io/ghost/24/ about the release.
* Write a topic on https://discuss.lightbend.com/
* Write a release on https://github.com/playframework/playframework/releases
* Send an internal email to eng-updates
* Tweet about the new release.

**Tip**: To get a list of authors:

```
git fetch --tags && git shortlog -s 2.6.13..2.6.14 | cut -c8- | sort
```

**Verification**: Go to https://discuss.lightbend.com/ and https://twitter.com/playframework look for the
message and tweet.
