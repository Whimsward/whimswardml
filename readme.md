# Whimsward Markup Language

I'm calling it a markup language, but I think the intent for a .whim file is much like the use case for YAML or TOML, where the file type is primarily for configuration.

Essentially, I want to be able to declare Narrative Elements like Sequences, Places, or Characters and have those declarations be parsed by another program (Whimward Narrative Engine) to spin up the elements that were declared.


I'm also spitballing being able to communicate the **content** of declared elements, and debating whether I would want to have the file type be the same for both use cases, or how to manage the two concerns.

Stay tuned!

## Architecture Outline

```
root
/my_narrative.whim
|sequences/
-/my_first_sequence/
--/my_first_sequence.whim
|locations/
-/my_first_location/
--/my_first_location.whim
--/a_place_in_first_location/
---/a_place_in_first_location.whim
|characters/
-/this_guy.whim
|artifacts/
```

So a given .whim file should designate itself in its first line

<Canon> - expects subdirectories for narratives, sequences, locations, characters, and artifacts

<Narrative> - expects subdirectories for sequences, locations, artifacts, and characters

<Sequence> - expects subdirectories for each subsequence referenced in the file

<Location> - expects subdirectories for each sublocation referenced in the file

<Character>

<Artifact>

The main file in the root directory of the project is the primary reference for everything else.