# Contribution Guide

## Introduction

Thank you for your interest in contributing to Fornjot. We appreciate the help!

This document teaches you how to...

- ...report a bug.
- ...suggest an improvement
- ...make an improvement.
- ...find work to do.

Each of these topics is addressed in one of the following sections. After that, there's also a collection of optional guidelines you can follow when making your contribution.


## Reporting Bugs

To report a bug, please [open an issue](https://github.com/hannobraun/Fornjot/issues/new) in Fornjot's GitHub repository.

Feel free to first check the [list of open issues][issues], and if you find an existing one for your bug, add your voice there. If you're not sure or don't have the time, **don't worry, just open an issue**. We'd rather deal with duplicate issues than not hear about a bug at all.


## Suggesting Improvements

There are two ways to suggest an improvement, like a new feature or changes to an existing one:

- [Open an issue][issues]
- Start a discussion on [Matrix] or [Discussions]

We use issues to track work that is mostly actionable, so stuff that someone can work on right now, or at least soon. Having too many issues open that aren't actionable will make it harder to track the actionable work.

If you think your request is an obvious improvement, open an issue. If want to discuss a larger addition, long-term plans, or just aren't sure, start a discussion. **Just use your judgement, this isn't a big deal.** Worst case, your issue will be closed, or you'll be asked to open one.


## Making Improvements

If you want to fix a bug, add a new feature, or improve an existing one, just fork the repository, make your change, and submit a pull request. Once submitted, [@hannobraun] will review the pull request, give feedback and possibly request changes, and once everything is in order, merge it.

Pull requests are always welcome. But of course, there's a risk that yours might not be accepted. Bug fixes and other obvious improvements are usually safe, but new features might be deemed out of scope.

If you don't want to risk spending time on work that might not be merged, you can start a discussion first. [Matrix] or [Discussions] are the best ways to do that.


## Finding Work

You want to get involved, but aren't sure what to work on? We use [issues] to track actionable work that can start soon or (ideally) right now. So if you want, dig into that and get busy.

There are a lot of open issues, however. If you need some more guidance, there are some ways to narrow it down:

- Issues that are suitable for starting out are labeled as [`good first issue`](https://github.com/hannobraun/Fornjot/labels/good%20first%20issue). Those typically don't require any deep knowledge of the Fornjot code base, so they're ideal to get your feet wet.
- Some issues that need extra attention are labeled as [`help wanted`](https://github.com/hannobraun/Fornjot/labels/help%20wanted). Don't take that too seriously though. Help is welcome everywhere, not just on issues explicitly labeled as such.
- We also track various [milestones](https://github.com/hannobraun/Fornjot/milestones). Feel free to check those out, pick one that seems most interesting to you, and browse through the list of issues assigned to it.

Finally, feel free to just ask. If you have a specific issue in mind, just comment there. Or direct your query to [Matrix] or [Discussions].

If you're not a programmer or are looking for some variety, you can also work on [the website](https://github.com/hannobraun/www.fornjot.app).


## Additional Guidelines

Let's put one thing up front: The following guidelines are just that, guidelines. **These are not mandatory rules** (well, except for the few that are enforced in the CI build).

If you're not sure about something or think you might forget: **don't worry.** These guidelines are here to help make process as smooth as possible, not hinder anyone's work. Just submit a pull request and we'll figure out together what it takes to get it merged.

### Issues

Before starting to work on an issue, feel free to leave a message there first. Maybe others are working on the same issue too, or maybe there is new information that hasn't been posted in the issue yet.

### Pull Requests

- If your pull request is a work in progress, please submit it as a draft to make that clear. Once you believe it is ready to be reviewed, convert the draft into a pull request.
- Once your pull request has been reviewed, but not yet merged, please add any additional changes as new commits. This makes the review process much easier.
- If you're making changes before the pull request has been reviewed, for example in response to the CI build, feel free to modify the existing commits, if that makes things clearer.

### Commits

- Focus each commit on one change. Don't combine multiple changes into the same commit.
- Don't make commits too large, unless it can't be avoided.

This makes it much easier to review your commits, as many small and focused commits are much easier to review than few large ones.

Ideally, each commit should compile, without warnings or test failures. This isn't critical, but is a huge help when [bisecting](https://git-scm.com/docs/git-bisect).

### Commit Messages

The ideal commit message consists of the following:
- An initial line of up to 50 characters.
- A blank line following it.
- Any number of additional lines, limited to 72 characters.

This is based on the [official guideline](https://git-scm.com/docs/git-commit#_discussion) and makes sure the commit is properly formatted by various tools (e.g. on GitHub or in `git log`).

Further, the initial line ideally follows these guidelines:
- Summarize the change itself and the intent behind it. This is often not possible in the limited space. Second-best is to summarize the change, or even just where it happened, and leave the intent for the rest of the message.
- Use the imperative mood, i.e. formulate the initial line like a command or request. For example, write "Add a feature" instead of "Added a feature" or "Adding a feature". This is often simplest and most compact, and therefore easiest to read.

The commit as a whole ideally follows these guidelines:
- First and foremost, document the *intent* behind the change. Explain *why* you did something. Explaining the change itself is secondary.
- Ideally, the change itself is small and clear enough that it doesn't need explanation, or even a summary. If it does though, that should go into the commit message.
- Refrain from explaining how code you added or changed works, beyond a short summary. While such explanation is often highly beneficial, it belongs in the code itself, as a comment.
- If the intent behind a change is relevant to understanding the code after the change, then leave even that out of the commit message, and add it as a comment instead.

### Formatting

We use [rustfmt](https://github.com/rust-lang/rustfmt) for formatting.


[issues]: https://github.com/hannobraun/Fornjot/issues
[Matrix]: https://matrix.to/#/#fornjot:braun-odw.eu
[Discussions]: https://github.com/hannobraun/Fornjot/discussions
[@hannobraun]: https://github.com/hannobraun
