# Syntax of .dlg files

## Comments

Comments, like as in many other languages, are declared with two slashes: ```//```

```js
Common text // comment
```

## Characters and their states

TODO

```js
@name // now character with alias @
@:state // состояние
@name:state // персонаж + состояние

@ // описательный текст от рассказчика
```

## Sections and links

```js
#anchor // создание метки в этом файле
```

## Перемещения по меткам

```js
:move #anchor // перемещение на метку в этом файле
:back // возвращение на место, откуда было произведено последнее перемещение на метку
```

## Menus

Пример меню для перемещения между метками:

```js
:menu text
:opt(#anchor) text // кнопка выбора, перемещающая на метку в этом файле
:opt(#file/anchor) text // кнопка выбора, перемещающая на метку в другом файле
:opt(:state; #anchor) text // кнопка выбора, перемещающая на метку, а при наведении изменяет состояние текущего персонажа
:opt(@name:state; #anchor) text // кнопка выбора, перемещающая на метку, а при наведении изменяет состояние персонажа @name
```

Пример меню, для выбора значения локальной переменной:

```js
:menu(var_name) text // текст меню
:opt(=value) text // кнопка выбора, устанавливающая переменной var_name значение value
:opt(=value2) text2 // кнопка выбора, устанавливающая var_name значение value2
:opt(=value3; @name:state; #anchor) text3 // кнопка выбора, устанавливающая var_name значение value3, при наведении на кнопку устанавливает состояние персонажу и перемещает диалог на метку #anchor
```

## Commands

```js
:event "event_name" // вызов ивента
```

## Conditions

```js
:if condition1
    Будет отрисовано, если condition1 == true
:elseif condition2
    Будет отрисовано, если condition2 == true
:else
    Будет отрисовано в противном случае
:endif
```
