

/* 
    Macro:
    - Macros are similar to functions, but it's expand into additional code
    - Macros use an exclamation point to call/invoke
    - Data can be printed using println! and there are two ways to supply data to the macro:
        - {:?}
        - {varname:?}
*/

/* 
    println macro:
    Example:
    let life = 42;
    println!("{:?}", life);

    Notes:
    curly braces { } indicates that's a token, avaliable for usage or "receive" a variable 
    :? indicates that's a debug print, it's only for debug purpose, not meant by seen to an end user 
*/