# Warning!
~~Halfwit is alpha software. Only the baseline functionality is implemented, and it's a bit rough around the edges, but it does fulfill its intended purpose, and I've used it to solve a couple real-world problems. If you end up using it (successfully or otherwise), I'd really appreciate some feedback, feel free to raise an issue if you run into any bugs or have any feature suggestions. I'm currently focused on other projects, though, so don't expect frequent updates.~~ Somehow, it seems I've lost the functional halfwit code I had. I suspect it was on a laptop I no longer have, and never got pushed to Github. 

---

# Halfwit
Halfwit is a general-purpose bisector, inspired by `git bisect` and similar tools.

## ...what?
Have you ever had a bunch of files in a folder, and wanted to know which of them is causing some certain behavior? 300+ Minecraft/Factorio/Skyrim/Fallout mods installed and you can't tell which one of them is crashing the game? That's what Halfwit solves. Halfwit enables and disables files, and then runs an arbitrary program. Based on what the program does, it enables and disables different files and runs it again. It keeps doing this until it can determine which file(s) are causing the program to behave how it was. 

It seems pretty niche at first, but this type of problem crops up surprisingly often. Halfwit was initially concieved for modded Minecraft, but I've since ran into at least 4 unrelated situations it would have fixed. <!--Plus, it's *absurdly* configurable, and easily extendable, so there's a lot you can do with it.-->

<!--## Adapter Scripts
One crucial part of using Halfwit is adapter scripts. When Halfwit runs a program, it expects "success" to be represented by exit code 0, and "failure" to be represented by any other exit code. If your program behaves like that already, great! If not, you're gonna have to write an adapter script.

All an adapter script does is change whatever behavior you're investigating into the behavior Halfwit expects. You can write it in whatever language you like. I recommend bash, since it's easy to work with environment variables (halfwit sets a lot of them for you). Halfwit already comes with a few simple adapter scripts for some common tasks, which also serve as a good reference and starting point for your own. The following few might be especially helpful:

- `scripts/timeout.sh` - For tasks that take set time and then never exit.
- `scripts/manual.sh` - For tasks too complex to automate.
- `scripts/cpu_threshold.sh` - For tasks that use 100% CPU for a bit, then stop, and hang around.
- `scripts/invert.sh` - For when you want to find what's causing a process to exit with code 0.

Feel free to submit your own adapter scripts if you think other people might get use out of them!-->
