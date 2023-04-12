# Style Guide

<div align="justify" style="font-weight: bold; font-size: 18px;">
This repository follows the rust style guidelines. As such it is recommended to format the code
using the cargo formatting command before submitting a pull request.
</div>

```console
$ cargo fmt
```
or
```console
$ rustfmt <file-name>
```

## Additional Guidelines
Although rust guidelines handle most of the rules, there are some additional guidelines to follow.

- ### Indentation

    1. Always add 4-space indentation instead of tabs to keep the code format consistent across
    repositories.

    2. Do not nest items more than 3 levels. Although, 4 levels or more may sometimes be
    unavoidable, try to avoid it as much as possible.
    ```rust
        // Below code is alright but it can be written much more clearly.
        match (
            match gio::Application::default() {
                Some(app) => app,
                None => {
                    panic!("Couldn't build default application object");
                }
            }
        ).downcast() {
            Ok(app) => app,
            Err(_) => {
                panic!("Error while creating default application");
            }
        }
    ```
    ```rust
        // Ain't this cleaner?
        if let Some(app) = gio::Application::default() {
            if let Ok(downcast) = app.downcast() {
                return downcast;
            }
        }
        panic!("Error while creating default application");
    ```
    ```rust
        // Better yet.
        gio::Application::default()
            .expect("Error while creating default application")
            .downcast()
            .expect("Error while downcasting application")
    ```
    In the case of `Result`/`Option`, it is recommended to propagate.

    Take this stupid code for example.
    ```rust
        fn divide(numerator: i32, denominator: i32) -> Result<i32, i32> {
            if denominator <= 0 {
                return Err(numenator);
            } else {
                Ok(numenator/denominator)
            }
        }

        fn calculate(x: i32, y: i32) -> Result<i32, i32> {
            match divide(x, y) {
                Ok(num) => Ok(num),
                Err(num) => Err(num),
            }
        }
    ```

    This could be written like this.
    ```rust
        fn divide(numerator: i32, denominator: i32) -> Result<i32, i32> {
            if denominator <= 0 {
                return Err(DivideByZero);
            }
            Ok(numenator/denominator)
        }

        fn calculate(x: i32, y: i32) -> Result<i32, i32> {
            Ok(divide(x, y)?)
        }
    ```

- ### Naming

    1. Keep variable names short and understandable. Variable name should explain what that
    variable is used for. Don't make variable name too short.
    ```rust
        // Bad variable names.
        let w = ApplicationWindow::new();
        let wi = ApplicationWindow::new();
        let wnd = ApplicationWindow::new();
        let win = ApplicationWindow::new();
        let wndw = ApplicationWindow::new();

        // Instead add an understandable variable name.
        let window = ApplicationWindow::new();
    ```

    2. If you require more than 2 words to describe a variable, refactor and change the name. A
    variable can acquire meaning based on the context hence, it is not necessary to make the
    variable name too specific if the meaning can be drawn from context easily.
    ```rust
        // This variable name would suffice given the context.
        let window = ApplicationWindow::new();
        // In a Application 'activate' function this kind of name would be counter-productive.
        let main_application_window = ApplicationWindow::new();

        // It is also better to use abbreviations if their meaning can be easily derived.
        // For example, instead of this:
        let temporary_variable = foo::new();
        // It is better to use if there are no other 'tmp' variables and it cannot be confused
        // for something else.
        let tmp = foo::new();
    ```

    3. Precede a type with `TW` to denote that a type is project specific and not a default GTK
    provided
    type.
    ```rust
        // Naming it this way could lead to confusion with GtkApplicationWindow type.
        struct ApplicationWindow {
            ...
        }

        // Adding 'TW' ensures that it is a project specific type.
        struct TWApplicationWindow {
            ...
        }
    ```

    4. While naming custom types follow the naming structure of `NameActionContext`. Here, only the
    `Name` is mandatory for structs, `Action` and `Context` can be added in case of `Traits` or
    conflicting types to seperate them. Always have clear understandable struct names.
    ```rust
        // Instead of writing this.
        struct Button1;
        struct Button2;

        // This would be more understandable.
        struct FileButton;
        struct EditButton;
    ```
    ```rust
        // In case of traits.
        trait ButtonTrait {
            fn clicked() {
                ...
            }
        }

        // This would be better.
        // Also, always write 'Action' as a verb in first form. As in, write 'ButtonListen'
        // instead of 'ButtonListening'. Avoid turning the 'Action' to a noun as well.
        // As in writing 'ButtonListener'.
        trait ButtonListen {
            fn clicked() {
                ...
            }
        }

        // If the trait only applies to button in context of a certain assertment say,
        // validation then we can write the trait as.
        trait ButtonListenValidate {
            fn clicked() {
                ...
            }
        }
    ```

    As a general rule, always make sure the structs/objects are made up of nouns whereas, traits
    should have verbs. As an example, if `ButtonListenValidate` were a struct then it would be
    `ButtonListenerValidation`. Notice how `Context` part of the word was converted to a noun as
    well.

- ### Numericals

    1. When writing numbers always suffix it with a type so that it is easier to recognise the type
    of the number. This rule is strongly encouraged to follow as it helps with readability.
    ```rust
        // Instead of this.
        let x: i32 = 1;

        // Write this.
        let x = 1_i32;
    ```
    ```rust
        // Especially helpful when you have a f32 and f64.
        let mut x = 1_f32;
        let mut y = 1_f64;

        // These both would work in this case but adding suffix helps.
        x += 2.0;
        y += 1.0;

        // Like this.
        x += 2_f32;
        y += 1_f64;
    ```
    ```rust
        // Also when you have various integer types.
        let x: i32 = 0;
        let y: u32 = 0;

        // When adding or subtracting, it might be easier as it makes you conscious about the
        // type of number you are using. Less panics this way.
        let x = 0_i32;
        let y = 0_u32;

        let x -= 1_i32;
        // Cannot subtract from 0_u32.
    ```

    2. If number size goes beyond 4 digits then seperate the numbers with an underscore following
    the metric system format.
    ```rust
        // Instead of writing this.
        let x = 10000_i32;
        let y = 100000_i32;

        // Write this.
        let x = 10_000_i32;
        let y = 1_00_000_i32;
    ```

- ### Comments

    1. Multiline comments are not great. Purely subjective and for aesthetic reasons. If you like
    to use multiline comments then all the power to you but, could you please not do it in this
    repository?
    ```rust
        /*Not 'cleaner' looking (subjective).
        Do people like using these?*/

         // This looks much more 'cleaner' (subjective).
         // This honestly should be least of anybody's worries.
         // This doesn't affect the code. Just stating preferences.
    ```

    2. Also it is recommended to use comments to document code. Forget the 'code should document
    itself' bullcrap. Just add comments where necessary. Don't go overboard though.

    This is an example of going overboard.
    ```rust
        /// This function is used to divide two `i32` numbers. It takes two `i32` numbers as
        /// arguments. If denominator is zero or less then it will return a `DivideByZero`
        /// error or else it will return Ok(result). The function has `#[inline]` attribute
        /// as it is small enough to be inlined.
        #[inline]
        fn divide(numerator: i32, denominator: i32) -> Result<i32, i32> {
            // Checking if denominator less than zero.
            if denominator <= 0 {
                // Denominator is less than zero hence returning a 'DivideByZero' error.
                return Err(DivideByZero);
            }
            // Or else safely divide and return an 'OK'.
            Ok(numenator/denominator)
        }
    ```

    This is enough.
    ```rust
        /// This function takes two `i32` numbers and divides them.
        /// An [`Err`] is returned in case denominator is zero or less than zero.
        #[inline]
        fn divide(numerator: i32, denominator: i32) -> Result<i32, i32> {
            if denominator <= 0 {
                return Err(DivideByZero);
            }
            Ok(numenator/denominator)
        }
    ```

    3. Use documentation comments for special types and important functions. Module level document
    comment is not necessary for modules that do not possess special types or critical processing
    (i.e. helper modules).
    ```rust
        //! This is a module level document comment.

        /// Trait to handle button callbacks after validation.
        trait ButtonListenValidate {
            // No need to document the internal functions as they are pretty self explainatory.
            fn clicked();
        }
    ```

    You don't need to explain the internal variables of a struct if they are self explainatory.
    ```rust
        /// This is a struct to handle the page object.
        struct Page {
            count: u32,
            buffer: String,
        }
    ```

    However, it may be useful in some cases to explain the internals.
    ```rust
        /// This is an enum to handle errors.
        enum MyErrors {
            /// Here you should explain what each error denotes.
            FileIOError,
            UserIOError,
            ValidationError,
        }
    ```

- ### Practices

    1. Avoid working with generics if the given code can be written otherwise.
    ```rust
        // There is no need for a generics.
        struct Page<T> {
            count: T,
            buffer: String,
        }

        // Here we can just put count as 'usize' or 'u32' as we know we won't be needing any
        // other numerical type.
        struct Page {
            count: u32,
            buffer: String,
        }
    ```
    ```rust
        // Here is an example where you should use generics.

        // Some types.
        struct Square;
        struct Circle;
        struct Triangle;

        // Trait that bounds them.
        trait Draw {
            fn draw();
        }

        // Implementations.
        impl Draw for Square {
            fn draw() {
                ...
            }
        }
        impl Draw for Circle {
            fn draw() {
                ...
            }
        }
        impl Draw for Triangle {
            fn draw() {
                ...
            }
        }

        // You need to accept a function that only takes items that are of the bound draw.
        fn render<T: Draw>(shapes: T) {
            ...
        }
    ```
    Even then, generics are somewhat discouraged as it sometimes overcomplicates code where simpler
    solutions exist.

    2. Add an `unimplemented` macro for unimplemented part of code. Reserve `todo` macros for
    development. There shouldn't be any `todo`s in pull requests.
    ```rust
        // This is okay.
        fn foo() {
            unimplemented!()
        }

        // Instead of this.
        fn foo() {
            todo!()
        }
    ```
    Please do not pull request with a lot of unimplemented code. If there are more than two
    unimplemented macros then implement the code and then do a pull request.

    3. Destructure when possible to make things easier.
    ```rust
        // This works fine.
        let dimension = Dimensions::default();
        set_size(dimension.0, dimension.1);

        // But this is perfection.
        let (height, width) = Dimensions::default();
        set_size(height, width);
    ```

    4. If there is a complex calculation in the code that can affect readability, then it is ok to
    extract that into a new function. If it is small enough, say less than or equal to 5 lines then
    you can go ahead and add the `#[inline]` attribute.
    ```rust
        // A complex condition check.
        #[inline]
        fn position_in_bounds(x: f32, y: f32) -> bool {
                (x < XMAXF - (BOARD * 2_f32) && x >= XMINF)
            &&  (y <= (YMAXF - 1_f32) - EMPTY && y >= YMINF)
        }

        // Function to draw a sprite.
        fn draw(self) {
            let x = self.x;
            let y = self.y;
            // Guard clause.
            if !position_in_bounds(x, y) { return }
            ...
        }
    ```

## Conclusion

I hope that wasn't too overbearing for you. Please do follow the guidelines. If you fail to follow
them then it is not a big deal, it can be sorted out during pull request. Most important rule of
them all is to have fun while coding so don't forget that!
