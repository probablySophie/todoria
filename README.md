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
<Enter> Select | <F> Filter | <S> Sort | <?> Settings/Keys | <Q> Quit
```

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
