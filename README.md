# Recipes

## Libary and executable to parse makefile-like recipes.

```
# makes 4 250g pizza doughs
dough: water (368g), salt (18g), instant dry yeast (1.4g), 00 flour (613g)
	warm water # to a little more than luke warm
	mix water, yeast, salt together into wet mixture # until everything disolves
	...

sauce: peeled tomatos (1 can), minced garlic (4 cloves), sugar (to taste)
	mix until well blended

cheese: mozzarella (part skim 1 block low mostiture)

pizza: dough, sauce, cheese, itialian seasonings
	remove dough balls in morning to use for late lunch, dinner
	heat oven for 15+ minutes on highest
	...
```

## Build WASM module

This is still an experiment.

```
wasm-pack build --target web
```

## Test WASM module and recipe builder

This is still an experiment. No recipes will be saved.

```
python3 -m http.server
```

This includes three panes (textareas), one for the recipe, one for the CSS / HTML template, and another for the HTML of the rendered recipe.
The bottom output is the rendered HTML of the third pane.

Now go to http://localhost:8000 and give the UI a test.
