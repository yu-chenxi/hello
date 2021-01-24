# first level head

## second level head

### third level head

* unordered list 1

* unordered list 2

* unordered list 3

1. ordered list 1

2. ordered list 2

3. ordered list 3

| arch   |  os   |
|:-------|:-----:|
| risc-v | linux |
| x86    | linux |

*italic text*

**bold text**

***bold and italic text***

> block quote 1

> block quote 2

this is pragraph 1

time flies very fast


this is pragraph 2

<< the rust programming language >>

[![tux](assets/images/tux.png)](https://www.kernel.org "linux kernel site")


use **[google search](https://www.google.com)** to search *everything*!


### rust code block

```rust
fn prime(n: u32) -> bool {
    let r = n % 6; 
    if n >= 6 && r != 1 && r != 5 {
        return false;
    }
    let m = (n as f64).sqrt() as u32;
    for i in 2..=m {
        if n % i == 0 {
            return false;
        }
    }
    true
}

let v = [2u32, 7, 21, 31, 117, 32765, 65537];
for e in &v {
    println!("{} {} prime.", e, if prime(*e) { "is" } else { "is not" });
}
```

### rust snippet

```rust
let x = 3;
println!("{:.3}", (x as f64).sqrt());
```

### golang snippet

```go
func five() int {
    return 5
}
```

### rust-lang features

> *  fast & secure
>
> * steep learning curve
>  
>  reliable compiler checking
>
> * modern toolchains
