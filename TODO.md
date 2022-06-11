- For events we should change our approach. We can provide people the ability to parse them via json
  and possibly have sdk helpers, but creating different types is not scalable. Instead pass type in as an enum.
- Possibly make the comfy-tables crate respect NO_COLOR
- Take a look at all the places we unwrap and clean up if needed.
- Reflection doesn't work.
- Move validators of the models in the new fn to the api. THe api can do much better at validating that things
  are correct.
- Implement object store
- Refactor storage into smaller functions
- To prevent issues when a trigger container is being restarted, it needs to somehow communicate it is brand new
  and ready to reload pipeline settings.
- We may need to rewrite the config package to allow for:
  - cleaner and less obtuse env variables parsing
  - the ability to alter configuration and write it back to the file.
- Create a namespace set command that allows the user to switch between namespaces and save it in their configuration file.
- Document/Comment all libraries
- Because of the way pipelines work now it is possible to write your pipeline in any language
  as long as we have a way to run the native compiler.
- We can potentially auto detect languages by looking for auto language structure.
- We can also just straight up read from things like json/toml since it all compiles back to json anyway.
- Fix this regression: {{- if not (len $trigger.Events) 0 }} recent events:{{- end }} in pipeline get
- Fix events for all cli stuff.
- We should be more detailed on some of the parameters in proto, instead of 'id' use 'pipeline_id'
- IN our integration testing test that cascading deletes works
- See where we can clean up some of the extra impls we made assuming we would import from models instead of a separate sdk
- Separate store_keys into it's own table
- Reevaluate if we need docker-cancellations
