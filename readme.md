# Whimsward Markup Language

I'm calling it a markup language, but I think the intent for a .whim file is much like the use case for YAML or TOML, where the file type is primarily for configuration.

Essentially, I want to be able to declare Narrative Elements like Sequences, Places, or Characters and have those declarations be parsed by another program (Whimsward Narrative Engine) to spin up the elements that were declared.


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


`<!whimdoc Canon>` - expects subdirectories for narratives, sequences, locations, characters, and artifacts

`<!whimdoc Narrative>` - expects subdirectories for sequences, locations, artifacts, and characters

`<!whimdoc Sequence>` - expects subdirectories for each subsequence referenced in the file

`<!whimdoc Location>` - expects subdirectories for each sublocation referenced in the file

`<!whimdoc Character>` - expects no subdirectories

`<!whimdoc Artifact>` - expects no subdirectories

The main file in the root directory of the project is the primary reference for everything else.

### Head Matter

`FIELDTAG::tag-content;` - For example, a Character has a Name. This can be designated using `NAME::This-Guy;` A Character may also have a list of Artifacts. This reference can be made by `ARTIFACTS::That-Sword.whim,Salamander-Scale.whim;`

For fields that have tag content including ".whim" the content (comma separated) will prompt the engine to reference that file name in the corresponding directory.

Fields with tag content that is not a reference to another file will parse the text of that content preserving capitals and converting `-` to a single space. 

Fields with tag content enclosed in 'single quotes' will be parsed as raw strings. This is recommended for any longer text content or shorter content that should include the more conventional use of the hyphen character.