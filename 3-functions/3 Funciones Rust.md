# Funciones Rust

Las funciones son bloques de código que realizan una tarea específica. En Rust, las funciones se definen con la palabra clave `fn`. Hasta ahí lo fácil y evidente. Pero Rust tiene algunas características poco familiares si vienes de JavaScript, Java, C# o TypeScript que necesitan un poco de explicación.

## Pre requisitos

Para seguir este tutorial necesitas tener un conocimiento básico de Rust. Si no es así, te recomiendo que leas mis tutoriales anteriores sobre Rust.

- [1 - Hola Rust](https://www.linkedin.com/pulse/hola-rust-alberto-basalo-qrbbf/)
- [2 - Programas Rust](https://www.linkedin.com/pulse/programas-rust-alberto-basalo-2innf/?trackingId=4s1IfVWNTF26jvlDKsVHmg%3D%3D)

Ahora sí, vamos a por las funciones. Voy a usar como ejemplo el programa de cifrado César que lee el contenido de un fichero, lo cifra y lo guarda en otro fichero. Al final del tutorial, tendrás el enlace al código completo.


## Argumentos 

### Argumentos simples

Antes de nada vamos a empezar por la falta de argumentos. Porque, por supuesto, una función puede no tener argumentos. En este caso, la lista de argumentos está vacía.

```rust
fn print_instructions() {
    println!("🔑 Encrypt and decrypt files using the Caesar cipher.");
    println!("📘  The program reads the file content and encrypts it using the key.");
    println!("📘  The key is a string that is used to shift the characters in the text.");
    println!("📘  The program prints the original and encrypted text to the console.");
    println!("📘  The program requires two command line arguments: the file name and the key.");
    println!("🚀 Example: cargo run example.txt key");
}
```

Pero, lo más habitual es que una función tenga uno o más argumentos. En Rust, los argumentos se definen entre paréntesis, y cada argumento se compone de un nombre y un tipo.

```rust
fn print_end(start_time: std::time::Instant) {
    let duration = start_time.elapsed();
    let duration_ms = get_milliseconds(duration);
    println!("🦀 Program completed in: {:?} ms", duration_ms);
}
```


### Propiedad y préstamos

Vamos ahora con una función más interesante. Por ahora no te preocupes de su contenido, bastante tenemos con los argumentos. 

Esta función, llamada `read_file`, recibe un argumento de tipo `&String`. Pero, ¿qué demonios es ese símbolo `&`? Para entenderlo, necesitamos hablar de propiedad y ver esta función en un contexto más amplio.

```rust
fn read_file(file_name: &String) -> String {
    let content: Result<String, std::io::Error> = fs::read_to_string(file_name);
    match content {
        Ok(content) => return content,
        Err(error) => {
            eprintln!("💣 Error reading file: {}", error);
            std::process::exit(1)
        }
    }
}
```

En Rust, las variables tienen una propiedad. Cuando pasas una variable a una función, puedes transferir la propiedad de la variable a la función. Esto significa que la función puede modificar o destruir la variable. Pero lo más loco es que tú dejas de tener acceso a ella. Algo que en este caso no queremos.

Dedicaremos un tutorial entero a la propiedad, pero por ahora basta con saber que si quieres que la función no modifique la variable, debes pasar una referencia a la variable. Y eso es lo que hace el símbolo `&`.

## Retorno de valores

Vamos ahora un tema más sencillo, o eso parece... Porque el retorno de valores nn Rust, también tiene sus peculiaridades. Para empezar, una función puede no devolver nada, cómo en el caso de la función `print_instructions` que vimos antes.

Si retornas un valor, debes indicar el tipo de dato que vas a devolver. En Rust, el tipo de dato se indica con una flecha `->` seguida del tipo de dato. Por ejemplo, la función `get_milliseconds` devuelve un valor de tipo `u64`.

```rust
fn get_milliseconds(duration: std::time::Duration) -> u64 {
    return duration.as_secs() * 1000 + duration.subsec_millis() as u64;
}
```

Decirte que en Rust, la última expresión de una función se considera el valor de retorno. Por lo que no es necesario usar la palabra clave `return` ni el `;` del final. Es habitual que te encuentres con funciones que no usan `return` explícito.

```rust
fn get_milliseconds(duration: std::time::Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_millis() as u64
}
```


## Retorno opcional

En ocasiones, una función puede no devolver un valor. Por ejemplo, si la función falla. En estos casos, puedes devolver un valor especial llamado `Option` que es un uso muy particular e inteligente de los enums que veremos en un tutorial posterior. Por ahora, basta con saber que `Option` es un enumerado que puede tener dos valores: `Some` o `None`.

```rust
fn get_base_code_option(the_char: char) -> Option<u8> {
    if the_char.is_ascii_alphabetic() == false {
        return None;
    }
    let base_case_code: u8 = if the_char.is_ascii_lowercase() {
        LOWER_CASE_BASE
    } else {
        UPPER_CASE_BASE
    };
    Some(base_case_code)
}
```
Esta función devuelve el código ascci de la primera letra minúsculas o mayúsculas. Pero si la letra no es alfabética, devuelve `None`.

Claro que recibir un `Option` puede ser un poco incómodo hasta que te acostumbras a tratar con la instrucción `match`. Aunque hay otras formas formas de hacerlo, la que te muestro la encuentro bastante elegante y, desde luego, segura.

```rust
 let base_case_code: u8 = match get_base_code_option(clean_char) {
    None => return clean_char,
    Some(base_case_code) => base_case_code,
};
```

## Retorno de errores

Un caso especial de retorno opcional es aquel en que una de las opciones es... un error. En Rust, los errores se manejan con el tipo `Result`. Puedes pensar en él como un enumerado que  tiene dos valores posibles: `Ok` o `Err`. 

```rust
fn read_args() -> Result<CliArgs, std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_instructions();
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "⚠️ - Please provide the file name and key as arguments.",
        ));
    }
    let cli_args = CliArgs {
        clean_file_name: args[1].clone(),
        key_string: args[2].clone(),
    };
    return Ok(cli_args);
}
```

En la función `read_args` se comprueba que se han pasado dos argumentos. Si no es así, se imprime un mensaje de error y se devuelve un error. Si todo va bien, se devuelve un valor de tipo `CliArgs`.

Procesar la respuesta es similar a lo que hemos visto con `Option`. En este caso, usamos `match` para manejar los dos posibles valores de retorno.

```rust
let base_case_code: u8 = match get_base_code_option(clean_char) {
    None => return clean_char,
    Some(base_case_code) => base_case_code,
};
```	

## Comentarios

Por último, quiero mostrarte algo que me parece curioso, que en Rust los comentarios se escriben con `///` y que admiten formato Markdown. No son muy agradables de ver en el editor, pero es muy útil para documentar tu código y generar documentación automáticamente. 

```rust
/// Encrypts a character using the **Caesar cipher**.
///
/// This function takes a clean character and a shift value as input.
/// It applies the Caesar cipher to the character using the shift value.
/// If the character is not an ASCII alphabetic character, then it is left unchanged.
/// ### Arguments
/// * `clean_char` - A character that holds the clean text to be encrypted.
/// * `shift` - A u8 that holds the shift value.
/// ### Returns
/// * `char` - The encrypted character.
/// ### Example
/// ```
/// let encrypted = caesar_cipher_char('a', 3);
/// ```
fn caesar_cipher_char(clean_char: char, shift: u8) -> char {
    let base_case_code: u8 = match get_base_code_option(clean_char) {
        None => return clean_char,
        Some(base_case_code) => base_case_code,
    };
    let clean_code: u8 = clean_char as u8;
    let ciphered_code: u8 = ((clean_code - base_case_code + shift) % CASE_LENGTH) + base_case_code;
    let ciphered_char: char = ciphered_code as char;
    return ciphered_char;
}
```	

## Resumen

En este tutorial hemos visto cómo se usan las funciones en Rust con sus argumentos y cómo se manejan los valores de retorno. También tienes ejemplos de cómo se devuelven valores opcionales y errores.

Si quieres ver el programa de cifrado César completo, puedes encontrarlo en mi repositorio de GitHub.

En próximos tutoriales, profundizaremos en los concepto de propiedad y préstamo, y seguiremos viendo más casos de uso de los enums y otros conceptos avanzados de Rust. ¡Hasta la próxima!