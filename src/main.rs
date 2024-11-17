#[derive(Debug)]

enum Media {
    Book{title: String , autor : String},
    Movie{title:String , director: String},
    AudioBook{title: String},
    Serie{title: String},
    Radio{title: String},

}

// impl utilizando Pattern Matching with Enum. 

impl Media {
    fn description (&self) -> String {
          match self {
            Media::Book {title, autor} => {
                format!("Book: {} {}",title, autor)
            },
            Media::Movie {title,director} => {
                format!("Movie: {} {}", title, director)
            },
            Media::AudioBook {title} => {
                format!("AudioBook {} ", title)
            }
            Media::Serie {title} => {
                format!("Serie: {} ", title)
            }
            Media::Radio {title} => {
                format!("Radio: {}", title)

            }
        }
    } 

   
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    //función que me sirve para (1) en main.
    //el uso de Self es generalmente preferido en la comunidad de Rust por sus ventajas en términos de mantenibilidad, claridad y consistencia 
    //con las prácticas comunes del lenguaje. 
    fn new() -> Self {
        Catalog {items: vec![]}
    }
    
    
    fn add(&mut self, media: Media){
        self.items.push(media);
    }

    //&self indica un préstamo inmutable de toda la estructura.
    //
    fn get_by_index(&self, index: usize) -> Option<&Media> {
       
        if self.items.len() > index {
          
        Some(&self.items[index])
        
        } else {
           
        None
        }
          
        
    }
// mi option enum personalizado.
}


 fn main() {
    //creamos un enum y asignamos un valor en AudioBook.
    let audiobook = Media::AudioBook { 
        title: String::from("An audiobook")
     };
     let book = Media::Book {
        title: String::from("Good Book"),
        autor: String::from("Good Autor"),
     };
     let movie = Media::Movie {
        title: String::from("Bad movie"),
        director: String::from("Bad director")
     };
     let serie= Media::Serie { title: String::from("Good serie") };

     let radio= Media::Radio {title: String::from("Good Radio")};
   
     // (1) inicializa  la impl
     let mut catalog = Catalog::new();

     // (2) aqui llamo a la fn add, lo que le envie por argumento la fn add lo va a agregar al vector self.items.
     catalog.add(audiobook);
     catalog.add(book);
     catalog.add(movie);
     catalog.add(serie);
     catalog.add(radio);
    // utilize un math para reemplazar el custom_enum with un option_enum 
     match catalog.get_by_index(4) {
        
       Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here!");
        }

     }
    // Lo mismo se puede hacer utilizando un pattern_matching en este caso if_let
     
     /* if let Some(value)= catalog.get_by_index(3242){
        println!("There is a value: {:#?}", value);
     } else {
        println!("No There is a value here!");
     } */

 }
