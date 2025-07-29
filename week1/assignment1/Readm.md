### Explore during Learning :

🔁 How is it different from #[proc_macro]?

| Macro Type	| Syntax used in code  |	Use case |
|---------------|----------------------|-------------|
|`#[proc_macro]`  |	`my_macro!(...) `    |Like function-macro (e.g. println!)|
|`#[proc_macro_derive]`	|`#[derive(MyMacro)]`	|Add impl for struct
|`#[proc_macro_attribute]`|	`#[my_macro]` above struct	|Modify struct/func/item
---

1. For creating patching file - `git diff master > submission-week-1-assignment.patch`