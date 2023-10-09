//// 2023/10/09 // 19:42 //

//// #19 Traits (Rasgos)

// Un trait indica al compilador una funcionalidad que tendrá un tipo en particular y
// que puede compartir con otros tipos. Podemos usar traits para definir el
// comportamiento compartido de una manera abstracta. Incluso, podemos traits
// para especificar que un tipo genérico puede ser cualquier tipo que tenga cierto
// comportamiento.

// Los traits son similares a una característica que a menudo se denomina interface
// en otros lenguajes de programación, aunque con algunas diferencias.

// El comportamiento de un tipo de datos se define como los métodos que
// podemos invocar en ese tipo. Diferentes tipos de datos comparten el mismo
// comportamiento si podemos llamar a los mismos métodos en todos esos tipos.

// Las definiciones de traits nos permiten que diferentes tipos de datos compartan
// un mismo comportamiento.


pub trait Resumen {
    // fn resumir(&self) -> String;

    fn resumir(&self) -> String {
        // String::from("Leer más....")
        format!("Leer más de {}...", self.resumir_autor())
    }

    fn resumir_autor(&self) -> String;
}

// pub struct Noticia {
//     pub titular: String,
//     pub localidad: String,
//     pub autor: String,
//     pub contenido: String,
// }

// impl Resumen for Noticia {
//     fn resumir(&self) -> String {
//         format!("{}, por  {} ({})", self.titular, self.autor, self.localidad)
//     }
// }

pub struct Tweet {
    pub usuario: String,
    pub contenido: String,
    pub respuesta: bool,
    pub retweet: bool,
}

impl Resumen for Tweet {
    // fn resumir(&self) -> String {
    //     format!("{}: {}", self.usuario, self.contenido)
    // }

    fn resumir_autor(&self) -> String {
        format!("@{}", self.usuario)
    }

}

// pub fn notificar(item: &impl Resumen) {
//     println!("Última hora: {}", item.resumir());
// }

pub fn notificar<T: Resumen>(item: &T) {
    println!("Última hora: {}", item.resumir());
}

fn main() {

    let tweet = Tweet {
        usuario: String::from("luis"),
        contenido: String::from("me encanta Rust"),
        respuesta: false,
        retweet: false,
    };
    println!("Un nuevo tweet: {}", tweet.resumir());
    
    // let noticia = Noticia {
    //     titular: String::from("El titular de esta noticia"),
    //     autor: String::from("Luis"),
    //     localidad: String::from("Granada"),
    //     contenido: String::from("Este es el contenido de esta estupenda noticia")
    // };
    // println!("Una nueva noticia: {}", String::from("aa"));
    // println!("Una nueva noticia: {}", noticia.resumir());
    
    notificar(&tweet);

}
