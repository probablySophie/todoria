# ToDoria

A TUI todo app!  Uses the [todo.txt](https://github.com/todotxt/todo.txt) spec via [`todo-txt-rs`](https://github.com/probablySophie/todo-txt-rs) & [`ratatui`](https://github.com/ratatui/ratatui) for the visuals.  

****

### Using...

**Ratatui** [Widgets](https://ratatui.rs/examples/widgets/)  

* [Blocks](https://ratatui.rs/showcase/widgets#block)  
* [Calendar](https://ratatui.rs/showcase/widgets#calendar)!!!  
* [Gauge](https://ratatui.rs/showcase/widgets/#gauge)? For sub-task completion  
* [Table](https://ratatui.rs/showcase/widgets/#table) for lists of tasks!  
* [Tabs](https://ratatui.rs/showcase/widgets/#tabs)!!  
* [Making Custom Widgets](https://ratatui.rs/recipes/widgets/custom/)  

3rd party

* [Menus!](https://ratatui.rs/showcase/third-party-widgets#tui-menu--)?  
* [Scroll-view](https://ratatui.rs/showcase/third-party-widgets#tui-scrollview--)?  

***

### Direct Dependencies

|Name|Size|Usage|
|-|-|-|
|`todo-txt-rs`||Handles the `todo.txt` items & stuff|
|[`Confy`](https://crates.io/crates/confy)|16.3 KiB|Config creation, loading, and saving|
|[`serde`](https://crates.io/crates/serde)|54.7 KiB|`Confy` dependency|
|`ratatui`||The TUI|
|`crossterm`||`ratatui` backend|
|[`dirs`](https://crates.io/crates/dirs)|12 KiB|Cross-platform directories|


****

### Plans!

Bottom Menu Options:  

```
<Enter> Select | <F> Filter | <S> Sort | <N> New | <?> Settings/Keys | <Q> Quit
```

Complete Refactor because that's just who I am 🙃

```bash
app/ # Everything in here should be generic and non-specific
├── tests/ # App specific tests
├── action.rs # Enum for handling events
├── state.rs # Enum for handling current states
├── app.rs
├── keybinds.rs
├── settings.rs
└── screens.rs # The handler for screens, we should also have a trait that makes screens impl new, draw, & more
screens/ # The project specific screens that the app will need
├── base.rs # An empty screen for copy/pasting to make new screens
├── focused.rs
├── main.rs
├── settings.rs
└── table.rs
tests/ # All of the test files for everything

main.rs # This should contain impls for things in app that would be specific
todos.rs
```

Refactor todo?  
* [ ] Make app the app stuff more agnostic
* [ ] Have `main.rs`:
  * [ ] Define the `Action` and `State` structs & how the `App` handles them (display & on_action) with a macro?  
  * [ ] Handle app specific `App.load()` and `App.do_the_thing()` (and rename that to `handle_event` or something)
  * [ ] Define the screens that the App needs to have
  * [ ] Define the default keybinds (with a macro)
* [x] Make a trait that screens will `impl`
* [ ] Change `Render::Screens` to contain `Option<TheScreen>`


Open straight into a table view of the current Todo items.

**Settings**

> * Hotkeys  
> * Default filters & sorting  
> * Load  
> * Auto-load? (and what to)
> * Save  
> * Auto-save? (and where to)

**Filter** and **Sort**  

> Open menus with *clear* and the filter & sorting options.  

**Quit**

> If there have been changes, opens a *would you like to save your changes?*



Arrow keys & `HJKL` vim movement.  


****

### What's in a name?

![A photo of a Tyler's Tree Frog!!!](/meta/freg.jpg)  
*Source: [Wikipedia](https://en.wikipedia.org/wiki/File:Litoria_tyleri.jpg)*  

This little cutie is a [Tyler's Tree Frog](https://en.wikipedia.org/wiki/Tyler%27s_tree_frog)!  
Its scientific name is `Litoria tyleri` and climbs trees in eastern Australia!  

And how the project's name happened:  
```
Litoria
  toria
  ^todo
  todoria
```
