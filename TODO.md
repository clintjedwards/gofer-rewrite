- For events we should change our approach. We can provide people the ability to parse them via json
  and possibly have sdk helpers, but creating different types is not scaleable. Instead pass type in as an enum.
- Possibly make the comfy-tables crate respect NO_COLOR
- Take a look at all the places we unwrap and clean up if needed.
- Reflection doesn't work.
- Move validators of the models in the new fn to the api. THe api can do much better at validating that things
  are correct.
