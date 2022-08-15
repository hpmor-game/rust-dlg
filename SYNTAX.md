# Syntax of .dlg files

## ✅ Comments

Comments, like as in many other languages, are declared with two slashes: ```//```

```js
Common text // comment
```

## ✅ Characters and their states

TODO

```js
@alice // set current speaking character to `alice`
@:calm // set `calm` state for current speaking character (in this example - alice)
@alice:calm // sat current speaking character AND his state

@ // narrator's text
```

## ✅ Sections and links

```js
// start of initial section

Text in initial section

#section_1 // end of initial section, start of section_1

Text in section_1

#section_2 // end of section_1, start of section_2

Text in section_2
```

## ⏳ Going between sections

```js
:move #section_1 // go to section_1
:back // go to previous section
```

## 🔧 Menus

Example of a menu for moving between sections:

```js
:menu Menu title
:opt(#section_1) Go to section_1
:opt(@alice:calm; #section_2) Go to section_1 and set current speaking character to @alice:calm
```

Example of a menu for selecting the value of a variable `var_name`:

```js
:menu(var_name) Menu title
:opt(=value_1) Set var_name to value_1
:opt(=value_2) Set var_name to value_2
:opt(=value_3; @alice:calm; #section_1) Set var_name to value_2, set current speaking character to @alice:calm and go to section_1
```

## ⏳ Commands

```js
:event "event_name" // call an event
```

## ⏳ Conditions

```js
:if condition_1
    will be displayed only if condition_1 == true
:elseif condition_2
    will be displayed only if condition_2 == true
:else
    will be displayed else
:endif
```
