# Warning!
Halfwit is pre-alpha software. Whatever functionality you find is likely to be incomplete or buggy, the README makes promises it can't keep (for now!), and it's generally a Bad Idea to use. That being said, feel free to dive in and test whatever's there, send issues, write PRs, etc, even for minor things. There is no fix too minor, no bug too small, and no issue too meaningless, I'm happy to hear any and all feedback you have. 

Oh, and one more thing. If it's been ~2 months since I've written a commit, **please** make an issue about it, even if you're just passing by and don't intend to use Halfwit. Just a simple "hey, work on this" is all it takes, you won't annoy me. I want to work on this project, I just forget sometimes.

---

# Halfwit
Halfwit is a general-purpose bisector.

## ...what?
Have you ever had a bunch of files in a folder, and wanted to know which of them is causing some certain behavior? 300+ Minecraft/Factorio/Skyrim/Fallout mods installed and you can't tell which one of them is crashing the game? That's what Halfwit solves. Halfwit enables and disables files, and then runs an arbitrary program. Based on what the program does, it enables and disables different files and runs it again. It keeps doing this until it knows which file(s) are causing the program to behave how it was. 

It seems pretty niche at first, but this type of problem crops up surprisingly often. Halfwit was initially concieved for modded Minecraft, but I've since ran into at least 4 unrelated situations it would have fixed. Plus, it's *absurdly* configurable, and easily extendable, so there's a lot you can do with it.

## Adapter Scripts
One crucial part of using Halfwit is adapter scripts. When Halfwit runs a program, it expects "success" to be represented by exit code 0, and "failure" to be represented by anything else. If your program behaves like that already, great! If not, you're gonna have to write an adapter script, which is simpler than it sounds. 

All an adapter script does is change whatever behavior you're investigating into the behavior Halfwit expects. Don't worry, you don't have to learn a new language, it's just a program. You can write it in whatever language you like. I recommend bash, since it's easy. Halfwit already comes with a few simple adapter scripts for some common tasks, which also serve as a good reference and starting point for your own. The following few might be especially helpful:

- `scripts/timeout.sh` - For tasks that take set time and then never exit.
- `scripts/manual.sh` - For tasks too complex to automate.
- `scripts/cpu_threshold.sh` - For tasks that use 100% CPU for a bit, then stop, and hang around.
- `scripts/invert.sh` - For when you want to find what's causing a process to exit with code 0.

Feel free to submit your own adapter scripts if you think other people might get use out of them!
